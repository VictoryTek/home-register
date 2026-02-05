//! User authentication and management API endpoints
//! 
//! Provides endpoints for login, registration, profile management, 
//! admin user management, and initial setup.

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder, Result, Scope};
use deadpool_postgres::Pool;
use log::{error, info, warn};
use uuid::Uuid;

use crate::auth::{
    extract_token, generate_token, hash_password, verify_password, verify_token,
    generate_reset_token, validate_password, validate_username, validate_email,
    AuthContext,
};
use crate::db::DatabaseService;
use crate::models::{
    ApiResponse, ErrorResponse, LoginRequest, LoginResponse,
    AdminCreateUserRequest, AdminUpdateUserRequest, UpdateProfileRequest,
    ChangePasswordRequest, ResetPasswordRequest, ForgotPasswordRequest,
    UserResponse, SetupStatusResponse, InitialSetupRequest,
    UpdateUserSettingsRequest, CreateInventoryShareRequest, UpdateInventoryShareRequest,
};

// ==================== Helper Functions ====================

/// Extract and verify auth context from request
async fn get_auth_context_from_request(
    req: &HttpRequest,
    pool: &Pool,
) -> Result<AuthContext, HttpResponse> {
    let token = match extract_token(req) {
        Some(t) => t,
        None => {
            return Err(HttpResponse::Unauthorized().json(ErrorResponse {
                success: false,
                error: "No authentication token provided".to_string(),
                message: Some("Please log in to access this resource".to_string()),
            }));
        }
    };

    let claims = match verify_token(&token) {
        Ok(c) => c,
        Err(e) => {
            return Err(HttpResponse::Unauthorized().json(ErrorResponse {
                success: false,
                error: format!("Invalid token: {}", e),
                message: Some("Please log in again".to_string()),
            }));
        }
    };

    let auth_ctx = match AuthContext::from_claims(&claims) {
        Ok(ctx) => ctx,
        Err(_) => {
            return Err(HttpResponse::Unauthorized().json(ErrorResponse {
                success: false,
                error: "Invalid user ID in token".to_string(),
                message: Some("Please log in again".to_string()),
            }));
        }
    };

    // Verify user still exists and is active
    let db_service = DatabaseService::new(pool.clone());
    match db_service.get_user_by_id(auth_ctx.user_id).await {
        Ok(Some(user)) => {
            if !user.is_active {
                return Err(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "Account is deactivated".to_string(),
                    message: Some("Your account has been deactivated. Contact an administrator.".to_string()),
                }));
            }
        }
        Ok(None) => {
            return Err(HttpResponse::Unauthorized().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: Some("Please log in again".to_string()),
            }));
        }
        Err(e) => {
            error!("Database error verifying user: {}", e);
            return Err(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Database error".to_string(),
                message: Some("Unable to verify user".to_string()),
            }));
        }
    }

    Ok(auth_ctx)
}

/// Require admin privileges
async fn require_admin(
    req: &HttpRequest,
    pool: &Pool,
) -> Result<AuthContext, HttpResponse> {
    let auth_ctx = get_auth_context_from_request(req, pool).await?;
    
    if !auth_ctx.is_admin {
        return Err(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Admin privileges required".to_string(),
            message: Some("You don't have permission to access this resource".to_string()),
        }));
    }
    
    Ok(auth_ctx)
}

// ==================== Public Endpoints ====================

/// Check if initial setup is needed (no users exist)
#[get("/auth/setup/status")]
pub async fn get_setup_status(pool: web::Data<Pool>) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_user_count().await {
        Ok(count) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(SetupStatusResponse {
                    needs_setup: count == 0,
                    user_count: count,
                }),
                message: None,
                error: None,
            }))
        }
        Err(e) => {
            error!("Error checking setup status: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to check setup status".to_string()),
            }))
        }
    }
}

/// Initial setup - create first admin user (only works when no users exist)
#[post("/auth/setup")]
pub async fn initial_setup(
    pool: web::Data<Pool>,
    req: web::Json<InitialSetupRequest>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Verify no users exist
    match db_service.get_user_count().await {
        Ok(count) if count > 0 => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Setup already completed".to_string(),
                message: Some("An admin user already exists".to_string()),
            }));
        }
        Err(e) => {
            error!("Error checking user count: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to check setup status".to_string()),
            }));
        }
        _ => {}
    }
    
    // Validate input
    if let Err(msg) = validate_username(&req.username) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid username".to_string()),
        }));
    }
    if let Err(msg) = validate_email(&req.email) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid email".to_string()),
        }));
    }
    if let Err(msg) = validate_password(&req.password) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid password".to_string()),
        }));
    }
    
    // Hash password
    let password_hash = match hash_password(req.password.clone()).await {
        Ok(hash) => hash,
        Err(e) => {
            error!("Error hashing password: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to hash password".to_string(),
                message: None,
            }));
        }
    };
    
    // Create admin user
    let user = match db_service.create_user(
        &req.username,
        &req.email,
        &req.full_name,
        &password_hash,
        true,  // is_admin
        true,  // is_active
    ).await {
        Ok(u) => u,
        Err(e) => {
            error!("Error creating admin user: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to create user: {}", e),
                message: None,
            }));
        }
    };
    
    // Create default settings for user
    if let Err(e) = db_service.create_user_settings(user.id).await {
        warn!("Failed to create user settings: {}", e);
    }
    
    // Optionally create first inventory
    if let Some(inventory_name) = &req.inventory_name {
        if !inventory_name.is_empty() {
            // Note: This would need to set user_id on the inventory
            // For now, we'll skip this as inventories need migration
        }
    }
    
    // Generate token
    let token = match generate_token(&user) {
        Ok(t) => t,
        Err(e) => {
            error!("Error generating token: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to generate token".to_string(),
                message: None,
            }));
        }
    };
    
    info!("Initial setup completed - created admin user: {}", user.username);
    
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(LoginResponse {
            token,
            user: user.into(),
        }),
        message: Some("Setup completed successfully".to_string()),
        error: None,
    }))
}

/// User login
#[post("/auth/login")]
pub async fn login(
    pool: web::Data<Pool>,
    req: web::Json<LoginRequest>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Find user by username or email
    let user = match db_service.get_user_by_username_or_email(&req.username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            // Don't reveal whether username exists
            return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                success: false,
                error: "Invalid credentials".to_string(),
                message: Some("Username or password is incorrect".to_string()),
            }));
        }
        Err(e) => {
            error!("Database error during login: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Database error".to_string(),
                message: None,
            }));
        }
    };
    
    // Check if user is active
    if !user.is_active {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Account deactivated".to_string(),
            message: Some("Your account has been deactivated. Contact an administrator.".to_string()),
        }));
    }
    
    // Verify password
    let password_valid = match verify_password(req.password.clone(), user.password_hash.clone()).await {
        Ok(valid) => valid,
        Err(e) => {
            error!("Error verifying password: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Password verification failed".to_string(),
                message: None,
            }));
        }
    };
    
    if !password_valid {
        return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
            success: false,
            error: "Invalid credentials".to_string(),
            message: Some("Username or password is incorrect".to_string()),
        }));
    }
    
    // Generate token
    let token = match generate_token(&user) {
        Ok(t) => t,
        Err(e) => {
            error!("Error generating token: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to generate token".to_string(),
                message: None,
            }));
        }
    };
    
    info!("User logged in: {}", user.username);
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(LoginResponse {
            token,
            user: user.into(),
        }),
        message: Some("Login successful".to_string()),
        error: None,
    }))
}

/// Register new user (public registration after initial setup)
#[post("/auth/register")]
pub async fn register(
    pool: web::Data<Pool>,
    req: web::Json<crate::models::CreateUserRequest>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Check that at least one user exists (initial setup has been done)
    match db_service.get_user_count().await {
        Ok(count) if count == 0 => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Initial setup required".to_string(),
                message: Some("Please complete the initial setup before registering users".to_string()),
            }));
        }
        Err(e) => {
            error!("Database error checking user count: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Database error".to_string(),
                message: None,
            }));
        }
        _ => {}
    }
    
    // Validate username
    if let Err(msg) = validate_username(&req.username) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid username".to_string()),
        }));
    }
    
    // Validate email
    if let Err(msg) = validate_email(&req.email) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid email".to_string()),
        }));
    }
    
    // Validate password
    if let Err(msg) = validate_password(&req.password) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid password".to_string()),
        }));
    }
    
    // Check if username already exists
    if let Ok(Some(_)) = db_service.get_user_by_username(&req.username).await {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: "Username already taken".to_string(),
            message: Some("Please choose a different username".to_string()),
        }));
    }
    
    // Check if email already exists
    if let Ok(Some(_)) = db_service.get_user_by_email(&req.email).await {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: "Email already registered".to_string(),
            message: Some("Please use a different email or log in".to_string()),
        }));
    }
    
    // Hash password
    let password_hash = match hash_password(req.password.clone()).await {
        Ok(hash) => hash,
        Err(e) => {
            error!("Error hashing password: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to process password".to_string(),
                message: None,
            }));
        }
    };
    
    // Create user (non-admin, active)
    let user = match db_service.create_user(
        &req.username,
        &req.email,
        &req.full_name,
        &password_hash,
        false, // not admin
        true,  // active
    ).await {
        Ok(u) => u,
        Err(e) => {
            error!("Error creating user: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to create account".to_string()),
            }));
        }
    };
    
    // Create default settings for the user
    if let Err(e) = db_service.create_user_settings(user.id).await {
        warn!("Failed to create user settings for {}: {}", user.username, e);
    }
    
    // Generate token for immediate login
    let token = match generate_token(&user) {
        Ok(t) => t,
        Err(e) => {
            error!("Error generating token: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Account created but login failed".to_string(),
                message: Some("Please log in manually".to_string()),
            }));
        }
    };
    
    info!("New user registered: {}", user.username);
    
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(LoginResponse {
            token,
            user: user.into(),
        }),
        message: Some("Registration successful".to_string()),
        error: None,
    }))
}

/// Request password reset (sends email if configured)
#[post("/auth/forgot-password")]
pub async fn forgot_password(
    pool: web::Data<Pool>,
    req: web::Json<ForgotPasswordRequest>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Always return success to prevent email enumeration
    let response = HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: None::<()>,
        message: Some("If an account with that email exists, a password reset link has been sent.".to_string()),
        error: None,
    });
    
    // Find user by email
    let user = match db_service.get_user_by_email(&req.email).await {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(response),
        Err(e) => {
            error!("Database error during forgot password: {}", e);
            return Ok(response);
        }
    };
    
    // Generate reset token
    let token = generate_reset_token();
    
    // Store token
    if let Err(e) = db_service.create_password_reset_token(user.id, &token).await {
        error!("Error creating reset token: {}", e);
        return Ok(response);
    }
    
    // TODO: Send email with reset link
    // For now, just log the token (in production, send email)
    info!("Password reset token generated for user {} (token: {})", user.username, token);
    
    Ok(response)
}

/// Reset password with token
#[post("/auth/reset-password")]
pub async fn reset_password(
    pool: web::Data<Pool>,
    req: web::Json<ResetPasswordRequest>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Validate new password
    if let Err(msg) = validate_password(&req.new_password) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid password".to_string()),
        }));
    }
    
    // Get user ID from token
    let user_id = match db_service.get_user_id_from_reset_token(&req.token).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid or expired token".to_string(),
                message: Some("Please request a new password reset".to_string()),
            }));
        }
        Err(e) => {
            error!("Database error checking reset token: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Database error".to_string(),
                message: None,
            }));
        }
    };
    
    // Hash new password
    let password_hash = match hash_password(req.new_password.clone()).await {
        Ok(hash) => hash,
        Err(e) => {
            error!("Error hashing password: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to hash password".to_string(),
                message: None,
            }));
        }
    };
    
    // Update password
    if let Err(e) = db_service.update_user_password(user_id, &password_hash).await {
        error!("Error updating password: {}", e);
        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: "Failed to update password".to_string(),
            message: None,
        }));
    }
    
    // Delete the used token
    let _ = db_service.delete_password_reset_token(&req.token).await;
    
    info!("Password reset successful for user {}", user_id);
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: None::<()>,
        message: Some("Password reset successful. You can now log in with your new password.".to_string()),
        error: None,
    }))
}

// ==================== Authenticated User Endpoints ====================

/// Get current user's profile
#[get("/auth/me")]
pub async fn get_current_user(
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let auth_ctx = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_user_by_id(auth_ctx.user_id).await {
        Ok(Some(user)) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(UserResponse::from(user)),
                message: None,
                error: None,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error getting user profile: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Update current user's profile
#[put("/auth/me")]
pub async fn update_current_user(
    pool: web::Data<Pool>,
    req: HttpRequest,
    body: web::Json<UpdateProfileRequest>,
) -> Result<impl Responder> {
    let auth_ctx = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    // Validate email if provided
    if let Some(ref email) = body.email {
        if let Err(msg) = validate_email(email) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: msg.to_string(),
                message: Some("Invalid email".to_string()),
            }));
        }
    }
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Check if email is already taken by another user
    if let Some(ref email) = body.email {
        if let Ok(Some(existing)) = db_service.get_user_by_email(email).await {
            if existing.id != auth_ctx.user_id {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: "Email already in use".to_string(),
                    message: Some("This email is already associated with another account".to_string()),
                }));
            }
        }
    }
    
    match db_service.update_user_profile(
        auth_ctx.user_id,
        body.email.as_deref(),
        body.full_name.as_deref(),
    ).await {
        Ok(Some(user)) => {
            info!("User {} updated their profile", auth_ctx.username);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(UserResponse::from(user)),
                message: Some("Profile updated successfully".to_string()),
                error: None,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error updating user profile: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Change current user's password
#[put("/auth/password")]
pub async fn change_password(
    pool: web::Data<Pool>,
    req: HttpRequest,
    body: web::Json<ChangePasswordRequest>,
) -> Result<impl Responder> {
    let auth_ctx = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    // Validate new password
    if let Err(msg) = validate_password(&body.new_password) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid new password".to_string()),
        }));
    }
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Get user to verify current password
    let user = match db_service.get_user_by_id(auth_ctx.user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: None,
            }));
        }
        Err(e) => {
            error!("Error getting user: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Database error".to_string(),
                message: None,
            }));
        }
    };
    
    // Verify current password
    let password_valid = match verify_password(body.current_password.clone(), user.password_hash).await {
        Ok(valid) => valid,
        Err(e) => {
            error!("Error verifying password: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Password verification failed".to_string(),
                message: None,
            }));
        }
    };
    
    if !password_valid {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Current password is incorrect".to_string(),
            message: None,
        }));
    }
    
    // Hash new password
    let password_hash = match hash_password(body.new_password.clone()).await {
        Ok(hash) => hash,
        Err(e) => {
            error!("Error hashing password: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to hash password".to_string(),
                message: None,
            }));
        }
    };
    
    // Update password
    if let Err(e) = db_service.update_user_password(auth_ctx.user_id, &password_hash).await {
        error!("Error updating password: {}", e);
        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: "Failed to update password".to_string(),
            message: None,
        }));
    }
    
    info!("User {} changed their password", auth_ctx.username);
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: None::<()>,
        message: Some("Password changed successfully".to_string()),
        error: None,
    }))
}

// ==================== User Settings Endpoints ====================

/// Get current user's settings
#[get("/auth/settings")]
pub async fn get_user_settings(
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let auth_ctx = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_or_create_user_settings(auth_ctx.user_id).await {
        Ok(settings) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(settings),
                message: None,
                error: None,
            }))
        }
        Err(e) => {
            error!("Error getting user settings: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Update current user's settings
#[put("/auth/settings")]
pub async fn update_user_settings(
    pool: web::Data<Pool>,
    req: HttpRequest,
    body: web::Json<UpdateUserSettingsRequest>,
) -> Result<impl Responder> {
    let auth_ctx = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Ensure settings exist
    let _ = db_service.get_or_create_user_settings(auth_ctx.user_id).await;
    
    match db_service.update_user_settings(auth_ctx.user_id, body.into_inner()).await {
        Ok(Some(settings)) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(settings),
                message: Some("Settings updated successfully".to_string()),
                error: None,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "Settings not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error updating user settings: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

// ==================== Admin User Management Endpoints ====================

/// Get all users (admin only)
#[get("/admin/users")]
pub async fn admin_get_users(
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let _auth_ctx = match require_admin(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_all_users().await {
        Ok(users) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(users.clone()),
                message: Some(format!("Retrieved {} users", users.len())),
                error: None,
            }))
        }
        Err(e) => {
            error!("Error getting users: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Get a specific user (admin only)
#[get("/admin/users/{id}")]
pub async fn admin_get_user(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let _auth_ctx = match require_admin(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    let user_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(UserResponse::from(user)),
                message: None,
                error: None,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error getting user: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Create a new user (admin only)
#[post("/admin/users")]
pub async fn admin_create_user(
    pool: web::Data<Pool>,
    req: HttpRequest,
    body: web::Json<AdminCreateUserRequest>,
) -> Result<impl Responder> {
    let _auth_ctx = match require_admin(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    // Validate input
    if let Err(msg) = validate_username(&body.username) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid username".to_string()),
        }));
    }
    if let Err(msg) = validate_email(&body.email) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid email".to_string()),
        }));
    }
    if let Err(msg) = validate_password(&body.password) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: msg.to_string(),
            message: Some("Invalid password".to_string()),
        }));
    }
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Check if username already exists
    if let Ok(Some(_)) = db_service.get_user_by_username(&body.username).await {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: "Username already exists".to_string(),
            message: None,
        }));
    }
    
    // Check if email already exists
    if let Ok(Some(_)) = db_service.get_user_by_email(&body.email).await {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: "Email already exists".to_string(),
            message: None,
        }));
    }
    
    // Hash password
    let password_hash = match hash_password(body.password.clone()).await {
        Ok(hash) => hash,
        Err(e) => {
            error!("Error hashing password: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to hash password".to_string(),
                message: None,
            }));
        }
    };
    
    // Create user
    match db_service.create_user(
        &body.username,
        &body.email,
        &body.full_name,
        &password_hash,
        body.is_admin,
        body.is_active,
    ).await {
        Ok(user) => {
            // Create default settings
            let _ = db_service.create_user_settings(user.id).await;
            
            info!("Admin created new user: {}", user.username);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(UserResponse::from(user)),
                message: Some("User created successfully".to_string()),
                error: None,
            }))
        }
        Err(e) => {
            error!("Error creating user: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to create user: {}", e),
                message: None,
            }))
        }
    }
}

/// Update a user (admin only)
#[put("/admin/users/{id}")]
pub async fn admin_update_user(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<AdminUpdateUserRequest>,
) -> Result<impl Responder> {
    let auth_ctx = match require_admin(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    let user_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Validate username if provided
    if let Some(ref username) = body.username {
        if let Err(msg) = validate_username(username) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: msg.to_string(),
                message: Some("Invalid username".to_string()),
            }));
        }
    }
    
    // Validate email if provided
    if let Some(ref email) = body.email {
        if let Err(msg) = validate_email(email) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: msg.to_string(),
                message: Some("Invalid email".to_string()),
            }));
        }
    }
    
    // Prevent admin from demoting themselves
    if user_id == auth_ctx.user_id {
        if let Some(false) = body.is_admin {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Cannot remove your own admin privileges".to_string(),
                message: None,
            }));
        }
        if let Some(false) = body.is_active {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Cannot deactivate your own account".to_string(),
                message: None,
            }));
        }
    }
    
    // Protect last admin
    if let Some(false) = body.is_admin {
        let admin_count = db_service.count_admin_users().await.unwrap_or(0);
        if admin_count <= 1 {
            let target_user = db_service.get_user_by_id(user_id).await.ok().flatten();
            if target_user.map(|u| u.is_admin).unwrap_or(false) {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: "Cannot remove admin privileges from the last admin".to_string(),
                    message: None,
                }));
            }
        }
    }
    
    match db_service.admin_update_user(user_id, body.into_inner()).await {
        Ok(Some(user)) => {
            info!("Admin updated user: {}", user.username);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(UserResponse::from(user)),
                message: Some("User updated successfully".to_string()),
                error: None,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error updating user: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to update user: {}", e),
                message: None,
            }))
        }
    }
}

/// Delete a user (admin only)
#[delete("/admin/users/{id}")]
pub async fn admin_delete_user(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let auth_ctx = match require_admin(&req, pool.get_ref()).await {
        Ok(ctx) => ctx,
        Err(response) => return Ok(response),
    };
    
    let user_id = path.into_inner();
    
    // Prevent admin from deleting themselves
    if user_id == auth_ctx.user_id {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Cannot delete your own account".to_string(),
            message: None,
        }));
    }
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Check if this is the last admin
    let target_user = match db_service.get_user_by_id(user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: None,
            }));
        }
        Err(e) => {
            error!("Error getting user: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Database error".to_string(),
                message: None,
            }));
        }
    };
    
    if target_user.is_admin {
        let admin_count = db_service.count_admin_users().await.unwrap_or(0);
        if admin_count <= 1 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Cannot delete the last admin".to_string(),
                message: None,
            }));
        }
    }
    
    match db_service.delete_user(user_id).await {
        Ok(true) => {
            info!("Admin deleted user: {} (ID: {})", target_user.username, user_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: None::<()>,
                message: Some("User deleted successfully".to_string()),
                error: None,
            }))
        }
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error deleting user: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to delete user: {}", e),
                message: None,
            }))
        }
    }
}

// ==================== Inventory Sharing Endpoints ====================

/// Get shares for an inventory
#[get("/inventories/{id}/shares")]
pub async fn get_inventory_shares(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let auth = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };
    
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Check if user has permission to view shares (must be owner or have full access)
    let permission = match db_service.get_user_permission_for_inventory(auth.user_id, inventory_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                success: false,
                error: "Access denied".to_string(),
                message: Some("You don't have access to this inventory".to_string()),
            }));
        }
        Err(e) => {
            error!("Error checking permission: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }));
        }
    };
    
    if !permission.can_manage_sharing() && !auth.is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Insufficient permissions".to_string(),
            message: Some("You need full access to view shares".to_string()),
        }));
    }
    
    match db_service.get_inventory_shares(inventory_id).await {
        Ok(shares) => {
            info!("Retrieved {} shares for inventory {}", shares.len(), inventory_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(shares),
                message: Some("Shares retrieved successfully".to_string()),
                error: None,
            }))
        }
        Err(e) => {
            error!("Error retrieving shares: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Share an inventory with another user
#[post("/inventories/{id}/shares")]
pub async fn create_inventory_share(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<CreateInventoryShareRequest>,
) -> Result<impl Responder> {
    let auth = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };
    
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Check if user has permission to share (must be owner or have full access)
    let permission = match db_service.get_user_permission_for_inventory(auth.user_id, inventory_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                success: false,
                error: "Access denied".to_string(),
                message: Some("You don't have access to this inventory".to_string()),
            }));
        }
        Err(e) => {
            error!("Error checking permission: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }));
        }
    };
    
    if !permission.can_manage_sharing() && !auth.is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Insufficient permissions".to_string(),
            message: Some("You need full access to share this inventory".to_string()),
        }));
    }
    
    // Find the user to share with
    let target_user = match db_service.get_user_by_username_or_email(&body.shared_with_username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "User not found".to_string(),
                message: Some(format!("No user found with username or email: {}", body.shared_with_username)),
            }));
        }
        Err(e) => {
            error!("Error finding user: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }));
        }
    };
    
    // Don't allow sharing with self
    if target_user.id == auth.user_id {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Cannot share with yourself".to_string(),
            message: None,
        }));
    }
    
    match db_service.create_inventory_share(
        inventory_id,
        target_user.id,
        auth.user_id,
        body.permission_level,
    ).await {
        Ok(share) => {
            info!("User {} shared inventory {} with {} (permission: {:?})", 
                auth.username, inventory_id, target_user.username, body.permission_level);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(share),
                message: Some(format!("Inventory shared with {}", target_user.username)),
                error: None,
            }))
        }
        Err(e) => {
            // Check for duplicate share
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: "Already shared".to_string(),
                    message: Some(format!("This inventory is already shared with {}", target_user.username)),
                }));
            }
            error!("Error creating share: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Update share permission level
#[put("/shares/{share_id}")]
pub async fn update_inventory_share(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateInventoryShareRequest>,
) -> Result<impl Responder> {
    let _auth = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };
    
    let share_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Update the share permission
    match db_service.update_inventory_share(share_id, body.permission_level).await {
        Ok(Some(share)) => {
            info!("Updated share {} permission to {:?}", share_id, body.permission_level);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(share),
                message: Some("Share permission updated".to_string()),
                error: None,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "Share not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error updating share: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Remove a share
#[delete("/shares/{share_id}")]
pub async fn delete_inventory_share(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let auth = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };
    
    let share_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.delete_inventory_share(share_id).await {
        Ok(true) => {
            info!("User {} deleted share {}", auth.username, share_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: None::<()>,
                message: Some("Share removed successfully".to_string()),
                error: None,
            }))
        }
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "Share not found".to_string(),
                message: None,
            }))
        }
        Err(e) => {
            error!("Error deleting share: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Get inventories accessible to the current user (owned + shared)
#[get("/auth/inventories")]
pub async fn get_my_inventories(
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let auth = match get_auth_context_from_request(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_accessible_inventories(auth.user_id).await {
        Ok(inventories) => {
            info!("User {} retrieved {} accessible inventories", auth.username, inventories.len());
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(inventories),
                message: None,
                error: None,
            }))
        }
        Err(e) => {
            error!("Error retrieving inventories: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: None,
            }))
        }
    }
}

/// Create scope with all auth-related routes
pub fn auth_scope() -> Scope {
    web::scope("")
        // Public endpoints
        .service(get_setup_status)
        .service(initial_setup)
        .service(login)
        .service(register)
        .service(forgot_password)
        .service(reset_password)
        // Authenticated user endpoints
        .service(get_current_user)
        .service(update_current_user)
        .service(change_password)
        .service(get_user_settings)
        .service(update_user_settings)
        .service(get_my_inventories)
        // Inventory sharing endpoints
        .service(get_inventory_shares)
        .service(create_inventory_share)
        .service(update_inventory_share)
        .service(delete_inventory_share)
        // Admin endpoints
        .service(admin_get_users)
        .service(admin_get_user)
        .service(admin_create_user)
        .service(admin_update_user)
        .service(admin_delete_user)
}

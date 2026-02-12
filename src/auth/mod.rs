//! Authentication and authorization module
//!
//! Provides JWT token handling, password hashing with Argon2, and auth middleware for Actix-Web.

use actix_web::HttpRequest;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use std::sync::OnceLock;
use uuid::Uuid;

use crate::models::{Claims, User};

// ==================== JWT Secret Management ====================

/// Global JWT secret cache - initialized once at startup
static JWT_SECRET: OnceLock<String> = OnceLock::new();

/// Initialize and get JWT secret
/// Tries multiple sources in order:
/// 1. Docker secret file (/run/secrets/jwt_secret)
/// 2. Custom path via JWT_SECRET_FILE env var
/// 3. JWT_SECRET environment variable
/// 4. Auto-generated secret persisted to /app/data/jwt_secret
/// 5. Fallback to auto-generated (not persisted, will change on restart)
pub fn get_or_init_jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| {
        // Try to read existing secret from various sources
        if let Some(secret) = read_jwt_secret() {
            if secret.len() >= 32 {
                log::info!("Using existing JWT secret");
                return secret;
            } else {
                log::warn!(
                    "JWT_SECRET must be at least 32 characters for cryptographic security. \
                     Current length: {}. Generate a secure secret with: openssl rand -base64 32",
                    secret.len()
                );
            }
        }

        // No valid secret found - auto-generate and try to persist
        log::warn!("No JWT_SECRET found. Auto-generating a random secret.");

        let secret = generate_random_secret(64);

        // Try to persist to /app/data/jwt_secret for container restarts
        let persist_path = "/app/data/jwt_secret";
        if let Err(e) = std::fs::create_dir_all("/app/data") {
            log::debug!("Could not create /app/data directory: {}", e);
        }

        if let Err(e) = std::fs::write(persist_path, &secret) {
            log::warn!(
                "Failed to persist auto-generated JWT secret to {}: {}. \
                 Tokens will be invalidated on restart.",
                persist_path,
                e
            );
        } else {
            log::info!("Auto-generated JWT secret persisted to {}", persist_path);
        }

        secret
    })
}

/// Read JWT secret from various sources
fn read_jwt_secret() -> Option<String> {
    // 1. Try custom path from JWT_SECRET_FILE env var
    if let Ok(custom_path) = env::var("JWT_SECRET_FILE") {
        if let Ok(content) = std::fs::read_to_string(&custom_path) {
            let secret = content.trim().to_string();
            if !secret.is_empty() {
                log::info!("Read JWT secret from custom path: {}", custom_path);
                return Some(secret);
            }
        }
    }

    // 2. Try Docker secret
    if let Ok(content) = std::fs::read_to_string("/run/secrets/jwt_secret") {
        let secret = content.trim().to_string();
        if !secret.is_empty() {
            log::info!("Read JWT secret from Docker secrets");
            return Some(secret);
        }
    }

    // 3. Try persisted auto-generated secret
    if let Ok(content) = std::fs::read_to_string("/app/data/jwt_secret") {
        let secret = content.trim().to_string();
        if !secret.is_empty() {
            log::info!("Read JWT secret from persisted file");
            return Some(secret);
        }
    }

    // 4. Try environment variable
    if let Ok(secret) = env::var("JWT_SECRET") {
        if !secret.is_empty() {
            log::info!("Read JWT secret from environment variable");
            return Some(secret);
        }
    }

    None
}

/// Generate a cryptographically secure random string
fn generate_random_secret(length: usize) -> String {
    use rand::Rng;
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

// ==================== JWT Token Handling ====================

/// Get JWT secret - wrapper for the cached secret
pub fn jwt_secret() -> String {
    get_or_init_jwt_secret().to_string()
}

/// Get JWT token lifetime in hours from environment
pub fn jwt_token_lifetime_hours() -> i64 {
    env::var("JWT_TOKEN_LIFETIME_HOURS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(24) // Default to 24 hours
}

/// Generate a JWT token for a user
pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let token_lifetime_hours = jwt_token_lifetime_hours();
    let expiration = (now + chrono::Duration::hours(token_lifetime_hours)).timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        is_admin: user.is_admin,
        exp: expiration,
        iat: now.timestamp() as usize,
    };

    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

/// Verify and decode a JWT token
pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(jwt_secret().as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["sub", "exp", "iat"]);

    decode::<Claims>(token, &key, &validation).map(|data| data.claims)
}

/// Extract JWT token from Authorization header or auth_token cookie
pub fn extract_token(req: &HttpRequest) -> Option<String> {
    // Try Authorization header first (Bearer token)
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    // Fall back to cookie
    if let Some(cookie) = req.cookie("auth_token") {
        return Some(cookie.value().to_string());
    }

    None
}

// ==================== Password Hashing ====================

/// Hash a password using Argon2id
/// Uses spawn_blocking to avoid blocking the async runtime
pub async fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    })
    .await
    .map_err(|_| argon2::password_hash::Error::Algorithm)?
}

/// Verify a password against a hash
/// Uses spawn_blocking to avoid blocking the async runtime
pub async fn verify_password(
    password: String,
    hash_str: String,
) -> Result<bool, argon2::password_hash::Error> {
    tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash_str)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    })
    .await
    .map_err(|_| argon2::password_hash::Error::Algorithm)?
}

// ==================== Auth Context ====================

/// Authentication context passed to handlers via request extensions
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

impl AuthContext {
    pub fn from_claims(claims: &Claims) -> Result<Self, uuid::Error> {
        Ok(Self {
            user_id: Uuid::parse_str(&claims.sub)?,
            username: claims.username.clone(),
            is_admin: claims.is_admin,
        })
    }
}

// ==================== Helper Functions ====================

/// Validate password complexity
pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long");
    }
    if password.len() > 128 {
        return Err("Password must be at most 128 characters long");
    }
    // Could add more complexity requirements here
    Ok(())
}

/// Validate username format
pub fn validate_username(username: &str) -> Result<(), &'static str> {
    if username.len() < 3 {
        return Err("Username must be at least 3 characters long");
    }
    if username.len() > 50 {
        return Err("Username must be at most 50 characters long");
    }
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err("Username can only contain letters, numbers, underscores, and hyphens");
    }
    Ok(())
}

// ==================== Testing Helpers ====================

/// Synchronous password hashing for tests (do not use in async contexts)
#[cfg(test)]
pub fn hash_password_sync(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Synchronous password verification for tests (do not use in async contexts)
#[cfg(test)]
pub fn verify_password_sync(password: &str, hash_str: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash_str)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Alias for generate_token to match common naming convention
pub fn create_token(user_id: &Uuid, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let user = User {
        id: *user_id,
        username: username.to_string(),
        password_hash: String::new(), // Not used for token generation
        is_admin: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    generate_token(&user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        assert!(validate_password("short").is_err());
        assert!(validate_password("validpassword123").is_ok());
    }

    #[test]
    fn test_username_validation() {
        assert!(validate_username("ab").is_err());
        assert!(validate_username("valid_user-123").is_ok());
        assert!(validate_username("invalid user").is_err());
    }
}

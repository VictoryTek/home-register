use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: Option<i32>,
    pub inventory_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub purchase_price: Option<f64>,
    pub warranty_expiry: Option<String>,
    pub notes: Option<String>,
    pub quantity: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct CreateInventoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateInventoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateItemRequest {
    pub inventory_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub purchase_price: Option<f64>,
    pub warranty_expiry: Option<String>,
    pub notes: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub purchase_price: Option<f64>,
    pub warranty_expiry: Option<String>,
    pub notes: Option<String>,
    pub quantity: Option<i32>,
    pub inventory_id: Option<i32>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub message: Option<String>,
}

// Categories
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

// Tags
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
    pub color: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

// Custom Fields
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomField {
    pub id: Option<i32>,
    pub category_id: i32,
    pub name: String,
    pub field_type: String,
    pub options: Option<String>, // JSON string
    pub required: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomFieldValue {
    pub id: Option<i32>,
    pub item_id: i32,
    pub custom_field_id: i32,
    pub value: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Extended Item structure with relationships
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemWithRelations {
    pub id: Option<i32>,
    pub inventory_id: i32,
    pub category_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub purchase_price: Option<f64>,
    pub warranty_expiry: Option<String>,
    pub notes: Option<String>,
    pub quantity: Option<i32>,
    pub image_url: Option<String>,
    pub purchase_link: Option<String>,
    pub warranty_info: Option<String>,
    pub condition: Option<String>,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    // Relations
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
    pub custom_fields: Vec<CustomFieldWithValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomFieldWithValue {
    pub field: CustomField,
    pub value: Option<String>,
}

// Organizer Types - flexible categorization per inventory
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizerType {
    pub id: Option<i32>,
    pub inventory_id: i32,
    pub name: String,
    pub input_type: String, // "select" or "text"
    pub is_required: bool,
    pub display_order: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct CreateOrganizerTypeRequest {
    pub name: String,
    pub input_type: Option<String>, // defaults to "select"
    pub is_required: Option<bool>,  // defaults to false
    pub display_order: Option<i32>, // defaults to 0
}

#[derive(Deserialize, Debug)]
pub struct UpdateOrganizerTypeRequest {
    pub name: Option<String>,
    pub input_type: Option<String>,
    pub is_required: Option<bool>,
    pub display_order: Option<i32>,
}

// Organizer Options - predefined values for "select" type organizers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizerOption {
    pub id: Option<i32>,
    pub organizer_type_id: i32,
    pub name: String,
    pub display_order: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct CreateOrganizerOptionRequest {
    pub name: String,
    pub display_order: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateOrganizerOptionRequest {
    pub name: Option<String>,
    pub display_order: Option<i32>,
}

// Item Organizer Values - links items to organizer values
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemOrganizerValue {
    pub id: Option<i32>,
    pub item_id: i32,
    pub organizer_type_id: i32,
    pub organizer_option_id: Option<i32>, // For "select" type
    pub text_value: Option<String>,       // For "text" type
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct SetItemOrganizerValueRequest {
    pub organizer_type_id: i32,
    pub organizer_option_id: Option<i32>, // For "select" type
    pub text_value: Option<String>,       // For "text" type
}

#[derive(Deserialize, Debug)]
pub struct SetItemOrganizerValuesRequest {
    pub values: Vec<SetItemOrganizerValueRequest>,
}

// Extended response with organizer details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizerTypeWithOptions {
    #[serde(flatten)]
    pub organizer_type: OrganizerType,
    pub options: Vec<OrganizerOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemOrganizerValueWithDetails {
    pub organizer_type_id: i32,
    pub organizer_type_name: String,
    pub input_type: String,
    pub is_required: bool,
    pub value: Option<String>,         // Display value (option name or text value)
    pub organizer_option_id: Option<i32>,
    pub text_value: Option<String>,
}

// ==================== User & Authentication Models ====================

use uuid::Uuid;

/// User model - represents a user in the system
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    #[serde(skip_serializing)]  // Never serialize password_hash
    pub password_hash: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User response without sensitive data (for API responses)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            is_admin: user.is_admin,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Request to create a new user (registration)
#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub password: String,
}

/// Request for admin to create a new user with additional options
#[derive(Deserialize, Debug)]
pub struct AdminCreateUserRequest {
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub password: String,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default = "default_true")]
    pub is_active: bool,
}

fn default_true() -> bool {
    true
}

/// Request for admin to update a user
#[derive(Deserialize, Debug)]
pub struct AdminUpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub is_admin: Option<bool>,
    pub is_active: Option<bool>,
}

/// Login request
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response with JWT token
#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

/// Request to update current user's profile
#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub email: Option<String>,
    pub full_name: Option<String>,
}

/// Request to change password
#[derive(Deserialize, Debug)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Request to reset password with token
#[derive(Deserialize, Debug)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

/// Request to initiate password reset
#[derive(Deserialize, Debug)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // User ID
    pub username: String,
    pub is_admin: bool,
    pub exp: usize,        // Expiration time
    pub iat: usize,        // Issued at
}

// ==================== Permission Models ====================

/// Permission levels for shared inventories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PermissionLevel {
    View,   // Can only view inventory and items
    Edit,   // Can view, add, and edit items
    Full,   // Full access: view, add, edit, delete items and manage sharing
}

impl PermissionLevel {
    pub fn can_view(&self) -> bool {
        true // All levels can view
    }

    pub fn can_edit(&self) -> bool {
        matches!(self, PermissionLevel::Edit | PermissionLevel::Full)
    }

    pub fn can_delete(&self) -> bool {
        matches!(self, PermissionLevel::Full)
    }

    pub fn can_manage_sharing(&self) -> bool {
        matches!(self, PermissionLevel::Full)
    }
}

impl std::fmt::Display for PermissionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionLevel::View => write!(f, "view"),
            PermissionLevel::Edit => write!(f, "edit"),
            PermissionLevel::Full => write!(f, "full"),
        }
    }
}

impl std::str::FromStr for PermissionLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "view" => Ok(PermissionLevel::View),
            "edit" => Ok(PermissionLevel::Edit),
            "full" => Ok(PermissionLevel::Full),
            _ => Err(format!("Invalid permission level: {}", s)),
        }
    }
}

/// Inventory share record
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryShare {
    pub id: Uuid,
    pub inventory_id: i32,
    pub shared_with_user_id: Uuid,
    pub shared_by_user_id: Uuid,
    pub permission_level: PermissionLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Inventory share with user details for API responses
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryShareWithUser {
    pub id: Uuid,
    pub inventory_id: i32,
    pub shared_with_user: UserResponse,
    pub shared_by_user: UserResponse,
    pub permission_level: PermissionLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to share an inventory
#[derive(Deserialize, Debug)]
pub struct CreateInventoryShareRequest {
    pub shared_with_username: String,  // Username or email of user to share with
    pub permission_level: PermissionLevel,
}

/// Request to update share permissions
#[derive(Deserialize, Debug)]
pub struct UpdateInventoryShareRequest {
    pub permission_level: PermissionLevel,
}

// ==================== User Settings Models ====================

/// User settings/preferences
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub theme: String,
    pub default_inventory_id: Option<i32>,
    pub items_per_page: i32,
    pub date_format: String,
    pub currency: String,
    pub notifications_enabled: bool,
    pub settings_json: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to update user settings
#[derive(Deserialize, Debug)]
pub struct UpdateUserSettingsRequest {
    pub theme: Option<String>,
    pub default_inventory_id: Option<i32>,
    pub items_per_page: Option<i32>,
    pub date_format: Option<String>,
    pub currency: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub settings_json: Option<serde_json::Value>,
}

// ==================== First-time Setup Models ====================

/// Request for initial admin setup (first run)
#[derive(Deserialize, Debug)]
pub struct InitialSetupRequest {
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub password: String,
    pub inventory_name: Option<String>,  // Optional first inventory name
}

/// Response for setup status check
#[derive(Serialize, Debug)]
pub struct SetupStatusResponse {
    pub needs_setup: bool,
    pub user_count: i64,
}

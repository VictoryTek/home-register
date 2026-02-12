use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub user_id: Option<uuid::Uuid>,
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

#[derive(Deserialize, Debug, Validate)]
pub struct CreateInventoryRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: String,
    #[validate(length(max = 5000, message = "Description must be under 5000 characters"))]
    pub description: Option<String>,
    #[validate(length(max = 500, message = "Location must be under 500 characters"))]
    pub location: Option<String>,
    #[validate(length(max = 2000, message = "Image URL must be under 2000 characters"))]
    pub image_url: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UpdateInventoryRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 5000, message = "Description must be under 5000 characters"))]
    pub description: Option<String>,
    #[validate(length(max = 500, message = "Location must be under 500 characters"))]
    pub location: Option<String>,
    #[validate(length(max = 2000, message = "Image URL must be under 2000 characters"))]
    pub image_url: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct CreateItemRequest {
    pub inventory_id: Option<i32>,
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: String,
    #[validate(length(max = 5000, message = "Description must be under 5000 characters"))]
    pub description: Option<String>,
    #[validate(length(max = 255, message = "Category must be under 255 characters"))]
    pub category: Option<String>,
    #[validate(length(max = 500, message = "Location must be under 500 characters"))]
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    #[validate(range(
        min = 0.0,
        max = 1_000_000_000.0,
        message = "Price must be between 0 and 1 billion"
    ))]
    pub purchase_price: Option<f64>,
    pub warranty_expiry: Option<String>,
    #[validate(length(max = 10000, message = "Notes must be under 10000 characters"))]
    pub notes: Option<String>,
    #[validate(range(
        min = 0,
        max = 1_000_000,
        message = "Quantity must be between 0 and 1 million"
    ))]
    pub quantity: Option<i32>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UpdateItemRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 5000, message = "Description must be under 5000 characters"))]
    pub description: Option<String>,
    #[validate(length(max = 255, message = "Category must be under 255 characters"))]
    pub category: Option<String>,
    #[validate(length(max = 500, message = "Location must be under 500 characters"))]
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    #[validate(range(
        min = 0.0,
        max = 1_000_000_000.0,
        message = "Price must be between 0 and 1 billion"
    ))]
    pub purchase_price: Option<f64>,
    pub warranty_expiry: Option<String>,
    #[validate(length(max = 10000, message = "Notes must be under 10000 characters"))]
    pub notes: Option<String>,
    #[validate(range(
        min = 0,
        max = 1_000_000,
        message = "Quantity must be between 0 and 1 million"
    ))]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    pub value: Option<String>, // Display value (option name or text value)
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
    pub full_name: String,
    #[serde(skip_serializing)] // Never serialize password_hash
    pub password_hash: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_codes_generated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub recovery_codes_confirmed: bool,
}

/// User response without sensitive data (for API responses)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
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
    pub full_name: String,
    pub password: String,
}

/// Request for admin to create a new user with additional options
#[derive(Deserialize, Debug)]
pub struct AdminCreateUserRequest {
    pub username: String,
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
    pub full_name: Option<String>,
}

/// Request to change password
#[derive(Deserialize, Debug)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Request to reset password with token
/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub is_admin: bool,
    pub exp: u64, // Expiration time (Unix timestamp)
    pub iat: u64, // Issued at (Unix timestamp)
}

// ==================== Permission Models ====================

/// Permission levels for shared inventories (per-inventory)
/// The 4-tier system:
/// 1. View - View shared inventory and its items
/// 2. `EditItems` - View + Edit item details only (not add/remove)
/// 3. `EditInventory` - `EditItems` + Edit inventory details, add/remove items
/// 4. `AllAccess` - User-to-user grant via `UserAccessGrant` table (full access to ALL grantor's inventories)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
    View,          // Can only view inventory and items
    EditItems,     // Can view and edit item details (not add/remove items)
    EditInventory, // Can view, edit items, add/remove items, edit inventory details
}

impl PermissionLevel {
    /// Can view inventory and items
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_view(&self) -> bool {
        true // All levels can view
    }

    /// Can edit existing item details (name, description, etc.)
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_edit_items(&self) -> bool {
        matches!(
            self,
            PermissionLevel::EditItems | PermissionLevel::EditInventory
        )
    }

    /// Can add new items to inventory
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_add_items(&self) -> bool {
        matches!(self, PermissionLevel::EditInventory)
    }

    /// Can remove items from inventory
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_remove_items(&self) -> bool {
        matches!(self, PermissionLevel::EditInventory)
    }

    /// Can edit inventory details (name, description, etc.)
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_edit_inventory(&self) -> bool {
        matches!(self, PermissionLevel::EditInventory)
    }

    /// Can manage organizers for inventory
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_manage_organizers(&self) -> bool {
        matches!(self, PermissionLevel::EditInventory)
    }

    // Legacy method - maps to can_edit_items for backward compatibility
    #[deprecated(note = "Use can_edit_items() instead")]
    #[allow(dead_code)]
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_edit(&self) -> bool {
        self.can_edit_items()
    }

    // Legacy method - only owner or AllAccess users can delete inventory
    #[deprecated(note = "Deletion requires ownership or AllAccess grant")]
    #[allow(dead_code)]
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_delete(&self) -> bool {
        false // Per-inventory shares cannot delete - requires ownership or AllAccess
    }

    // Legacy method - only owner or AllAccess users can manage sharing
    #[deprecated(note = "Sharing management requires ownership or AllAccess grant")]
    #[allow(dead_code)]
    #[must_use = "permission check result should be used to enforce access control"]
    pub fn can_manage_sharing(&self) -> bool {
        false // Per-inventory shares cannot manage sharing - requires ownership or AllAccess
    }
}

impl std::fmt::Display for PermissionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionLevel::View => write!(f, "view"),
            PermissionLevel::EditItems => write!(f, "edit_items"),
            PermissionLevel::EditInventory => write!(f, "edit_inventory"),
        }
    }
}

impl std::str::FromStr for PermissionLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "view" => Ok(PermissionLevel::View),
            "edit_items" | "edit" => Ok(PermissionLevel::EditItems),
            "edit_inventory" | "full" => Ok(PermissionLevel::EditInventory),
            _ => Err(format!("Invalid permission level: {s}")),
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
    pub shared_with_username: String, // Username of user to share with
    pub permission_level: PermissionLevel,
}

/// Request to update share permissions
#[derive(Deserialize, Debug)]
pub struct UpdateInventoryShareRequest {
    pub permission_level: PermissionLevel,
}

// ==================== User Access Grant Models (All Access Tier) ====================

/// User access grant - grants a user full access to ALL inventories of another user
/// This is the "All Access" tier of the 4-tier permission system
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccessGrant {
    pub id: Uuid,
    pub grantor_user_id: Uuid, // User granting access
    pub grantee_user_id: Uuid, // User receiving access
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User access grant with user details for API responses
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccessGrantWithUsers {
    pub id: Uuid,
    pub grantor: UserResponse,
    pub grantee: UserResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a user access grant (All Access)
#[derive(Deserialize, Debug)]
pub struct CreateUserAccessGrantRequest {
    pub grantee_username: String, // Username of user to grant access to
}

// ==================== Ownership Transfer Models ====================

/// Request to transfer inventory ownership to another user
#[derive(Deserialize, Debug)]
pub struct TransferOwnershipRequest {
    pub new_owner_username: String, // Username of user to transfer ownership to
}

/// Response for ownership transfer operation
#[derive(Serialize, Debug)]
pub struct TransferOwnershipResponse {
    pub inventory_id: i32,
    pub inventory_name: String,
    pub previous_owner: UserResponse,
    pub new_owner: UserResponse,
    pub items_transferred: i64,
    pub shares_removed: i64,
}

/// Summary of effective permissions a user has for an inventory.
/// This is a data transfer object (DTO) for API responses.
#[allow(clippy::struct_excessive_bools, reason = "DTO for API responses where explicit booleans improve clarity")]
#[derive(Serialize, Debug, Clone)]
pub struct EffectivePermissions {
    pub can_view: bool,
    pub can_edit_items: bool,
    pub can_add_items: bool,
    pub can_remove_items: bool,
    pub can_edit_inventory: bool,
    pub can_delete_inventory: bool,
    pub can_manage_sharing: bool,
    pub can_manage_organizers: bool,
    pub is_owner: bool,
    pub has_all_access: bool, // Via UserAccessGrant
    pub permission_source: PermissionSource,
}

/// Where the user's permissions come from
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PermissionSource {
    Owner,          // User owns the inventory
    AllAccess,      // Via UserAccessGrant from owner
    InventoryShare, // Via InventoryShare record
    None,           // No access
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
    pub full_name: String,
    pub password: String,
    pub inventory_name: Option<String>, // Optional first inventory name
}

/// Response for setup status check
#[derive(Serialize, Debug)]
pub struct SetupStatusResponse {
    pub needs_setup: bool,
    pub user_count: i64,
}

// ==================== Recovery Codes Models ====================

/// Recovery code stored in database (hashed)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RecoveryCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code_hash: String,
    pub is_used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Response when generating recovery codes (contains plaintext codes - only shown once!)
#[derive(Serialize, Debug)]
pub struct RecoveryCodesResponse {
    pub codes: Vec<String>,
    pub generated_at: DateTime<Utc>,
    pub message: String,
}

/// Request to confirm recovery codes have been saved
#[derive(Deserialize, Debug)]
pub struct ConfirmRecoveryCodesRequest {
    pub confirmed: bool,
}

/// Request to use a recovery code to reset password
#[derive(Deserialize, Debug)]
pub struct UseRecoveryCodeRequest {
    pub username: String,
    pub recovery_code: String,
    pub new_password: String,
}

/// Response after successfully using a recovery code
#[derive(Serialize, Debug)]
pub struct RecoveryCodeUsedResponse {
    pub success: bool,
    pub message: String,
    pub remaining_codes: i32,
}

/// Status of user's recovery codes
#[derive(Serialize, Debug)]
pub struct RecoveryCodesStatus {
    pub has_codes: bool,
    pub codes_confirmed: bool,
    pub unused_count: i32,
    pub generated_at: Option<DateTime<Utc>>,
}

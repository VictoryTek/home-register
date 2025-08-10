use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
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

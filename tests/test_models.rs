// Tests for model validation

use home_registry::models::{
    CreateInventoryRequest, CreateItemRequest, UpdateInventoryRequest, UpdateItemRequest,
};
use validator::Validate;

#[test]
fn test_create_inventory_validation() {
    // Valid inventory
    let valid = CreateInventoryRequest {
        name: "Test Inventory".to_string(),
        description: Some("A test inventory".to_string()),
        location: Some("Home".to_string()),
        image_url: None,
    };
    assert!(valid.validate().is_ok());

    // Invalid: empty name
    let invalid = CreateInventoryRequest {
        name: "".to_string(),
        description: None,
        location: None,
        image_url: None,
    };
    assert!(invalid.validate().is_err());

    // Invalid: name too long
    let invalid = CreateInventoryRequest {
        name: "a".repeat(256),
        description: None,
        location: None,
        image_url: None,
    };
    assert!(invalid.validate().is_err());

    // Invalid: description too long
    let invalid = CreateInventoryRequest {
        name: "Valid Name".to_string(),
        description: Some("x".repeat(5001)),
        location: None,
        image_url: None,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_create_item_validation() {
    // Valid item
    let valid = CreateItemRequest {
        inventory_id: Some(1),
        name: "Test Item".to_string(),
        description: Some("Test description".to_string()),
        category: Some("Electronics".to_string()),
        location: Some("Shelf A".to_string()),
        purchase_date: Some("2024-01-01".to_string()),
        purchase_price: Some(99.99),
        warranty_expiry: Some("2025-01-01".to_string()),
        notes: Some("Test notes".to_string()),
        quantity: Some(1),
    };
    assert!(valid.validate().is_ok());

    // Invalid: empty name
    let invalid = CreateItemRequest {
        inventory_id: Some(1),
        name: "".to_string(),
        description: None,
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: None,
        warranty_expiry: None,
        notes: None,
        quantity: None,
    };
    assert!(invalid.validate().is_err());

    // Invalid: price negative
    let invalid = CreateItemRequest {
        inventory_id: Some(1),
        name: "Valid Name".to_string(),
        description: None,
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: Some(-10.0),
        warranty_expiry: None,
        notes: None,
        quantity: None,
    };
    assert!(invalid.validate().is_err());

    // Invalid: price too high
    let invalid = CreateItemRequest {
        inventory_id: Some(1),
        name: "Valid Name".to_string(),
        description: None,
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: Some(2_000_000_000.0),
        warranty_expiry: None,
        notes: None,
        quantity: None,
    };
    assert!(invalid.validate().is_err());

    // Invalid: quantity negative
    let invalid = CreateItemRequest {
        inventory_id: Some(1),
        name: "Valid Name".to_string(),
        description: None,
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: None,
        warranty_expiry: None,
        notes: None,
        quantity: Some(-1),
    };
    assert!(invalid.validate().is_err());

    // Invalid: quantity too high
    let invalid = CreateItemRequest {
        inventory_id: Some(1),
        name: "Valid Name".to_string(),
        description: None,
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: None,
        warranty_expiry: None,
        notes: None,
        quantity: Some(2_000_000),
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_update_inventory_validation() {
    // Valid update
    let valid = UpdateInventoryRequest {
        name: Some("Updated Name".to_string()),
        description: Some("Updated description".to_string()),
        location: Some("New location".to_string()),
        image_url: Some("https://example.com/image.jpg".to_string()),
    };
    assert!(valid.validate().is_ok());

    // Valid: all None (no changes)
    let valid = UpdateInventoryRequest {
        name: None,
        description: None,
        location: None,
        image_url: None,
    };
    assert!(valid.validate().is_ok());

    // Invalid: empty name
    let invalid = UpdateInventoryRequest {
        name: Some("".to_string()),
        description: None,
        location: None,
        image_url: None,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_update_item_validation() {
    // Valid update
    let valid = UpdateItemRequest {
        name: Some("Updated Item".to_string()),
        description: Some("Updated".to_string()),
        category: Some("New Category".to_string()),
        location: Some("New Location".to_string()),
        purchase_date: Some("2024-06-01".to_string()),
        purchase_price: Some(199.99),
        warranty_expiry: Some("2026-06-01".to_string()),
        notes: Some("Updated notes".to_string()),
        quantity: Some(5),
        inventory_id: Some(2),
    };
    assert!(valid.validate().is_ok());

    // Valid: all None (no changes)
    let valid = UpdateItemRequest {
        name: None,
        description: None,
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: None,
        warranty_expiry: None,
        notes: None,
        quantity: None,
        inventory_id: None,
    };
    assert!(valid.validate().is_ok());

    // Invalid: empty name
    let invalid = UpdateItemRequest {
        name: Some("".to_string()),
        description: None,
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: None,
        warranty_expiry: None,
        notes: None,
        quantity: None,
        inventory_id: None,
    };
    assert!(invalid.validate().is_err());
}

use crate::models::{
    CreateInventoryRequest, CreateItemRequest, CreateOrganizerOptionRequest,
    CreateOrganizerTypeRequest, Inventory, Item, ItemOrganizerValue,
    ItemOrganizerValueWithDetails, OrganizerOption, OrganizerType, OrganizerTypeWithOptions,
    SetItemOrganizerValueRequest, UpdateItemRequest,
    UpdateOrganizerOptionRequest, UpdateOrganizerTypeRequest,
    // User-related models
    User, UserResponse, AdminUpdateUserRequest,
    UserSettings, UpdateUserSettingsRequest,
    InventoryShare, InventoryShareWithUser, PermissionLevel,
    // User Access Grant models (All Access tier)
    UserAccessGrant, UserAccessGrantWithUsers,
    EffectivePermissions, PermissionSource,
};
use chrono::{DateTime, Utc};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use log::info;
use std::env;
use tokio_postgres::NoTls;
use uuid::Uuid;

pub async fn get_pool() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Parse DATABASE_URL: postgres://user:password@host:port/database
    let url = db_url.strip_prefix("postgres://").unwrap_or(&db_url);
    let parts: Vec<&str> = url.split('@').collect();

    if parts.len() != 2 {
        panic!("Invalid DATABASE_URL format");
    }

    let auth_parts: Vec<&str> = parts[0].split(':').collect();
    let host_parts: Vec<&str> = parts[1].split('/').collect();
    let host_port: Vec<&str> = host_parts[0].split(':').collect();

    let user = auth_parts.get(0).unwrap_or(&"postgres").to_string();
    let password = auth_parts.get(1).unwrap_or(&"password").to_string();
    let host = host_port.get(0).unwrap_or(&"localhost").to_string();
    let port = host_port.get(1).unwrap_or(&"5432").parse::<u16>().unwrap_or(5432);
    let dbname = host_parts.get(1).unwrap_or(&"home_inventory").to_string();

    let mut cfg = Config::new();
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.dbname = Some(dbname);
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(None, NoTls)
        .expect("Failed to create database pool")
}

pub struct DatabaseService {
    pool: Pool,
}

impl DatabaseService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_all_items(&self) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, inventory_id, name, description, category, location, purchase_date::text, purchase_price::float8, warranty_expiry::text, notes, quantity, created_at, updated_at 
             FROM items ORDER BY created_at DESC",
                &[],
            )
            .await?;

        let mut items = Vec::new();
        for row in rows {
            let item = Item {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                description: row.get(3),
                category: row.get(4),
                location: row.get(5),
                purchase_date: row.get::<_, Option<String>>(6),
                purchase_price: row.get(7),
                warranty_expiry: row.get::<_, Option<String>>(8),
                notes: row.get(9),
                quantity: row.get(10),
                created_at: row.get::<_, Option<DateTime<Utc>>>(11),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(12),
            };
            items.push(item);
        }

        info!("Retrieved {} items from database", items.len());
        Ok(items)
    }

    pub async fn get_item_by_id(
        &self,
        id: i32,
    ) -> Result<Option<Item>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, inventory_id, name, description, category, location, purchase_date::text, purchase_price::float8, warranty_expiry::text, notes, quantity, created_at, updated_at 
             FROM items WHERE id = $1",
                &[&id],
            )
            .await?;

        if let Some(row) = rows.first() {
            let item = Item {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                description: row.get(3),
                category: row.get(4),
                location: row.get(5),
                purchase_date: row.get::<_, Option<String>>(6),
                purchase_price: row.get(7),
                warranty_expiry: row.get::<_, Option<String>>(8),
                notes: row.get(9),
                quantity: row.get(10),
                created_at: row.get::<_, Option<DateTime<Utc>>>(11),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(12),
            };
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    pub async fn create_item(
        &self,
        request: CreateItemRequest,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        // Convert date strings to proper format or None
        let purchase_date: Option<chrono::NaiveDate> = request.purchase_date
            .as_ref()
            .filter(|s| !s.is_empty())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
            
        let warranty_expiry: Option<chrono::NaiveDate> = request.warranty_expiry
            .as_ref()
            .filter(|s| !s.is_empty())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

        // Handle price properly - convert to None if not provided
        let purchase_price_param: Option<f64> = request.purchase_price;

        let row = client
            .query_one(
                "INSERT INTO items (inventory_id, name, description, category, location, purchase_date, purchase_price, warranty_expiry, notes, quantity) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
             RETURNING id, inventory_id, name, description, category, location, purchase_date::text, purchase_price::float8, warranty_expiry::text, notes, quantity, created_at, updated_at",
                &[
                    &request.inventory_id.unwrap_or(1),
                    &request.name,
                    &request.description,
                    &request.category,
                    &request.location,
                    &purchase_date,
                    &purchase_price_param,
                    &warranty_expiry,
                    &request.notes,
                    &request.quantity,
                ],
            )
            .await?;

        let item = Item {
            id: Some(row.get(0)),
            inventory_id: row.get(1),
            name: row.get(2),
            description: row.get(3),
            category: row.get(4),
            location: row.get(5),
            purchase_date: row.get::<_, Option<String>>(6),
            purchase_price: row.get(7),
            warranty_expiry: row.get::<_, Option<String>>(8),
            notes: row.get(9),
            quantity: row.get(10),
            created_at: row.get::<_, Option<DateTime<Utc>>>(11),
            updated_at: row.get::<_, Option<DateTime<Utc>>>(12),
        };

        info!("Created new item: {} (ID: {:?})", item.name, item.id);
        Ok(item)
    }

    pub async fn update_item(
        &self,
        id: i32,
        request: UpdateItemRequest,
    ) -> Result<Option<Item>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        // Build dynamic update query
        let mut fields = Vec::new();
        let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(ref name) = request.name {
            fields.push(format!("name = ${}", param_count));
            values.push(name);
            param_count += 1;
        }
        if let Some(ref description) = request.description {
            fields.push(format!("description = ${}", param_count));
            values.push(description);
            param_count += 1;
        }
        if let Some(ref category) = request.category {
            fields.push(format!("category = ${}", param_count));
            values.push(category);
            param_count += 1;
        }
        if let Some(ref location) = request.location {
            fields.push(format!("location = ${}", param_count));
            values.push(location);
            param_count += 1;
        }
        if let Some(ref purchase_price) = request.purchase_price {
            fields.push(format!("purchase_price = ${}", param_count));
            values.push(purchase_price);
            param_count += 1;
        }
        if let Some(ref quantity) = request.quantity {
            fields.push(format!("quantity = ${}", param_count));
            values.push(quantity);
            param_count += 1;
        }
        if let Some(ref notes) = request.notes {
            fields.push(format!("notes = ${}", param_count));
            values.push(notes);
            param_count += 1;
        }
        if let Some(ref inventory_id) = request.inventory_id {
            fields.push(format!("inventory_id = ${}", param_count));
            values.push(inventory_id);
            param_count += 1;
        }

        // Handle date fields
        let purchase_date_val: Option<chrono::NaiveDate>;
        if let Some(ref pd) = request.purchase_date {
            let date_str = pd.trim();
            purchase_date_val = if date_str.is_empty() {
                None
            } else {
                chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
            };
            fields.push(format!("purchase_date = ${}", param_count));
            values.push(&purchase_date_val);
            param_count += 1;
        }
        
        let warranty_expiry_val: Option<chrono::NaiveDate>;
        if let Some(ref we) = request.warranty_expiry {
            let date_str = we.trim();
            warranty_expiry_val = if date_str.is_empty() {
                None
            } else {
                chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
            };
            fields.push(format!("warranty_expiry = ${}", param_count));
            values.push(&warranty_expiry_val);
            param_count += 1;
        }

        if fields.is_empty() {
            return self.get_item_by_id(id).await;
        }

        fields.push("updated_at = NOW()".to_string());
        values.push(&id);

        let query = format!(
            "UPDATE items SET {} WHERE id = ${} RETURNING id, inventory_id, name, description, category, location, purchase_date::text, purchase_price::float8, warranty_expiry::text, notes, quantity, created_at, updated_at",
            fields.join(", "),
            param_count
        );

        let rows = client.query(&query, &values).await?;

        if let Some(row) = rows.first() {
            let item = Item {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                description: row.get(3),
                category: row.get(4),
                location: row.get(5),
                purchase_date: row.get::<_, Option<String>>(6),
                purchase_price: row.get(7),
                warranty_expiry: row.get::<_, Option<String>>(8),
                notes: row.get(9),
                quantity: row.get(10),
                created_at: row.get::<_, Option<DateTime<Utc>>>(11),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(12),
            };
            info!("Updated item ID: {}", id);
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_item(&self, id: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute("DELETE FROM items WHERE id = $1", &[&id])
            .await?;

        let deleted = rows_affected > 0;
        if deleted {
            info!("Deleted item ID: {}", id);
        }
        Ok(deleted)
    }

    pub async fn search_items(
        &self,
        query: &str,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let search_pattern = format!("%{}%", query.to_lowercase());
        let rows = client
            .query(
                "SELECT id, inventory_id, name, description, category, location, purchase_date::text, purchase_price::float8, warranty_expiry::text, notes, quantity, created_at, updated_at 
             FROM items 
             WHERE LOWER(name) LIKE $1 OR LOWER(description) LIKE $1 OR LOWER(category) LIKE $1 OR LOWER(location) LIKE $1
             ORDER BY created_at DESC",
                &[&search_pattern],
            )
            .await?;

        let mut items = Vec::new();
        for row in rows {
            let item = Item {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                description: row.get(3),
                category: row.get(4),
                location: row.get(5),
                purchase_date: row.get::<_, Option<String>>(6),
                purchase_price: row.get(7),
                warranty_expiry: row.get::<_, Option<String>>(8),
                notes: row.get(9),
                quantity: row.get(10),
                created_at: row.get::<_, Option<DateTime<Utc>>>(11),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(12),
            };
            items.push(item);
        }

        info!("Found {} items matching search query: '{}'", items.len(), query);
        Ok(items)
    }

    // Inventory operations
    pub async fn get_inventory_by_id(
        &self,
        id: i32,
    ) -> Result<Option<Inventory>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, name, description, location, image_url, user_id, created_at, updated_at 
                 FROM inventories WHERE id = $1",
                &[&id],
            )
            .await?;

        if let Some(row) = rows.first() {
            let inventory = Inventory {
                id: Some(row.get(0)),
                name: row.get(1),
                description: row.get(2),
                location: row.get(3),
                image_url: row.get(4),
                user_id: row.get(5),
                created_at: row.get::<_, Option<DateTime<Utc>>>(6),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
            };
            Ok(Some(inventory))
        } else {
            Ok(None)
        }
    }

    pub async fn create_inventory(
        &self,
        request: CreateInventoryRequest,
        user_id: uuid::Uuid,
    ) -> Result<Inventory, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let row = client
            .query_one(
                "INSERT INTO inventories (name, description, location, image_url, user_id) 
                 VALUES ($1, $2, $3, $4, $5) 
                 RETURNING id, name, description, location, image_url, user_id, created_at, updated_at",
                &[&request.name, &request.description, &request.location, &request.image_url, &user_id],
            )
            .await?;

        let inventory = Inventory {
            id: Some(row.get(0)),
            name: row.get(1),
            description: row.get(2),
            location: row.get(3),
            image_url: row.get(4),
            user_id: row.get(5),
            created_at: row.get::<_, Option<DateTime<Utc>>>(6),
            updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
        };

        info!("Created new inventory: {} (ID: {:?})", inventory.name, inventory.id);
        Ok(inventory)
    }

    pub async fn update_inventory(
        &self,
        id: i32,
        request: crate::models::UpdateInventoryRequest,
    ) -> Result<Option<Inventory>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        // Build dynamic update query
        let mut fields = Vec::new();
        let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(ref name) = request.name {
            fields.push(format!("name = ${}", param_count));
            values.push(name);
            param_count += 1;
        }
        if let Some(ref description) = request.description {
            fields.push(format!("description = ${}", param_count));
            values.push(description);
            param_count += 1;
        }
        if let Some(ref location) = request.location {
            fields.push(format!("location = ${}", param_count));
            values.push(location);
            param_count += 1;
        }
        if let Some(ref image_url) = request.image_url {
            fields.push(format!("image_url = ${}", param_count));
            values.push(image_url);
            param_count += 1;
        }

        if fields.is_empty() {
            return self.get_inventory_by_id(id).await;
        }

        fields.push("updated_at = NOW()".to_string());
        values.push(&id);

        let query = format!(
            "UPDATE inventories SET {} WHERE id = ${} RETURNING id, name, description, location, image_url, user_id, created_at, updated_at",
            fields.join(", "),
            param_count
        );

        let rows = client.query(&query, &values).await?;

        if let Some(row) = rows.first() {
            let inventory = Inventory {
                id: Some(row.get(0)),
                name: row.get(1),
                description: row.get(2),
                location: row.get(3),
                image_url: row.get(4),
                user_id: row.get(5),
                created_at: row.get::<_, Option<DateTime<Utc>>>(6),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
            };
            info!("Updated inventory ID: {}", id);
            Ok(Some(inventory))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_inventory(&self, id: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute("DELETE FROM inventories WHERE id = $1", &[&id])
            .await?;

        let deleted = rows_affected > 0;
        if deleted {
            info!("Deleted inventory ID: {} (CASCADE: organizers and items)", id);
        }
        Ok(deleted)
    }

    pub async fn get_items_by_inventory(
        &self,
        inventory_id: i32,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, inventory_id, name, description, category, location, purchase_date::text, purchase_price::float8, warranty_expiry::text, notes, quantity, created_at, updated_at 
                 FROM items WHERE inventory_id = $1 ORDER BY created_at DESC",
                &[&inventory_id],
            )
            .await?;

        let mut items = Vec::new();
        for row in rows {
            let item = Item {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                description: row.get(3),
                category: row.get(4),
                location: row.get(5),
                purchase_date: row.get::<_, Option<String>>(6),
                purchase_price: row.get(7),
                warranty_expiry: row.get::<_, Option<String>>(8),
                notes: row.get(9),
                quantity: row.get(10),
                created_at: row.get::<_, Option<DateTime<Utc>>>(11),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(12),
            };
            items.push(item);
        }

        info!("Retrieved {} items for inventory {}", items.len(), inventory_id);
        Ok(items)
    }

    // ==================== Organizer Type Operations ====================

    pub async fn get_organizer_types_by_inventory(
        &self,
        inventory_id: i32,
    ) -> Result<Vec<OrganizerType>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, inventory_id, name, input_type, is_required, display_order, created_at, updated_at 
                 FROM organizer_types WHERE inventory_id = $1 ORDER BY display_order ASC, name ASC",
                &[&inventory_id],
            )
            .await?;

        let mut organizers = Vec::new();
        for row in rows {
            let organizer = OrganizerType {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                input_type: row.get(3),
                is_required: row.get(4),
                display_order: row.get(5),
                created_at: row.get::<_, Option<DateTime<Utc>>>(6),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
            };
            organizers.push(organizer);
        }

        info!("Retrieved {} organizer types for inventory {}", organizers.len(), inventory_id);
        Ok(organizers)
    }

    pub async fn get_organizer_types_with_options_by_inventory(
        &self,
        inventory_id: i32,
    ) -> Result<Vec<OrganizerTypeWithOptions>, Box<dyn std::error::Error>> {
        let organizer_types = self.get_organizer_types_by_inventory(inventory_id).await?;
        
        let mut result = Vec::new();
        for organizer_type in organizer_types {
            let options = if organizer_type.input_type == "select" {
                self.get_organizer_options(organizer_type.id.unwrap()).await?
            } else {
                Vec::new()
            };
            
            result.push(OrganizerTypeWithOptions {
                organizer_type,
                options,
            });
        }

        Ok(result)
    }

    pub async fn get_organizer_type_by_id(
        &self,
        id: i32,
    ) -> Result<Option<OrganizerType>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, inventory_id, name, input_type, is_required, display_order, created_at, updated_at 
                 FROM organizer_types WHERE id = $1",
                &[&id],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(Some(OrganizerType {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                input_type: row.get(3),
                is_required: row.get(4),
                display_order: row.get(5),
                created_at: row.get::<_, Option<DateTime<Utc>>>(6),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_organizer_type(
        &self,
        inventory_id: i32,
        request: CreateOrganizerTypeRequest,
    ) -> Result<OrganizerType, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let input_type = request.input_type.unwrap_or_else(|| "select".to_string());
        let is_required = request.is_required.unwrap_or(false);
        let display_order = request.display_order.unwrap_or(0);

        let row = client
            .query_one(
                "INSERT INTO organizer_types (inventory_id, name, input_type, is_required, display_order) 
                 VALUES ($1, $2, $3, $4, $5) 
                 RETURNING id, inventory_id, name, input_type, is_required, display_order, created_at, updated_at",
                &[&inventory_id, &request.name, &input_type, &is_required, &display_order],
            )
            .await?;

        let organizer = OrganizerType {
            id: Some(row.get(0)),
            inventory_id: row.get(1),
            name: row.get(2),
            input_type: row.get(3),
            is_required: row.get(4),
            display_order: row.get(5),
            created_at: row.get::<_, Option<DateTime<Utc>>>(6),
            updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
        };

        info!("Created organizer type: {} (ID: {:?})", organizer.name, organizer.id);
        Ok(organizer)
    }

    pub async fn update_organizer_type(
        &self,
        id: i32,
        request: UpdateOrganizerTypeRequest,
    ) -> Result<Option<OrganizerType>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let mut fields = Vec::new();
        let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(ref name) = request.name {
            fields.push(format!("name = ${}", param_count));
            values.push(name);
            param_count += 1;
        }
        if let Some(ref input_type) = request.input_type {
            fields.push(format!("input_type = ${}", param_count));
            values.push(input_type);
            param_count += 1;
        }
        if let Some(ref is_required) = request.is_required {
            fields.push(format!("is_required = ${}", param_count));
            values.push(is_required);
            param_count += 1;
        }
        if let Some(ref display_order) = request.display_order {
            fields.push(format!("display_order = ${}", param_count));
            values.push(display_order);
            param_count += 1;
        }

        if fields.is_empty() {
            return self.get_organizer_type_by_id(id).await;
        }

        fields.push("updated_at = NOW()".to_string());
        values.push(&id);

        let query = format!(
            "UPDATE organizer_types SET {} WHERE id = ${} 
             RETURNING id, inventory_id, name, input_type, is_required, display_order, created_at, updated_at",
            fields.join(", "),
            param_count
        );

        let rows = client.query(&query, &values).await?;

        if let Some(row) = rows.first() {
            let organizer = OrganizerType {
                id: Some(row.get(0)),
                inventory_id: row.get(1),
                name: row.get(2),
                input_type: row.get(3),
                is_required: row.get(4),
                display_order: row.get(5),
                created_at: row.get::<_, Option<DateTime<Utc>>>(6),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
            };
            info!("Updated organizer type ID: {}", id);
            Ok(Some(organizer))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_organizer_type(&self, id: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute("DELETE FROM organizer_types WHERE id = $1", &[&id])
            .await?;

        let deleted = rows_affected > 0;
        if deleted {
            info!("Deleted organizer type ID: {}", id);
        }
        Ok(deleted)
    }

    // ==================== Organizer Option Operations ====================

    pub async fn get_organizer_options(
        &self,
        organizer_type_id: i32,
    ) -> Result<Vec<OrganizerOption>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, organizer_type_id, name, display_order, created_at, updated_at 
                 FROM organizer_options WHERE organizer_type_id = $1 ORDER BY display_order ASC, name ASC",
                &[&organizer_type_id],
            )
            .await?;

        let mut options = Vec::new();
        for row in rows {
            let option = OrganizerOption {
                id: Some(row.get(0)),
                organizer_type_id: row.get(1),
                name: row.get(2),
                display_order: row.get(3),
                created_at: row.get::<_, Option<DateTime<Utc>>>(4),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(5),
            };
            options.push(option);
        }

        info!("Retrieved {} options for organizer type {}", options.len(), organizer_type_id);
        Ok(options)
    }

    pub async fn get_organizer_option_by_id(
        &self,
        id: i32,
    ) -> Result<Option<OrganizerOption>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, organizer_type_id, name, display_order, created_at, updated_at 
                 FROM organizer_options WHERE id = $1",
                &[&id],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(Some(OrganizerOption {
                id: Some(row.get(0)),
                organizer_type_id: row.get(1),
                name: row.get(2),
                display_order: row.get(3),
                created_at: row.get::<_, Option<DateTime<Utc>>>(4),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(5),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_organizer_option(
        &self,
        organizer_type_id: i32,
        request: CreateOrganizerOptionRequest,
    ) -> Result<OrganizerOption, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let display_order = request.display_order.unwrap_or(0);

        let row = client
            .query_one(
                "INSERT INTO organizer_options (organizer_type_id, name, display_order) 
                 VALUES ($1, $2, $3) 
                 RETURNING id, organizer_type_id, name, display_order, created_at, updated_at",
                &[&organizer_type_id, &request.name, &display_order],
            )
            .await?;

        let option = OrganizerOption {
            id: Some(row.get(0)),
            organizer_type_id: row.get(1),
            name: row.get(2),
            display_order: row.get(3),
            created_at: row.get::<_, Option<DateTime<Utc>>>(4),
            updated_at: row.get::<_, Option<DateTime<Utc>>>(5),
        };

        info!("Created organizer option: {} (ID: {:?})", option.name, option.id);
        Ok(option)
    }

    pub async fn update_organizer_option(
        &self,
        id: i32,
        request: UpdateOrganizerOptionRequest,
    ) -> Result<Option<OrganizerOption>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let mut fields = Vec::new();
        let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(ref name) = request.name {
            fields.push(format!("name = ${}", param_count));
            values.push(name);
            param_count += 1;
        }
        if let Some(ref display_order) = request.display_order {
            fields.push(format!("display_order = ${}", param_count));
            values.push(display_order);
            param_count += 1;
        }

        if fields.is_empty() {
            return self.get_organizer_option_by_id(id).await;
        }

        fields.push("updated_at = NOW()".to_string());
        values.push(&id);

        let query = format!(
            "UPDATE organizer_options SET {} WHERE id = ${} 
             RETURNING id, organizer_type_id, name, display_order, created_at, updated_at",
            fields.join(", "),
            param_count
        );

        let rows = client.query(&query, &values).await?;

        if let Some(row) = rows.first() {
            let option = OrganizerOption {
                id: Some(row.get(0)),
                organizer_type_id: row.get(1),
                name: row.get(2),
                display_order: row.get(3),
                created_at: row.get::<_, Option<DateTime<Utc>>>(4),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(5),
            };
            info!("Updated organizer option ID: {}", id);
            Ok(Some(option))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_organizer_option(&self, id: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute("DELETE FROM organizer_options WHERE id = $1", &[&id])
            .await?;

        let deleted = rows_affected > 0;
        if deleted {
            info!("Deleted organizer option ID: {}", id);
        }
        Ok(deleted)
    }

    // ==================== Item Organizer Value Operations ====================

    pub async fn get_item_organizer_values(
        &self,
        item_id: i32,
    ) -> Result<Vec<ItemOrganizerValueWithDetails>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT 
                    iov.organizer_type_id,
                    ot.name as organizer_type_name,
                    ot.input_type,
                    ot.is_required,
                    COALESCE(oo.name, iov.text_value) as display_value,
                    iov.organizer_option_id,
                    iov.text_value
                 FROM item_organizer_values iov
                 JOIN organizer_types ot ON iov.organizer_type_id = ot.id
                 LEFT JOIN organizer_options oo ON iov.organizer_option_id = oo.id
                 WHERE iov.item_id = $1
                 ORDER BY ot.display_order ASC, ot.name ASC",
                &[&item_id],
            )
            .await?;

        let mut values = Vec::new();
        for row in rows {
            let value = ItemOrganizerValueWithDetails {
                organizer_type_id: row.get(0),
                organizer_type_name: row.get(1),
                input_type: row.get(2),
                is_required: row.get(3),
                value: row.get(4),
                organizer_option_id: row.get(5),
                text_value: row.get(6),
            };
            values.push(value);
        }

        info!("Retrieved {} organizer values for item {}", values.len(), item_id);
        Ok(values)
    }

    pub async fn set_item_organizer_value(
        &self,
        item_id: i32,
        request: SetItemOrganizerValueRequest,
    ) -> Result<ItemOrganizerValue, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        // Use UPSERT to insert or update the value
        let row = client
            .query_one(
                "INSERT INTO item_organizer_values (item_id, organizer_type_id, organizer_option_id, text_value) 
                 VALUES ($1, $2, $3, $4) 
                 ON CONFLICT (item_id, organizer_type_id) 
                 DO UPDATE SET organizer_option_id = $3, text_value = $4, updated_at = NOW()
                 RETURNING id, item_id, organizer_type_id, organizer_option_id, text_value, created_at, updated_at",
                &[&item_id, &request.organizer_type_id, &request.organizer_option_id, &request.text_value],
            )
            .await?;

        let value = ItemOrganizerValue {
            id: Some(row.get(0)),
            item_id: row.get(1),
            organizer_type_id: row.get(2),
            organizer_option_id: row.get(3),
            text_value: row.get(4),
            created_at: row.get::<_, Option<DateTime<Utc>>>(5),
            updated_at: row.get::<_, Option<DateTime<Utc>>>(6),
        };

        info!("Set organizer value for item {} type {}", item_id, request.organizer_type_id);
        Ok(value)
    }

    pub async fn set_item_organizer_values(
        &self,
        item_id: i32,
        values: Vec<SetItemOrganizerValueRequest>,
    ) -> Result<Vec<ItemOrganizerValue>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        for request in values {
            let result = self.set_item_organizer_value(item_id, request).await?;
            results.push(result);
        }
        Ok(results)
    }

    pub async fn delete_item_organizer_value(
        &self,
        item_id: i32,
        organizer_type_id: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute(
                "DELETE FROM item_organizer_values WHERE item_id = $1 AND organizer_type_id = $2",
                &[&item_id, &organizer_type_id],
            )
            .await?;

        let deleted = rows_affected > 0;
        if deleted {
            info!("Deleted organizer value for item {} type {}", item_id, organizer_type_id);
        }
        Ok(deleted)
    }

    #[allow(dead_code)]
    pub async fn clear_item_organizer_values(
        &self,
        item_id: i32,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute(
                "DELETE FROM item_organizer_values WHERE item_id = $1",
                &[&item_id],
            )
            .await?;

        info!("Cleared {} organizer values for item {}", rows_affected, item_id);
        Ok(rows_affected)
    }

    // ==================== User Operations ====================

    /// Get user count for setup status check
    pub async fn get_user_count(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let row = client.query_one("SELECT COUNT(*) FROM users", &[]).await?;
        Ok(row.get(0))
    }

    /// Get a user by ID
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT id, username, full_name, password_hash, is_admin, is_active, created_at, updated_at 
                 FROM users WHERE id = $1",
                &[&id],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(Some(User {
                id: row.get(0),
                username: row.get(1),
                full_name: row.get(2),
                password_hash: row.get(3),
                is_admin: row.get(4),
                is_active: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get a user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT id, username, full_name, password_hash, is_admin, is_active, created_at, updated_at 
                 FROM users WHERE LOWER(username) = LOWER($1)",
                &[&username],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(Some(User {
                id: row.get(0),
                username: row.get(1),
                full_name: row.get(2),
                password_hash: row.get(3),
                is_admin: row.get(4),
                is_active: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get a user by username or email - now only checks username (for login)
    pub async fn get_user_by_username_or_email(&self, identifier: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT id, username, full_name, password_hash, is_admin, is_active, created_at, updated_at 
                 FROM users WHERE LOWER(username) = LOWER($1)",
                &[&identifier],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(Some(User {
                id: row.get(0),
                username: row.get(1),
                full_name: row.get(2),
                password_hash: row.get(3),
                is_admin: row.get(4),
                is_active: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get all users (admin only)
    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let rows = client
            .query(
                "SELECT id, username, full_name, is_admin, is_active, created_at, updated_at 
                 FROM users ORDER BY created_at DESC",
                &[],
            )
            .await?;

        let users = rows
            .iter()
            .map(|row| UserResponse {
                id: row.get(0),
                username: row.get(1),
                full_name: row.get(2),
                is_admin: row.get(3),
                is_active: row.get(4),
                created_at: row.get(5),
                updated_at: row.get(6),
            })
            .collect();

        Ok(users)
    }

    /// Create a new user
    pub async fn create_user(
        &self,
        username: &str,
        full_name: &str,
        password_hash: &str,
        is_admin: bool,
        is_active: bool,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let row = client
            .query_one(
                "INSERT INTO users (username, full_name, password_hash, is_admin, is_active) 
                 VALUES ($1, $2, $3, $4, $5) 
                 RETURNING id, username, full_name, password_hash, is_admin, is_active, created_at, updated_at",
                &[&username, &full_name, &password_hash, &is_admin, &is_active],
            )
            .await?;

        let user = User {
            id: row.get(0),
            username: row.get(1),
            full_name: row.get(2),
            password_hash: row.get(3),
            is_admin: row.get(4),
            is_active: row.get(5),
            created_at: row.get(6),
            updated_at: row.get(7),
        };

        info!("Created new user: {} (ID: {})", user.username, user.id);
        Ok(user)
    }

    /// Update a user's profile
    pub async fn update_user_profile(
        &self,
        id: Uuid,
        full_name: Option<&str>,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let mut fields = Vec::new();
        let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(ref n) = full_name {
            fields.push(format!("full_name = ${}", param_count));
            values.push(n);
            param_count += 1;
        }

        if fields.is_empty() {
            return self.get_user_by_id(id).await;
        }

        fields.push("updated_at = NOW()".to_string());
        values.push(&id);

        let query = format!(
            "UPDATE users SET {} WHERE id = ${} 
             RETURNING id, username, full_name, password_hash, is_admin, is_active, created_at, updated_at",
            fields.join(", "),
            param_count
        );

        let rows = client.query(&query, &values).await?;

        if let Some(row) = rows.first() {
            Ok(Some(User {
                id: row.get(0),
                username: row.get(1),
                full_name: row.get(2),
                password_hash: row.get(3),
                is_admin: row.get(4),
                is_active: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            }))
        } else {
            Ok(None)
        }
    }

    /// Admin update user
    pub async fn admin_update_user(
        &self,
        id: Uuid,
        request: AdminUpdateUserRequest,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let mut fields = Vec::new();
        let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(ref username) = request.username {
            fields.push(format!("username = ${}", param_count));
            values.push(username);
            param_count += 1;
        }
        if let Some(ref full_name) = request.full_name {
            fields.push(format!("full_name = ${}", param_count));
            values.push(full_name);
            param_count += 1;
        }
        if let Some(ref is_admin) = request.is_admin {
            fields.push(format!("is_admin = ${}", param_count));
            values.push(is_admin);
            param_count += 1;
        }
        if let Some(ref is_active) = request.is_active {
            fields.push(format!("is_active = ${}", param_count));
            values.push(is_active);
            param_count += 1;
        }

        if fields.is_empty() {
            return self.get_user_by_id(id).await;
        }

        fields.push("updated_at = NOW()".to_string());
        values.push(&id);

        let query = format!(
            "UPDATE users SET {} WHERE id = ${} 
             RETURNING id, username, full_name, password_hash, is_admin, is_active, created_at, updated_at",
            fields.join(", "),
            param_count
        );

        let rows = client.query(&query, &values).await?;

        if let Some(row) = rows.first() {
            Ok(Some(User {
                id: row.get(0),
                username: row.get(1),
                full_name: row.get(2),
                password_hash: row.get(3),
                is_admin: row.get(4),
                is_active: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update user password
    pub async fn update_user_password(
        &self,
        id: Uuid,
        password_hash: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute(
                "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
                &[&password_hash, &id],
            )
            .await?;

        Ok(rows_affected > 0)
    }

    /// Delete a user
    pub async fn delete_user(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .await?;

        let deleted = rows_affected > 0;
        if deleted {
            info!("Deleted user ID: {}", id);
        }
        Ok(deleted)
    }

    /// Count admin users
    pub async fn count_admin_users(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let row = client
            .query_one("SELECT COUNT(*) FROM users WHERE is_admin = true", &[])
            .await?;
        Ok(row.get(0))
    }

    // ==================== User Settings Operations ====================

    /// Get user settings
    pub async fn get_user_settings(&self, user_id: Uuid) -> Result<Option<UserSettings>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, user_id, theme, default_inventory_id, items_per_page, date_format, 
                        currency, notifications_enabled, settings_json, created_at, updated_at 
                 FROM user_settings WHERE user_id = $1",
                &[&user_id],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(Some(UserSettings {
                id: row.get(0),
                user_id: row.get(1),
                theme: row.get(2),
                default_inventory_id: row.get(3),
                items_per_page: row.get(4),
                date_format: row.get(5),
                currency: row.get(6),
                notifications_enabled: row.get(7),
                settings_json: row.get(8),
                created_at: row.get(9),
                updated_at: row.get(10),
            }))
        } else {
            Ok(None)
        }
    }

    /// Create default user settings
    pub async fn create_user_settings(&self, user_id: Uuid) -> Result<UserSettings, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let row = client
            .query_one(
                "INSERT INTO user_settings (user_id) VALUES ($1) 
                 RETURNING id, user_id, theme, default_inventory_id, items_per_page, date_format, 
                           currency, notifications_enabled, settings_json, created_at, updated_at",
                &[&user_id],
            )
            .await?;

        Ok(UserSettings {
            id: row.get(0),
            user_id: row.get(1),
            theme: row.get(2),
            default_inventory_id: row.get(3),
            items_per_page: row.get(4),
            date_format: row.get(5),
            currency: row.get(6),
            notifications_enabled: row.get(7),
            settings_json: row.get(8),
            created_at: row.get(9),
            updated_at: row.get(10),
        })
    }

    /// Update user settings
    pub async fn update_user_settings(
        &self,
        user_id: Uuid,
        request: UpdateUserSettingsRequest,
    ) -> Result<Option<UserSettings>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let mut fields = Vec::new();
        let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_count = 1;

        if let Some(ref theme) = request.theme {
            fields.push(format!("theme = ${}", param_count));
            values.push(theme);
            param_count += 1;
        }
        if let Some(ref default_inventory_id) = request.default_inventory_id {
            fields.push(format!("default_inventory_id = ${}", param_count));
            values.push(default_inventory_id);
            param_count += 1;
        }
        if let Some(ref items_per_page) = request.items_per_page {
            fields.push(format!("items_per_page = ${}", param_count));
            values.push(items_per_page);
            param_count += 1;
        }
        if let Some(ref date_format) = request.date_format {
            fields.push(format!("date_format = ${}", param_count));
            values.push(date_format);
            param_count += 1;
        }
        if let Some(ref currency) = request.currency {
            fields.push(format!("currency = ${}", param_count));
            values.push(currency);
            param_count += 1;
        }
        if let Some(ref notifications_enabled) = request.notifications_enabled {
            fields.push(format!("notifications_enabled = ${}", param_count));
            values.push(notifications_enabled);
            param_count += 1;
        }
        if let Some(ref settings_json) = request.settings_json {
            fields.push(format!("settings_json = ${}", param_count));
            values.push(settings_json);
            param_count += 1;
        }

        if fields.is_empty() {
            return self.get_user_settings(user_id).await;
        }

        fields.push("updated_at = NOW()".to_string());
        values.push(&user_id);

        let query = format!(
            "UPDATE user_settings SET {} WHERE user_id = ${} 
             RETURNING id, user_id, theme, default_inventory_id, items_per_page, date_format, 
                       currency, notifications_enabled, settings_json, created_at, updated_at",
            fields.join(", "),
            param_count
        );

        let rows = client.query(&query, &values).await?;

        if let Some(row) = rows.first() {
            Ok(Some(UserSettings {
                id: row.get(0),
                user_id: row.get(1),
                theme: row.get(2),
                default_inventory_id: row.get(3),
                items_per_page: row.get(4),
                date_format: row.get(5),
                currency: row.get(6),
                notifications_enabled: row.get(7),
                settings_json: row.get(8),
                created_at: row.get(9),
                updated_at: row.get(10),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get or create user settings
    pub async fn get_or_create_user_settings(&self, user_id: Uuid) -> Result<UserSettings, Box<dyn std::error::Error>> {
        if let Some(settings) = self.get_user_settings(user_id).await? {
            Ok(settings)
        } else {
            self.create_user_settings(user_id).await
        }
    }

    // ==================== Inventory Sharing Operations ====================

    /// Share an inventory with a user
    pub async fn create_inventory_share(
        &self,
        inventory_id: i32,
        shared_with_user_id: Uuid,
        shared_by_user_id: Uuid,
        permission_level: PermissionLevel,
    ) -> Result<InventoryShare, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let permission_str = permission_level.to_string();
        let row = client
            .query_one(
                "INSERT INTO inventory_shares (inventory_id, shared_with_user_id, shared_by_user_id, permission_level) 
                 VALUES ($1, $2, $3, $4) 
                 RETURNING id, inventory_id, shared_with_user_id, shared_by_user_id, permission_level, created_at, updated_at",
                &[&inventory_id, &shared_with_user_id, &shared_by_user_id, &permission_str],
            )
            .await?;

        let perm_str: String = row.get(4);
        Ok(InventoryShare {
            id: row.get(0),
            inventory_id: row.get(1),
            shared_with_user_id: row.get(2),
            shared_by_user_id: row.get(3),
            permission_level: perm_str.parse().unwrap_or(PermissionLevel::View),
            created_at: row.get(5),
            updated_at: row.get(6),
        })
    }

    /// Get shares for an inventory
    pub async fn get_inventory_shares(
        &self,
        inventory_id: i32,
    ) -> Result<Vec<InventoryShareWithUser>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT 
                    s.id, s.inventory_id, s.permission_level, s.created_at, s.updated_at,
                    sw.id, sw.username, sw.full_name, sw.is_admin, sw.is_active, sw.created_at, sw.updated_at,
                    sb.id, sb.username, sb.full_name, sb.is_admin, sb.is_active, sb.created_at, sb.updated_at
                 FROM inventory_shares s
                 JOIN users sw ON s.shared_with_user_id = sw.id
                 JOIN users sb ON s.shared_by_user_id = sb.id
                 WHERE s.inventory_id = $1
                 ORDER BY s.created_at DESC",
                &[&inventory_id],
            )
            .await?;

        let shares = rows
            .iter()
            .map(|row| {
                let perm_str: String = row.get(2);
                InventoryShareWithUser {
                    id: row.get(0),
                    inventory_id: row.get(1),
                    permission_level: perm_str.parse().unwrap_or(PermissionLevel::View),
                    created_at: row.get(3),
                    updated_at: row.get(4),
                    shared_with_user: UserResponse {
                        id: row.get(5),
                        username: row.get(6),
                        full_name: row.get(7),
                        is_admin: row.get(8),
                        is_active: row.get(9),
                        created_at: row.get(10),
                        updated_at: row.get(11),
                    },
                    shared_by_user: UserResponse {
                        id: row.get(12),
                        username: row.get(13),
                        full_name: row.get(14),
                        is_admin: row.get(15),
                        is_active: row.get(16),
                        created_at: row.get(17),
                        updated_at: row.get(18),
                    },
                }
            })
            .collect();

        Ok(shares)
    }

    /// Get comprehensive effective permissions for a user on an inventory
    pub async fn get_effective_permissions(
        &self,
        user_id: Uuid,
        inventory_id: i32,
    ) -> Result<EffectivePermissions, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        // Check if user is the owner
        let owner_rows = client
            .query(
                "SELECT user_id FROM inventories WHERE id = $1",
                &[&inventory_id],
            )
            .await?;

        if let Some(row) = owner_rows.first() {
            let owner_id: Option<Uuid> = row.get(0);
            if owner_id == Some(user_id) {
                return Ok(EffectivePermissions {
                    can_view: true,
                    can_edit_items: true,
                    can_add_items: true,
                    can_remove_items: true,
                    can_edit_inventory: true,
                    can_delete_inventory: true,
                    can_manage_sharing: true,
                    can_manage_organizers: true,
                    is_owner: true,
                    has_all_access: false,
                    permission_source: PermissionSource::Owner,
                });
            }

            // Check for All Access grant from the owner  
            if let Some(owner_uuid) = owner_id {
                let all_access_rows = client
                    .query(
                        "SELECT id FROM user_access_grants 
                         WHERE grantor_user_id = $1 AND grantee_user_id = $2",
                        &[&owner_uuid, &user_id],
                    )
                    .await?;

                if !all_access_rows.is_empty() {
                    return Ok(EffectivePermissions {
                        can_view: true,
                        can_edit_items: true,
                        can_add_items: true,
                        can_remove_items: true,
                        can_edit_inventory: true,
                        can_delete_inventory: true,
                        can_manage_sharing: true,
                        can_manage_organizers: true,
                        is_owner: false,
                        has_all_access: true,
                        permission_source: PermissionSource::AllAccess,
                    });
                }
            }
        }

        // Check for per-inventory share
        let share_rows = client
            .query(
                "SELECT permission_level FROM inventory_shares 
                 WHERE inventory_id = $1 AND shared_with_user_id = $2",
                &[&inventory_id, &user_id],
            )
            .await?;

        if let Some(row) = share_rows.first() {
            let perm_str: String = row.get(0);
            let permission = perm_str.parse().unwrap_or(PermissionLevel::View);
            
            return Ok(EffectivePermissions {
                can_view: permission.can_view(),
                can_edit_items: permission.can_edit_items(),
                can_add_items: permission.can_add_items(),
                can_remove_items: permission.can_remove_items(),
                can_edit_inventory: permission.can_edit_inventory(),
                can_delete_inventory: false, // Only owner or AllAccess can delete
                can_manage_sharing: false,   // Only owner or AllAccess can manage sharing
                can_manage_organizers: permission.can_manage_organizers(),
                is_owner: false,
                has_all_access: false,
                permission_source: PermissionSource::InventoryShare,
            });
        }

        // No access
        Ok(EffectivePermissions {
            can_view: false,
            can_edit_items: false,
            can_add_items: false,
            can_remove_items: false,
            can_edit_inventory: false,
            can_delete_inventory: false,
            can_manage_sharing: false,
            can_manage_organizers: false,
            is_owner: false,
            has_all_access: false,
            permission_source: PermissionSource::None,
        })
    }

    /// Update inventory share permission
    pub async fn update_inventory_share(
        &self,
        share_id: Uuid,
        permission_level: PermissionLevel,
    ) -> Result<Option<InventoryShare>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let permission_str = permission_level.to_string();
        let rows = client
            .query(
                "UPDATE inventory_shares SET permission_level = $1, updated_at = NOW() 
                 WHERE id = $2 
                 RETURNING id, inventory_id, shared_with_user_id, shared_by_user_id, permission_level, created_at, updated_at",
                &[&permission_str, &share_id],
            )
            .await?;

        if let Some(row) = rows.first() {
            let perm_str: String = row.get(4);
            Ok(Some(InventoryShare {
                id: row.get(0),
                inventory_id: row.get(1),
                shared_with_user_id: row.get(2),
                shared_by_user_id: row.get(3),
                permission_level: perm_str.parse().unwrap_or(PermissionLevel::View),
                created_at: row.get(5),
                updated_at: row.get(6),
            }))
        } else {
            Ok(None)
        }
    }

    /// Delete inventory share
    pub async fn delete_inventory_share(&self, share_id: Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute("DELETE FROM inventory_shares WHERE id = $1", &[&share_id])
            .await?;

        Ok(rows_affected > 0)
    }

    /// Get inventories accessible to a user (owned, shared via inventory_shares, or via All Access grants)
    pub async fn get_accessible_inventories(&self, user_id: Uuid) -> Result<Vec<Inventory>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        // Query includes:
        // 1. Inventories owned by the user (i.user_id = $1)
        // 2. Inventories shared directly with the user (inventory_shares)
        // 3. Inventories owned by users who granted All Access to this user (user_access_grants)
        let rows = client
            .query(
                "SELECT DISTINCT i.id, i.name, i.description, i.location, i.image_url, i.user_id, i.created_at, i.updated_at 
                 FROM inventories i
                 LEFT JOIN inventory_shares s ON i.id = s.inventory_id AND s.shared_with_user_id = $1
                 LEFT JOIN user_access_grants g ON i.user_id = g.grantor_user_id AND g.grantee_user_id = $1
                 WHERE i.user_id = $1 
                    OR s.shared_with_user_id = $1
                    OR g.grantee_user_id = $1
                 ORDER BY i.name ASC",
                &[&user_id],
            )
            .await?;

        let inventories = rows
            .iter()
            .map(|row| Inventory {
                id: Some(row.get(0)),
                name: row.get(1),
                description: row.get(2),
                location: row.get(3),
                image_url: row.get(4),
                user_id: row.get(5),
                created_at: row.get::<_, Option<DateTime<Utc>>>(6),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(7),
            })
            .collect();

        Ok(inventories)
    }

    // ==================== User Access Grant Operations (All Access Tier) ====================

    /// Create a user access grant (All Access tier)
    pub async fn create_user_access_grant(
        &self,
        grantor_user_id: Uuid,
        grantee_user_id: Uuid,
    ) -> Result<UserAccessGrant, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let row = client
            .query_one(
                "INSERT INTO user_access_grants (grantor_user_id, grantee_user_id) 
                 VALUES ($1, $2) 
                 RETURNING id, grantor_user_id, grantee_user_id, created_at, updated_at",
                &[&grantor_user_id, &grantee_user_id],
            )
            .await?;

        Ok(UserAccessGrant {
            id: row.get(0),
            grantor_user_id: row.get(1),
            grantee_user_id: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        })
    }

    /// Get all access grants where the user is the grantor (people who can access my inventories)
    pub async fn get_user_access_grants_by_grantor(
        &self,
        grantor_user_id: Uuid,
    ) -> Result<Vec<UserAccessGrantWithUsers>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT 
                    g.id, g.created_at, g.updated_at,
                    gr.id, gr.username, gr.full_name, gr.is_admin, gr.is_active, gr.created_at, gr.updated_at,
                    ge.id, ge.username, ge.full_name, ge.is_admin, ge.is_active, ge.created_at, ge.updated_at
                 FROM user_access_grants g
                 JOIN users gr ON g.grantor_user_id = gr.id
                 JOIN users ge ON g.grantee_user_id = ge.id
                 WHERE g.grantor_user_id = $1
                 ORDER BY g.created_at DESC",
                &[&grantor_user_id],
            )
            .await?;

        let grants = rows
            .iter()
            .map(|row| UserAccessGrantWithUsers {
                id: row.get(0),
                created_at: row.get(1),
                updated_at: row.get(2),
                grantor: UserResponse {
                    id: row.get(3),
                    username: row.get(4),
                    full_name: row.get(5),
                    is_admin: row.get(6),
                    is_active: row.get(7),
                    created_at: row.get(8),
                    updated_at: row.get(9),
                },
                grantee: UserResponse {
                    id: row.get(10),
                    username: row.get(11),
                    full_name: row.get(12),
                    is_admin: row.get(13),
                    is_active: row.get(14),
                    created_at: row.get(15),
                    updated_at: row.get(16),
                },
            })
            .collect();

        Ok(grants)
    }

    /// Get all access grants where the user is the grantee (users who gave me access)
    pub async fn get_user_access_grants_by_grantee(
        &self,
        grantee_user_id: Uuid,
    ) -> Result<Vec<UserAccessGrantWithUsers>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT 
                    g.id, g.created_at, g.updated_at,
                    gr.id, gr.username, gr.full_name, gr.is_admin, gr.is_active, gr.created_at, gr.updated_at,
                    ge.id, ge.username, ge.full_name, ge.is_admin, ge.is_active, ge.created_at, ge.updated_at
                 FROM user_access_grants g
                 JOIN users gr ON g.grantor_user_id = gr.id
                 JOIN users ge ON g.grantee_user_id = ge.id
                 WHERE g.grantee_user_id = $1
                 ORDER BY g.created_at DESC",
                &[&grantee_user_id],
            )
            .await?;

        let grants = rows
            .iter()
            .map(|row| UserAccessGrantWithUsers {
                id: row.get(0),
                created_at: row.get(1),
                updated_at: row.get(2),
                grantor: UserResponse {
                    id: row.get(3),
                    username: row.get(4),
                    full_name: row.get(5),
                    is_admin: row.get(6),
                    is_active: row.get(7),
                    created_at: row.get(8),
                    updated_at: row.get(9),
                },
                grantee: UserResponse {
                    id: row.get(10),
                    username: row.get(11),
                    full_name: row.get(12),
                    is_admin: row.get(13),
                    is_active: row.get(14),
                    created_at: row.get(15),
                    updated_at: row.get(16),
                },
            })
            .collect();

        Ok(grants)
    }

    /// Delete a user access grant
    pub async fn delete_user_access_grant(&self, grant_id: Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows_affected = client
            .execute("DELETE FROM user_access_grants WHERE id = $1", &[&grant_id])
            .await?;

        Ok(rows_affected > 0)
    }

    /// Get a user access grant by ID
    pub async fn get_user_access_grant_by_id(
        &self,
        grant_id: Uuid,
    ) -> Result<Option<UserAccessGrant>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, grantor_user_id, grantee_user_id, created_at, updated_at 
                 FROM user_access_grants WHERE id = $1",
                &[&grant_id],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(Some(UserAccessGrant {
                id: row.get(0),
                grantor_user_id: row.get(1),
                grantee_user_id: row.get(2),
                created_at: row.get(3),
                updated_at: row.get(4),
            }))
        } else {
            Ok(None)
        }
    }

    // ==================== Ownership Transfer Operations ====================

    /// Transfer ownership of an inventory from one user to another
    /// This operation:
    /// 1. Updates the inventory's user_id to the new owner
    /// 2. Removes all existing shares for the inventory (new owner controls sharing)
    /// 3. The previous owner loses all access
    pub async fn transfer_inventory_ownership(
        &self,
        inventory_id: i32,
        from_user_id: Uuid,
        to_user_id: Uuid,
    ) -> Result<(i64, i64), Box<dyn std::error::Error>> {
        let mut client = self.pool.get().await?;
        
        // Start a transaction for atomic operation
        let transaction = client.transaction().await?;

        // Verify the inventory exists and is owned by from_user_id
        let verify_result = transaction
            .query_opt(
                "SELECT id FROM inventories WHERE id = $1 AND user_id = $2",
                &[&inventory_id, &from_user_id],
            )
            .await?;

        if verify_result.is_none() {
            return Err("Inventory not found or you are not the owner".into());
        }

        // Verify the target user exists
        let target_user = transaction
            .query_opt(
                "SELECT id FROM users WHERE id = $1 AND is_active = true",
                &[&to_user_id],
            )
            .await?;

        if target_user.is_none() {
            return Err("Target user not found or is inactive".into());
        }

        // Count items that will be transferred (for reporting)
        let items_count: i64 = transaction
            .query_one(
                "SELECT COUNT(*) FROM items WHERE inventory_id = $1",
                &[&inventory_id],
            )
            .await?
            .get(0);

        // Transfer ownership by updating user_id
        transaction
            .execute(
                "UPDATE inventories SET user_id = $1, updated_at = NOW() WHERE id = $2",
                &[&to_user_id, &inventory_id],
            )
            .await?;

        // Remove all existing shares for this inventory (new owner will manage sharing)
        let shares_removed = transaction
            .execute(
                "DELETE FROM inventory_shares WHERE inventory_id = $1",
                &[&inventory_id],
            )
            .await?;

        // Commit the transaction
        transaction.commit().await?;

        info!(
            "Transferred ownership of inventory {} from {:?} to {:?}. Items: {}, Shares removed: {}",
            inventory_id, from_user_id, to_user_id, items_count, shares_removed
        );

        Ok((items_count, shares_removed as i64))
    }
}


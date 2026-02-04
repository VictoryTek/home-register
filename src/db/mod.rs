use crate::models::{
    CreateInventoryRequest, CreateItemRequest, CreateOrganizerOptionRequest,
    CreateOrganizerTypeRequest, Inventory, Item, ItemOrganizerValue,
    ItemOrganizerValueWithDetails, OrganizerOption, OrganizerType, OrganizerTypeWithOptions,
    SetItemOrganizerValueRequest, UpdateItemRequest,
    UpdateOrganizerOptionRequest, UpdateOrganizerTypeRequest,
};
use chrono::{DateTime, Utc};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use log::info;
use std::env;
use tokio_postgres::NoTls;

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
    pub async fn get_all_inventories(&self) -> Result<Vec<Inventory>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, name, description, location, image_url, created_at, updated_at 
                 FROM inventories ORDER BY name ASC",
                &[],
            )
            .await?;

        let mut inventories = Vec::new();
        for row in rows {
            let inventory = Inventory {
                id: Some(row.get(0)),
                name: row.get(1),
                description: row.get(2),
                location: row.get(3),
                image_url: row.get(4),
                created_at: row.get::<_, Option<DateTime<Utc>>>(5),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(6),
            };
            inventories.push(inventory);
        }

        info!("Retrieved {} inventories from database", inventories.len());
        Ok(inventories)
    }

    pub async fn get_inventory_by_id(
        &self,
        id: i32,
    ) -> Result<Option<Inventory>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, name, description, location, image_url, created_at, updated_at 
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
                created_at: row.get::<_, Option<DateTime<Utc>>>(5),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(6),
            };
            Ok(Some(inventory))
        } else {
            Ok(None)
        }
    }

    pub async fn create_inventory(
        &self,
        request: CreateInventoryRequest,
    ) -> Result<Inventory, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let row = client
            .query_one(
                "INSERT INTO inventories (name, description, location, image_url) 
                 VALUES ($1, $2, $3, $4) 
                 RETURNING id, name, description, location, image_url, created_at, updated_at",
                &[&request.name, &request.description, &request.location, &request.image_url],
            )
            .await?;

        let inventory = Inventory {
            id: Some(row.get(0)),
            name: row.get(1),
            description: row.get(2),
            location: row.get(3),
            image_url: row.get(4),
            created_at: row.get::<_, Option<DateTime<Utc>>>(5),
            updated_at: row.get::<_, Option<DateTime<Utc>>>(6),
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
            "UPDATE inventories SET {} WHERE id = ${} RETURNING id, name, description, location, image_url, created_at, updated_at",
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
                created_at: row.get::<_, Option<DateTime<Utc>>>(5),
                updated_at: row.get::<_, Option<DateTime<Utc>>>(6),
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
}

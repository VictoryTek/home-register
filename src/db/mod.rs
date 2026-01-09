use crate::models::{CreateInventoryRequest, CreateItemRequest, Inventory, Item, UpdateItemRequest};
use chrono::{DateTime, NaiveDate, Utc};
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
}

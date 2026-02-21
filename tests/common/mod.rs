// Common test utilities

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use std::env;
use tokio_postgres::NoTls;

/// Create a test database pool
/// Uses `TEST_DATABASE_URL` env var if set, otherwise falls back to default test DB
#[allow(dead_code)]
pub fn create_test_pool() -> Pool {
    let database_url = env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:password@localhost:5432/home_inventory_test".to_string()
        })
    });

    let mut cfg = Config::new();
    let parts: Vec<&str> = database_url
        .trim_start_matches("postgres://")
        .split('@')
        .collect();

    if parts.len() == 2 {
        let user_pass: Vec<&str> = parts[0].split(':').collect();
        if user_pass.len() == 2 {
            cfg.user = Some(user_pass[0].to_string());
            cfg.password = Some(user_pass[1].to_string());
        }

        let host_db: Vec<&str> = parts[1].split('/').collect();
        if host_db.len() == 2 {
            let host_port: Vec<&str> = host_db[0].split(':').collect();
            if host_port.len() == 2 {
                cfg.host = Some(host_port[0].to_string());
                cfg.port = Some(host_port[1].parse().unwrap_or(5432));
            }
            cfg.dbname = Some(host_db[1].to_string());
        }
    }

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create test pool")
}

/// Generate a unique test username
/// Ensures the username is under 50 characters to fit DB constraint
#[allow(dead_code)]
pub fn test_username(prefix: &str) -> String {
    // Use first 8 characters of UUID to keep total length under 50
    let short_id = uuid::Uuid::new_v4()
        .to_string()
        .chars()
        .take(8)
        .collect::<String>();
    // Truncate prefix if needed to ensure total is under 50 chars
    let max_prefix_len = 40; // Leave room for underscore and short_id
    let truncated_prefix = if prefix.len() > max_prefix_len {
        &prefix[..max_prefix_len]
    } else {
        prefix
    };
    format!("{truncated_prefix}_{short_id}")
}

/// Generate a test password
#[allow(dead_code)]
pub fn test_password() -> String {
    "TestPassword123!".to_string()
}

/// Create a test user and return their credentials
#[allow(dead_code)]
pub async fn create_test_user(pool: &Pool, username: &str) -> (String, String) {
    use home_registry::db::DatabaseService;

    let password = test_password();
    let password_hash = home_registry::auth::hash_password(password.clone())
        .await
        .expect("Failed to hash password");

    let db = DatabaseService::new(pool.clone());
    let user = db
        .create_user(
            username,
            "Test User",    // full_name
            &password_hash, // password_hash
            false,          // is_admin
            true,           // is_active (must be true!)
        )
        .await
        .expect("Failed to create test user");

    (user.username, password)
}

/// Get a JWT token for a test user
#[allow(dead_code)]
pub async fn get_test_token(pool: &Pool, username: &str) -> String {
    use home_registry::auth::{generate_token, get_or_init_jwt_secret};
    use home_registry::db::DatabaseService;

    // Initialize JWT secret (critical for tests)
    get_or_init_jwt_secret();

    let db = DatabaseService::new(pool.clone());
    let user = db
        .get_user_by_username(username)
        .await
        .expect("Failed to get user")
        .expect("User not found");

    generate_token(&user).expect("Failed to create token")
}

/// Create an admin test user and return their credentials
#[allow(dead_code)]
pub async fn create_admin_user(pool: &Pool, username: &str) -> (String, String) {
    use home_registry::db::DatabaseService;

    let password = test_password();
    let password_hash = home_registry::auth::hash_password(password.clone())
        .await
        .expect("Failed to hash password");

    let db = DatabaseService::new(pool.clone());
    let user = db
        .create_user(
            username,
            "Test Admin",   // full_name
            &password_hash, // password_hash
            true,           // is_admin
            true,           // is_active (must be true!)
        )
        .await
        .expect("Failed to create admin user");

    (user.username, password)
}

/// Delete a test user by username
#[allow(dead_code)]
pub async fn delete_test_user(
    pool: &Pool,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use home_registry::db::DatabaseService;

    let db = DatabaseService::new(pool.clone());
    if let Some(user) = db.get_user_by_username(username).await? {
        db.delete_user(user.id).await?;
    }
    Ok(())
}

/// Create a test inventory and return its ID
#[allow(dead_code)]
pub async fn create_test_inventory(
    pool: &Pool,
    user_id: uuid::Uuid,
    name: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    use home_registry::db::DatabaseService;
    use home_registry::models::CreateInventoryRequest;

    let db = DatabaseService::new(pool.clone());
    let request = CreateInventoryRequest {
        name: name.to_string(),
        description: Some("Test inventory description".to_string()),
        location: Some("Test location".to_string()),
        image_url: None,
    };

    let inventory = db.create_inventory(request, user_id).await?;

    Ok(inventory.id.expect("Inventory should have ID"))
}

/// Delete a test inventory by ID
#[allow(dead_code)]
pub async fn delete_test_inventory(pool: &Pool, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    use home_registry::db::DatabaseService;

    let db = DatabaseService::new(pool.clone());
    db.delete_inventory(id).await?;
    Ok(())
}

/// Create a test item and return its ID
#[allow(dead_code)]
pub async fn create_test_item(
    pool: &Pool,
    inventory_id: i32,
    name: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    use home_registry::db::DatabaseService;
    use home_registry::models::CreateItemRequest;

    let db = DatabaseService::new(pool.clone());
    let request = CreateItemRequest {
        inventory_id: Some(inventory_id),
        name: name.to_string(),
        description: Some("Test item description".to_string()),
        category: None,
        location: None,
        purchase_date: None,
        purchase_price: None,
        warranty_expiry: None,
        notes: None,
        quantity: Some(1),
    };

    let item = db.create_item(request).await?;

    Ok(item.id.expect("Item should have ID"))
}

/// Delete a test item by ID
#[allow(dead_code)]
pub async fn delete_test_item(pool: &Pool, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    use home_registry::db::DatabaseService;

    let db = DatabaseService::new(pool.clone());
    db.delete_item(id).await?;
    Ok(())
}

/// Cleanup all test data for a given username prefix
#[allow(dead_code)]
pub async fn cleanup_test_data(
    pool: &Pool,
    username_prefix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use home_registry::db::DatabaseService;

    let db = DatabaseService::new(pool.clone());

    // Find and delete all test users matching the prefix
    let conn = pool.get().await?;
    let stmt = conn
        .prepare("SELECT id FROM users WHERE username LIKE $1")
        .await?;
    let rows = conn.query(&stmt, &[&format!("{username_prefix}%")]).await?;

    for row in rows {
        let user_id: uuid::Uuid = row.get(0);
        db.delete_user(user_id).await.ok(); // Ignore errors
    }

    Ok(())
}

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
#[allow(dead_code)]
pub fn test_username(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::new_v4())
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
            &password_hash,
            "Test User", // full_name
            false,       // is_admin
            false,       // recovery_codes_confirmed
        )
        .await
        .expect("Failed to create test user");

    (user.username, password)
}

/// Get a JWT token for a test user
#[allow(dead_code)]
pub async fn get_test_token(pool: &Pool, username: &str) -> String {
    use home_registry::auth::{create_token, get_or_init_jwt_secret};
    use home_registry::db::DatabaseService;

    // Initialize JWT secret
    let _ = get_or_init_jwt_secret();

    let db = DatabaseService::new(pool.clone());
    let user = db
        .get_user_by_username(username)
        .await
        .expect("Failed to get user")
        .expect("User not found");

    create_token(&user.id, &user.username).expect("Failed to create token")
}

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use std::env;
use tokio_postgres::NoTls;

pub async fn get_pool() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut cfg = Config::new();
    cfg.dbname = Some("home_inventory".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("password".to_string());
    cfg.host = Some("db".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    cfg.create_pool(None, NoTls).expect("Failed to create pool")
}

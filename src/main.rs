use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware::Logger};
use actix_files as fs;
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod api;

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "home-register",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8210".to_string());
    
    log::info!("Starting Home Inventory server at http://{}:{}", host, port);
    log::info!("Environment: {}", env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()));
    
    // Initialize database pool
    let pool = db::get_pool().await;
    log::info!("Database pool initialized successfully");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .route("/health", web::get().to(health))
            .service(api::init_routes())
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

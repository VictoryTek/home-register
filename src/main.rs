use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware::Logger, guard};
use actix_files as fs;
use dotenv::dotenv;
use std::env;

mod auth;
mod db;
mod models;
mod api;

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "home-registry",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now()
    }))
}

// Serve index.html for client-side routing (SPA fallback)
async fn spa_fallback() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8210".to_string());
    
    log::info!("Starting Home Inventory server at http://{}:{}", host, port);
    log::info!("Environment: {}", env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()));
    
    // Initialize JWT secret at startup (will auto-generate if not found)
    let _ = auth::get_or_init_jwt_secret();
    log::info!("JWT token lifetime: {} hours", auth::jwt_token_lifetime_hours());
    
    // Initialize database pool
    let pool = db::get_pool().await;
    log::info!("Database pool initialized successfully");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            // API routes - MUST come first
            .service(api::init_routes())
            .route("/health", web::get().to(health))
            // Serve static assets (js, css, images, etc.)
            .service(fs::Files::new("/assets", "static/assets"))
            // Root route - serve index.html
            .route("/", web::get().to(|| async {
                fs::NamedFile::open_async("static/index.html").await
            }))
            // Logo files at root level
            .route("/logo_icon.png", web::get().to(|| async {
                fs::NamedFile::open_async("static/logo_icon.png").await
            }))
            .route("/logo_full.png", web::get().to(|| async {
                fs::NamedFile::open_async("static/logo_full.png").await
            }))
            .route("/logo_icon3.png", web::get().to(|| async {
                fs::NamedFile::open_async("static/logo_icon3.png").await
            }))
            .route("/favicon.ico", web::get().to(|| async {
                fs::NamedFile::open_async("static/favicon.ico").await
            }))
            .route("/manifest.json", web::get().to(|| async {
                fs::NamedFile::open_async("static/manifest.json").await
            }))
            // Catch-all for SPA client-side routing - serve index.html for everything else
            // This comes last so API and static routes are handled first
            .route("/{path:.*}", web::get().to(spa_fallback))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

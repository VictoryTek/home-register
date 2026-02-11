use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware::{Logger, DefaultHeaders}};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
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
        "version": env!("CARGO_PKG_VERSION"),
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
    
    // Initialize database pool with proper error handling (no panics)
    let pool = match db::get_pool().await {
        Ok(p) => {
            log::info!("Database pool initialized successfully");
            p
        }
        Err(e) => {
            log::error!("Failed to initialize database pool: {}", e);
            std::process::exit(1);
        }
    };

    // Rate limiting configuration: 1 request every 100ms (10 per second), burst of 30
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(1)
        .burst_size(30)
        .finish()
        .expect("Failed to build rate limiter configuration");
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // Allow requests with no origin (same-origin requests)
                // Allow localhost in development
                let origin_str = origin.to_str().unwrap_or("");
                origin_str.starts_with("http://localhost")
                    || origin_str.starts_with("https://localhost")
                    || origin_str.starts_with("http://127.0.0.1")
                    || origin_str.starts_with("https://127.0.0.1")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Security headers
            .wrap(DefaultHeaders::new()
                .add(("X-Frame-Options", "DENY"))
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
                .add(("Permissions-Policy", "geolocation=(), microphone=(), camera=()"))
                .add(("Content-Security-Policy", 
                      "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'")))
            .wrap(cors)
            .wrap(Logger::default())
            // Global rate limiting
            .wrap(Governor::new(&governor_conf))
            // API routes
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
            // Service Worker files for PWA
            .route("/sw.js", web::get().to(|| async {
                fs::NamedFile::open_async("static/sw.js").await
            }))
            .route("/workbox-{filename:.*}.js", web::get().to(|path: web::Path<String>| async move {
                let filename = path.into_inner();
                fs::NamedFile::open_async(format!("static/workbox-{}", filename)).await
            }))
            // Catch-all for SPA client-side routing - serve index.html for everything else
            // This comes last so API and static routes are handled first
            .route("/{path:.*}", web::get().to(spa_fallback))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

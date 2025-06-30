use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod api;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    println!("Starting server at http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .service(api::init_routes())
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

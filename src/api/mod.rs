use actix_web::{get, post, web, HttpResponse, Responder, Result, Scope};
use crate::db::DatabaseService;
use crate::models::{ApiResponse, CreateItemRequest, ErrorResponse, CreateInventoryRequest};
use deadpool_postgres::Pool;
use log::{error, info};

#[get("/")]
pub async fn index() -> impl Responder {
    // Serve the static HTML file instead of embedded HTML
    match std::fs::read_to_string("static/index.html") {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(_) => HttpResponse::InternalServerError().body("Could not load index page")
    }
}

#[get("/health")]
pub async fn api_health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Home Inventory Manager is running",
        "timestamp": chrono::Utc::now()
    }))
}

// Inventories API endpoints
#[get("/inventories")]
pub async fn get_inventories(pool: web::Data<Pool>) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_all_inventories().await {
        Ok(inventories) => {
            info!("Successfully retrieved {} inventories from database", inventories.len());
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(inventories.clone()),
                message: Some(format!("Retrieved {} inventories", inventories.len())),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error retrieving inventories: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve inventories".to_string()),
            }))
        }
    }
}

#[post("/inventories")]
pub async fn create_inventory(
    pool: web::Data<Pool>,
    req: web::Json<CreateInventoryRequest>
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.create_inventory(req.into_inner()).await {
        Ok(inventory) => {
            info!("Successfully created inventory: {}", inventory.name);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(inventory),
                message: Some("Inventory created successfully".to_string()),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error creating inventory: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to create inventory".to_string()),
            }))
        }
    }
}

#[get("/inventories/{id}")]
pub async fn get_inventory(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_inventory_by_id(inventory_id).await {
        Ok(Some(inventory)) => {
            info!("Successfully retrieved inventory with id: {}", inventory_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(inventory),
                message: Some("Inventory retrieved successfully".to_string()),
                error: None,
            }))
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Inventory with id {} not found", inventory_id),
                message: Some("Inventory not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error retrieving inventory: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve inventory".to_string()),
            }))
        }
    }
}

// Inventory-specific item endpoints
#[get("/inventories/{id}/items")]
pub async fn get_inventory_items(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_items_by_inventory(inventory_id).await {
        Ok(items) => {
            info!("Successfully retrieved {} items for inventory {}", items.len(), inventory_id);
            let items_count = items.len();
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(items),
                message: Some(format!("Retrieved {} items for inventory", items_count)),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error retrieving items for inventory {}: {}", inventory_id, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve inventory items".to_string()),
            }))
        }
    }
}

#[post("/inventories/{id}/items")]
pub async fn create_inventory_item(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    mut req: web::Json<CreateItemRequest>
) -> Result<impl Responder> {
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    // Ensure the inventory_id in the request matches the path parameter
    req.inventory_id = Some(inventory_id);
    
    match db_service.create_item(req.into_inner()).await {
        Ok(item) => {
            info!("Successfully created item '{}' for inventory {}", item.name, inventory_id);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(item),
                message: Some("Item created successfully".to_string()),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error creating item for inventory {}: {}", inventory_id, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to create item".to_string()),
            }))
        }
    }
}

// Create scope with all API routes
pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(api_health)
        // Inventory routes
        .service(get_inventories)
        .service(create_inventory)
        .service(get_inventory)
        // Inventory-specific item routes
        .service(get_inventory_items)
        .service(create_inventory_item)
}

// Alias for backward compatibility
pub fn init_routes() -> Scope {
    api_scope()
}

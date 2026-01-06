use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result, Scope};
use crate::db::DatabaseService;
use crate::models::{ApiResponse, CreateItemRequest, ErrorResponse, UpdateItemRequest, CreateInventoryRequest, UpdateInventoryRequest};
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
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(items.clone()),
                message: Some(format!("Retrieved {} items", items.len())),
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

#[put("/inventories/{id}")]
pub async fn update_inventory(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::Json<UpdateInventoryRequest>
) -> Result<impl Responder> {
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.update_inventory(inventory_id, req.into_inner()).await {
        Ok(Some(inventory)) => {
            info!("Successfully updated inventory with id: {}", inventory_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(inventory),
                message: Some("Inventory updated successfully".to_string()),
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
            error!("Error updating inventory: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to update inventory".to_string()),
            }))
        }
    }
}

// Items API endpoints
#[get("/items")]
pub async fn get_items(pool: web::Data<Pool>) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_all_items().await {
        Ok(items) => {
            info!("Successfully retrieved {} items from database", items.len());
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(items.clone()),
                message: Some(format!("Retrieved {} items", items.len())),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error retrieving items: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve items".to_string()),
            }))
        }
    }
}

#[get("/items/{id}")]
pub async fn get_item(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let item_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_item_by_id(item_id).await {
        Ok(Some(item)) => {
            info!("Successfully retrieved item with id: {}", item_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(item),
                message: Some("Item retrieved successfully".to_string()),
                error: None,
            }))
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Item with id {} not found", item_id),
                message: Some("Item not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error retrieving item: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve item".to_string()),
            }))
        }
    }
}

#[post("/items")]
pub async fn create_item(
    pool: web::Data<Pool>,
    req: web::Json<CreateItemRequest>
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.create_item(req.into_inner()).await {
        Ok(item) => {
            info!("Successfully created item: {}", item.name);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(item),
                message: Some("Item created successfully".to_string()),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error creating item: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to create item".to_string()),
            }))
        }
    }
}

#[put("/items/{id}")]
pub async fn update_item(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::Json<UpdateItemRequest>
) -> Result<impl Responder> {
    let item_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.update_item(
        item_id,
        req.into_inner()
    ).await {
        Ok(Some(item)) => {
            info!("Successfully updated item with id: {}", item_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(item),
                message: Some("Item updated successfully".to_string()),
                error: None,
            }))
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Item with id {} not found", item_id),
                message: Some("Item not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error updating item: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to update item".to_string()),
            }))
        }
    }
}

#[delete("/items/{id}")]
pub async fn delete_item(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let item_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.delete_item(item_id).await {
        Ok(true) => {
            info!("Successfully deleted item with id: {}", item_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(()),
                message: Some("Item deleted successfully".to_string()),
                error: None,
            }))
        },
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Item with id {} not found", item_id),
                message: Some("Item not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error deleting item: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to delete item".to_string()),
            }))
        }
    }
}

#[get("/items/search/{query}")]
pub async fn search_items(
    pool: web::Data<Pool>,
    path: web::Path<String>
) -> Result<impl Responder> {
    let query = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.search_items(&query).await {
        Ok(items) => {
            info!("Successfully searched items with query '{}', found {} results", query, items.len());
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(items.clone()),
                message: Some(format!("Found {} items matching '{}'", items.len(), query)),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error searching items: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to search items".to_string()),
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
        .service(get_inventory_items)
        .service(update_inventory)
        // Item routes
        .service(get_items)
        .service(get_item)
        .service(create_item)
        .service(update_item)
        .service(delete_item)
        .service(search_items)
}

// Alias for backward compatibility
pub fn init_routes() -> Scope {
    api_scope()
}

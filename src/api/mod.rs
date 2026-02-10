pub mod auth;

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder, Result, Scope};
use crate::db::DatabaseService;
use crate::models::{
    ApiResponse, CreateItemRequest, ErrorResponse, UpdateItemRequest, 
    CreateInventoryRequest, UpdateInventoryRequest,
    CreateOrganizerTypeRequest, UpdateOrganizerTypeRequest,
    CreateOrganizerOptionRequest, UpdateOrganizerOptionRequest,
    SetItemOrganizerValuesRequest,
};
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
    http_req: HttpRequest,
    req: web::Json<CreateInventoryRequest>
) -> Result<impl Responder> {
    let auth = match auth::get_auth_context_from_request(&http_req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };
    
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.create_inventory(req.into_inner(), auth.user_id).await {
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

#[delete("/inventories/{id}")]
pub async fn delete_inventory(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.delete_inventory(inventory_id).await {
        Ok(true) => {
            info!("Successfully deleted inventory with id: {}", inventory_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(()),
                message: Some("Inventory deleted successfully".to_string()),
                error: None,
            }))
        },
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Inventory with id {} not found", inventory_id),
                message: Some("Inventory not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error deleting inventory: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to delete inventory".to_string()),
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

// ==================== Organizer Type Endpoints ====================

#[get("/inventories/{id}/organizers")]
pub async fn get_inventory_organizers(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_organizer_types_with_options_by_inventory(inventory_id).await {
        Ok(organizers) => {
            info!("Successfully retrieved {} organizers for inventory {}", organizers.len(), inventory_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(organizers.clone()),
                message: Some(format!("Retrieved {} organizers", organizers.len())),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error retrieving organizers: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve organizers".to_string()),
            }))
        }
    }
}

#[post("/inventories/{id}/organizers")]
pub async fn create_organizer_type(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::Json<CreateOrganizerTypeRequest>
) -> Result<impl Responder> {
    let inventory_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.create_organizer_type(inventory_id, req.into_inner()).await {
        Ok(organizer) => {
            info!("Successfully created organizer type: {}", organizer.name);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(organizer),
                message: Some("Organizer type created successfully".to_string()),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error creating organizer type: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to create organizer type".to_string()),
            }))
        }
    }
}

#[get("/organizers/{id}")]
pub async fn get_organizer_type(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let organizer_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_organizer_type_by_id(organizer_id).await {
        Ok(Some(organizer)) => {
            info!("Successfully retrieved organizer type with id: {}", organizer_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(organizer),
                message: Some("Organizer type retrieved successfully".to_string()),
                error: None,
            }))
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Organizer type with id {} not found", organizer_id),
                message: Some("Organizer type not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error retrieving organizer type: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve organizer type".to_string()),
            }))
        }
    }
}

#[put("/organizers/{id}")]
pub async fn update_organizer_type(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::Json<UpdateOrganizerTypeRequest>
) -> Result<impl Responder> {
    let organizer_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.update_organizer_type(organizer_id, req.into_inner()).await {
        Ok(Some(organizer)) => {
            info!("Successfully updated organizer type with id: {}", organizer_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(organizer),
                message: Some("Organizer type updated successfully".to_string()),
                error: None,
            }))
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Organizer type with id {} not found", organizer_id),
                message: Some("Organizer type not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error updating organizer type: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to update organizer type".to_string()),
            }))
        }
    }
}

#[delete("/organizers/{id}")]
pub async fn delete_organizer_type(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let organizer_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.delete_organizer_type(organizer_id).await {
        Ok(true) => {
            info!("Successfully deleted organizer type with id: {}", organizer_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(()),
                message: Some("Organizer type deleted successfully".to_string()),
                error: None,
            }))
        },
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Organizer type with id {} not found", organizer_id),
                message: Some("Organizer type not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error deleting organizer type: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to delete organizer type".to_string()),
            }))
        }
    }
}

// ==================== Organizer Option Endpoints ====================

#[get("/organizers/{id}/options")]
pub async fn get_organizer_options(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let organizer_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_organizer_options(organizer_id).await {
        Ok(options) => {
            info!("Successfully retrieved {} options for organizer {}", options.len(), organizer_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(options.clone()),
                message: Some(format!("Retrieved {} options", options.len())),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error retrieving organizer options: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve organizer options".to_string()),
            }))
        }
    }
}

#[post("/organizers/{id}/options")]
pub async fn create_organizer_option(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::Json<CreateOrganizerOptionRequest>
) -> Result<impl Responder> {
    let organizer_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.create_organizer_option(organizer_id, req.into_inner()).await {
        Ok(option) => {
            info!("Successfully created organizer option: {}", option.name);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(option),
                message: Some("Organizer option created successfully".to_string()),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error creating organizer option: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to create organizer option".to_string()),
            }))
        }
    }
}

#[put("/organizer-options/{id}")]
pub async fn update_organizer_option(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::Json<UpdateOrganizerOptionRequest>
) -> Result<impl Responder> {
    let option_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.update_organizer_option(option_id, req.into_inner()).await {
        Ok(Some(option)) => {
            info!("Successfully updated organizer option with id: {}", option_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(option),
                message: Some("Organizer option updated successfully".to_string()),
                error: None,
            }))
        },
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Organizer option with id {} not found", option_id),
                message: Some("Organizer option not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error updating organizer option: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to update organizer option".to_string()),
            }))
        }
    }
}

#[delete("/organizer-options/{id}")]
pub async fn delete_organizer_option(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let option_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.delete_organizer_option(option_id).await {
        Ok(true) => {
            info!("Successfully deleted organizer option with id: {}", option_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(()),
                message: Some("Organizer option deleted successfully".to_string()),
                error: None,
            }))
        },
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Organizer option with id {} not found", option_id),
                message: Some("Organizer option not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error deleting organizer option: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to delete organizer option".to_string()),
            }))
        }
    }
}

// ==================== Item Organizer Value Endpoints ====================

#[get("/items/{id}/organizer-values")]
pub async fn get_item_organizer_values(
    pool: web::Data<Pool>,
    path: web::Path<i32>
) -> Result<impl Responder> {
    let item_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_item_organizer_values(item_id).await {
        Ok(values) => {
            info!("Successfully retrieved {} organizer values for item {}", values.len(), item_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(values.clone()),
                message: Some(format!("Retrieved {} organizer values", values.len())),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error retrieving item organizer values: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to retrieve item organizer values".to_string()),
            }))
        }
    }
}

#[put("/items/{id}/organizer-values")]
pub async fn set_item_organizer_values(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::Json<SetItemOrganizerValuesRequest>
) -> Result<impl Responder> {
    let item_id = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.set_item_organizer_values(item_id, req.into_inner().values).await {
        Ok(values) => {
            info!("Successfully set {} organizer values for item {}", values.len(), item_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(values),
                message: Some("Item organizer values updated successfully".to_string()),
                error: None,
            }))
        },
        Err(e) => {
            error!("Error setting item organizer values: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to set item organizer values".to_string()),
            }))
        }
    }
}

#[delete("/items/{item_id}/organizer-values/{organizer_type_id}")]
pub async fn delete_item_organizer_value(
    pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>
) -> Result<impl Responder> {
    let (item_id, organizer_type_id) = path.into_inner();
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.delete_item_organizer_value(item_id, organizer_type_id).await {
        Ok(true) => {
            info!("Successfully deleted organizer value for item {} type {}", item_id, organizer_type_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(()),
                message: Some("Item organizer value deleted successfully".to_string()),
                error: None,
            }))
        },
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Organizer value not found for item {} type {}", item_id, organizer_type_id),
                message: Some("Item organizer value not found".to_string()),
            }))
        },
        Err(e) => {
            error!("Error deleting item organizer value: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                message: Some("Failed to delete item organizer value".to_string()),
            }))
        }
    }
}

// API 404 handler - returns JSON instead of HTML
async fn api_not_found(req: HttpRequest) -> impl Responder {
    log::warn!("API 404: {}", req.uri());
    HttpResponse::NotFound().json(ErrorResponse {
        success: false,
        error: "Endpoint not found".to_string(),
        message: Some(format!("The API endpoint {} does not exist", req.uri())),
    })
}

// Create scope with all API routes
pub fn api_scope() -> Scope {
    web::scope("/api")
        .service(api_health)
        // Auth routes (setup, login, profile, admin user management) - imported directly to avoid nested scope
        .service(auth::get_setup_status)
        .service(auth::initial_setup)
        .service(auth::login)
        .service(auth::register)
        .service(auth::get_current_user)
        .service(auth::update_current_user)  
        .service(auth::change_password)
        .service(auth::get_user_settings)
        .service(auth::update_user_settings)
        .service(auth::get_my_inventories)
        .service(auth::get_inventory_shares)
        .service(auth::create_inventory_share)
        .service(auth::update_inventory_share)
        .service(auth::delete_inventory_share)
        .service(auth::get_my_access_grants)
        .service(auth::get_received_access_grants)
        .service(auth::create_access_grant)
        .service(auth::delete_access_grant)
        .service(auth::admin_get_users)
        .service(auth::admin_get_user)
        .service(auth::admin_create_user)
        .service(auth::admin_update_user)
        .service(auth::admin_delete_user)
        // Inventory routes
        .service(get_inventories)
        .service(create_inventory)
        .service(get_inventory)
        .service(get_inventory_items)
        .service(get_inventory_organizers)
        .service(create_organizer_type)
        .service(update_inventory)
        .service(delete_inventory)
        // Item routes
        .service(get_items)
        .service(get_item)
        .service(create_item)
        .service(update_item)
        .service(delete_item)
        .service(search_items)
        .service(get_item_organizer_values)
        .service(set_item_organizer_values)
        .service(delete_item_organizer_value)
        // Organizer routes
        .service(get_organizer_type)
        .service(update_organizer_type)
        .service(delete_organizer_type)
        .service(get_organizer_options)
        .service(create_organizer_option)
        .service(update_organizer_option)
        .service(delete_organizer_option)
        // Catch-all for non-existent API endpoints
        .default_service(web::to(api_not_found))
}

// Alias for backward compatibility
pub fn init_routes() -> Scope {
    api_scope()
}

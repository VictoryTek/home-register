//! Backup & Restore API endpoints
//!
//! Provides endpoints for creating, listing, downloading, uploading,
//! restoring, and deleting database backups. All endpoints require
//! admin authentication.

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder, Result};
use deadpool_postgres::Pool;
use log::{error, info};
use std::path::Path;

use crate::api::auth;
use crate::db::DatabaseService;
use crate::models::{ApiResponse, BackupData, BackupInfo, BackupMetadata, ErrorResponse};

/// Directory where backup files are stored
const BACKUPS_DIR: &str = "backups";

/// Multipart form for backup file upload
#[derive(MultipartForm)]
struct BackupUploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

// ==================== Helper Functions ====================

/// Validate backup filename to prevent path traversal attacks
fn validate_backup_filename(filename: &str) -> std::result::Result<(), String> {
    if !Path::new(filename)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        return Err("Only .json backup files are allowed".to_string());
    }
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err("Invalid filename".to_string());
    }
    if !filename.starts_with("home_registry_") {
        return Err("Invalid backup filename format".to_string());
    }
    Ok(())
}

/// Format file size in human-readable format
#[allow(clippy::cast_precision_loss)]
fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Generate a timestamped backup filename
fn generate_backup_filename(prefix: &str) -> String {
    let timestamp = chrono::Utc::now().format("%Y.%m.%d.%H.%M.%S").to_string();
    format!("{prefix}_{timestamp}.json")
}

/// Ensure the backups directory exists
async fn ensure_backups_dir() -> std::io::Result<()> {
    tokio::fs::create_dir_all(BACKUPS_DIR).await
}

/// Create a backup file and return its info
async fn create_backup_file(
    db_service: &DatabaseService,
    filename_prefix: &str,
) -> std::result::Result<BackupInfo, Box<dyn std::error::Error>> {
    ensure_backups_dir().await?;

    let filename = generate_backup_filename(filename_prefix);
    let filepath = format!("{BACKUPS_DIR}/{filename}");

    // Export all database data
    let db_content = db_service.export_all_data().await?;

    // Create backup envelope with metadata
    let backup_data = BackupData {
        metadata: BackupMetadata {
            version: "1.0".to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            database_type: "postgresql".to_string(),
            description: None,
        },
        data: db_content,
    };

    // Serialize to pretty JSON
    let json_content = serde_json::to_string_pretty(&backup_data)?;
    let file_size = json_content.len() as u64;

    // Write to file
    tokio::fs::write(&filepath, &json_content).await?;

    info!(
        "Backup created: {} ({})",
        filename,
        format_file_size(file_size)
    );

    Ok(BackupInfo {
        name: filename,
        date: backup_data.metadata.created_at,
        size: format_file_size(file_size),
    })
}

// ==================== API Handlers ====================

/// Create a new backup of all database data
///
/// POST /api/backup/create
/// Requires: Admin authentication
#[post("/backup/create")]
pub async fn create_backup(pool: web::Data<Pool>, req: HttpRequest) -> Result<impl Responder> {
    let auth = match auth::require_admin(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };

    let db_service = DatabaseService::new(pool.get_ref().clone());

    match create_backup_file(&db_service, "home_registry").await {
        Ok(backup_info) => {
            info!(
                "Backup created by admin user {}: {}",
                auth.username, backup_info.name
            );
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(backup_info),
                message: Some("Backup created successfully".to_string()),
                error: None,
            }))
        },
        Err(e) => {
            error!("Failed to create backup: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some("Failed to create backup".to_string()),
            }))
        },
    }
}

/// List all available backup files
///
/// GET /api/backup/list
/// Requires: Admin authentication
#[get("/backup/list")]
pub async fn list_backups(pool: web::Data<Pool>, req: HttpRequest) -> Result<impl Responder> {
    let auth = match auth::require_admin(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };

    // Ensure backups directory exists
    if let Err(e) = ensure_backups_dir().await {
        error!("Failed to create backups directory: {}", e);
        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: "An internal error occurred".to_string(),
            message: Some("Failed to access backups directory".to_string()),
        }));
    }

    // Read directory and collect backup file info
    let mut backups: Vec<BackupInfo> = Vec::new();
    let mut entries = match tokio::fs::read_dir(BACKUPS_DIR).await {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to read backups directory: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some("Failed to read backups directory".to_string()),
            }));
        },
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "json" {
                if let Ok(metadata) = entry.metadata().await {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let date = metadata
                        .modified()
                        .map(|t| {
                            let datetime: chrono::DateTime<chrono::Utc> = t.into();
                            datetime.to_rfc3339()
                        })
                        .unwrap_or_default();
                    let size = format_file_size(metadata.len());

                    backups.push(BackupInfo { name, date, size });
                }
            }
        }
    }

    // Sort by date, newest first
    backups.sort_by(|a, b| b.date.cmp(&a.date));

    let count = backups.len();
    info!("Listed {} backups for admin user {}", count, auth.username);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(backups),
        message: Some(format!("Retrieved {count} backups")),
        error: None,
    }))
}

/// Download a backup file
///
/// GET /api/backup/download/{filename}
/// Requires: Admin authentication
#[get("/backup/download/{filename}")]
pub async fn download_backup(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let auth = match auth::require_admin(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };

    let filename = path.into_inner();

    // Validate filename to prevent path traversal
    if let Err(e) = validate_backup_filename(&filename) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e,
            message: Some("Invalid backup filename".to_string()),
        }));
    }

    let filepath = format!("{BACKUPS_DIR}/{filename}");

    // Verify file exists
    if !tokio::fs::try_exists(&filepath).await.unwrap_or(false) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: "Backup file not found".to_string(),
            message: Some(format!("Backup '{filename}' does not exist")),
        }));
    }

    // Read file content
    match tokio::fs::read(&filepath).await {
        Ok(content) => {
            info!(
                "Backup '{}' downloaded by admin user {}",
                filename, auth.username
            );
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .insert_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{filename}\""),
                ))
                .body(content))
        },
        Err(e) => {
            error!("Failed to read backup file '{}': {}", filename, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some("Failed to read backup file".to_string()),
            }))
        },
    }
}

/// Upload a backup file
///
/// POST /api/backup/upload
/// Requires: Admin authentication
/// Accepts: multipart/form-data with a 'file' field containing a .json backup
#[post("/backup/upload")]
pub async fn upload_backup(
    pool: web::Data<Pool>,
    req: HttpRequest,
    MultipartForm(form): MultipartForm<BackupUploadForm>,
) -> Result<impl Responder> {
    let auth = match auth::require_admin(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };

    // Get the original filename
    let original_filename = form
        .file
        .file_name
        .as_deref()
        .unwrap_or("unknown.json")
        .to_string();

    // Validate file extension
    if !Path::new(&original_filename)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Only .json backup files are allowed".to_string(),
            message: Some("Please upload a valid JSON backup file".to_string()),
        }));
    }

    // Read the temp file content
    let temp_path = form.file.file.path();
    let content = match tokio::fs::read(temp_path).await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read uploaded file: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some("Failed to process uploaded file".to_string()),
            }));
        },
    };

    // Validate JSON structure
    let backup_data: BackupData = match serde_json::from_slice(&content) {
        Ok(data) => data,
        Err(e) => {
            error!("Invalid backup file format: {}", e);
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid backup file format".to_string(),
                message: Some("The uploaded file is not a valid Home Registry backup".to_string()),
            }));
        },
    };

    // Validate backup version
    if backup_data.metadata.version != "1.0" {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!(
                "Unsupported backup version: {}",
                backup_data.metadata.version
            ),
            message: Some("This backup version is not supported".to_string()),
        }));
    }

    // Ensure backups directory exists
    if let Err(e) = ensure_backups_dir().await {
        error!("Failed to create backups directory: {}", e);
        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: "An internal error occurred".to_string(),
            message: Some("Failed to save backup file".to_string()),
        }));
    }

    // Determine target filename — use the original name if valid, otherwise generate
    let mut target_filename = if validate_backup_filename(&original_filename).is_ok() {
        original_filename
    } else {
        generate_backup_filename("home_registry")
    };

    // Avoid silently overwriting an existing backup file
    let mut filepath = format!("{BACKUPS_DIR}/{target_filename}");
    if tokio::fs::try_exists(&filepath).await.unwrap_or(false) {
        // Append a numeric suffix to make the filename unique
        let stem = target_filename.trim_end_matches(".json");
        for i in 1..=100 {
            let candidate = format!("{stem}_{i}.json");
            let candidate_path = format!("{BACKUPS_DIR}/{candidate}");
            if !tokio::fs::try_exists(&candidate_path)
                .await
                .unwrap_or(false)
            {
                target_filename = candidate;
                filepath = candidate_path;
                break;
            }
        }
    }

    let file_size = content.len() as u64;

    // Write to backups directory
    if let Err(e) = tokio::fs::write(&filepath, &content).await {
        error!("Failed to save uploaded backup: {}", e);
        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: "An internal error occurred".to_string(),
            message: Some("Failed to save backup file".to_string()),
        }));
    }

    info!(
        "Backup '{}' uploaded by admin user {} ({})",
        target_filename,
        auth.username,
        format_file_size(file_size)
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(BackupInfo {
            name: target_filename,
            date: backup_data.metadata.created_at,
            size: format_file_size(file_size),
        }),
        message: Some("Backup uploaded successfully".to_string()),
        error: None,
    }))
}

/// Restore database from a backup file
///
/// POST /api/backup/restore/{filename}
/// Requires: Admin authentication
/// Creates an automatic backup before restoring (safety net)
#[post("/backup/restore/{filename}")]
pub async fn restore_backup(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let auth = match auth::require_admin(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };

    let filename = path.into_inner();

    // Validate filename
    if let Err(e) = validate_backup_filename(&filename) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e,
            message: Some("Invalid backup filename".to_string()),
        }));
    }

    let filepath = format!("{BACKUPS_DIR}/{filename}");

    // Verify backup file exists
    if !tokio::fs::try_exists(&filepath).await.unwrap_or(false) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: "Backup file not found".to_string(),
            message: Some(format!("Backup '{filename}' does not exist")),
        }));
    }

    let db_service = DatabaseService::new(pool.get_ref().clone());

    // AUTO-BACKUP: Create a backup before restoring (safety net)
    let auto_backup_info = match create_backup_file(&db_service, "home_registry_auto_pre_restore")
        .await
    {
        Ok(info) => {
            info!("Auto-backup created before restore: {}", info.name);
            info
        },
        Err(e) => {
            error!("Failed to create auto-backup before restore: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some(
                    "Failed to create safety backup before restore. Restore aborted.".to_string(),
                ),
            }));
        },
    };

    // Read and parse the backup file
    let content = match tokio::fs::read(&filepath).await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read backup file '{}': {}", filename, e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some("Failed to read backup file".to_string()),
            }));
        },
    };

    let backup_data: BackupData = match serde_json::from_slice(&content) {
        Ok(data) => data,
        Err(e) => {
            error!("Invalid backup file format '{}': {}", filename, e);
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid backup file format".to_string(),
                message: Some("The backup file could not be parsed".to_string()),
            }));
        },
    };

    // Validate backup version
    if backup_data.metadata.version != "1.0" {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!(
                "Unsupported backup version: {}",
                backup_data.metadata.version
            ),
            message: Some("This backup version is not supported".to_string()),
        }));
    }

    // Perform the restore within a transaction
    match db_service.import_all_data(&backup_data.data).await {
        Ok(()) => {
            info!(
                "Backup '{}' restored by admin user {}",
                filename, auth.username
            );
            Ok(HttpResponse::Ok().json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some(format!(
                    "Backup restored successfully. A pre-restore backup was created: {}",
                    auto_backup_info.name
                )),
                error: None,
            }))
        },
        Err(e) => {
            error!("Failed to restore backup '{}': {}", filename, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some(format!(
                    "Failed to restore backup. Your pre-restore backup is available: {}",
                    auto_backup_info.name
                )),
            }))
        },
    }
}

/// Delete a backup file
///
/// DELETE /api/backup/{filename}
/// Requires: Admin authentication
#[delete("/backup/{filename}")]
pub async fn delete_backup(
    pool: web::Data<Pool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let auth = match auth::require_admin(&req, pool.get_ref()).await {
        Ok(a) => a,
        Err(e) => return Ok(e),
    };

    let filename = path.into_inner();

    // Validate filename to prevent path traversal
    if let Err(e) = validate_backup_filename(&filename) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e,
            message: Some("Invalid backup filename".to_string()),
        }));
    }

    let filepath = format!("{BACKUPS_DIR}/{filename}");

    // Verify file exists
    if !tokio::fs::try_exists(&filepath).await.unwrap_or(false) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: "Backup file not found".to_string(),
            message: Some(format!("Backup '{filename}' does not exist")),
        }));
    }

    // Delete the file
    match tokio::fs::remove_file(&filepath).await {
        Ok(()) => {
            info!(
                "Backup '{}' deleted by admin user {}",
                filename, auth.username
            );
            Ok(HttpResponse::Ok().json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some(format!("Backup {filename} deleted successfully")),
                error: None,
            }))
        },
        Err(e) => {
            error!("Failed to delete backup '{}': {}", filename, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "An internal error occurred".to_string(),
                message: Some("Failed to delete backup file".to_string()),
            }))
        },
    }
}

# Home Registry - AI Coding Instructions

## Project Overview
Home Registry is a Rust-based home inventory management system inspired by the HomeBox project. Built with **Actix-Web** + **PostgreSQL**, it provides a web-based interface to track belongings with categories, tags, and custom fields.

**Tech Stack**: Rust 2021 edition, Actix-Web 4, PostgreSQL 16, deadpool-postgres for connection pooling, tokio async runtime.

## ⚠️ CRITICAL: Use Context7 First

**BEFORE implementing ANY new feature, adding dependencies, or making significant changes:**

1. **Resolve the library ID**: Use `resolve-library-id` with the library/framework name
   - Examples: "actix-web", "tokio-postgres", "deadpool-postgres", "serde_json"
2. **Fetch current documentation**: Use `get-library-docs` with the Context7-compatible library ID
3. **Review official patterns**: Study current API examples, best practices, and recommended approaches
4. **Implement using current standards**: Follow the library's latest patterns to avoid deprecated code

**Why this matters**: Libraries evolve quickly. Context7 ensures you're using current APIs, not outdated patterns. This prevents technical debt and compatibility issues.

**Apply this rule to:**
- Adding new dependencies to Cargo.toml
- Implementing new API endpoints with Actix-Web
- Database operations with tokio-postgres/deadpool
- JSON serialization patterns with serde
- Any external crate integration

## Architecture & Data Flow

### Layer Structure (3-tier)
1. **API Layer** ([src/api/mod.rs](../src/api/mod.rs)): Actix-Web handlers exposing REST endpoints under `/api` prefix
2. **Database Service** ([src/db/mod.rs](../src/db/mod.rs)): `DatabaseService` struct wraps database operations with connection pooling
3. **Models** ([src/models/mod.rs](../src/models/mod.rs)): Serde-enabled structs for serialization/deserialization

### Request Flow Example
```
HTTP Request → API Handler → DatabaseService method → PostgreSQL → Response wrapped in ApiResponse<T>
```

All API responses follow this pattern:
- Success: `ApiResponse { success: true, data: Some(T), message: Option<String>, error: None }`
- Error: `ErrorResponse { success: false, error: String, message: Option<String> }`

### Database Schema Evolution
Migrations in `migrations/` run automatically via Docker's `docker-entrypoint-initdb.d`. Key tables:
- **items**: Core inventory items (id, name, description, category, location, purchase_date, purchase_price, warranty_expiry, quantity, inventory_id)
- **inventories**: Logical groupings of items (id, name, description, location)
- **categories**, **tags**, **custom_fields**: Planned future features (tables created, API not fully implemented)

**Migration Pattern**: Sequential numbered files (001_, 002_, etc.). Later migrations may alter earlier schemas (e.g., 003-008 add columns/foreign keys).

## Development Workflow

### Local Development (Docker)
```bash
# Start PostgreSQL + app containers
docker-compose up -d

# View app logs
docker-compose logs -f app

# Access: http://localhost:8210
# Database: localhost:5432, user: postgres, pass: password, db: home_inventory
```

**Note**: Migrations run automatically on DB container startup. Static files served from `static/` directory.

### Building & Running Locally
```bash
# Requires DATABASE_URL environment variable
export DATABASE_URL="postgres://postgres:password@localhost:5432/home_inventory"
cargo build --release
cargo run
```

**Environment Variables** (in `.env` or docker-compose.yml):
- `DATABASE_URL`: PostgreSQL connection string
- `HOST`: Server bind address (default: 0.0.0.0)
- `PORT`: Server port (default: 8210)
- `RUST_ENV`: Environment mode (development/production)

### Testing
Tests are minimal currently (placeholder in [tests/integration_test.rs](../tests/integration_test.rs)). Run with:
```bash
cargo test
```

## Code Conventions & Patterns

### Database Service Pattern
All DB operations go through `DatabaseService` struct methods. Never query PostgreSQL directly in handlers.

```rust
// In API handler:
let db_service = DatabaseService::new(pool.get_ref().clone());
match db_service.get_item_by_id(id).await {
    Ok(Some(item)) => // handle success
    Ok(None) => // handle not found
    Err(e) => // handle error
}
```

### Date Handling Quirk
Purchase dates and warranty expiry are stored as `DATE` in PostgreSQL but handled as `Option<String>` in Rust models (not `NaiveDate`). When querying, cast to text:
```sql
SELECT purchase_date::text, warranty_expiry::text FROM items
```

### Price Handling
Purchase prices use `DECIMAL(10,2)` in DB but `Option<f64>` in Rust. When querying, cast to `float8`:
```sql
SELECT purchase_price::float8 FROM items
```

### API Route Organization
Routes use Actix-Web's procedural macros (`#[get]`, `#[post]`, etc.) and are grouped in `api_scope()`:
- Endpoint naming: `/api/{resource}` and `/api/{resource}/{id}`
- All handlers return `Result<impl Responder>` for error handling flexibility
- Use `web::Data<Pool>` for dependency injection of database pool

### Logging
Use `log` crate with `env_logger`. Log patterns:
- `info!()`: Successful operations with counts/IDs
- `error!()`: Failed operations with error details
- Logged in both handlers ([src/api/mod.rs](../src/api/mod.rs)) and DB service ([src/db/mod.rs](../src/db/mod.rs))

### File Organization Note
`src/api/` contains `mod_old.rs` and `mod_new.rs` (unused legacy/experimental files). Active API code is in `mod.rs`.

## Key Integration Points

### Database Pool Initialization
Pool created in [main.rs](../src/main.rs) at startup using custom `get_pool()` function that parses `DATABASE_URL` manually (not using standard parsing). Pool shared via `web::Data` across all handlers.

### Static File Serving
Actix-Files serves `static/` directory with index.html as fallback:
```rust
.service(fs::Files::new("/", "static/").index_file("index.html"))
```

### Health Check
`/health` endpoint returns JSON with service status, version (hardcoded "0.1.0"), and timestamp. Used for container health checks.

## Common Tasks

### Adding a New API Endpoint
1. Define request/response models in [src/models/mod.rs](../src/models/mod.rs)
2. Add database method in [src/db/mod.rs](../src/db/mod.rs) `DatabaseService` impl
3. Create handler in [src/api/mod.rs](../src/api/mod.rs) with `#[get/post/put/delete]` macro
4. Register handler in `api_scope()` function
5. Test via `curl` or static HTML forms

### Adding a Database Migration
1. Create new file: `migrations/00X_description.sql` (increment number)
2. Write SQL with `CREATE TABLE IF NOT EXISTS` / `ALTER TABLE IF NOT EXISTS` for idempotency
3. Add indexes for frequently queried columns
4. Restart docker-compose to apply (or manually run SQL)

### Handling Optional Fields
Most Item fields are `Option<T>`. In PostgreSQL queries:
- Use `Option<String>` for nullable text/date fields
- Use `Option<f64>` for nullable numeric fields
- Use `Option<i32>` for nullable foreign keys

## Project Status Notes
- **Work in Progress**: Features like categories, tags, and custom fields have database tables but incomplete API implementations
- **Frontend**: Static HTML in `static/index.html` - no JS framework currently
- **Authentication**: Not implemented (open access)
- **Tests**: Minimal coverage (placeholder only)

## Dependencies of Note
- `deadpool-postgres`: Connection pooling with `RecyclingMethod::Fast`
- `tokio-postgres`: Async PostgreSQL client (requires features: `with-chrono-0_4`, `with-serde_json-1`)
- `rust_decimal`: For PostgreSQL DECIMAL support (feature: `tokio-pg`)
- `chrono`: Date/time handling (feature: `serde`)
- `actix-cors`: CORS middleware included but not configured in main.rs yet

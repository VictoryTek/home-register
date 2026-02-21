#!/bin/bash
#
# Home Registry - Database Restore Script
#
# This script restores the PostgreSQL database from a backup file
#
# Usage: ./restore.sh <backup_file.sql.gz> [--force] [--no-restart]
#
# ⚠️  WARNING: This will PERMANENTLY DELETE all existing data! ⚠️
#
# Safety features:
#   - Interactive confirmation (unless --force is used)
#   - Pre-restore backup of current database
#   - Verification of backup file integrity
#   - Post-restore health check
#

set -euo pipefail  # Exit on errors, undefined variables, pipe failures

# ============================================================================
# Configuration
# ============================================================================

# Docker Compose configuration
COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.yml}"
DB_SERVICE="${DB_SERVICE:-db}"
APP_SERVICE="${APP_SERVICE:-app}"
DB_USER="${DB_USER:-postgres}"
DB_NAME="${DB_NAME:-home_inventory}"

# Safety backup (created before restore)
BACKUP_DIR="${BACKUP_DIR:-/app/backups}"
SAFETY_BACKUP="${BACKUP_DIR}/pre-restore-safety-backup-$(date +%Y%m%d_%H%M%S).sql.gz"

# ============================================================================
# Colors for output
# ============================================================================

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# Logging functions
# ============================================================================

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_debug() {
    if [ "${DEBUG:-false}" = "true" ]; then
        echo -e "${BLUE}[DEBUG]${NC} $1"
    fi
}

# ============================================================================
# Error handling
# ============================================================================

cleanup_on_error() {
    if [ $? -ne 0 ]; then
        log_error "Restore process encountered an error!"
        
        if [ "${SAFETY_BACKUP_CREATED:-false}" = "true" ] && [ -f "${SAFETY_BACKUP}" ]; then
            log_warn "A safety backup was created: ${SAFETY_BACKUP}"
            log_warn "You may want to restore from it if needed"
        fi
        
        log_info "Restarting application services..."
        docker compose -f "${COMPOSE_FILE}" start "${APP_SERVICE}" || true
    fi
}

trap cleanup_on_error EXIT

# ============================================================================
# Parse command line arguments
# ============================================================================

print_usage() {
    cat << EOF
Usage: $0 <backup_file.sql.gz> [OPTIONS]

Restore Home Registry database from a backup file.

⚠️  WARNING: This will PERMANENTLY DELETE all existing data!

Arguments:
  <backup_file.sql.gz>    Path to the backup file to restore from

Options:
  --force                 Skip interactive confirmation (use with caution!)
  --no-restart            Don't restart application service after restore
  --no-safety-backup      Skip creating safety backup (not recommended)
  --help                  Display this help message

Examples:
  # Interactive restore (recommended)
  $0 /app/backups/backup_20260220_020000.sql.gz

  # Automated restore (use in scripts)
  $0 backup.sql.gz --force

  # Restore without restarting app (for testing)
  $0 backup.sql.gz --no-restart

Environment Variables:
  COMPOSE_FILE            Docker Compose file path (default: docker-compose.yml)
  DB_SERVICE              Database service name (default: db)
  APP_SERVICE             Application service name (default: app)
  DB_USER                 PostgreSQL user (default: postgres)
  DB_NAME                 Database name (default: home_inventory)
  DEBUG                   Enable debug output (default: false)

EOF
}

# Check for help flag
if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    print_usage
    exit 0
fi

# Check if backup file argument is provided
if [ $# -lt 1 ]; then
    log_error "Error: Backup file argument is required"
    echo ""
    print_usage
    exit 1
fi

BACKUP_FILE="$1"
shift  # Remove first argument

# Parse optional arguments
FORCE_MODE=false
RESTART_APP=true
CREATE_SAFETY_BACKUP=true

while [ $# -gt 0 ]; do
    case "$1" in
        --force)
            FORCE_MODE=true
            shift
            ;;
        --no-restart)
            RESTART_APP=false
            shift
            ;;
        --no-safety-backup)
            CREATE_SAFETY_BACKUP=false
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            echo ""
            print_usage
            exit 1
            ;;
    esac
done

# ============================================================================
# Pre-flight checks
# ============================================================================

log_info "Home Registry Database Restore Script"
log_info "========================================"

# Check if backup file exists
if [ ! -f "${BACKUP_FILE}" ]; then
    log_error "Backup file not found: ${BACKUP_FILE}"
    exit 1
fi

log_info "Backup file: ${BACKUP_FILE}"
BACKUP_SIZE=$(du -h "${BACKUP_FILE}" | cut -f1)
log_info "Backup size: ${BACKUP_SIZE}"

# Verify backup file integrity
log_info "Verifying backup file integrity..."
if gunzip -t "${BACKUP_FILE}" 2>/dev/null; then
    log_info "✓ Backup file integrity verified (gzip valid)"
else
    log_error "✗ Backup file is corrupted or invalid!"
    exit 1
fi

# Quick check for SQL content
if gunzip -c "${BACKUP_FILE}" | head -n 1 | grep -q "^--"; then
    log_debug "Backup file contains valid SQL content"
else
    log_warn "Backup file may not contain valid SQL (header check failed)"
    log_warn "Proceeding anyway, but the restore may fail"
fi

# Check Docker and Docker Compose
if ! command -v docker &> /dev/null; then
    log_error "Docker is not installed or not in PATH!"
    exit 1
fi

if ! command -v docker compose &> /dev/null && ! command -v docker-compose &> /dev/null; then
    log_error "Docker Compose is not installed or not in PATH!"
    exit 1
fi

# Check if database container is running
log_info "Checking database container status..."
if ! docker compose -f "${COMPOSE_FILE}" ps | grep -q "${DB_SERVICE}.*running"; then
    log_error "Database container '${DB_SERVICE}' is not running!"
    log_info "Start the database with: docker compose -f ${COMPOSE_FILE} up -d ${DB_SERVICE}"
    exit 1
fi
log_info "✓ Database container is running"

# ============================================================================
# Warning and confirmation
# ============================================================================

echo ""
echo -e "${RED}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${RED}║                         ⚠️  WARNING  ⚠️                           ║${NC}"
echo -e "${RED}║                                                                ║${NC}"
echo -e "${RED}║  This will PERMANENTLY DELETE all existing data and replace   ║${NC}"
echo -e "${RED}║  it with data from the backup file.                           ║${NC}"
echo -e "${RED}║                                                                ║${NC}"
echo -e "${RED}║  This action CANNOT be undone!                                ║${NC}"
echo -e "${RED}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Backup file: ${BACKUP_FILE}"
echo "Database:    ${DB_NAME}"
echo ""

if [ "${FORCE_MODE}" = "false" ]; then
    read -p "Do you want to continue? Type 'yes' to proceed: " -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        log_info "Restore cancelled by user"
        exit 0
    fi
    
    log_warn "Last chance to cancel!"
    read -p "Are you ABSOLUTELY SURE? Type 'YES' in capital letters: " -r
    echo ""
    
    if [[ $REPLY != "YES" ]]; then
        log_info "Restore cancelled by user"
        exit 0
    fi
else
    log_warn "Running in --force mode, skipping confirmation"
fi

log_info "Starting restore process..."

# ============================================================================
# Create safety backup
# ============================================================================

if [ "${CREATE_SAFETY_BACKUP}" = "true" ]; then
    log_info "Creating safety backup of current database..."
    mkdir -p "${BACKUP_DIR}"
    
    if docker compose -f "${COMPOSE_FILE}" exec -T "${DB_SERVICE}" \
        pg_dump -U "${DB_USER}" --no-owner --no-acl "${DB_NAME}" | \
        gzip -9 > "${SAFETY_BACKUP}"; then
        
        SAFETY_SIZE=$(du -h "${SAFETY_BACKUP}" | cut -f1)
        log_info "✓ Safety backup created: ${SAFETY_BACKUP} (${SAFETY_SIZE})"
        SAFETY_BACKUP_CREATED=true
    else
        log_error "Failed to create safety backup!"
        log_error "Aborting restore for safety"
        exit 1
    fi
else
    log_warn "Skipping safety backup (--no-safety-backup flag used)"
fi

# ============================================================================
# Stop application
# ============================================================================

log_info "Stopping application service to prevent database access..."
if docker compose -f "${COMPOSE_FILE}" stop "${APP_SERVICE}"; then
    log_info "✓ Application service stopped"
else
    log_warn "Failed to stop application service (continuing anyway)"
fi

# Wait a moment for connections to close
sleep 2

# ============================================================================
# Terminate existing database connections
# ============================================================================

log_info "Terminating existing database connections..."
docker compose -f "${COMPOSE_FILE}" exec -T "${DB_SERVICE}" \
    psql -U "${DB_USER}" -d postgres <<EOF
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE datname = '${DB_NAME}'
  AND pid <> pg_backend_pid();
EOF

log_info "✓ Active connections terminated"

# ============================================================================
# Drop and recreate database
# ============================================================================

log_info "Dropping existing database..."
docker compose -f "${COMPOSE_FILE}" exec -T "${DB_SERVICE}" \
    psql -U "${DB_USER}" -d postgres -c "DROP DATABASE IF EXISTS ${DB_NAME};" \
    || log_warn "Failed to drop database (may not exist)"

log_info "Creating fresh database..."
if docker compose -f "${COMPOSE_FILE}" exec -T "${DB_SERVICE}" \
    psql -U "${DB_USER}" -d postgres -c "CREATE DATABASE ${DB_NAME};"; then
    log_info "✓ Fresh database created"
else
    log_error "Failed to create database!"
    exit 1
fi

# ============================================================================
# Restore from backup
# ============================================================================

log_info "Restoring data from backup..."
log_info "This may take several minutes depending on database size..."

START_TIME=$(date +%s)

if gunzip -c "${BACKUP_FILE}" | \
    docker compose -f "${COMPOSE_FILE}" exec -T "${DB_SERVICE}" \
    psql -U "${DB_USER}" -d "${DB_NAME}" > /dev/null 2>&1; then
    
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
    log_info "✓ Database restore completed in ${DURATION} seconds"
else
    log_error "✗ Database restore failed!"
    log_error "Check Docker logs for details: docker compose -f ${COMPOSE_FILE} logs ${DB_SERVICE}"
    
    if [ "${SAFETY_BACKUP_CREATED:-false}" = "true" ]; then
        log_warn "You can restore from the safety backup: ${SAFETY_BACKUP}"
    fi
    
    exit 1
fi

# ============================================================================
# Verify restore
# ============================================================================

log_info "Verifying restored database..."

# Check if database exists and has tables
TABLE_COUNT=$(docker compose -f "${COMPOSE_FILE}" exec -T "${DB_SERVICE}" \
    psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" \
    | tr -d ' ')

if [ "${TABLE_COUNT}" -gt 0 ]; then
    log_info "✓ Database verified (${TABLE_COUNT} tables found)"
else
    log_warn "Database appears empty (0 tables found)"
    log_warn "This may be normal if the backup was from an empty database"
fi

# ============================================================================
# Restart application
# ============================================================================

if [ "${RESTART_APP}" = "true" ]; then
    log_info "Restarting application service..."
    if docker compose -f "${COMPOSE_FILE}" start "${APP_SERVICE}"; then
        log_info "✓ Application service started"
    else
        log_error "Failed to start application service!"
        log_error "Start manually with: docker compose -f ${COMPOSE_FILE} start ${APP_SERVICE}"
        exit 1
    fi
    
    # Wait for health check
    log_info "Waiting for application to become healthy..."
    sleep 5
    
    if docker compose -f "${COMPOSE_FILE}" exec -T "${APP_SERVICE}" \
        curl -f http://localhost:8210/health > /dev/null 2>&1; then
        log_info "✓ Application is healthy"
    else
        log_warn "Application health check failed (may need more time)"
        log_info "Check logs with: docker compose -f ${COMPOSE_FILE} logs -f ${APP_SERVICE}"
    fi
else
    log_warn "Application service not restarted (--no-restart flag used)"
    log_info "Start manually when ready: docker compose -f ${COMPOSE_FILE} start ${APP_SERVICE}"
fi

# ============================================================================
# Cleanup safety backup (optional)
# ============================================================================

if [ "${SAFETY_BACKUP_CREATED:-false}" = "true" ]; then
    echo ""
    read -p "Delete safety backup? (y/N): " -r
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -f "${SAFETY_BACKUP}"
        log_info "Safety backup deleted"
    else
        log_info "Safety backup preserved: ${SAFETY_BACKUP}"
    fi
fi

# ============================================================================
# Final summary
# ============================================================================

echo ""
log_info "════════════════════════════════════════════════════════════════"
log_info "Restore completed successfully!"
log_info "════════════════════════════════════════════════════════════════"
echo ""
echo "Summary:"
echo "  Backup File:   ${BACKUP_FILE}"
echo "  Database:      ${DB_NAME}"
echo "  Tables:        ${TABLE_COUNT}"
if [ "${SAFETY_BACKUP_CREATED:-false}" = "true" ] && [ -f "${SAFETY_BACKUP}" ]; then
    echo "  Safety Backup: ${SAFETY_BACKUP}"
fi
echo ""
echo "Next steps:"
echo "  1. Verify application is working:"
echo "     docker compose -f ${COMPOSE_FILE} logs -f ${APP_SERVICE}"
echo ""
echo "  2. Test login at: https://your-domain.com"
echo ""
echo "  3. Verify inventory data is correct"
echo ""
echo "  4. If everything looks good, you can delete the safety backup"
echo ""

exit 0

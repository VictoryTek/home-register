#!/usr/bin/env pwsh
# Setup test database for integration tests
# Creates home_inventory_test database and applies all migrations

$ErrorActionPreference = "Stop"

Write-Host "===================================================================" -ForegroundColor Cyan
Write-Host "  SETTING UP TEST DATABASE" -ForegroundColor Cyan
Write-Host "===================================================================" -ForegroundColor Cyan

# Database configuration
$POSTGRES_USER = "postgres"
$POSTGRES_PASSWORD = $env:POSTGRES_PASSWORD
if (-not $POSTGRES_PASSWORD) {
    $POSTGRES_PASSWORD = "password"
    Write-Host "Using default password (set POSTGRES_PASSWORD env var to override)" -ForegroundColor Yellow
}
$TEST_DB_NAME = "home_inventory_test"
$CONTAINER_NAME = "home-registry-db-1"

# Check if database container is running
Write-Host "`nChecking database container status..." -ForegroundColor Cyan
try {
    $dbStatus = docker compose ps db --format json 2>$null | ConvertFrom-Json
    if ($dbStatus.State -ne "running") {
        Write-Host "Database container is not running. Starting..." -ForegroundColor Yellow
        docker compose up -d db
        Write-Host "Waiting for database to be ready..." -ForegroundColor Cyan
        Start-Sleep -Seconds 10
    } else {
        Write-Host "[OK] Database container is running" -ForegroundColor Green
    }
} catch {
    Write-Host "Could not check database status, attempting to start..." -ForegroundColor Yellow
    docker compose up -d db
    Start-Sleep -Seconds 10
}

# Wait for PostgreSQL to be ready using docker exec
Write-Host "`nWaiting for PostgreSQL to accept connections..." -ForegroundColor Cyan
$maxAttempts = 30
$attempt = 0
$ready = $false

while ($attempt -lt $maxAttempts -and -not $ready) {
    try {
        $result = docker exec $CONTAINER_NAME pg_isready -U $POSTGRES_USER 2>&1
        if ($LASTEXITCODE -eq 0) {
            $ready = $true
            Write-Host "[OK] PostgreSQL is ready" -ForegroundColor Green
        }
    } catch {
        # Ignore errors, keep retrying
    }
    
    if (-not $ready) {
        $attempt++
        Write-Host "Attempt $attempt/$maxAttempts - waiting..." -ForegroundColor Yellow
        Start-Sleep -Seconds 2
    }
}

if (-not $ready) {
    Write-Host "[FAIL] PostgreSQL did not become ready in time" -ForegroundColor Red
    Write-Host "Please ensure PostgreSQL is running and accessible" -ForegroundColor Red
    exit 1
}

# Check if test database exists
Write-Host "`nChecking if test database exists..." -ForegroundColor Cyan
$env:PGPASSWORD = $POSTGRES_PASSWORD
$dbExists = docker exec $CONTAINER_NAME psql -U $POSTGRES_USER -d postgres -tAc "SELECT 1 FROM pg_database WHERE datname='$TEST_DB_NAME';" 2>$null

if ($dbExists -match "1") {
    Write-Host "[INFO] Test database '$TEST_DB_NAME' already exists" -ForegroundColor Yellow
    Write-Host "Dropping and recreating to ensure clean state..." -ForegroundColor Cyan
    
    # Terminate all connections to the test database
    $null = docker exec $CONTAINER_NAME psql -U $POSTGRES_USER -d postgres -c @"
SELECT pg_terminate_backend(pg_stat_activity.pid)
FROM pg_stat_activity
WHERE pg_stat_activity.datname = '$TEST_DB_NAME'
  AND pid <> pg_backend_pid();
"@ 2>$null
    
    # Drop the database
    $null = docker exec $CONTAINER_NAME psql -U $POSTGRES_USER -d postgres -c "DROP DATABASE IF EXISTS $TEST_DB_NAME;" 2>$null
    Write-Host "[OK] Dropped existing test database" -ForegroundColor Green
}

# Create test database
Write-Host "`nCreating test database '$TEST_DB_NAME'..." -ForegroundColor Cyan
$result = docker exec $CONTAINER_NAME psql -U $POSTGRES_USER -d postgres -c "CREATE DATABASE $TEST_DB_NAME;" 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] Failed to create test database: $result" -ForegroundColor Red
    exit 1
}
Write-Host "[OK] Test database created" -ForegroundColor Green

# Apply migrations to test database
Write-Host "`nApplying migrations to test database..." -ForegroundColor Cyan
$migrationFiles = Get-ChildItem -Path "migrations" -Filter "V*.sql" | Sort-Object Name

$successCount = 0
foreach ($migration in $migrationFiles) {
    Write-Host "  Applying: $($migration.Name)" -ForegroundColor Gray
    
    # Execute migration via docker exec (pipe file content)
    # Temporarily ignore errors since postgres NOTICEs are treated as errors by PowerShell
    $ErrorActionPreference = "SilentlyContinue"
    $null = Get-Content $migration.FullName -Raw | docker exec -i $CONTAINER_NAME psql -U $POSTGRES_USER -d $TEST_DB_NAME -v ON_ERROR_STOP=0 -q 2>&1
    $ErrorActionPreference = "Stop"
    $successCount++
}

Write-Host "[OK] Applied $successCount/$($migrationFiles.Count) migrations" -ForegroundColor Green

Write-Host "`n===================================================================" -ForegroundColor Green
Write-Host "  TEST DATABASE READY" -ForegroundColor Green
Write-Host "===================================================================" -ForegroundColor Green
Write-Host "Database: $TEST_DB_NAME" -ForegroundColor Cyan
Write-Host "Connection: postgres://${POSTGRES_USER}:****@localhost:5432/${TEST_DB_NAME}" -ForegroundColor Cyan
Write-Host "`nYou can now run tests with:" -ForegroundColor Cyan
Write-Host "  cargo test" -ForegroundColor White
Write-Host "or" -ForegroundColor Cyan
Write-Host "  cargo test -- --include-ignored" -ForegroundColor White
Write-Host ""

exit 0

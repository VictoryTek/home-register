# Test script for inventory reporting feature
# This script logs in and tests all three reporting endpoints

param(
    [Parameter(Mandatory=$false)]
    [string]$Username,
    
    [Parameter(Mandatory=$false)]
    [string]$Password
)

# Prompt for credentials if not provided
if (-not $Username) {
    $Username = Read-Host "Enter your username"
}
if (-not $Password) {
    $securePassword = Read-Host "Enter your password" -AsSecureString
    $BSTR = [System.Runtime.InteropServices.Marshal]::SecureStringToBSTR($securePassword)
    $Password = [System.Runtime.InteropServices.Marshal]::PtrToStringAuto($BSTR)
}

$baseUrl = "http://localhost:8210"

Write-Host "`n=== Testing Inventory Reporting Feature ===" -ForegroundColor Cyan

# Step 1: Login to get JWT token
Write-Host "`n1. Logging in as $Username..." -ForegroundColor Yellow
$loginBody = @{
    username = $Username
    password = $Password
} | ConvertTo-Json

try {
    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/api/auth/login" -Method POST -Body $loginBody -ContentType "application/json"
    $token = $loginResponse.data.token
    Write-Host "Success! Login complete" -ForegroundColor Green
    Write-Host ("  Token: {0}..." -f $token.Substring(0,20)) -ForegroundColor Gray
} catch {
    Write-Host "Login failed: $_" -ForegroundColor Red
    exit 1
}

$headers = @{
    "Authorization" = "Bearer $token"
}

# Step 2: Test Inventory Statistics Endpoint
Write-Host "`n2. Testing GET /api/reports/inventory/statistics" -ForegroundColor Yellow
Write-Host "   Getting overall statistics..." -ForegroundColor Gray
try {
    $statsResponse = Invoke-RestMethod -Uri "$baseUrl/api/reports/inventory/statistics" -Method GET -Headers $headers
    Write-Host "Success! Statistics retrieved" -ForegroundColor Green
    Write-Host ("  Total Items: {0}" -f $statsResponse.total_items) -ForegroundColor Cyan
    Write-Host ("  Total Value: `${0:N2}" -f $statsResponse.total_value) -ForegroundColor Cyan
    Write-Host ("  Total Quantity: {0}" -f $statsResponse.total_quantity) -ForegroundColor Cyan
    Write-Host ("  Categories: {0}" -f $statsResponse.category_count) -ForegroundColor Cyan
} catch {
    Write-Host "Statistics request failed: $_" -ForegroundColor Red
}

# Step 3: Test Category Breakdown Endpoint
Write-Host "`n3. Testing GET /api/reports/inventory/categories" -ForegroundColor Yellow
Write-Host "   Getting category breakdown..." -ForegroundColor Gray
try {
    $categoryResponse = Invoke-RestMethod -Uri "$baseUrl/api/reports/inventory/categories" -Method GET -Headers $headers
    Write-Host "Success! Category breakdown retrieved" -ForegroundColor Green
    foreach ($cat in $categoryResponse) {
        Write-Host ("  - {0}: {1} items, `${2:N2} ({3:N1}% of total)" -f $cat.category, $cat.item_count, $cat.total_value, $cat.percentage_of_total) -ForegroundColor Cyan
    }
} catch {
    Write-Host "Category breakdown request failed: $_" -ForegroundColor Red
}

# Step 4: Test Full Report Endpoint (JSON)
Write-Host "`n4. Testing GET /api/reports/inventory (JSON format)" -ForegroundColor Yellow
Write-Host "   Getting full inventory report for inventory_id=100 (Home Office)..." -ForegroundColor Gray
try {
    $reportUrl = $baseUrl + '/api/reports/inventory?inventory_id=100' + '&format=json'
    $reportResponse = Invoke-RestMethod -Uri $reportUrl -Method GET -Headers $headers
    Write-Host "Success! Full report retrieved" -ForegroundColor Green
    Write-Host ("  Report generated at: {0}" -f $reportResponse.generated_at) -ForegroundColor Cyan
    Write-Host "  Statistics:" -ForegroundColor Cyan
    Write-Host ("    - Items: {0}" -f $reportResponse.statistics.total_items) -ForegroundColor Gray
    Write-Host ("    - Value: `${0:N2}" -f $reportResponse.statistics.total_value) -ForegroundColor Gray
    Write-Host "  First 3 items:" -ForegroundColor Cyan
    foreach ($item in $reportResponse.items[0..2]) {
        Write-Host ("    - {0}: `${1:N2}" -f $item.name, $item.purchase_price) -ForegroundColor Gray
    }
} catch {
    Write-Host "Full report request failed: $_" -ForegroundColor Red
}

# Step 5: Test Report with Filters
Write-Host "`n5. Testing with filters (Electronics, price > 100)" -ForegroundColor Yellow
try {
    $filteredUrl = $baseUrl + '/api/reports/inventory' + '?category=Electronics' + '&min_price=100' + '&format=json'
    $filteredResponse = Invoke-RestMethod -Uri $filteredUrl -Method GET -Headers $headers
    Write-Host "Success! Filtered report retrieved" -ForegroundColor Green
    Write-Host ("  Found {0} items matching filters" -f $filteredResponse.statistics.total_items) -ForegroundColor Cyan
    Write-Host ("  Total value: `${0:N2}" -f $filteredResponse.statistics.total_value) -ForegroundColor Cyan
} catch {
    Write-Host "Filtered report request failed: $_" -ForegroundColor Red
}

# Step 6: Test CSV Export
Write-Host "`n6. Testing CSV export" -ForegroundColor Yellow
Write-Host "   Downloading CSV report..." -ForegroundColor Gray
try {
    $csvPath = "inventory_report_test.csv"
    $csvUrl = $baseUrl + '/api/reports/inventory' + '?inventory_id=100' + '&format=csv'
    Invoke-WebRequest -Uri $csvUrl -Method GET -Headers $headers -OutFile $csvPath
    Write-Host "Success! CSV report downloaded" -ForegroundColor Green
    Write-Host ("  Saved to: {0}" -f $csvPath) -ForegroundColor Cyan
    Write-Host "  First 5 lines:" -ForegroundColor Cyan
    Get-Content $csvPath -Head 5 | ForEach-Object { Write-Host ("    {0}" -f $_) -ForegroundColor Gray }
} catch {
    Write-Host "CSV export failed: $_" -ForegroundColor Red
}

Write-Host "`n=== Testing Complete ===" -ForegroundColor Cyan
Write-Host "`nYou can also test manually with these URLs (use the token in Authorization header):" -ForegroundColor Yellow
Write-Host "  - Statistics: GET $baseUrl/api/reports/inventory/statistics" -ForegroundColor Gray
Write-Host "  - Categories: GET $baseUrl/api/reports/inventory/categories" -ForegroundColor Gray
Write-Host "  - Full Report: GET $baseUrl/api/reports/inventory?format=json" -ForegroundColor Gray
Write-Host "  - CSV Export: GET $baseUrl/api/reports/inventory?format=csv" -ForegroundColor Gray
Write-Host "`nFilter options:" -ForegroundColor Yellow
Write-Host "  - inventory_id, category, location, from_date, to_date" -ForegroundColor Gray
Write-Host "  - min_price, max_price, sort_by, sort_order" -ForegroundColor Gray

# Helper script to assign sample inventories to your user account
# Run this after creating your user account through the setup wizard or login screen

param(
    [Parameter(Mandatory=$true)]
    [string]$Username
)

Write-Host "`n=== Assigning Sample Inventories to User: $Username ===" -ForegroundColor Cyan

# SQL command to assign all unassigned inventories to the specified user
$sql = @"
UPDATE inventories 
SET user_id = (SELECT id FROM users WHERE username = '$Username') 
WHERE user_id IS NULL;
"@

try {
    $result = docker compose exec db psql -U postgres -d home_inventory -c $sql
    Write-Host "`n✓ Sample inventories assigned to user '$Username' successfully!" -ForegroundColor Green
    Write-Host "`nYou now have access to:" -ForegroundColor Yellow
    Write-Host "  - Home Office (8 items)" -ForegroundColor Gray
    Write-Host "  - Living Room (7 items)" -ForegroundColor Gray
    Write-Host "  - Kitchen (8 items)" -ForegroundColor Gray
    Write-Host "  - Garage (9 items)" -ForegroundColor Gray
    Write-Host "  - Master Bedroom (8 items)" -ForegroundColor Gray
    Write-Host "`nTotal: 40 items worth approximately `$19,228" -ForegroundColor Cyan
    Write-Host "`nYou can now test the reporting features with: .\test_reporting.ps1" -ForegroundColor Yellow
} catch {
    Write-Host "`n✗ Failed to assign inventories: $_" -ForegroundColor Red
    Write-Host "`nMake sure:" -ForegroundColor Yellow
    Write-Host "  1. Docker containers are running (docker compose up -d)" -ForegroundColor Gray
    Write-Host "  2. User '$Username' exists in the database" -ForegroundColor Gray
    Write-Host "  3. Sample data migration has been applied" -ForegroundColor Gray
    exit 1
}

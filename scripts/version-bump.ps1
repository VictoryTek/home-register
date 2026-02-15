#!/usr/bin/env pwsh
# scripts/version-bump.ps1 - Bump version across all files
# Works on Windows, Linux, and macOS with PowerShell

param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$NewVersion
)

$ErrorActionPreference = "Stop"

# Validate version format
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "❌ Error: Invalid version format. Expected: X.Y.Z" -ForegroundColor Red
    Write-Host "Example: ./scripts/version-bump.ps1 0.2.0" -ForegroundColor Yellow
    exit 1
}

Write-Host "`n🔄 Bumping version to $NewVersion..." -ForegroundColor Cyan

# Files to update
$cargoToml = "Cargo.toml"
$rootPackageJson = "package.json"
$frontendPackageJson = "frontend/package.json"
$dockerfile = "Dockerfile"

# Check if files exist
$filesToUpdate = @($cargoToml, $rootPackageJson, $frontendPackageJson, $dockerfile)
foreach ($file in $filesToUpdate) {
    if (-not (Test-Path $file)) {
        Write-Host "❌ Error: File not found: $file" -ForegroundColor Red
        exit 1
    }
}

Write-Host "`n🔍 Checking current versions..." -ForegroundColor Yellow

# Extract current versions
$cargoContent = Get-Content $cargoToml -Raw
$rootPackageJsonContent = Get-Content $rootPackageJson -Raw
$frontendPackageJsonContent = Get-Content $frontendPackageJson -Raw

if ($cargoContent -match 'version = "([\d\.]+)"') {
    $currentCargoVersion = $matches[1]
    Write-Host "  Current Cargo.toml version: $currentCargoVersion" -ForegroundColor Gray
}
if ($rootPackageJsonContent -match '"version": "([\d\.]+)"') {
    $currentRootVersion = $matches[1]
    Write-Host "  Current package.json version: $currentRootVersion" -ForegroundColor Gray
}
if ($frontendPackageJsonContent -match '"version": "([\d\.]+)"') {
    $currentFrontendVersion = $matches[1]
    Write-Host "  Current frontend/package.json version: $currentFrontendVersion" -ForegroundColor Gray
}

# Warn if versions don't match
if ($currentCargoVersion -ne $currentRootVersion -or $currentCargoVersion -ne $currentFrontendVersion) {
    Write-Host "`n⚠️  Warning: Version mismatch detected across files!" -ForegroundColor Yellow
    Write-Host "   Cargo.toml: $currentCargoVersion" -ForegroundColor Yellow
    Write-Host "   package.json: $currentRootVersion" -ForegroundColor Yellow
    Write-Host "   frontend/package.json: $currentFrontendVersion" -ForegroundColor Yellow
    Write-Host "   Continuing to update all files to $NewVersion...`n" -ForegroundColor Yellow
}

Write-Host "`n📝 Updating files..." -ForegroundColor Yellow

# Update Cargo.toml
Write-Host "  - $cargoToml"
$cargoContent = Get-Content $cargoToml -Raw
$cargoContent = $cargoContent -replace 'version = "[\d\.]+\"', "version = `"$NewVersion`""
Set-Content -Path $cargoToml -Value $cargoContent -NoNewline

# Update package.json (root)
Write-Host "  - $rootPackageJson"
$rootPackageJsonContent = Get-Content $rootPackageJson -Raw
$rootPackageJsonContent = $rootPackageJsonContent -replace '"version": "[\d\.]+"', "`"version`": `"$NewVersion`""
Set-Content -Path $rootPackageJson -Value $rootPackageJsonContent -NoNewline

# Update frontend/package.json
Write-Host "  - $frontendPackageJson"
$packageJsonContent = Get-Content $frontendPackageJson -Raw
$packageJsonContent = $packageJsonContent -replace '"version": "[\d\.]+"', "`"version`": `"$NewVersion`""
Set-Content -Path $frontendPackageJson -Value $packageJsonContent -NoNewline

# Update Dockerfile
Write-Host "  - $dockerfile"
$dockerfileContent = Get-Content $dockerfile -Raw
$dockerfileContent = $dockerfileContent -replace 'org\.opencontainers\.image\.version="[\d\.]+"', "org.opencontainers.image.version=`"$NewVersion`""
Set-Content -Path $dockerfile -Value $dockerfileContent -NoNewline

Write-Host "`n✅ Version bumped to $NewVersion in all files" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  1. Review changes:      git diff" -ForegroundColor White
Write-Host "  2. Commit changes:      git commit -am 'chore: bump version to $NewVersion'" -ForegroundColor White
Write-Host "  3. Tag for beta:        git tag v$NewVersion-beta.1" -ForegroundColor White
Write-Host "  4. Tag for stable:      git tag v$NewVersion" -ForegroundColor White
Write-Host "  5. Push with tags:      git push && git push --tags" -ForegroundColor White
Write-Host "  6. Trigger release:     Go to Actions → Release to GHCR → Run workflow`n" -ForegroundColor White

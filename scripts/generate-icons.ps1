# PWA Icon Generator for Home Registry
# Generates all required icon sizes from source favicon.png
# Requires: .NET Framework (built-in on Windows)

param(
    [string]$SourceImage = "frontend\public\favicon.png",
    [string]$OutputDir = "frontend\public\icons",
    [switch]$CopyToStatic = $true
)

# Add .NET System.Drawing assembly
Add-Type -AssemblyName System.Drawing

# Define all required icon sizes
$iconSizes = @(
    @{Size=16; Name="icon-16.png"},
    @{Size=32; Name="icon-32.png"},
    @{Size=48; Name="icon-48.png"},
    @{Size=72; Name="icon-72.png"},
    @{Size=96; Name="icon-96.png"},
    @{Size=120; Name="icon-120.png"},
    @{Size=128; Name="icon-128.png"},
    @{Size=144; Name="icon-144.png"},
    @{Size=152; Name="icon-152.png"},
    @{Size=167; Name="icon-167.png"},
    @{Size=180; Name="icon-180.png"},
    @{Size=192; Name="icon-192.png"},
    @{Size=256; Name="icon-256.png"},
    @{Size=384; Name="icon-384.png"},
    @{Size=512; Name="icon-512.png"}
)

# Function to resize image
function Resize-Image {
    param(
        [string]$InputPath,
        [string]$OutputPath,
        [int]$Size
    )
    
    try {
        # Load source image
        $sourceImage = [System.Drawing.Image]::FromFile($InputPath)
        
        # Create new bitmap with target size
        $destImage = New-Object System.Drawing.Bitmap($Size, $Size)
        
        # Create graphics object for high-quality resize
        $graphics = [System.Drawing.Graphics]::FromImage($destImage)
        $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
        $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
        $graphics.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality
        
        # Draw resized image
        $graphics.DrawImage($sourceImage, 0, 0, $Size, $Size)
        
        # Save to file
        $destImage.Save($OutputPath, [System.Drawing.Imaging.ImageFormat]::Png)
        
        # Clean up
        $graphics.Dispose()
        $destImage.Dispose()
        $sourceImage.Dispose()
        
        Write-Host "[OK] Generated: $OutputPath ($Size x $Size)" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Host "[ERROR] Failed to generate $OutputPath : $_" -ForegroundColor Red
        return $false
    }
}

# Function to create maskable icon with safe zone
function Create-MaskableIcon {
    param(
        [string]$InputPath,
        [string]$OutputPath,
        [int]$Size
    )
    
    try {
        # Load source image
        $sourceImage = [System.Drawing.Image]::FromFile($InputPath)
        
        # Create new bitmap with target size
        $destImage = New-Object System.Drawing.Bitmap($Size, $Size)
        
        # Create graphics object
        $graphics = [System.Drawing.Graphics]::FromImage($destImage)
        $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
        $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
        $graphics.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality
        
        # Fill background with theme color (#1a1a2e - dark blue)
        $backgroundColor = [System.Drawing.Color]::FromArgb(26, 26, 46)
        $graphics.Clear($backgroundColor)
        
        # Calculate safe zone (80% of size, centered)
        $safeZoneSize = [int]($Size * 0.8)
        $offset = [int]($Size * 0.1)
        
        # Draw source image in safe zone
        $graphics.DrawImage($sourceImage, $offset, $offset, $safeZoneSize, $safeZoneSize)
        
        # Save to file
        $destImage.Save($OutputPath, [System.Drawing.Imaging.ImageFormat]::Png)
        
        # Clean up
        $graphics.Dispose()
        $destImage.Dispose()
        $sourceImage.Dispose()
        
        Write-Host "[OK] Generated maskable: $OutputPath ($Size x $Size with safe zone)" -ForegroundColor Cyan
        return $true
    }
    catch {
        Write-Host "[ERROR] Failed to generate maskable $OutputPath : $_" -ForegroundColor Red
        return $false
    }
}

# Main execution
Write-Host "`n=== PWA Icon Generator ===" -ForegroundColor Yellow
Write-Host "Source: $SourceImage" -ForegroundColor Gray
Write-Host "Output: $OutputDir`n" -ForegroundColor Gray

# Verify source image exists
$sourceImagePath = Join-Path $PSScriptRoot "..\$SourceImage"
if (-not (Test-Path $sourceImagePath)) {
    Write-Host "Error: Source image not found at: $sourceImagePath" -ForegroundColor Red
    exit 1
}

# Get source image dimensions
$sourceImg = [System.Drawing.Image]::FromFile($sourceImagePath)
$sourceWidth = $sourceImg.Width
$sourceHeight = $sourceImg.Height
$sourceImg.Dispose()

Write-Host "Source dimensions: $sourceWidth x $sourceHeight" -ForegroundColor Gray

if ($sourceWidth -lt 512 -or $sourceHeight -lt 512) {
    Write-Host "WARNING: Source image is smaller than 512x512. Icons may be pixelated." -ForegroundColor Yellow
}

# Create output directory if it doesn't exist
$outputPath = Join-Path $PSScriptRoot "..\$OutputDir"
if (-not (Test-Path $outputPath)) {
    New-Item -ItemType Directory -Path $outputPath -Force | Out-Null
}

# Generate standard icons
Write-Host "`nGenerating standard icons..." -ForegroundColor Yellow
$successCount = 0
foreach ($icon in $iconSizes) {
    $outputFile = Join-Path $outputPath $icon.Name
    if (Resize-Image -InputPath $sourceImagePath -OutputPath $outputFile -Size $icon.Size) {
        $successCount++
    }
}

# Generate maskable icons (192 and 512 only)
Write-Host "`nGenerating maskable icons..." -ForegroundColor Yellow
$maskable192 = Join-Path $outputPath "icon-192-maskable.png"
$maskable512 = Join-Path $outputPath "icon-512-maskable.png"

if (Create-MaskableIcon -InputPath $sourceImagePath -OutputPath $maskable192 -Size 192) {
    $successCount++
}
if (Create-MaskableIcon -InputPath $sourceImagePath -OutputPath $maskable512 -Size 512) {
    $successCount++
}

# Copy to static directory if requested
if ($CopyToStatic) {
    Write-Host "`nCopying icons to static/icons/..." -ForegroundColor Yellow
    $staticIconsPath = Join-Path $PSScriptRoot "..\static\icons"
    if (-not (Test-Path $staticIconsPath)) {
        New-Item -ItemType Directory -Path $staticIconsPath -Force | Out-Null
    }
    Copy-Item -Path "$outputPath\*" -Destination $staticIconsPath -Force
    Write-Host "[OK] Icons copied to static/icons/" -ForegroundColor Green
}

# Summary
Write-Host "`n=== Generation Complete ===" -ForegroundColor Yellow
Write-Host "Successfully generated: $successCount / 17 icons" -ForegroundColor Green
Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "1. Update manifest.webmanifest files with new icon paths" -ForegroundColor Gray
Write-Host "2. Update HTML files with apple-touch-icon tags" -ForegroundColor Gray
Write-Host "3. Test PWA installation on iOS, Android, and Desktop" -ForegroundColor Gray
Write-Host "`nFor maskable icon testing, visit: https://maskable.app/" -ForegroundColor Cyan
Write-Host ""

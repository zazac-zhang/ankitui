#!/usr/bin/env pwsh

# Windows MSI Build Script for AnkiTUI

param(
    [string]$OutputDir = "dist",
    [string]$Version = "0.1.0",
    [string]$PackageName = "ankitui"
)

# Error handling
$ErrorActionPreference = "Stop"

Write-Host "Building AnkiTUI for Windows..." -ForegroundColor Green

# Build the Rust project
Write-Host "Building Rust binary..." -ForegroundColor Yellow
cargo build --release --target x86_64-pc-windows-msvc

if ($LASTEXITCODE -ne 0) {
    Write-Error "Rust build failed"
    exit 1
}

# Create output directory
New-Item -ItemType Directory -Force -Path $OutputDir

# Try to build MSI using WiX Toolset
Write-Host "Building MSI installer..." -ForegroundColor Yellow

# Check if WiX Toolset is available
$hasWiX = $false
try {
    $wixPath = Get-Command candle.exe -ErrorAction Stop | Select-Object -ExpandProperty Source
    Write-Host "Found WiX Toolset at: $wixPath" -ForegroundColor Green
    $hasWiX = $true
} catch {
    Write-Warning "WiX Toolset not found. Skipping MSI creation."
    Write-Host "To build MSI installer, install WiX Toolset from: https://wixtoolset.org/releases/" -ForegroundColor Cyan
}

if ($hasWiX -and (Test-Path "packaging\windows\Product.wxs")) {
    # Compile WiX source
    Set-Location packaging\windows

    # Update version in WiX file
    (Get-Content Product.wxs) -replace 'Version="[^"]*"', "Version=`"$Version`"" | Set-Content Product.wxs

    # Compile and link
    & candle.exe Product.wxs -arch x64
    if ($LASTEXITCODE -ne 0) {
        Write-Error "WiX compilation failed"
        exit 1
    }

    & light.exe Product.wixobj -out "..\..\$OutputDir\$PackageName-$Version-x86_64.msi"
    if ($LASTEXITCODE -ne 0) {
        Write-Error "WiX linking failed"
        exit 1
    }

    Set-Location ..\..\..
    Write-Host "MSI installer created: $OutputDir\$PackageName-$Version-x86_64.msi" -ForegroundColor Green
} else {
    # Create a simple zip package if WiX is not available
    Write-Host "Creating zip package..." -ForegroundColor Yellow

    $zipName = "$PackageName-$Version-windows-x86_64.zip"
    $zipPath = Join-Path $OutputDir $zipName
    $tempDir = "temp-package"

    New-Item -ItemType Directory -Force -Path $tempDir
    Copy-Item "target\x86_64-pc-windows-msvc\release\ankitui.exe" -Destination $tempDir

    # Copy documentation if available
    if (Test-Path "README.md") { Copy-Item "README.md" -Destination $tempDir }
    if (Test-Path "LICENSE") { Copy-Item "LICENSE" -Destination $tempDir }

    # Create zip
    Compress-Archive -Path "$tempDir\*" -DestinationPath $zipPath -Force
    Remove-Item -Recurse -Force $tempDir

    Write-Host "Zip package created: $zipPath" -ForegroundColor Green
}

Write-Host "Windows package build completed!" -ForegroundColor Green
Write-Host "Output directory: $OutputDir" -ForegroundColor Cyan
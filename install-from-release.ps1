# cutl Windows PowerShell Installer
# Downloads and installs the latest cutl CLI release for Windows

$ErrorActionPreference = "Stop"

Write-Host "Installing cutl CLI for Windows..." -ForegroundColor Cyan

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$platform = "windows-$arch"

# Installation directory
$installDir = "$env:LOCALAPPDATA\cutl\bin"
Write-Host "Install directory: $installDir"

# Create directory if it doesn't exist
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

# Fetch latest release info
Write-Host "Fetching latest release..."
$releaseUrl = "https://api.github.com/repos/ragilhadi/cutl/releases/latest"
$release = Invoke-RestMethod -Uri $releaseUrl
$version = $release.tag_name
Write-Host "Latest version: $version"

# Find the Windows asset
$assetName = "cutl-$platform.zip"
$asset = $release.assets | Where-Object { $_.name -eq $assetName }

if (-not $asset) {
    Write-Host "Error: Could not find $assetName in release assets" -ForegroundColor Red
    exit 1
}

$downloadUrl = $asset.browser_download_url
Write-Host "Downloading from: $downloadUrl"

# Download the archive
$zipPath = "$env:TEMP\cutl-$platform.zip"
Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath

# Extract
Write-Host "Extracting..."
Expand-Archive -Path $zipPath -DestinationPath $env:TEMP -Force

# Move binary to install directory
Move-Item -Path "$env:TEMP\cutl.exe" -Destination "$installDir\cutl.exe" -Force

# Clean up
Remove-Item -Path $zipPath -Force

Write-Host "`n✓ Installed to $installDir\cutl.exe" -ForegroundColor Green

# Check if directory is in PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    Write-Host "`n⚠️  WARNING: $installDir is not in your PATH" -ForegroundColor Yellow
    Write-Host "`nTo add it to your PATH, run this command in PowerShell (as Administrator):"
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$installDir', 'User')" -ForegroundColor Cyan
    Write-Host "`nThen restart your terminal."
} else {
    Write-Host "`n✓ $installDir is already in your PATH" -ForegroundColor Green
}

Write-Host "`nInstallation complete! Run: cutl --help"
Write-Host "Version: $version"

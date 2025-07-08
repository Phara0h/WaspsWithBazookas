# WaspsWithBazookas Windows Installer
# This script automatically downloads and installs the appropriate binary for Windows

param(
    [string]$Version = "latest"
)

# Configuration
$Repo = "Phara0h/WaspsWithBazookas"
$InstallDir = "$env:USERPROFILE\.local\bin"
$Binaries = @("hive.exe", "wasp.exe", "test-dummy.exe")

# Function to write colored output
function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Function to detect architecture
function Get-SystemInfo {
    Write-Status "Detecting your system..."
    
    $Arch = $env:PROCESSOR_ARCHITECTURE
    if ($Arch -eq "AMD64") {
        $Arch = "x86_64"
    } else {
        Write-Error "Unsupported architecture: $Arch"
        exit 1
    }
    
    Write-Success "Detected: windows-$Arch"
    return $Arch
}

# Function to get latest version
function Get-LatestVersion {
    if ($Version -eq "latest") {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $Version = $response.tag_name
    }
    Write-Status "Installing version: $Version"
    return $Version
}

# Function to download and install
function Install-WaspsWithBazookas {
    param(
        [string]$Arch,
        [string]$Version
    )
    
    $AssetName = "waspswithbazookas-windows-$Arch"
    $DownloadUrl = "https://github.com/$Repo/releases/download/$Version/${AssetName}.tar.gz"
    $TempDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
    
    Write-Status "Downloading from: $DownloadUrl"
    
    # Create temp directory
    New-Item -ItemType Directory -Path $TempDir -Force | Out-Null
    
    try {
        # Download the release
        $ArchivePath = Join-Path $TempDir "waspswithbazookas.tar.gz"
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath
        
        # Create installation directory
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        
        # Extract (Windows doesn't have tar by default, so we'll use 7-Zip if available)
        Write-Status "Extracting to $InstallDir..."
        
        # Try to use 7-Zip if available
        $7zPath = Get-Command 7z -ErrorAction SilentlyContinue
        if ($7zPath) {
            & 7z x $ArchivePath -o"$TempDir" -y | Out-Null
            & 7z x (Join-Path $TempDir "waspswithbazookas.tar") -o"$TempDir" -y | Out-Null
        } else {
            # Fallback: try to use tar if available (Windows 10 1803+)
            try {
                tar -xzf $ArchivePath -C $TempDir
            } catch {
                Write-Error "Could not extract archive. Please install 7-Zip or ensure tar is available."
                exit 1
            }
        }
        
        # Install each binary
        foreach ($binary in $Binaries) {
            $SourcePath = Join-Path $TempDir $binary
            $DestPath = Join-Path $InstallDir $binary
            
            if (Test-Path $SourcePath) {
                Copy-Item $SourcePath $DestPath -Force
                Write-Success "Installed $binary"
            } else {
                Write-Warning "Binary $binary not found in release"
            }
        }
        
    } finally {
        # Cleanup
        Remove-Item $TempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# Function to add to PATH
function Add-ToPath {
    $CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    if ($CurrentPath -notlike "*$InstallDir*") {
        $NewPath = "$InstallDir;$CurrentPath"
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        Write-Success "Added $InstallDir to PATH"
        Write-Warning "Please restart your terminal or refresh environment variables"
    } else {
        Write-Status "PATH already configured"
    }
}

# Function to verify installation
function Test-Installation {
    Write-Status "Verifying installation..."
    
    foreach ($binary in $Binaries) {
        $BinaryPath = Join-Path $InstallDir $binary
        if (Test-Path $BinaryPath) {
            try {
                $Version = & $BinaryPath --version 2>$null
                if ($Version) {
                    Write-Success "$binary is installed: $Version"
                } else {
                    Write-Success "$binary is installed"
                }
            } catch {
                Write-Success "$binary is installed"
            }
        } else {
            Write-Warning "$binary not found"
        }
    }
}

# Function to show usage
function Show-Usage {
    Write-Host "WaspsWithBazookas Windows Installer"
    Write-Host ""
    Write-Host "Usage: .\install.ps1 [VERSION]"
    Write-Host ""
    Write-Host "Arguments:"
    Write-Host "  VERSION    Version to install (default: latest)"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\install.ps1                    # Install latest version"
    Write-Host "  .\install.ps1 v2.0.0            # Install specific version"
    Write-Host ""
    Write-Host "The installer will:"
    Write-Host "  1. Detect your system architecture"
    Write-Host "  2. Download the appropriate binary"
    Write-Host "  3. Install to %USERPROFILE%\.local\bin"
    Write-Host "  4. Add to your PATH"
    Write-Host "  5. Verify the installation"
}

# Main execution
function Main {
    Write-Host "🐝 WaspsWithBazookas Windows Installer"
    Write-Host "======================================"
    Write-Host ""
    
    # Check if help is requested
    if ($args[0] -eq "-h" -or $args[0] -eq "--help") {
        Show-Usage
        return
    }
    
    # Check if running as administrator (not required but recommended)
    if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
        Write-Warning "Not running as administrator. Some operations might fail."
    }
    
    # Run installation steps
    $Arch = Get-SystemInfo
    $Version = Get-LatestVersion
    Install-WaspsWithBazookas -Arch $Arch -Version $Version
    Add-ToPath
    Test-Installation
    
    Write-Host ""
    Write-Success "Installation complete! 🎉"
    Write-Host ""
    Write-Host "Quick start:"
    Write-Host "  1. Restart your terminal or refresh environment variables"
    Write-Host "  2. Start the hive: hive --port 4269"
    Write-Host "  3. Start a wasp: wasp --hive-url http://localhost:4269 --port 3001"
    Write-Host "  4. Start test dummy: test-dummy --port 8080"
    Write-Host ""
    Write-Host "For more information, visit: https://github.com/$Repo"
}

# Run main function
Main $args 
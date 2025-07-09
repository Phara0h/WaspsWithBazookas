#!/bin/bash

# WaspsWithBazookas Universal Installer
# This script automatically downloads and installs the appropriate binary for your system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="Phara0h/WaspsWithBazookas"
VERSION=${1:-2.0.2}
INSTALL_DIR="${HOME}/.local/bin"
BINARIES=("hive" "wasp" "test-dummy")

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to detect OS and architecture
detect_system() {
    print_status "Detecting your system..."
    
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    
    case $OS in
        linux*)
            OS="linux"
            ;;
        darwin*)
            OS="macos"
            ;;
        msys*|cygwin*|mingw*)
            OS="windows"
            ;;
        *)
            print_error "Unsupported operating system: $OS"
            exit 1
            ;;
    esac
    
    case $ARCH in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            print_error "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac
    
    print_success "Detected: $OS-$ARCH"
}

# Function to get latest version
get_latest_version() {
    if [ "$VERSION" = "latest" ]; then
        VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    fi
    print_status "Installing version: $VERSION"
}

# Function to download and install
download_and_install() {
    local asset_name="waspswithbazookas-${OS}-${ARCH}"
    local download_url="https://github.com/$REPO/releases/download/$VERSION/${asset_name}.tar.gz"
    local temp_dir=$(mktemp -d)
    
    print_status "Downloading from: $download_url"
    
    # Download the release
    if ! curl -L -o "${temp_dir}/waspswithbazookas.tar.gz" "$download_url"; then
        print_error "Failed to download release. Please check the URL and try again."
        exit 1
    fi
    
    # Create installation directory
    mkdir -p "$INSTALL_DIR"
    
    # Extract and install
    print_status "Extracting to $INSTALL_DIR..."
    tar -xzf "${temp_dir}/waspswithbazookas.tar.gz" -C "$temp_dir"
    
    # Install each binary
    for binary in "${BINARIES[@]}"; do
        if [ -f "${temp_dir}/$binary" ]; then
            cp "${temp_dir}/$binary" "$INSTALL_DIR/"
            chmod +x "$INSTALL_DIR/$binary"
            print_success "Installed $binary"
        else
            print_warning "Binary $binary not found in release"
        fi
    done
    
    # Cleanup
    rm -rf "$temp_dir"
}

# Function to add to PATH
add_to_path() {
    local shell_rc=""
    
    case $SHELL in
        */zsh)
            shell_rc="$HOME/.zshrc"
            ;;
        */bash)
            shell_rc="$HOME/.bashrc"
            ;;
        *)
            shell_rc="$HOME/.profile"
            ;;
    esac
    
    if ! grep -q "$INSTALL_DIR" "$shell_rc" 2>/dev/null; then
        echo "" >> "$shell_rc"
        echo "# WaspsWithBazookas" >> "$shell_rc"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_rc"
        print_success "Added $INSTALL_DIR to PATH in $shell_rc"
        print_warning "Please restart your terminal or run: source $shell_rc"
    else
        print_status "PATH already configured"
    fi
}

# Function to verify installation
verify_installation() {
    print_status "Verifying installation..."
    
    for binary in "${BINARIES[@]}"; do
        if command -v "$binary" >/dev/null 2>&1; then
            local version=$("$binary" --version 2>/dev/null || echo "unknown version")
            print_success "$binary is installed: $version"
        else
            print_warning "$binary not found in PATH"
        fi
    done
}

# Function to show usage
show_usage() {
    echo "WaspsWithBazookas Universal Installer"
    echo ""
    echo "Usage: $0 [VERSION]"
    echo ""
    echo "Arguments:"
    echo "  VERSION    Version to install (default: latest)"
    echo ""
    echo "Examples:"
    echo "  $0                    # Install latest version"
    echo "  $0 v2.0.0            # Install specific version"
    echo ""
    echo "The installer will:"
    echo "  1. Detect your OS and architecture"
    echo "  2. Download the appropriate binary"
    echo "  3. Install to ~/.local/bin"
    echo "  4. Add to your PATH"
    echo "  5. Verify the installation"
}

# Main execution
main() {
    echo "🐝 WaspsWithBazookas Universal Installer"
    echo "========================================"
    echo ""
    
    # Check if help is requested
    if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
        show_usage
        exit 0
    fi
    
    # Check dependencies
    if ! command -v curl >/dev/null 2>&1; then
        print_error "curl is required but not installed. Please install curl first."
        exit 1
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        print_error "tar is required but not installed. Please install tar first."
        exit 1
    fi
    
    # Run installation steps
    detect_system
    get_latest_version
    download_and_install
    add_to_path
    verify_installation
    
    echo ""
    print_success "Installation complete! 🎉"
    echo ""
    echo "Quick start:"
    echo "  1. Restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
    echo "  2. Start the hive: hive --port 4269"
    echo "  3. Start a wasp: wasp --hive-url http://localhost:4269 --port 3001"
    echo "  4. Start test dummy: test-dummy --port 8080"
    echo ""
    echo "For more information, visit: https://github.com/$REPO"
}

# Run main function
main "$@" 
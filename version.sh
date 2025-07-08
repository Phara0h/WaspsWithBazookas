#!/bin/bash

# WaspsWithBazookas Version Script
# This script mimics the npm version script behavior with semantic versioning

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get current version from Cargo.toml
get_current_version() {
    local version=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
    echo "$version"
}

# Function to parse version components
parse_version() {
    local version=$1
    local major=$(echo "$version" | cut -d'.' -f1)
    local minor=$(echo "$version" | cut -d'.' -f2)
    local patch=$(echo "$version" | cut -d'.' -f3 | cut -d'-' -f1)
    echo "$major $minor $patch"
}

# Function to bump version
bump_version() {
    local current_version=$1
    local bump_type=$2
    
    read -r major minor patch <<< "$(parse_version "$current_version")"
    
    case $bump_type in
        "major")
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        "minor")
            minor=$((minor + 1))
            patch=0
            ;;
        "patch")
            patch=$((patch + 1))
            ;;
        *)
            print_error "Invalid bump type: $bump_type. Use major, minor, or patch"
            exit 1
            ;;
    esac
    
    echo "${major}.${minor}.${patch}"
}

# Function to update version in files
update_version_in_files() {
    local new_version=$1
    print_status "Updating version to $new_version in files..."
    
    # Update Cargo.toml
    if [ -f "Cargo.toml" ]; then
        sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
        rm -f Cargo.toml.bak
        print_success "Updated Cargo.toml"
    fi
    
    # Update Homebrew formula
    if [ -f "Formula/waspswithbazookas.rb" ]; then
        sed -i.bak "s/^  version \".*\"/  version \"$new_version\"/" Formula/waspswithbazookas.rb
        rm -f Formula/waspswithbazookas.rb.bak
        print_success "Updated Formula/waspswithbazookas.rb"
    fi
    
    # Update Chocolatey package
    if [ -f "chocolatey/waspswithbazookas.nuspec" ]; then
        sed -i.bak "s/<version>.*<\/version>/<version>$new_version<\/version>/" chocolatey/waspswithbazookas.nuspec
        rm -f chocolatey/waspswithbazookas.nuspec.bak
        print_success "Updated chocolatey/waspswithbazookas.nuspec"
    fi
    
    # Update Chocolatey install script
    if [ -f "chocolatey/tools/chocolateyinstall.ps1" ]; then
        sed -i.bak "s/url = 'https:\/\/github.com\/Phara0h\/WaspsWithBazookas\/releases\/download\/v.*\/waspswithbazookas-windows-x86_64.tar.gz'/url = 'https:\/\/github.com\/Phara0h\/WaspsWithBazookas\/releases\/download\/v$new_version\/waspswithbazookas-windows-x86_64.tar.gz'/" chocolatey/tools/chocolateyinstall.ps1
        rm -f chocolatey/tools/chocolateyinstall.ps1.bak
        print_success "Updated chocolatey/tools/chocolateyinstall.ps1"
    fi
    
    # Update install scripts
    if [ -f "install.sh" ]; then
        sed -i.bak "s/VERSION=\${1:-.*}/VERSION=\${1:-$new_version}/" install.sh
        rm -f install.sh.bak
        print_success "Updated install.sh"
    fi
    
    if [ -f "install.ps1" ]; then
        sed -i.bak "s/param(\s*\[string\]\$Version = \".*\")/param(\n    [string]\$Version = \"$new_version\")/" install.ps1
        rm -f install.ps1.bak
        print_success "Updated install.ps1"
    fi
    
    # Update documentation files that reference version
    find docs/ -name "*.md" -type f -exec sed -i.bak "s/version.*2\.0\.0/version $new_version/g" {} \;
    find docs/ -name "*.md" -type f -exec sed -i.bak "s/v2\.0\.0/v$new_version/g" {} \;
    find . -name "*.md.bak" -delete
    print_success "Updated documentation files"
    
    # Update README.md if it exists
    if [ -f "README.md" ]; then
        sed -i.bak "s/version.*2\.0\.0/version $new_version/g" README.md
        sed -i.bak "s/v2\.0\.0/v$new_version/g" README.md
        rm -f README.md.bak
        print_success "Updated README.md"
    fi
}

# Function to create git tag
create_git_tag() {
    local new_version=$1
    local tag_name="v$new_version"
    
    print_status "Creating git tag: $tag_name"
    
    # Check if tag already exists
    if git tag -l | grep -q "^$tag_name$"; then
        print_warning "Tag $tag_name already exists"
        read -p "Do you want to delete and recreate it? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git tag -d "$tag_name"
            git push origin ":refs/tags/$tag_name" 2>/dev/null || true
        else
            print_error "Tag already exists. Aborting."
            exit 1
        fi
    fi
    
    git tag "$tag_name"
    print_success "Created git tag: $tag_name"
}

# Check required dependencies
check_dependencies() {
    print_status "Checking dependencies..."
    
    local missing_deps=()
    
    if ! command_exists git; then
        missing_deps+=("git")
    fi
    
    if ! command_exists sed; then
        missing_deps+=("sed")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing dependencies: ${missing_deps[*]}"
        print_status "Please install missing dependencies"
        exit 1
    fi
    
    print_success "All dependencies found"
}


# Function to generate changelog
generate_changelog() {
    print_status "Generating changelog..."
    
    if command_exists auto-changelog; then
        # Create a temporary package.json for auto-changelog if it doesn't exist
        if [ ! -f "package.json" ]; then
            print_status "Creating temporary package.json for auto-changelog..."
            cat > package.json << EOF
{
  "name": "waspswithbazookas",
  "version": "$(get_current_version)",
  "description": "Distributed load testing tool - like bees with machine guns, but way more power!",
  "repository": {
    "type": "git",
    "url": "https://github.com/Phara0h/WaspsWithBazookas.git"
  },
  "license": "GPL-2.0"
}
EOF
            local temp_package_json=true
        else
            local temp_package_json=false
        fi
        
        if [ -f "changelog-template.hbs" ]; then
            auto-changelog -l false --sort-commits date-desc --package --hide-credit --template changelog-template.hbs -p --commit-limit false
            print_success "Changelog generated: CHANGELOG.md"
        else
            print_warning "changelog-template.hbs not found, using default template"
            auto-changelog -l false --sort-commits date-desc --package --hide-credit -p --commit-limit false
            print_success "Changelog generated: CHANGELOG.md"
        fi
        
        # Clean up temporary package.json if we created it
        if [ "$temp_package_json" = true ]; then
            rm -f package.json
            print_status "Cleaned up temporary package.json"
        fi
    else
        print_warning "auto-changelog not found, skipping changelog generation"
    fi
}

# Function to squash markdown files
squash_markdown() {
    print_status "Squashing markdown files..."
    
    if command_exists mdsquash; then
        local intro_file="docs/INTRO.md"
        local changelog="CHANGELOG.md"
        local output="README.md"
        
        # Create input files list
        local input_files="$changelog"
        
        mdsquash -t "$intro_file" -i "$input_files" -o "$output"
        print_success "Markdown files squashed: $output"
    else
        print_warning "mdsquash not found, skipping markdown squashing"
    fi
}

# Function to add files to git
add_to_git() {
    print_status "Adding files to git..."
    
    # Add all modified files
    git add -A
    
    # Commit with version bump message
    local new_version=$1
    local bump_type=$2
    git commit -m "chore: bump version to $new_version ($bump_type)"
    
    print_success "Added files to git"
}

# Function to push to git
push_to_git() {
    print_status "Pushing to git..."
    
    git push
    git push --tags
    print_success "Pushed to git"
}

# Function to show usage
show_usage() {
    echo "WaspsWithBazookas Version Script"
    echo ""
    echo "Usage: $0 [bump_type] [options]"
    echo ""
    echo "Bump Types:"
    echo "  major    - Bump major version (1.0.0 -> 2.0.0)"
    echo "  minor    - Bump minor version (1.0.0 -> 1.1.0)"
    echo "  patch    - Bump patch version (1.0.0 -> 1.0.1)"
    echo ""
    echo "Options:"
    echo "  --no-tag     - Don't create git tag"
    echo "  --no-push    - Don't push to git"
    echo "  --no-changelog - Skip changelog generation"
    echo "  --no-squash  - Skip markdown squashing"
    echo ""
    echo "Commands:"
    echo "  changelog    - Generate changelog only"
    echo "  squash       - Squash markdown files only"
    echo "  git-add      - Add files to git only"
    echo "  git-push     - Push to git only"
    echo "  help         - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 patch                    # Bump patch version"
    echo "  $0 minor --no-push          # Bump minor version, don't push"
    echo "  $0 major --no-tag --no-push # Bump major version, don't tag or push"
    echo ""
    echo "Current version: $(get_current_version)"
}

# Main execution
main() {
    local bump_type=$1
    local no_tag=false
    local no_push=false
    local no_changelog=false
    local no_squash=false
    
    # Parse options
    shift
    while [[ $# -gt 0 ]]; do
        case $1 in
            --no-tag)
                no_tag=true
                shift
                ;;
            --no-push)
                no_push=true
                shift
                ;;
            --no-changelog)
                no_changelog=true
                shift
                ;;
            --no-squash)
                no_squash=true
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    print_status "Starting WaspsWithBazookas version script..."
    
    # Check dependencies
    check_dependencies
    
    # Get current version
    local current_version=$(get_current_version)
    print_status "Current version: $current_version"
    
    # Calculate new version
    local new_version=$(bump_version "$current_version" "$bump_type")
    print_status "New version: $new_version"
    
    # Update version in files
    update_version_in_files "$new_version"
    
    # Generate changelog
    if [ "$no_changelog" = false ]; then
        generate_changelog
    fi
    
    
    # Squash markdown files
    if [ "$no_squash" = false ]; then
        squash_markdown
    fi
    
    # Add files to git
    add_to_git "$new_version" "$bump_type"
    
    # Create git tag
    if [ "$no_tag" = false ]; then
        create_git_tag "$new_version"
    fi
    
    print_success "Version script completed successfully!"
    print_status "Version bumped from $current_version to $new_version"
    
    # Ask if user wants to push
    if [ "$no_push" = false ]; then
        read -p "Do you want to push to git? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            push_to_git
        else
            print_status "Skipping git push. Run 'git push && git push --tags' manually when ready."
        fi
    else
        print_status "Skipping git push (--no-push specified). Run 'git push && git push --tags' manually when ready."
    fi
}

# Handle command line arguments
case "${1:-}" in
    "major"|"minor"|"patch")
        main "$@"
        ;;
    "changelog")
        check_dependencies
        generate_changelog
        ;;
    "squash")
        check_dependencies
        squash_markdown
        ;;
    "git-add")
        add_to_git
        ;;
    "git-push")
        push_to_git
        ;;
    "help"|"-h"|"--help")
        show_usage
        ;;
    "")
        print_error "No bump type specified"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
    *)
        print_error "Unknown command: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac 
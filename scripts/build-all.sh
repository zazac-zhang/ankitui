#!/bin/bash

# AnkiTUI Universal Build Script
# Builds packages for all supported platforms

set -euo pipefail

# Configuration
PACKAGE_NAME="ankitui"
VERSION="0.1.0"
DIST_DIR="dist"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_step() {
    echo -e "${CYAN}[STEP]${NC} $1"
}

# Detect operating system
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Check build dependencies
check_dependencies() {
    log_step "Checking build dependencies..."

    local missing_deps=()

    # Check for Cargo
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo")
    fi

    # Check platform-specific dependencies
    local os=$(detect_os)
    case $os in
        "linux")
            if ! command -v dpkg-deb &> /dev/null; then
                missing_deps+=("dpkg-deb")
            fi
            if ! command -v fpm &> /dev/null && ! command -v cargo-deb &> /dev/null; then
                log_warning "Neither fpm nor cargo-deb found. DEB package building may fail."
                log_info "Install with: gem install fpm OR cargo install cargo-deb"
            fi
            ;;
        "macos")
            if ! command -v create-dmg &> /dev/null; then
                missing_deps+=("create-dmg")
            fi
            if ! command -v xcodebuild &> /dev/null; then
                missing_deps+=("xcodebuild")
            fi
            ;;
        "windows")
            log_warning "Windows detected. Use WSL for full packaging capabilities."
            ;;
    esac

    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        exit 1
    fi

    log_success "All dependencies found"
}

# Clean previous builds
clean_build() {
    log_step "Cleaning previous builds..."

    # Clean Rust artifacts
    cargo clean

    # Remove dist directory
    rm -rf "$DIST_DIR"
    mkdir -p "$DIST_DIR"

    # Remove package-specific build artifacts
    rm -rf staging-* pkg-* ankitui.app *.dmg *.pkg *.deb 2>/dev/null || true

    log_success "Build environment cleaned"
}

# Build for current platform
build_current_platform() {
    log_step "Building for current platform..."

    # Build in release mode
    cargo build --release

    if [ ! -f "target/release/ankitui" ]; then
        log_error "Build failed. Binary not found."
        exit 1
    fi

    log_success "Platform build completed"
}

# Build cross-platform binaries
build_cross_platform() {
    log_step "Building cross-platform binaries..."

    # Add targets
    local targets=(
        "x86_64-unknown-linux-gnu"
        "x86_64-pc-windows-gnu"
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )

    for target in "${targets[@]}"; do
        log_info "Adding target: $target"
        rustup target add "$target"
    done

    # Build for each target
    for target in "${targets[@]}"; do
        log_info "Building for $target..."
        cargo build --release --target "$target"
    done

    log_success "Cross-platform builds completed"
}

# Create tarball package
create_tarball() {
    log_step "Creating tarball package..."

    local os=$(detect_os)
    local arch="x86_64"
    local archive_name="${PACKAGE_NAME}-${VERSION}-${os}-${arch}"
    local archive_dir="$archive_name"

    # Create package directory
    mkdir -p "$archive_dir"

    # Copy binary
    cp "target/release/ankitui" "$archive_dir/"

    # Copy documentation
    if [ -f "README.md" ]; then
        cp README.md "$archive_dir/"
    fi

    if [ -f "LICENSE" ]; then
        cp LICENSE "$archive_dir/"
    fi

    # Copy installation scripts
    cp scripts/install.sh "$archive_dir/"
    cp scripts/uninstall.sh "$archive_dir/"

    # Create tarball
    tar -czf "$archive_name.tar.gz" "$archive_dir"

    # Cleanup
    rm -rf "$archive_dir"

    # Move to dist
    mv "$archive_name.tar.gz" "$DIST_DIR/"

    log_success "Tarball created: $DIST_DIR/$archive_name.tar.gz"
}

# Build platform-specific packages
build_platform_packages() {
    local os=$(detect_os)

    case $os in
        "linux")
            build_linux_packages
            ;;
        "macos")
            build_macos_packages
            ;;
        "windows")
            build_windows_packages
            ;;
        *)
            log_warning "Unsupported platform for native package building: $os"
            log_info "Only tarball package will be created"
            ;;
    esac
}

# Build Linux packages
build_linux_packages() {
    log_step "Building Linux packages..."

    # Build DEB package
    if [ -f "packaging/linux/build-deb.sh" ]; then
        log_info "Building DEB package..."
        bash packaging/linux/build-deb.sh
    else
        log_warning "DEB build script not found"
    fi

    # Try to build RPM package
    if command -v cargo-generate-rpm &> /dev/null; then
        log_info "Building RPM package..."
        cargo generate-rpm
    fi

    log_success "Linux packages built"
}

# Build macOS packages
build_macos_packages() {
    log_step "Building macOS packages..."

    if [ -f "packaging/macos/build-dmg.sh" ]; then
        log_info "Building macOS packages..."
        bash packaging/macos/build-dmg.sh
    else
        log_warning "macOS build script not found"
    fi

    log_success "macOS packages built"
}

# Build Windows packages
build_windows_packages() {
    log_step "Building Windows packages..."

    if [ -f "packaging/windows/build.ps1" ]; then
        log_info "Building Windows packages..."
        if command -v powershell.exe &> /dev/null; then
            powershell.exe -ExecutionPolicy Bypass -File packaging/windows/build.ps1
        else
            log_warning "PowerShell not available. Skip Windows package building."
        fi
    else
        log_warning "Windows build script not found"
    fi

    log_success "Windows packages processed"
}

# Create source package
create_source_package() {
    log_step "Creating source package..."

    local source_name="${PACKAGE_NAME}-${VERSION}-source"
    local temp_dir="temp-source"

    # Create temporary directory
    mkdir -p "$temp_dir"

    # Copy source files (excluding build artifacts)
    rsync -av --exclude-from='.gitignore' \
          --exclude 'target' \
          --exclude 'dist' \
          --exclude '.git' \
          --exclude 'temp-*' \
          --exclude 'staging-*' \
          --exclude 'pkg-*' \
          --exclude '*.dmg' \
          --exclude '*.pkg' \
          --exclude '*.deb' \
          --exclude '*.rpm' \
          . "$temp_dir/$source_name/"

    # Create tarball
    tar -czf "$source_name.tar.gz" -C "$temp_dir" "$source_name"

    # Cleanup
    rm -rf "$temp_dir"

    # Move to dist
    mv "$source_name.tar.gz" "$DIST_DIR/"

    log_success "Source package created: $DIST_DIR/$source_name.tar.gz"
}

# Generate checksums
generate_checksums() {
    log_step "Generating checksums..."

    cd "$DIST_DIR"

    # Generate SHA256 checksums
    sha256sum * > SHA256SUMS

    # Generate MD5 checksums for legacy compatibility
    md5sum * > MD5SUMS 2>/dev/null || log_warning "md5sum not available"

    cd ..

    log_success "Checksums generated in $DIST_DIR/"
}

# Create build information
create_build_info() {
    log_step "Creating build information..."

    local build_info_file="$DIST_DIR/BUILD_INFO.txt"

    cat > "$build_info_file" << EOF
AnkiTUI Build Information
========================

Package: $PACKAGE_NAME
Version: $VERSION
Build Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Build Platform: $(detect_os)
Build Host: $(hostname)
Git Commit: $(git rev-parse HEAD 2>/dev/null || echo "Unknown")
Git Branch: $(git branch --show-current 2>/dev/null || echo "Unknown")
Rust Version: $(rustc --version)
Cargo Version: $(cargo --version)

Package Contents:
EOF

    # List package files with sizes
    cd "$DIST_DIR"
    for file in *; do
        if [ -f "$file" ]; then
            local size=$(du -h "$file" | cut -f1)
            local sha256=$(sha256sum "$file" | cut -d' ' -f1)
            echo "  $file ($size) - SHA256: $sha256" >> "../$build_info_file"
        fi
    done
    cd ..

    cat >> "$build_info_file" << EOF

Installation Instructions:
========================

Tarball Package:
1. Extract: tar -xzf ${PACKAGE_NAME}-${VERSION}-*.tar.gz
2. Run: cd ${PACKAGE_NAME}-${VERSION}-*
3. Run: sudo ./install.sh

DEB Package (Debian/Ubuntu):
1. Install: sudo dpkg -i ${PACKAGE_NAME}_${VERSION}_*.deb
2. Fix dependencies: sudo apt-get install -f

RPM Package (Fedora/CentOS):
1. Install: sudo rpm -i ${PACKAGE_NAME}-${VERSION}-*.rpm

macOS Package:
1. Double-click the .dmg file
2. Drag AnkiTUI to Applications
3. Or run: sudo installer -pkg ${PACKAGE_NAME}-${VERSION}-macos.pkg -target /

Windows Package:
1. Double-click the .msi file
2. Follow installation wizard

Source Package:
1. Extract: tar -xzf ${PACKAGE_NAME}-${VERSION}-source.tar.gz
2. Install Rust: https://rustup.rs/
3. Build: cargo build --release
4. Install: sudo cp target/release/ankitui /usr/local/bin/

Uninstallation:
===============
Run the uninstall script: ./scripts/uninstall.sh
Or use system package manager to remove

Support:
========
GitHub: https://github.com/your-username/ankitui
Issues: https://github.com/your-username/ankitui/issues
Documentation: https://docs.ankitui.com

Thank you for using AnkiTUI!
EOF

    log_success "Build information created: $build_info_file"
}

# Show build summary
show_build_summary() {
    log_step "Build Summary"

    echo
    echo "Build completed successfully!"
    echo "Packages created in $DIST_DIR/:"
    echo

    cd "$DIST_DIR"
    for file in *; do
        if [ -f "$file" ]; then
            local size=$(du -h "$file" | cut -f1)
            echo "  ✓ $file ($size)"
        fi
    done
    cd ..

    echo
    echo "Total packages: $(ls -1 "$DIST_DIR" | wc -l)"
    echo "Total size: $(du -sh "$DIST_DIR" | cut -f1)"
    echo
    log_info "Check BUILD_INFO.txt for detailed installation instructions"
}

# Main build process
main() {
    echo "AnkiTUI Universal Build Script"
    echo "=============================="
    echo "Package: $PACKAGE_NAME"
    echo "Version: $VERSION"
    echo "Platform: $(detect_os)"
    echo

    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ]; then
        log_error "Please run this script from the AnkiTUI project root directory"
        exit 1
    fi

    # Parse command line arguments
    local clean_only=false
    local cross_platform=false
    local source_only=false

    while [[ $# -gt 0 ]]; do
        case $1 in
            --clean)
                clean_only=true
                shift
                ;;
            --cross-platform)
                cross_platform=true
                shift
                ;;
            --source-only)
                source_only=true
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo
                echo "Options:"
                echo "  --clean              Clean build artifacts only"
                echo "  --cross-platform     Build for all supported platforms"
                echo "  --source-only        Create source package only"
                echo "  --help               Show this help message"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                echo "Use --help for available options"
                exit 1
                ;;
        esac
    done

    check_dependencies

    if [ "$clean_only" = true ]; then
        clean_build
        log_success "Clean completed"
        exit 0
    fi

    clean_build

    if [ "$source_only" = true ]; then
        create_source_package
        generate_checksums
        create_build_info
        show_build_summary
        exit 0
    fi

    if [ "$cross_platform" = true ]; then
        build_cross_platform
    else
        build_current_platform
    fi

    create_tarball
    create_source_package
    build_platform_packages
    generate_checksums
    create_build_info
    show_build_summary
}

# Handle script interruption
trap 'log_error "Build interrupted"; exit 1' INT TERM

# Run main function
main "$@"
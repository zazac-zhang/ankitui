#!/bin/bash

# Linux DEB Package Build Script for AnkiTUI

set -euo pipefail

# Configuration
PACKAGE_NAME="ankitui"
VERSION="0.1.0"
ARCHITECTURE="amd64"
MAINTAINER="AnkiTUI Team <team@ankitui.com>"
DESCRIPTION="A terminal-based spaced repetition learning system compatible with Anki's SM-2 algorithm"
HOMEPAGE="https://github.com/your-username/ankitui"
LICENSE="MIT OR Apache-2.0"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

# Check dependencies
check_dependencies() {
    log_info "Checking build dependencies..."

    local deps=("cargo" "dpkg-deb" "fpm" "fakeroot")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            log_error "Missing dependency: $dep"

            case $dep in
                "fpm")
                    echo "Install with: gem install fpm"
                    ;;
                "dpkg-deb")
                    echo "Install with: sudo apt-get install dpkg-dev"
                    ;;
                "fakeroot")
                    echo "Install with: sudo apt-get install fakeroot"
                    ;;
                *)
                    echo "Please install $dep"
                    ;;
            esac
            exit 1
        fi
    done

    log_success "All dependencies found"
}

# Build the Rust project
build_rust_binary() {
    log_info "Building Rust binary..."

    # Build for Linux target
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        cargo build --release
    else
        # Cross-compilation for non-Linux systems
        rustup target add x86_64-unknown-linux-gnu
        cargo build --release --target x86_64-unknown-linux-gnu
    fi

    log_success "Rust binary built successfully"
}

# Create DEB package using cargo-deb (preferred method)
build_with_cargo_deb() {
    if command -v cargo-deb &> /dev/null; then
        log_info "Building DEB package with cargo-deb..."

        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            cargo deb
        else
            # For cross-compilation
            cargo deb --target x86_64-unknown-linux-gnu
        fi

        log_success "DEB package created with cargo-deb"
        return 0
    else
        log_warning "cargo-deb not found, falling back to fpm method"
        return 1
    fi
}

# Create DEB package using fpm
build_with_fpm() {
    log_info "Building DEB package with fpm..."

    # Create package directory structure
    local pkg_dir="pkg-deb"
    rm -rf "$pkg_dir"
    mkdir -p "$pkg_dir/DEBIAN"
    mkdir -p "$pkg_dir/usr/bin"
    mkdir -p "$pkg_dir/usr/share/doc/$PACKAGE_NAME"

    # Copy binary
    local binary_path
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        binary_path="target/release/ankitui"
    else
        binary_path="target/x86_64-unknown-linux-gnu/release/ankitui"
    fi

    cp "$binary_path" "$pkg_dir/usr/bin/"
    chmod 755 "$pkg_dir/usr/bin/ankitui"

    # Copy documentation
    [ -f "README.md" ] && cp README.md "$pkg_dir/usr/share/doc/$PACKAGE_NAME/"
    [ -f "LICENSE" ] && cp LICENSE "$pkg_dir/usr/share/doc/$PACKAGE_NAME/"
    [ -f "LICENSE-MIT" ] && cp LICENSE-MIT "$pkg_dir/usr/share/doc/$PACKAGE_NAME/"
    [ -f "LICENSE-APACHE" ] && cp LICENSE-APACHE "$pkg_dir/usr/share/doc/$PACKAGE_NAME/"

    # Create control file
    cat > "$pkg_dir/DEBIAN/control" << EOF
Package: $PACKAGE_NAME
Version: $VERSION
Architecture: $ARCHITECTURE
Maintainer: $MAINTAINER
Depends: libc6, libgcc-s1, libsqlite3-0
Section: utils
Priority: optional
Homepage: $HOMEPAGE
Description: $DESCRIPTION
 AnkiTUI is a terminal-based spaced repetition learning system
 compatible with Anki's SM-2 algorithm. It provides a complete TUI
 interface for reviewing flashcards, managing decks, and tracking
 learning progress.
 .
 Features:
  • Terminal-based user interface
  • SM-2 spaced repetition algorithm
  • Deck and card management
  • Import/export functionality
  • Learning statistics and progress tracking
EOF

    # Create md5sums
    cd "$pkg_dir"
    find usr -type f -exec md5sum {} + > DEBIAN/md5sums 2>/dev/null || true
    cd ..

    # Calculate installed size
    local installed_size
    installed_size=$(du -s "$pkg_dir" | cut -f1)
    echo "Installed-Size: $installed_size" >> "$pkg_dir/DEBIAN/control"

    # Build the package
    log_info "Creating DEB package..."
    fakeroot dpkg-deb --build "$pkg_dir" "${PACKAGE_NAME}_${VERSION}_${ARCHITECTURE}.deb"

    # Cleanup
    rm -rf "$pkg_dir"

    log_success "DEB package created: ${PACKAGE_NAME}_${VERSION}_${ARCHITECTURE}.deb"
}

# Main execution
main() {
    log_info "Starting DEB package build for $PACKAGE_NAME v$VERSION"

    check_dependencies
    build_rust_binary

    # Try cargo-deb first, fall back to fpm
    if ! build_with_cargo_deb; then
        build_with_fpm
    fi

    # Create dist directory
    mkdir -p dist
    mv *.deb dist/ 2>/dev/null || true

    log_success "DEB package build completed successfully!"
    log_info "Package location: dist/"
}

# Run main function
main "$@"
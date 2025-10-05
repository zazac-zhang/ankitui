#!/bin/bash

# macOS DMG/PKG Build Script for AnkiTUI

set -euo pipefail

# Configuration
PACKAGE_NAME="ankitui"
VERSION="0.1.0"
BUNDLE_ID="com.ankitui.app"
BUNDLE_NAME="AnkiTUI"
AUTHOR="AnkiTUI Team"
COPYRIGHT="Copyright © 2024 AnkiTUI Team. All rights reserved."

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
    log_info "Checking macOS build dependencies..."

    local deps=("cargo" "create-dmg" "xcodebuild" "hdiutil")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            log_error "Missing dependency: $dep"

            case $dep in
                "create-dmg")
                    echo "Install with: brew install create-dmg"
                    ;;
                "xcodebuild")
                    echo "Install Xcode Command Line Tools: xcode-select --install"
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
    log_info "Building Rust binary for macOS..."

    # Build for macOS
    cargo build --release

    log_success "Rust binary built successfully"
}

# Create macOS app bundle
create_app_bundle() {
    log_info "Creating macOS app bundle..."

    local app_dir="$BUNDLE_NAME.app"
    local contents_dir="$app_dir/Contents"
    local macos_dir="$contents_dir/MacOS"
    local resources_dir="$contents_dir/Resources"

    # Create directory structure
    rm -rf "$app_dir"
    mkdir -p "$macos_dir"
    mkdir -p "$resources_dir"

    # Copy binary
    cp "target/release/ankitui" "$macos_dir/"
    chmod +x "$macos_dir/ankitui"

    # Create Info.plist
    cat > "$contents_dir/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>$BUNDLE_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$BUNDLE_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_ID</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>ankitui</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
    <key>NSAppTransportSecurity</key>
    <dict>
        <key>NSAllowsArbitraryLoads</key>
        <false/>
    </dict>
    <key>NSHumanReadableCopyright</key>
    <string>$COPYRIGHT</string>
    <key>CFBundleGetInfoString</key>
    <string>$BUNDLE_NAME $VERSION, $COPYRIGHT</string>
</dict>
</plist>
EOF

    # Create PkgInfo
    echo "APPL????" > "$contents_dir/PkgInfo"

    # Create wrapper script for Terminal app
    cat > "$macos_dir/ankitui-terminal" << 'EOF'
#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Open a new Terminal window and run the application
osascript -e "tell application \"Terminal\" to do script \"'$SCRIPT_DIR/ankitui'\""

# Alternative: Run directly in current Terminal (uncomment if preferred)
# "$SCRIPT_DIR/ankitui"
EOF
    chmod +x "$macos_dir/ankitui-terminal"

    # Create icon placeholder (if you have a real .icns file, replace this)
    if [ ! -f "$resources_dir/AppIcon.icns" ]; then
        log_warning "No icon found. You may want to add AppIcon.icns for better appearance."
    fi

    log_success "App bundle created: $app_dir"
}

# Create DMG using create-dmg
create_dmg() {
    log_info "Creating DMG installer..."

    local dmg_name="${PACKAGE_NAME}-${VERSION}-macos"
    local volume_name="$BUNDLE_NAME"

    # Create DMG (skip background and icon if not available)
    local dmg_args=(
        "--volname" "$volume_name"
        "--window-pos" "200" "120"
        "--window-size" "600" "300"
        "--icon-size" "100"
        "--icon" "$BUNDLE_NAME.app" "175" "120"
        "--hide-extension" "$BUNDLE_NAME.app"
        "--app-drop-link" "425" "120"
        "--disk-image-size" "100"
        "--hdiutil-quiet"
        "$dmg_name.dmg"
        "$BUNDLE_NAME.app"
    )

    # Add README and LICENSE if they exist
    [ -f "README.md" ] && dmg_args+=("README.md")
    [ -f "LICENSE" ] && dmg_args+=("LICENSE")

    # Add background if available
    if [ -f "packaging/macos/dmg-background.png" ]; then
        dmg_args+=("--background" "packaging/macos/dmg-background.png")
    else
        log_warning "Background image not found, creating DMG without background"
    fi

    # Add icon if available
    if [ -f "$BUNDLE_NAME.app/Contents/Resources/AppIcon.icns" ]; then
        dmg_args+=("--volicon" "$BUNDLE_NAME.app/Contents/Resources/AppIcon.icns")
    else
        log_warning "Icon file not found, creating DMG without custom icon"
    fi

    create-dmg "${dmg_args[@]}"

    log_success "DMG created: $dmg_name.dmg"
}

# Create PKG using pkgbuild
create_pkg() {
    log_info "Creating PKG installer..."

    local pkg_name="${PACKAGE_NAME}-${VERSION}-macos"
    local component_pkg="component.pkg"
    local distribution_file="Distribution"

    # Create component package
    pkgbuild \
        --root "$BUNDLE_NAME.app" \
        --install-location "/Applications/$BUNDLE_NAME.app" \
        --identifier "$BUNDLE_ID" \
        --version "$VERSION" \
        --ownership preserve \
        --scripts packaging/macos/scripts \
        "$component_pkg"

    # Create distribution file for installer customization
    cat > "$distribution_file" << EOF
<?xml version="1.0" encoding="utf-8"?>
<installer-script minSpecVersion="1.000000" generator="AnkiTUI Build Script">
    <title>$BUNDLE_NAME</title>
    <organization>$AUTHOR</organization>
    <domains enable_anywhere="true" enable_currentUserHome="true" enable_localSystem="true"/>
    <options customize="never" allow-external-scripts="no" hostArchitectures="x86_64,arm64"/>
    <choices-outline>
        <line choice="default">
            <line choice="com.ankitui.app"/>
        </line>
    </choices-outline>
    <choice id="default" visible="false">
        <choice id="com.ankitui.app" visible="true" title="AnkiTUI Application" description="Install the AnkiTUI terminal-based learning application">
            <pkg-ref id="$BUNDLE_ID"/>
        </choice>
    </choice>
    <pkg-ref id="$BUNDLE_ID" version="$VERSION" onConclusion="none">$component_pkg</pkg-ref>
    <background file="packaging/macos/pkg-background.png" alignment="bottomleft" scaling="tofit"/>
    <welcome file="packaging/macos/welcome.txt"/>
    <conclusion file="packaging/macos/conclusion.txt"/>
    <license file="LICENSE"/>
</installer-script>
EOF

    # Build final PKG
    productbuild \
        --distribution "$distribution_file" \
        --resources packaging/macos/resources \
        --package-path . \
        "$pkg_name.pkg"

    # Cleanup
    rm -f "$component_pkg" "$distribution_file"

    log_success "PKG created: $pkg_name.pkg"
}

# Create installation scripts
create_installation_scripts() {
    log_info "Creating installation scripts..."

    local scripts_dir="packaging/macos/scripts"
    mkdir -p "$scripts_dir"

    # Preinstall script
    cat > "$scripts_dir/preinstall" << 'EOF'
#!/bin/bash

# Pre-installation script for AnkiTUI

# Check if previous version exists
if [ -d "/Applications/AnkiTUI.app" ]; then
    echo "Previous version of AnkiTUI found. Backing up user data..."

    # Backup user data before replacement
    if [ -d "$HOME/.config/ankitui" ]; then
        cp -r "$HOME/.config/ankitui" "$HOME/.config/ankitui.backup.$(date +%Y%m%d_%H%M%S)"
    fi
fi

exit 0
EOF

    # Postinstall script
    cat > "$scripts_dir/postinstall" << 'EOF'
#!/bin/bash

# Post-installation script for AnkiTUI

# Set up shell completion
echo "Setting up shell completion..."

# bash completion
if [ -d "/etc/bash_completion.d" ]; then
    cat > /etc/bash_completion.d/ankitui << 'BASH_COMPLETION'
_ankitui_completion() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="review import export stats edit config deck db --help --version --deck --limit"

    if [[ ${cur} == * ]] ; then
        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
        return 0
    fi
}
complete -F _ankitui_completion ankitui
BASH_COMPLETION
fi

# zsh completion
if [ -d "/usr/local/share/zsh-completions" ]; then
    cat > /usr/local/share/zsh-completions/_ankitui << 'ZSH_COMPLETION'
#compdef ankitui
_ankitui() {
    _arguments -C \
        '1: :->command' \
        '*:: :->args' \
        && ret=0

    case $state in
        command)
            _values 'ankitui commands' \
                'review[Start review session]' \
                'import[Import cards]' \
                'export[Export cards]' \
                'stats[Show statistics]' \
                'edit[Edit decks or cards]' \
                'config[Configuration management]' \
                'deck[Deck management]' \
                'db[Database utilities]' \
                '--help[Show help]' \
                '--version[Show version]'
            ;;
        args)
            case $line[1] in
                review)
                    _arguments '--deck[Specify deck]:deck:' '--limit[Set card limit]:limit:'
                    ;;
            esac
            ;;
    esac

    return ret
}
_ankitui
ZSH_COMPLETION
fi

# Create symlink for command-line usage
if [ ! -L "/usr/local/bin/ankitui" ]; then
    ln -s "/Applications/AnkiTUI.app/Contents/MacOS/ankitui" "/usr/local/bin/ankitui" 2>/dev/null || true
fi

echo "AnkiTUI installation completed successfully!"
EOF

    # Make scripts executable
    chmod +x "$scripts_dir/preinstall"
    chmod +x "$scripts_dir/postinstall"

    log_success "Installation scripts created"
}

# Create uninstallation script
create_uninstall_script() {
    log_info "Creating uninstallation script..."

    cat > "packaging/macos/uninstall.sh" << 'EOF'
#!/bin/bash

# AnkiTUI Uninstallation Script for macOS

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running with appropriate permissions
check_permissions() {
    if [[ $EUID -ne 0 ]]; then
        log_warning "This script should be run with sudo for complete removal."
        read -p "Continue without sudo? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Remove application
remove_application() {
    log_info "Removing AnkiTUI application..."

    if [ -d "/Applications/AnkiTUI.app" ]; then
        rm -rf "/Applications/AnkiTUI.app"
        log_success "Application removed from /Applications"
    else
        log_warning "AnkiTUI.app not found in /Applications"
    fi
}

# Remove command-line symlink
remove_cli_symlink() {
    log_info "Removing command-line symlink..."

    if [ -L "/usr/local/bin/ankitui" ]; then
        rm -f "/usr/local/bin/ankitui"
        log_success "Command-line symlink removed"
    fi
}

# Remove shell completions
remove_shell_completions() {
    log_info "Removing shell completions..."

    # Remove bash completion
    if [ -f "/etc/bash_completion.d/ankitui" ]; then
        rm -f "/etc/bash_completion.d/ankitui"
        log_success "Bash completion removed"
    fi

    # Remove zsh completion
    if [ -f "/usr/local/share/zsh-completions/_ankitui" ]; then
        rm -f "/usr/local/share/zsh-completions/_ankitui"
        log_success "Zsh completion removed"
    fi
}

# Cleanup user data
cleanup_user_data() {
    log_warning "User data cleanup option"
    read -p "Remove all user data and configuration? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log_info "Removing user data..."

        # Remove user configuration
        if [ -d "$HOME/.config/ankitui" ]; then
            rm -rf "$HOME/.config/ankitui"
            log_success "User configuration removed"
        fi

        # Remove user data
        if [ -d "$HOME/.local/share/ankitui" ]; then
            rm -rf "$HOME/.local/share/ankitui"
            log_success "User data removed"
        fi

        # Remove cache
        if [ -d "$HOME/.cache/ankitui" ]; then
            rm -rf "$HOME/.cache/ankitui"
            log_success "Cache removed"
        fi

        # Remove logs
        if [ -d "$HOME/Library/Logs/ankitui" ]; then
            rm -rf "$HOME/Library/Logs/ankitui"
            log_success "Logs removed"
        fi

        log_success "All user data has been removed."
    else
        log_info "User data preserved. You can manually remove it later if desired."
        log_info "User data locations:"
        log_info "  Configuration: ~/.config/ankitui"
        log_info "  Data: ~/.local/share/ankitui"
        log_info "  Cache: ~/.cache/ankitui"
        log_info "  Logs: ~/Library/Logs/ankitui"
    fi
}

# Main uninstallation process
main() {
    echo "AnkiTUI Uninstallation Script"
    echo "============================="
    echo

    read -p "This will uninstall AnkiTUI from your system. Continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Uninstallation cancelled."
        exit 0
    fi

    check_permissions
    remove_application
    remove_cli_symlink
    remove_shell_completions
    cleanup_user_data

    echo
    log_success "AnkiTUI has been successfully uninstalled!"
    echo "Thank you for using AnkiTUI!"
}

# Run main function
main "$@"
EOF

    chmod +x "packaging/macos/uninstall.sh"

    log_success "Uninstallation script created"
}

# Main execution
main() {
    log_info "Starting macOS package build for $PACKAGE_NAME v$VERSION"

    # Check if we're on macOS
    if [[ "$OSTYPE" != "darwin"* ]]; then
        log_error "This script must be run on macOS"
        exit 1
    fi

    check_dependencies
    build_rust_binary
    create_app_bundle

    # Create output directory
    mkdir -p dist

    # Create both DMG and PKG
    create_dmg
    create_pkg

    # Move packages to dist directory
    mv *.dmg dist/ 2>/dev/null || true
    mv *.pkg dist/ 2>/dev/null || true

    # Cleanup app bundle
    rm -rf "$BUNDLE_NAME.app"

    log_success "macOS package build completed successfully!"
    log_info "Packages location: dist/"
    log_info "Created files:"
    ls -la dist/
}

# Run main function
main "$@"
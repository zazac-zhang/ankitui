#!/bin/bash

# AnkiTUI Universal Uninstallation Script
# Supports: Linux, macOS, Windows (via WSL)

set -euo pipefail

# Configuration
PACKAGE_NAME="ankitui"

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

# Find installation directories
find_installation() {
    local possible_dirs=(
        "/usr/local/bin"
        "/usr/bin"
        "$HOME/.local/bin"
        "$HOME/bin"
        "$HOME/.cargo/bin"
    )

    local install_dir=""
    for dir in "${possible_dirs[@]}"; do
        if [ -f "$dir/ankitui" ]; then
            install_dir="$dir"
            break
        fi
    done

    if [ -z "$install_dir" ]; then
        log_warning "AnkiTUI binary not found in common installation directories"
        echo "Please specify the installation directory:"
        read -p "Installation directory: " install_dir

        if [ ! -f "$install_dir/ankitui" ]; then
            log_error "ankitui binary not found in $install_dir"
            exit 1
        fi
    fi

    echo "$install_dir"
}

# Remove binary
remove_binary() {
    local install_dir="$1"
    local binary_path="$install_dir/ankitui"

    log_step "Removing AnkiTUI binary..."

    if [ -f "$binary_path" ]; then
        rm -f "$binary_path"
        log_success "Binary removed from $binary_path"
    else
        log_warning "Binary not found at $binary_path"
    fi
}

# Remove shell completions
remove_completions() {
    log_step "Removing shell completions..."

    local os=$(detect_os)
    local completion_dirs=()

    case $os in
        "linux")
            completion_dirs=(
                "/etc/bash_completion.d"
                "/usr/share/bash-completion/completions"
                "$HOME/.local/share/bash-completion/completions"
                "/usr/local/share/bash-completion/completions"
            )
            ;;
        "macos")
            completion_dirs=(
                "/usr/local/etc/bash_completion.d"
                "/etc/bash_completion.d"
                "$HOME/.local/share/bash-completion/completions"
            )
            ;;
    esac

    # Remove bash completion
    local bash_completion_removed=false
    for dir in "${completion_dirs[@]}"; do
        if [ -f "$dir/ankitui" ]; then
            rm -f "$dir/ankitui"
            log_success "Bash completion removed from $dir/ankitui"
            bash_completion_removed=true
        fi
    done

    if ! $bash_completion_removed; then
        log_warning "No bash completion file found"
    fi

    # Remove zsh completion
    local zsh_completion_dirs=(
        "$HOME/.local/share/zsh-completions"
        "$HOME/.zsh/completions"
        "/usr/local/share/zsh-completions"
        "/usr/share/zsh-completions"
    )

    local zsh_completion_removed=false
    for dir in "${zsh_completion_dirs[@]}"; do
        if [ -f "$dir/_ankitui" ]; then
            rm -f "$dir/_ankitui"
            log_success "Zsh completion removed from $dir/_ankitui"
            zsh_completion_removed=true
        fi
    done

    if ! $zsh_completion_removed; then
        log_warning "No zsh completion file found"
    fi
}

# Remove desktop entry (Linux)
remove_desktop_entry() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        log_step "Removing desktop entry..."

        local desktop_files=(
            "$HOME/.local/share/applications/ankitui.desktop"
            "/usr/share/applications/ankitui.desktop"
            "/usr/local/share/applications/ankitui.desktop"
        )

        local desktop_removed=false
        for file in "${desktop_files[@]}"; do
            if [ -f "$file" ]; then
                rm -f "$file"
                log_success "Desktop entry removed from $file"
                desktop_removed=true
            fi
        done

        if ! $desktop_removed; then
            log_warning "No desktop entry file found"
        fi
    fi
}

# Clean PATH from shell configurations
clean_path() {
    log_step "Cleaning PATH from shell configurations..."

    local shell_configs=(
        "$HOME/.bashrc"
        "$HOME/.bash_profile"
        "$HOME/.profile"
        "$HOME/.zshrc"
        "$HOME/.zprofile"
        "$HOME/.zshenv"
    )

    local configs_modified=false

    for config in "${shell_configs[@]}"; do
        if [ -f "$config" ]; then
            # Check if AnkiTUI PATH addition exists
            if grep -q "# AnkiTUI PATH addition" "$config"; then
                # Create backup
                cp "$config" "$config.backup.$(date +%Y%m%d_%H%M%S)"

                # Remove AnkiTUI PATH lines
                sed -i '/# AnkiTUI PATH addition/,/^export PATH/d' "$config"

                log_success "Removed AnkiTUI PATH from $config"
                log_info "Backup created: $config.backup.$(date +%Y%m%d_%H%M%S)"
                configs_modified=true
            fi
        fi
    done

    if ! $configs_modified; then
        log_warning "No PATH modifications found in shell configurations"
    else
        log_info "Please restart your shell or run 'source ~/.bashrc' (or appropriate config file)"
    fi
}

# Cleanup user data
cleanup_user_data() {
    log_step "User data cleanup..."

    echo "AnkiTUI stores data in the following locations:"
    echo "  Configuration: ~/.config/ankitui"
    echo "  Data: ~/.local/share/ankitui"
    echo "  Cache: ~/.cache/ankitui"
    echo "  Logs (macOS): ~/Library/Logs/ankitui"
    echo "  Temp files: /tmp/ankitui_*"
    echo

    read -p "Remove all user data and configuration? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log_info "Removing user data..."

        # Remove configuration
        if [ -d "$HOME/.config/ankitui" ]; then
            rm -rf "$HOME/.config/ankitui"
            log_success "Configuration directory removed"
        fi

        # Remove data
        if [ -d "$HOME/.local/share/ankitui" ]; then
            rm -rf "$HOME/.local/share/ankitui"
            log_success "Data directory removed"
        fi

        # Remove cache
        if [ -d "$HOME/.cache/ankitui" ]; then
            rm -rf "$HOME/.cache/ankitui"
            log_success "Cache directory removed"
        fi

        # Remove macOS logs
        if [ -d "$HOME/Library/Logs/ankitui" ]; then
            rm -rf "$HOME/Library/Logs/ankitui"
            log_success "macOS logs removed"
        fi

        # Remove temp files
        find /tmp -name "ankitui_*" -type d -exec rm -rf {} + 2>/dev/null || true
        find /tmp -name "ankitui_*" -type f -delete 2>/dev/null || true

        log_success "All user data has been removed"
    else
        log_info "User data preserved. You can manually remove it later if desired."
    fi
}

# Remove system-wide files (requires sudo)
remove_system_files() {
    if [[ $EUID -eq 0 ]]; then
        log_step "Removing system-wide files..."

        # Remove system completions
        local system_completions=(
            "/etc/bash_completion.d/ankitui"
            "/usr/share/bash-completion/completions/ankitui"
            "/usr/local/share/bash-completion/completions/ankitui"
        )

        for completion in "${system_completions[@]}"; do
            if [ -f "$completion" ]; then
                rm -f "$completion"
                log_success "System completion removed: $completion"
            fi
        done

        # Remove system desktop entry
        if [ -f "/usr/share/applications/ankitui.desktop" ]; then
            rm -f "/usr/share/applications/ankitui.desktop"
            log_success "System desktop entry removed"
        fi

        # Remove system zsh completions
        local system_zsh_completions=(
            "/usr/share/zsh-completions/_ankitui"
            "/usr/local/share/zsh-completions/_ankitui"
        )

        for completion in "${system_zsh_completions[@]}"; do
            if [ -f "$completion" ]; then
                rm -f "$completion"
                log_success "System zsh completion removed: $completion"
            fi
        done
    else
        log_info "Not running as root. System-wide files will not be removed."
        log_info "Run with sudo to remove system-wide files: sudo $0"
    fi
}

# Verify uninstallation
verify_uninstallation() {
    log_step "Verifying uninstallation..."

    local install_dir=$(find_installation)
    local binary_path="$install_dir/ankitui"

    if [ -f "$binary_path" ]; then
        log_error "Binary still exists at $binary_path"
        log_error "Uninstallation may have failed"
        exit 1
    fi

    # Check if ankitui command is still available in PATH
    if command -v ankitui &> /dev/null; then
        log_warning "ankitui command is still available in PATH"
        log_info "This may be due to multiple installations or cached PATH"
        log_info "Please restart your shell and verify the installation has been removed"
    else
        log_success "AnkiTUI successfully removed from PATH"
    fi
}

# Show post-uninstallation information
show_post_uninstall_info() {
    echo
    log_success "AnkiTUI uninstallation completed!"
    echo
    echo "What was removed:"
    echo "  ✓ AnkiTUI binary"
    echo "  ✓ Shell completions"
    echo "  ✓ Desktop entry (if applicable)"
    echo "  ✓ PATH modifications (with backups)"
    echo
    if [[ "$REPLY" =~ ^[Yy]$ ]]; then
        echo "  ✓ User data and configuration"
    else
        echo "  ✗ User data preserved (at your request)"
    fi
    echo
    echo "Thank you for using AnkiTUI!"
    echo "We hope you had a great learning experience!"
    echo
    log_info "If you change your mind, you can always reinstall from:"
    echo "  https://github.com/your-username/ankitui"
}

# Main uninstallation process
main() {
    echo "AnkiTUI Universal Uninstallation Script"
    echo "======================================="
    echo "Platform: $(detect_os)"
    echo

    # Check if AnkiTUI is installed
    if ! command -v ankitui &> /dev/null; then
        log_warning "AnkiTUI does not appear to be installed or not in PATH"
        read -p "Continue with uninstallation anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Uninstallation cancelled."
            exit 0
        fi
    fi

    echo "This will uninstall AnkiTUI from your system."
    echo
    read -p "Continue with uninstallation? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Uninstallation cancelled."
        exit 0
    fi

    local install_dir=$(find_installation)

    remove_binary "$install_dir"
    remove_completions
    remove_desktop_entry
    clean_path
    cleanup_user_data
    remove_system_files
    verify_uninstallation
    show_post_uninstall_info
}

# Handle script interruption
trap 'log_error "Uninstallation interrupted"; exit 1' INT TERM

# Run main function
main "$@"
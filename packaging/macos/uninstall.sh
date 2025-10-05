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

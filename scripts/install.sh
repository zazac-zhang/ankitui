#!/bin/bash

# AnkiTUI Universal Installation Script
# Supports: Linux, macOS, Windows (via WSL)

set -euo pipefail

# Configuration
PACKAGE_NAME="ankitui"
VERSION="0.1.0"
MIN_RUST_VERSION="1.70.0"

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

# Check system requirements
check_requirements() {
    log_step "Checking system requirements..."

    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        log_error "Rust/Cargo not found. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi

    # Check Rust version
    local rust_version
    rust_version=$(rustc --version | cut -d' ' -f2)
    if ! printf '%s\n%s\n' "$MIN_RUST_VERSION" "$rust_version" | sort -V -C; then
        log_error "Rust version $rust_version is too old. Minimum required: $MIN_RUST_VERSION"
        echo "Please update Rust: rustup update"
        exit 1
    fi

    # Check for required system packages based on OS
    local os=$(detect_os)
    case $os in
        "linux")
            check_linux_dependencies
            ;;
        "macos")
            check_macos_dependencies
            ;;
        "windows")
            check_windows_dependencies
            ;;
    esac

    log_success "System requirements met"
}

# Check Linux dependencies
check_linux_dependencies() {
    local missing_deps=()

    # Check for common dependencies
    local deps=("sqlite3" "pkg-config")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing_deps+=("$dep")
        fi
    done

    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        echo "Install with:"
        echo "  Ubuntu/Debian: sudo apt-get install ${missing_deps[*]}"
        echo "  CentOS/RHEL: sudo yum install ${missing_deps[*]}"
        echo "  Fedora: sudo dnf install ${missing_deps[*]}"
        exit 1
    fi
}

# Check macOS dependencies
check_macos_dependencies() {
    # Check for Xcode Command Line Tools
    if ! xcode-select -p &> /dev/null; then
        log_warning "Xcode Command Line Tools not found"
        read -p "Install Xcode Command Line Tools? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            xcode-select --install
            log_info "Please wait for Xcode Command Line Tools to install, then run this script again."
            exit 0
        fi
    fi

    # Check for Homebrew (optional but recommended)
    if ! command -v brew &> /dev/null; then
        log_warning "Homebrew not found. Some features may not work optimally."
        echo "Install Homebrew: /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    fi
}

# Check Windows dependencies
check_windows_dependencies() {
    log_warning "Windows detected. Using WSL is recommended for best compatibility."

    # Check for Git Bash or similar environment
    if ! command -v cargo &> /dev/null; then
        log_error "This script requires a Unix-like environment on Windows."
        echo "Please use WSL (Windows Subsystem for Linux) or Git Bash."
        exit 1
    fi
}

# Determine installation directory
get_install_dir() {
    local os=$(detect_os)
    case $os in
        "linux")
            if [ -w "/usr/local/bin" ]; then
                echo "/usr/local/bin"
            else
                echo "$HOME/.local/bin"
            fi
            ;;
        "macos")
            echo "/usr/local/bin"
            ;;
        "windows")
            echo "$HOME/.local/bin"
            ;;
        *)
            echo "$HOME/.local/bin"
            ;;
    esac
}

# Build the application
build_application() {
    log_step "Building AnkiTUI..."

    # Clean previous builds
    cargo clean

    # Build in release mode
    cargo build --release

    if [ ! -f "target/release/ankitui" ]; then
        log_error "Build failed. Binary not found."
        exit 1
    fi

    log_success "Application built successfully"
}

# Install the application
install_application() {
    local install_dir=$(get_install_dir)
    local binary_source="target/release/ankitui"

    log_step "Installing AnkiTUI to $install_dir..."

    # Create install directory if it doesn't exist
    mkdir -p "$install_dir"

    # Copy binary
    cp "$binary_source" "$install_dir/ankitui"
    chmod +x "$install_dir/ankitui"

    # Add to PATH if needed
    update_path "$install_dir"

    log_success "Application installed to $install_dir"
}

# Update PATH in shell configuration
update_path() {
    local install_dir="$1"

    # Check if install_dir is in PATH
    if echo "$PATH" | grep -q "$install_dir"; then
        return 0
    fi

    log_step "Adding $install_dir to PATH..."

    # Determine shell configuration file
    local shell_config=""
    if [ -n "${ZSH_VERSION:-}" ]; then
        shell_config="$HOME/.zshrc"
    elif [ -n "${BASH_VERSION:-}" ]; then
        if [ -f "$HOME/.bashrc" ]; then
            shell_config="$HOME/.bashrc"
        elif [ -f "$HOME/.bash_profile" ]; then
            shell_config="$HOME/.bash_profile"
        else
            shell_config="$HOME/.profile"
        fi
    fi

    if [ -n "$shell_config" ]; then
        echo "" >> "$shell_config"
        echo "# AnkiTUI PATH addition" >> "$shell_config"
        echo "export PATH=\"$install_dir:\$PATH\"" >> "$shell_config"
        log_info "Added to PATH in $shell_config"
        log_warning "Run 'source $shell_config' or restart your shell to use AnkiTUI"
    else
        log_warning "Could not determine shell configuration file"
        log_info "Please add the following to your shell configuration:"
        echo "  export PATH=\"$install_dir:\$PATH\""
    fi
}

# Install shell completions
install_completions() {
    log_step "Installing shell completions..."

    local completion_dir
    local os=$(detect_os)

    case $os in
        "linux")
            # Try common completion directories
            for dir in "/etc/bash_completion.d" "/usr/share/bash-completion/completions" "$HOME/.local/share/bash-completion/completions"; do
                if [ -d "$dir" ] && [ -w "$dir" ]; then
                    completion_dir="$dir"
                    break
                fi
            done
            ;;
        "macos")
            if [ -d "/usr/local/etc/bash_completion.d" ]; then
                completion_dir="/usr/local/etc/bash_completion.d"
            fi
            ;;
    esac

    if [ -n "$completion_dir" ]; then
        install_bash_completion "$completion_dir"
    else
        log_warning "Could not find writable bash completion directory"
        log_info "You can manually install completions by copying the completion script to your completion directory"
    fi

    # Install zsh completion
    install_zsh_completion
}

# Install bash completion
install_bash_completion() {
    local completion_dir="$1"

    cat > "$completion_dir/ankitui" << 'EOF'
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
EOF

    log_success "Bash completion installed to $completion_dir/ankitui"
}

# Install zsh completion
install_zsh_completion() {
    local zsh_comp_dir="$HOME/.local/share/zsh-completions"
    mkdir -p "$zsh_comp_dir"

    cat > "$zsh_comp_dir/_ankitui" << 'EOF'
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
EOF

    log_success "Zsh completion installed to $zsh_comp_dir/_ankitui"
    log_info "Add $zsh_comp_dir to your fpath in .zshrc if not already present"
}

# Create desktop entry (Linux)
create_desktop_entry() {
    if [[ "$OSTYPE" == "linux-gnu"* ]] && [ -d "$HOME/.local/share/applications" ]; then
        log_step "Creating desktop entry..."

        cat > "$HOME/.local/share/applications/ankitui.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=AnkiTUI
Comment=A terminal-based spaced repetition learning system
Exec=ankitui
Icon=ankitui
Terminal=true
Categories=Education;Utility;
Keywords=anki;cards;learning;flashcards;spaced-repetition;
EOF

        log_success "Desktop entry created"
    fi
}

# Verify installation
verify_installation() {
    log_step "Verifying installation..."

    local install_dir=$(get_install_dir)
    local binary_path="$install_dir/ankitui"

    if [ ! -f "$binary_path" ]; then
        log_error "Binary not found at $binary_path"
        exit 1
    fi

    if ! "$binary_path" --version &> /dev/null; then
        log_error "Binary test failed"
        exit 1
    fi

    log_success "Installation verified successfully"
}

# Show post-installation information
show_post_install_info() {
    echo
    log_success "AnkiTUI installation completed!"
    echo
    echo "Usage:"
    echo "  ankitui              # Start the TUI application"
    echo "  ankitui --help       # Show help"
    echo "  ankitui review       # Start review session"
    echo "  ankitui import       # Import cards"
    echo "  ankitui stats        # Show statistics"
    echo
    echo "Configuration directory:"
    echo "  ~/.config/ankitui"
    echo
    echo "Data directory:"
    echo "  ~/.local/share/ankitui"
    echo
    echo "For more information, run:"
    echo "  ankitui --help"
    echo
    log_info "Enjoy using AnkiTUI!"
}

# Main installation process
main() {
    echo "AnkiTUI Universal Installation Script"
    echo "====================================="
    echo "Version: $VERSION"
    echo "Platform: $(detect_os)"
    echo

    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ]; then
        log_error "Please run this script from the AnkiTUI project root directory"
        exit 1
    fi

    check_requirements
    build_application
    install_application
    install_completions
    create_desktop_entry
    verify_installation
    show_post_install_info
}

# Handle script interruption
trap 'log_error "Installation interrupted"; exit 1' INT TERM

# Run main function
main "$@"
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AnkiTUI is a terminal-based spaced repetition learning system compatible with Anki's SM-2 algorithm. It's built with a 4-layer architecture: TUI (presentation), Core (business logic), Data (persistence), and Util (utilities/configuration).

## Build and Development Commands

```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run the application
cargo run

# Run with specific arguments
cargo run -- review --deck deck_name

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

## Project Architecture

The codebase follows a strict 4-layer architecture with recent modular refactoring:

### 1. Data Layer (`src/data/`)
- **ContentStore**: TOML-based storage for user-defined content (cards, decks)
- **StateStore**: SQLite-based storage for system-maintained state (SM-2 algorithm data)
- **SyncAdapter**: Coordinates between content and state stores
- **Models**: Core data structures (Card, Deck, CardContent, CardStateData)

### 2. Core Layer (`src/core/`)
- **DeckManager**: Complete deck lifecycle management and card operations
- **Scheduler**: SM-2 spaced repetition algorithm implementation
- **SessionController**: Review session lifecycle and card queue management
- **StatsEngine**: Learning statistics and visualization data generation

### 3. TUI Layer (`src/tui/`) - Recently Refactored
- **App**: Legacy application state (being replaced by ApplicationState)
- **ApplicationState**: New centralized state management with error integration
- **Components**: Legacy UI components (being replaced)
- **UI Components** (`ui_components/`): New modular component system
  - **MainMenuComponent**: Main menu interface
  - **CardReviewComponent**: Card review interface
  - **Dialogs**: Confirmation and message dialogs
- **State Management**:
  - **AppStateManager**: Centralized UI state transitions
  - **StateTransition**: State change tracking with context
  - **SystemMessage**: Application-wide messaging system
- **Error Handling**:
  - **ErrorIntegration**: Unified error processing with auto-recovery
  - **TUIError**: Structured error types with recovery actions
  - **UserMessage**: User-friendly error presentation
- **Events**: Keyboard input handling and event loop
- **Visualization**: ASCII charts and progress indicators
- **Performance**: Performance monitoring and optimization
- **Search**: Advanced card and deck search functionality
- **Settings Panels**: Modular settings configuration interface

### 4. Config Layer (`src/config/`) - Recently Separated
- **ConfigManager**: TOML configuration file management
- **Scheduler Config**: SM-2 algorithm parameters
- **UI Config**: Theme and display settings
- **Daily Config**: Daily study limits and goals
- **Shortcuts Config**: Keyboard shortcut customization
- **Data Config**: Data directory and storage settings

### 5. Util Layer (`src/util/`)
- **CLI**: Command-line interface with comprehensive subcommands
- **Error**: Legacy error handling (being integrated into TUI error system)

## Key Data Models

### Card Structure
- **CardContent** (TOML): `front`, `back`, `tags`, `media`, `custom`
- **CardStateData** (SQLite): `due`, `interval`, `ease_factor`, `reps`, `lapses`, `state`
- **Card**: Combined view of content + state data

### Core Enums
- **Rating**: Again(0), Hard(1), Good(2), Easy(3)
- **CardState**: New, Learning, Review, Relearning
- **AppState**: Legacy TUI application states (MainMenu, DeckSelection, Review, etc.)
- **AppUIState**: New centralized UI state management
- **ErrorSeverity**: Critical, Error, Warning, Info
- **RecoveryAction**: Auto-recovery actions for error handling

## Development Workflow

1. **Code Organization**: Respect the 5-layer architecture - avoid cross-layer dependencies
2. **State Management**: Use the new `ApplicationState` for centralized state management
3. **Error Handling**: Use the integrated error system with `TUIError` and auto-recovery
4. **UI Components**: Use the modular `ui_components/` system for new UI development
5. **Async Operations**: Most core operations are async - use `.await` appropriately
6. **Testing**: Each module has comprehensive unit tests - maintain high test coverage
7. **Configuration**: Use modular config system in `src/config/` for all settings

## Testing Strategy

- Unit tests for each module in their respective files
- Integration tests are in the same files as the modules they test
- Use `tempfile` for creating isolated test environments
- Mock data is used but real integration is tested where possible

## Dependencies and Tech Stack

- **TUI**: ratatui + crossterm for terminal interface
- **Database**: SQLite with sqlx for async operations
- **Serialization**: serde + toml for configuration and content
- **CLI**: clap for command-line argument parsing
- **Async**: tokio runtime
- **Error Handling**: anyhow + thiserror
- **UUID**: uuid for unique identifiers
- **Time**: chrono for date/time handling

## Current Project Status

The project is in active development with architectural refactoring:
- ✅ Data layer with dual storage (TOML + SQLite)
- ✅ Core business logic with SM-2 algorithm
- ✅ Legacy TUI interface with ASCII visualizations
- ✅ Util layer with configuration and CLI
- 🔄 **Active Refactoring**: TUI layer modernization with:
  - New centralized state management (`ApplicationState`)
  - Modular UI components system (`ui_components/`)
  - Integrated error handling with auto-recovery
  - Separated configuration modules
  - Performance monitoring and optimization
  - Advanced search functionality
  - Settings panels interface

## Common Development Tasks

### Adding New Features
1. Determine which layer the feature belongs to
2. Implement the core logic with tests
3. Add UI components using the new modular system
4. Update configuration modules if applicable
5. Add CLI commands if required

### Database Schema Changes
1. Modify `StateStore` migration logic
2. Update `models.rs` if changing data structures
3. Ensure backward compatibility
4. Update relevant tests

### Adding New UI Components (Recommended Approach)
1. Create component in `src/tui/ui_components/`
2. Add component to `src/tui/ui_components/mod.rs`
3. Integrate with `ApplicationState` for state management
4. Use the error integration system for error handling
5. Add to appropriate settings panels if needed

### Legacy UI Components (Deprecated)
1. Create component in `src/tui/components.rs` (only for maintenance)
2. Add rendering logic to `App::render`
3. Add event handling in `App::handle_action`
4. Update `AppState` enum if needed

### Working with the New State Management
1. Use `ApplicationState` instead of `App` for new development
2. Leverage `AppStateManager` for state transitions
3. Use `SystemMessage` for application-wide notifications
4. Implement error handling with `ErrorIntegration` and auto-recovery

## Configuration

Configuration is stored in `~/.config/ankitui/config.toml` and includes modular settings:
- **Scheduler parameters**: SM-2 algorithm settings (`src/config/scheduler.rs`)
- **UI themes**: Display settings and themes (`src/config/ui.rs`)
- **Keyboard shortcuts**: Customizable key bindings (`src/config/shortcuts.rs`)
- **Daily limits**: Study goals and daily limits (`src/config/daily.rs`)
- **Data paths**: Directory and storage settings (`src/config/data.rs`)

## CLI Commands

The application supports comprehensive command-line interface:
- `review` (default): Start review session
- `import`: Import cards from various formats (CSV, JSON, Anki APKG)
- `export`: Export cards to files (CSV, JSON, TOML)
- `stats`: Display learning statistics and progress
- `edit`: Edit decks or cards interactively
- `config`: Configuration management and validation
- `deck`: Deck management operations (create, delete, rename)
- `db`: Database maintenance utilities

## Development Environment

### Running the Application
- **TUI Mode**: `cargo run` (default interactive mode)
- **CLI Mode**: `cargo run -- [command]` (command-line mode)
- **Specific Deck**: `cargo run -- review --deck deck_name`
- **Debug Mode**: `RUST_LOG=debug cargo run`

### Code Quality Tools
- **Formatting**: `cargo fmt` (code formatting)
- **Linting**: `cargo clippy` (error detection)
- **Type Checking**: `cargo check` (fast compilation check)
- **Documentation**: `cargo doc --open` (generate and view docs)

### Testing
- **All Tests**: `cargo test`
- **With Output**: `cargo test -- --nocapture`
- **Specific Test**: `cargo test test_name`
- **Single Module**: `cargo test path::to::module`

## Code Refactoring Requirements

### Refactoring Principles
1. **Clean Architecture** - No backward compatibility, complete restructure
2. **Early Development** - Project is in early stage, allow breaking changes
4. **Clear Module Boundaries** - Establish clean dependencies and interfaces


## Fix build errors

### use short message format
cargo check --message-format short | grep error

### guild
not create new content to replace errror content
deepthink before fixing build errors

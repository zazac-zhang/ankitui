pub mod util;

// ============================================================================
// CORE BUSINESS LOGIC LAYER
// ============================================================================

// Data layer exports - Handles content and state storage
pub use ankitui_core::data::{
    Card,         // Combined card view (content + state)
    CardContent,  // User-defined card content (front, back, tags)
    CardState,    // Card learning states: New, Learning, Review, Relearning
    ContentStore, // TOML-based storage for user-defined content (cards, decks)
    Deck,         // Deck structure with metadata
    MediaRef,     // Media file references
    MediaType,    // Media type enum
    StateStore,   // SQLite-based storage for system state (SM-2 algorithm data)
    SyncAdapter,  // Coordinates between content and state stores
};

// Core business logic exports - Implements learning algorithms and session management
pub use ankitui_core::core::{
    CardStateManager,   // Individual card state progression
    CardTemplateEngine, // Dynamic card content templating
    DeckManager,        // Complete deck lifecycle management
    MediaManager,       // Rich content and media handling
    Rating,             // Card quality ratings: Again(0), Hard(1), Good(2), Easy(3)
    Scheduler,          // SM-2 spaced repetition algorithm
    SessionController,  // Review session management and card queuing
    StatsEngine,        // Learning statistics and visualization
    TagManager,         // Card categorization and filtering
};

// Configuration exports - Modular settings management
pub use ankitui_core::config::{
    Config,        // Main configuration structure
    ConfigManager, // TOML configuration file management
};

// ============================================================================
// TERMINAL USER INTERFACE LAYER
// ============================================================================

// Modern TUI system exports - New component-based architecture
pub use ankitui_tui::{
    // Core application
    App,                // Main application structure

    // Domain types
    AppState,           // Application states and transitions
    CardRating,         // Card rating types
    SessionState,       // Study session state
    UserPreferences,    // User preferences

    // Services
    DeckService,        // Deck management service
    StudyService,       // Study session service
    StatisticsService,  // Statistics service

    // Error handling
    TuiError,           // TUI-specific error types
    TuiResult,          // TUI result type

    // Version information
    VERSION,            // Application version
};

// ============================================================================
// UTILITY LAYER
// ============================================================================

// Utility functions and CLI interface
pub use util::*;

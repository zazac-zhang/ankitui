// ============================================================================
// ANKITUI CORE LIBRARY
// ============================================================================
//! Core business logic and data management for AnkiTUI spaced repetition system
//!
//! This library provides the essential functionality for managing flashcards,
//! implementing the SM-2 spaced repetition algorithm, and handling data persistence.
//! It serves as the foundation of the AnkiTUI application, offering a clean API
//! for building spaced repetition learning interfaces.
//!
//! ## Architecture Overview
//!
//! The AnkiTUI Core library is organized into three main layers:
//!
//! ### Configuration Layer (`config`)
//! Comprehensive configuration management with modular design:
//! - **Main Config**: Unified configuration structure with validation
//! - **Profiles**: Predefined templates for different user types (simple, student, power user)
//! - **Sub-modules**: Daily limits, scheduler parameters, UI settings, shortcuts, data storage
//! - **Provider Pattern**: Trait-based configuration access for dependency injection
//! - **Environment Support**: Optional environment variable overrides for containers/CI
//!
//! ### Core Business Logic (`core`)
//! Learning algorithms and session management:
//! - **SM-2 Algorithm**: Full implementation of SuperMemo 2 spaced repetition
//! - **Session Control**: Study session lifecycle and card queuing
//! - **Deck Management**: Complete deck lifecycle operations and statistics
//! - **Card State**: Individual card progression and state transitions
//! - **Template Engine**: Dynamic content generation with templating support
//! - **Statistics**: Learning analytics and progress tracking
//! - **Advanced Features**: Incremental learning, media management, tag organization
//!
//! ### Data Management (`data`)
//! Dual-store persistence architecture:
//! - **Content Store**: User-defined content in TOML format (readable, editable)
//! - **State Store**: System state in SQLite (performant, transactional)
//! - **Models**: Type-safe data structures with serialization support
//! - **Sync Adapter**: Coordinates between content and state stores
//! - **Separation Principle**: Clear division between user content and system state
//!
//! ## Key Features
//!
//! - **Spaced Repetition**: Scientifically-backed SM-2 algorithm implementation
//! - **Flexible Configuration**: Extensive customization options with profiles
//! - **Data Integrity**: ACID-compliant storage with validation and error handling
//! - **Performance**: Optimized for both single-card and bulk operations
//! - **Extensibility**: Plugin-like architecture for future enhancements
//! - **Testing**: Comprehensive test coverage with mockable components
//!
//! ## Usage Examples
//!
//! ```rust
//! use ankitui_core::*;
//!
//! // Initialize the core system
//! let config_manager = ConfigManager::new()?;
//! let deck_manager = DeckManager::new();
//! let scheduler = Scheduler::new(&config_manager);
//!
//! // Create a new deck
//! let deck = Deck::new("Vocabulary", "English words and definitions");
//! let deck_id = deck_manager.create_deck(deck)?;
//!
//! // Add cards to the deck
//! let card = CardContent::new(
//!     "Hello",
//!     "A common greeting",
//!     vec!["greeting", "basic"]
//! );
//! let card_id = deck_manager.add_card(deck_id, card)?;
//!
//! // Start a study session
//! let session = SessionController::new(&deck_manager, &scheduler);
//! let study_session = session.create_session(deck_id)?;
//!
//! // Study cards
//! while let Some(card) = study_session.next_card()? {
//!     // Present card to user and get their rating
//!     let rating = get_user_rating(); // Again, Hard, Good, or Easy
//!
//!     // Update the card's schedule
//!     scheduler.update_card_schedule(&card, rating)?;
//! }
//! ```
//!
//! ## Thread Safety and Performance
//!
//! All components are designed to be thread-safe and can be safely used across
//! multiple threads. The data layer uses proper synchronization primitives
//! and transactional operations to ensure data consistency even under concurrent
//! access.
//!
//! ## Error Handling
//!
//! The library uses `anyhow::Result` for error handling, providing detailed
//! error context and making it easy to debug configuration, data, or algorithm
//! issues. All critical operations are validated and provide meaningful error
//! messages.

pub mod config;
pub mod core;
pub mod data;

// ============================================================================
// CONFIGURATION LAYER EXTERNAL API
// ============================================================================

// Main configuration structures and management
pub use config::{
    get_config_profile, // Get specific profile by name

    get_config_profiles, // Get all available configuration profiles
    // Configuration utilities
    helpers,            // Configuration helper functions
    minimal_profile,    // Minimal settings for light usage
    power_user_profile, // Advanced configuration for power users
    simple_profile,     // Simple configuration for new users
    student_profile,    // Optimized settings for students
    // Core configuration types
    Config,        // Complete application configuration structure
    ConfigManager, // Configuration file management and validation

    // Configuration profiles and utilities
    ConfigProfile, // Predefined configuration templates
    // Configuration provider trait and helper structs
    ConfigProvider, // Trait for configuration access abstraction
    // Configuration modules for direct access
    DailyConfig,  // Daily study limits and scheduling preferences
    DailyLimits,  // Daily study limits bundle
    DataConfig,   // Data storage and backup settings
    DataSettings, // Data storage settings bundle

    SchedulerConfig, // SM-2 algorithm parameters configuration
    SchedulerParams, // Scheduler parameter bundle
    ShortcutConfig,  // Keyboard shortcuts customization
    ShortcutMap,     // Keyboard shortcuts mapping
    UiConfig,        // UI theme and display settings

    UiSettings, // UI settings bundle
};

// Default configuration constants
pub use config::{
    DEFAULT_BACKUP_COUNT,             // Default number of backups to keep (10)
    DEFAULT_BACKUP_INTERVAL,          // Default backup interval in hours (24)
    DEFAULT_DAY_END_HOUR,             // Day end hour for daily limits (23)
    DEFAULT_DAY_START_HOUR,           // Day start hour for daily limits (0)
    DEFAULT_EASY_BONUS,               // Easy rating bonus (1.3)
    DEFAULT_EASY_INTERVAL,            // Initial interval for "Easy" rating (4 days)
    DEFAULT_GOOD_INTERVAL,            // Initial interval for "Good" rating (1 day)
    DEFAULT_GRADUATING_INTERVAL,      // Interval for graduating cards (1 day)
    DEFAULT_HARD_MULTIPLIER,          // Hard interval multiplier (1.2)
    DEFAULT_INITIAL_FAILURE_INTERVAL, // Interval after first failure (1 day)
    DEFAULT_INTERVAL_MODIFIER,        // General interval modifier (1.0)
    DEFAULT_MAX_EASE_FACTOR,          // Maximum allowed ease factor (5.0)
    DEFAULT_MAX_INTERVAL,             // Maximum review interval (36500 days ~ 100 years)
    DEFAULT_MAX_NEW_CARDS,            // Default daily new card limit (20)
    DEFAULT_MAX_REVIEW_CARDS,         // Default daily review card limit (100)
    DEFAULT_MIN_EASE_FACTOR,          // Minimum allowed ease factor (1.3)
    DEFAULT_STARTING_EASE_FACTOR,     // Default SM-2 starting ease factor (2.5)
    DEFAULT_THEME,                    // Default UI theme ("default")
};

// ============================================================================
// CORE BUSINESS LOGIC LAYER EXTERNAL API
// ============================================================================

// Card and deck management
pub use core::{
    deck_manager::DeckStats, // Deck statistics and metrics
    DeckManager,             // Complete deck lifecycle operations
};

// Spaced repetition algorithm
pub use core::{
    scheduler::{Rating, Scheduler}, // SM-2 spaced repetition algorithm implementation
};

// Session and review management
pub use core::{
    session_controller::{SessionController, SessionStats}, // Review session lifecycle management and statistics
};

// Learning analytics and statistics
pub use core::{
    stats_engine::StatsEngine, // Learning statistics and visualization data
};

// Card state and template management
pub use core::{
    card_template_engine::TemplateError, // Template errors
    CardStateManager,                    // Individual card state progression
    CardTemplateEngine,                  // Dynamic card content templating
};

// Advanced learning features
pub use core::{
    MediaManager, // Rich content and media handling
    TagManager,   // Card categorization and filtering
};

// ============================================================================
// DATA MANAGEMENT LAYER EXTERNAL API
// ============================================================================

// Data storage and synchronization
pub use data::{
    ContentStore, // TOML-based user content storage
    StateStore,   // SQLite-based system state storage
    SyncAdapter,  // Content and state synchronization
};

// Core data models
pub use data::{
    Card,        // Combined card view (content + state)
    CardContent, // Card content structure (front, back, tags)
    CardState,   // Card learning states: New, Learning, Review, Relearning
    Deck,        // Deck structure with metadata
    MediaRef,    // Media file references
    MediaType,   // Media type enum
};

// ============================================================================
// RE-EXPORTED DEPENDENCIES (for convenience)
// ============================================================================

// Commonly used external types
pub use config::{
    dirs,  // System directory utilities
    serde, // Serialization/deserialization framework
};

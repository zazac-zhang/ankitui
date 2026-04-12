//! Core Business Logic Layer
//!
//! This layer contains the essential business logic that powers AnkiTUI's learning system.
//! It implements the SM-2 spaced repetition algorithm, manages learning sessions,
//! and provides the core functionality that bridges data persistence and user interface.
//!
//! ## Architecture Overview
//!
//! The core layer is organized into specialized modules, each handling a specific
//! aspect of the learning system:
//!
//! ### Learning Algorithm (`scheduler`)
//! - **Purpose**: Implements the SM-2 spaced repetition algorithm
//! - **Key Features**: Card scheduling, interval calculation, ease factor management
//! - **Exports**: `Scheduler`, `Rating` (Again, Hard, Good, Easy)
//! - **Usage**: Core algorithm for determining when cards should be reviewed
//!
//! ### Session Management (`session_controller`)
//! - **Purpose**: Manages study sessions and card queuing
//! - **Key Features**: Session lifecycle, card ordering, progress tracking
//! - **Exports**: `SessionController`, `SessionStats`
//! - **Usage**: Controls the flow of study sessions and tracks learning metrics
//!
//! ### Deck Management (`deck_manager`)
//! - **Purpose**: Complete deck lifecycle management
//! - **Key Features**: Deck creation, modification, statistics, organization
//! - **Exports**: `DeckManager`, `DeckStats`
//! - **Usage**: All operations related to deck organization and management
//!
//! ### Card State Management (`card_state_manager`)
//! - **Purpose**: Individual card state progression and lifecycle
//! - **Key Features**: State transitions, progress tracking, scheduling updates
//! - **Exports**: `CardStateManager`
//! - **Usage**: Manages each card's learning state and progression
//!
//! ### Template Engine (`card_template_engine`)
//! - **Purpose**: Dynamic card content generation and templating
//! - **Key Features**: Template parsing, variable substitution, error handling
//! - **Exports**: `CardTemplateEngine`, `TemplateError`
//! - **Usage**: Enables dynamic content generation for cards
//!
//! ### Statistics Engine (`stats_engine`)
//! - **Purpose**: Learning analytics and progress visualization
//! - **Key Features**: Performance metrics, learning trends, detailed statistics
//! - **Exports**: `StatsEngine`
//! - **Usage**: Provides comprehensive learning analytics and insights
//!
//! ### Advanced Features
//!
//! #### Incremental Learning (`incremental_learning`)
//! - **Purpose**: Advanced learning strategies and incremental study methods
//! - **Features**: Gradual exposure, difficulty progression, custom learning paths
//!
//! #### Media Management (`media_manager`)
//! - **Purpose**: Rich content and media file handling
//! - **Features**: Media linking, organization, validation, storage optimization
//!
//! #### Tag Management (`tag_manager`)
//! - **Purpose**: Card categorization and advanced filtering
//! - **Features**: Tag hierarchy, search, filtering, organization tools
//!
//! ## Design Principles
//!
//! 1. **Separation of Concerns**: Each module has a single, well-defined responsibility
//! 2. **Testability**: All components are designed for easy unit testing
//! 3. **Extensibility**: Plugin-like architecture allows for future enhancements
//! 4. **Performance**: Optimized for both single-card and bulk operations
//! 5. **Reliability**: Robust error handling and data consistency guarantees
//!
//! ## Usage Patterns
//!
//! ```rust
//! // Initialize core components
//! let scheduler = Scheduler::new();
//! let session_controller = SessionController::new();
//! let deck_manager = DeckManager::new();
//!
//! // Start a study session
//! let deck = deck_manager.get_deck(deck_id)?;
//! let mut session = session_controller.create_session(deck)?;
//!
//! // Process cards in the session
//! while let Some(card) = session.next_card()? {
//!     // Present card to user and get rating
//!     let rating = get_user_rating();
//!
//!     // Update card scheduling
//!     let next_review = scheduler.schedule_card(&card, rating)?;
//!     session.update_card_progress(card.id(), next_review)?;
//! }
//! ```

// Core algorithm modules
pub mod card_state_manager;
pub mod card_template_engine;
pub mod deck_manager;
pub mod incremental_learning;
pub mod media_manager;
pub mod scheduler;
pub mod session_controller;
pub mod stats_engine;
pub mod tag_manager;

// Re-export all public types for convenient access
pub use card_state_manager::*;
pub use card_template_engine::*;
pub use deck_manager::*;
pub use incremental_learning::*;
pub use media_manager::*;
pub use scheduler::*;
pub use session_controller::*;
pub use stats_engine::*;
pub use tag_manager::*;

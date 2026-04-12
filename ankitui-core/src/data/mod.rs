//! Data Management Layer
//!
//! This layer provides comprehensive data persistence and management capabilities for AnkiTUI.
//! It implements a clean separation between user-defined content and system-maintained state,
//! using different storage strategies optimized for each type of data.
//!
//! ## Architecture Overview
//!
//! The data layer is composed of four main components:
//!
//! ### Content Store (`content_store`)
//! - **Purpose**: Stores user-defined content (cards, decks, media references)
//! - **Format**: TOML files for human readability and manual editing
//! - **Characteristics**: User-editable, version-controllable, portable
//! - **Use Cases**: Card content, deck structures, media references, tags
//!
//! ### State Store (`state_store`)
//! - **Purpose**: Maintains system-generated learning state and analytics
//! - **Format**: SQLite database for performance and reliability
//! - **Characteristics**: System-managed, high-performance, transactional
//! - **Use Cases**: SM-2 algorithm data, review history, statistics, scheduling
//!
//! ### Models (`models`)
//! - **Purpose**: Defines data structures and type system
//! - **Format**: Rust structs with serialization support
//! - **Characteristics**: Type-safe, validated, documented
//! - **Use Cases**: Card entities, deck metadata, state enums, media types
//!
//! ### Sync Adapter (`sync_adapter`)
//! - **Purpose**: Coordinates between content and state stores
//! - **Format**: Trait-based abstraction layer
//! - **Characteristics**: Extensible, testable, consistent
//! - **Use Cases**: Data synchronization, backup/restore, migration
//!
//! ## Data Flow Principles
//!
//! 1. **Separation of Concerns**: User content and system state are stored separately
//! 2. **Atomic Operations**: All changes are transactional and consistent
//! 3. **Data Integrity**: Validation and constraints ensure data quality
//! 4. **Performance**: Optimized for both read-heavy and write-heavy operations
//! 5. **Portability**: Data formats support import/export and migration
//!
//! ## Usage Examples
//!
//! ```rust
//! // Initialize the data layer
//! let content_store = ContentStore::new(&data_dir)?;
//! let state_store = StateStore::new(&data_dir)?;
//! let sync_adapter = SyncAdapter::new(content_store, state_store);
//!
//! // Create a new deck
//! let deck = Deck::new("My Deck", "Description");
//! sync_adapter.create_deck(deck)?;
//!
//! // Add cards to the deck
//! let card = CardContent::new("Front", "Back", vec!["tag1"]);
//! sync_adapter.create_card(deck_id, card)?;
//! ```

pub mod content_store;
pub mod models;
pub mod state_store;
pub mod sync_adapter;

// Re-export all public types for convenience
pub use content_store::*;
pub use models::*;
pub use state_store::*;
pub use sync_adapter::*;

//! Data Management Layer
//!
//! This layer handles content storage (TOML) and state storage (SQLite)
//! with clear separation between user-defined content and system-maintained state.

pub mod content_store;
pub mod models;
pub mod state_store;
pub mod sync_adapter;

pub use content_store::*;
pub use models::*;
pub use state_store::*;
pub use sync_adapter::*;

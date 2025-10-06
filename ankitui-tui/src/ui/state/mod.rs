//! State management system for the TUI application

pub mod store;
pub mod selector;

// Re-export key types
pub use store::{StateStore, StateSubscription, AppState, Screen};
pub use selector::StateSelector;
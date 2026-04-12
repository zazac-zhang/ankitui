//! State management system for the TUI application

pub mod selector;
pub mod store;

// Re-export key types
pub use selector::StateSelector;
pub use store::{AppState, Screen, StateStore, StateSubscription};

//! UI layer for the TUI application

pub mod components;
pub mod layout;
pub mod theme;
pub mod render;
pub mod event;
pub mod state;
pub mod navigator;

// Re-export key UI components
pub use components::{Component, ComponentState, ComponentRegistry};
pub use state::{StateStore, AppState, Screen};
pub use event::{EventHandler, Command, CommandType};
pub use theme::Theme;
//! UI layer for the TUI application

pub mod components;
pub mod event;
pub mod layout;
pub mod navigator;
pub mod render;
pub mod state;
pub mod theme;

// Re-export key UI components
pub use components::{Component, ComponentRegistry, ComponentState};
pub use event::{Command, CommandType, EventHandler};
pub use state::{AppState, Screen, StateStore};
pub use theme::Theme;

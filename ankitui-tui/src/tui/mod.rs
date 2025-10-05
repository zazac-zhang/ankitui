//! Terminal User Interface Layer
//!
//! Clean architecture with modern component system and state management

// Modern module structure
pub mod components;
pub mod core;
pub mod rendering;
pub mod utils;

// Legacy app file (kept for reference during migration)
pub mod app;

// Modern re-exports
pub use core::{
    Action, ComponentRegistry, Events, RenderContext, StateManager,
    UIComponent,
};
pub use app::{AppState, App as Application};

// Component re-exports
pub use components::Menu as MainMenuComponent;

// Type aliases for backward compatibility
pub type App = Application;

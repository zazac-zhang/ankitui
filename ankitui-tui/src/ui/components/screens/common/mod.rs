//! DEPRECATED: This component is NOT connected to the runtime.
//! The actual rendering is in `ui/render/mod.rs` via `render_*` functions.
//! Do NOT modify this file expecting runtime behavior changes.
//!
//! Common screen components used across the application

use crate::ui::components::base::{Component, ComponentState, ContainerComponent};
use crate::ui::components::layout::Container;
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub mod confirm;
pub mod error;
pub mod input;
pub mod loading;

// Re-export common screens
pub use confirm::ConfirmScreen;
pub use error::ErrorScreen;
pub use input::InputScreen;
pub use loading::LoadingScreen;

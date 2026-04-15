//! DEPRECATED: This component is NOT connected to the runtime.
//! The actual rendering is in `ui/render/mod.rs` via `render_*` functions.
//! Do NOT modify this file expecting runtime behavior changes.
//!
//! Deck management screen components

use crate::ui::components::base::{Component, ComponentState, ContainerComponent};
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame};

pub mod browse;
pub mod create;
pub mod edit;
pub mod manage;

// Re-export deck screens
pub use browse::DeckScreen;
pub use create::DeckCreateScreen;
pub use edit::DeckEditScreen;
pub use manage::DeckManageScreen;

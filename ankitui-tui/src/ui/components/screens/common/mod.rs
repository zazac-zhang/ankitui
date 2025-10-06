//! Common screen components used across the application

use crate::ui::components::base::{Component, ComponentState, ContainerComponent};
use crate::ui::components::layout::Container;
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame, widgets::{Paragraph, Block, Borders}, style::{Style, Color}};

pub mod loading;
pub mod error;
pub mod confirm;
pub mod input;

// Re-export common screens
pub use loading::LoadingScreen;
pub use error::ErrorScreen;
pub use confirm::ConfirmScreen;
pub use input::InputScreen;
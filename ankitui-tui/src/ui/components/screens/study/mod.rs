//! Study session screen components

use crate::ui::components::base::{Component, ComponentState, ContainerComponent};
use crate::ui::components::layout::Container;
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame};

pub mod question;
pub mod answer;
pub mod rating;
pub mod finished;

// Re-export study screens
pub use question::StudyQuestionScreen;
pub use answer::StudyAnswerScreen;
pub use rating::StudyRatingScreen;
pub use finished::StudyFinishedScreen;
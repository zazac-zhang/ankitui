//! Study session screen components

use crate::ui::components::base::{Component, ComponentState, ContainerComponent};
use crate::ui::components::layout::Container;
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame};

pub mod answer;
pub mod finished;
pub mod question;
pub mod rating;

// Re-export study screens
pub use answer::StudyAnswerScreen;
pub use finished::StudyFinishedScreen;
pub use question::StudyQuestionScreen;
pub use rating::StudyRatingScreen;

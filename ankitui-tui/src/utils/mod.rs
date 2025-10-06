//! Utility modules for the TUI application

pub mod error;
pub mod result;

// Re-export commonly used types
pub use error::{TuiError, TuiResult};
pub use result::{AppResult, ComponentResult, NavigationResult, RenderResult};
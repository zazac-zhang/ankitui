//! Result types for the TUI application

use crate::utils::error::{TuiError, TuiResult};

/// Application result type
pub type AppResult = TuiResult<()>;

/// Component result type
pub type ComponentResult<T = ()> = TuiResult<T>;

/// Navigation result type
pub type NavigationResult<T = ()> = TuiResult<T>;

/// Render result type
pub type RenderResult<T = ()> = TuiResult<T>;
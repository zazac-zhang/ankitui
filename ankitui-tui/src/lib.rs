//! AnkiTUI V2 - Modern Terminal UI with Clean Architecture
//!
//! This package provides a clean, modular architecture for the AnkiTUI terminal interface,
//! following strict separation of concerns and dependency inversion principles.

pub mod app;
pub mod domain;
pub mod ui;
pub mod utils;

// Re-export key components for easier access
pub use app::{App, event_loop::run_event_loop_with_app};
pub use domain::*;
pub use utils::error::{TuiError, TuiResult};

/// Version information
pub const VERSION: &str = "0.1.0";

/// Initialize the TUI application
pub fn init() -> TuiResult<()> {
    // Initialize logging
    log::info!("Starting AnkiTUI v{}", VERSION);

    // Other initialization logic can go here
    Ok(())
}

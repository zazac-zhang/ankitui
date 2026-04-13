//! Application layer - Main application and event loop management

pub mod controller;
pub mod event_loop;
pub mod helpers;
pub mod main_app;
pub mod terminal;

// Re-export key application components
pub use controller::AppController;
pub use event_loop::{EventLoop, EventLoopConfig};
pub use main_app::{App, AppConfig};
pub use terminal::{TerminalConfig, TerminalManager};

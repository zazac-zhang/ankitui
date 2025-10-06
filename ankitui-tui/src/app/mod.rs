//! Application layer - Main application and event loop management

pub mod main_app;
pub mod event_loop;
pub mod terminal;
pub mod controller;

// Re-export key application components
pub use main_app::{App, AppConfig};
pub use event_loop::{EventLoop, EventLoopConfig};
pub use terminal::{TerminalManager, TerminalConfig};
pub use controller::AppController;
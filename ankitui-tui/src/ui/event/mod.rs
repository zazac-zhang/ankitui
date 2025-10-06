//! Event handling system for the TUI application

pub mod handler;
pub mod command;
pub mod keyboard;
pub mod mouse;

// Event types
use crossterm::event::Event as CrosstermEvent;

/// Application event wrapper
#[derive(Debug, Clone)]
pub enum Event {
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
    FocusGained,
    FocusLost,
    Paste(String),
}

// Re-export key types
pub use handler::EventHandler;
pub use command::{Command, CommandType};
pub use keyboard::{KeyEvent, KeyAction};
pub use mouse::{MouseEvent, MouseAction};
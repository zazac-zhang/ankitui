//! Error handling for the TUI application

use thiserror::Error;

/// TUI-specific error types
#[derive(Error, Debug)]
pub enum TuiError {
    #[error("Component error: {message}")]
    Component { message: String },

    #[error("State error: {message}")]
    State { message: String },

    #[error("Event error: {message}")]
    Event { message: String },

    #[error("Render error: {message}")]
    Render { message: String },

    #[error("Navigation error: {message}")]
    Navigation { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Core error: {0}")]
    Core(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] toml::de::Error),

    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

/// Result type for TUI operations
pub type TuiResult<T> = Result<T, TuiError>;

impl TuiError {
    /// Create a new component error
    pub fn component<S: Into<String>>(message: S) -> Self {
        Self::Component {
            message: message.into(),
        }
    }

    /// Create a new state error
    pub fn state<S: Into<String>>(message: S) -> Self {
        Self::State {
            message: message.into(),
        }
    }

    /// Create a new event error
    pub fn event<S: Into<String>>(message: S) -> Self {
        Self::Event {
            message: message.into(),
        }
    }

    /// Create a new render error
    pub fn render<S: Into<String>>(message: S) -> Self {
        Self::Render {
            message: message.into(),
        }
    }

    /// Create a new navigation error
    pub fn navigation<S: Into<String>>(message: S) -> Self {
        Self::Navigation {
            message: message.into(),
        }
    }
}
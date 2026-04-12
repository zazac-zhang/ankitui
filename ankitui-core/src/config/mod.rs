//! Configuration Module
//!
//! Provides unified configuration management for AnkiTUI with a clean,
//! simple interface focused on the core needs of a terminal-based flashcard system.
//!
//! ## Architecture
//!
//! This module is organized into several sub-modules:
//! - **config**: Core configuration structure and file operations
//! - **manager**: Configuration manager with lifecycle management
//! - **validator**: Configuration validation logic
//! - **provider**: Trait-based configuration access abstraction
//! - **profiles**: Predefined configuration templates
//! - **constants**: Default values and constants
//! - **daily**: Daily study limits and scheduling preferences
//! - **data**: Data storage and backup settings
//! - **scheduler**: SM-2 algorithm parameters configuration
//! - **shortcuts**: Keyboard shortcuts customization
//! - **ui**: UI theme and display settings

// Sub-modules
pub mod config;
pub mod constants;
pub mod daily;
pub mod data;
pub mod manager;
pub mod profiles;
pub mod provider;
pub mod scheduler;
pub mod shortcuts;
pub mod ui;
pub mod validator;

// Re-export dependencies for backward compatibility
pub use anyhow;
pub use dirs;
pub use serde;

// Re-export core types
pub use config::Config;
pub use constants::*;
pub use manager::ConfigManager;
pub use profiles::{
    get_config_profile, get_config_profiles, minimal_profile, power_user_profile, simple_profile, student_profile,
    ConfigProfile,
};
pub use provider::{ConfigProvider, DailyLimits, DataSettings, SchedulerParams, ShortcutMap, UiSettings};
pub use validator::{validate, ConfigValidator};

// Re-export sub-module types
pub use daily::DailyConfig;
pub use data::DataConfig;
pub use scheduler::SchedulerConfig;
pub use shortcuts::ShortcutConfig;
pub use ui::UiConfig;

/// Error type for configuration operations
pub type ConfigResult<T> = anyhow::Result<T>;

/// Configuration helper functions
pub mod helpers {
    pub use super::profiles::helpers::*;
}

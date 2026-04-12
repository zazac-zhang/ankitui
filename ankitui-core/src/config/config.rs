//! Main configuration structure and core functionality

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::validator::ConfigValidator;
use crate::config::{daily, data, scheduler, shortcuts, ui};

/// Global application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Scheduler configuration (SM-2 algorithm parameters)
    pub scheduler: scheduler::SchedulerConfig,
    /// UI theme and display settings
    pub ui: ui::UiConfig,
    /// Keyboard shortcuts mapping
    pub shortcuts: shortcuts::ShortcutConfig,
    /// Data storage configuration
    pub data: data::DataConfig,
    /// Daily limits and preferences
    pub daily: daily::DailyConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scheduler: scheduler::SchedulerConfig::default(),
            ui: ui::UiConfig::default(),
            shortcuts: shortcuts::ShortcutConfig::default(),
            data: data::DataConfig::default(),
            daily: daily::DailyConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from file, fallback to defaults
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let config = if path.exists() {
            // Load from config file (highest priority)
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read config file: {}", path.display()))?;

            let config: Config = toml::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

            config
        } else {
            // Use hardcoded defaults as fallback
            Config::default()
        };

        // Validate configuration
        ConfigValidator::validate(&config)?;

        Ok(config)
    }

    /// Load configuration with optional environment variable support
    /// Environment variables only used in specific scenarios (containers, CI/CD, etc.)
    pub fn from_file_with_env_support<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let mut config = Self::from_file(path)?;

        // Only apply environment variables if explicitly enabled
        if std::env::var("ANKITUI_ENV_ENABLED").is_ok() {
            Self::apply_env_overrides(&mut config)?;
            ConfigValidator::validate(&config)?;
        }

        Ok(config)
    }

    /// Apply environment variable overrides (only when enabled)
    fn apply_env_overrides(config: &mut Config) -> Result<()> {
        // Data directory
        if let Ok(data_dir) = std::env::var("ANKITUI_DATA_DIR") {
            config.data.data_dir = Some(PathBuf::from(data_dir));
        }

        // Daily limits
        if let Ok(max_new) = std::env::var("ANKITUI_MAX_NEW_CARDS") {
            config.daily.max_new_cards = max_new
                .parse()
                .context("Invalid ANKITUI_MAX_NEW_CARDS value")?;
        }

        if let Ok(max_review) = std::env::var("ANKITUI_MAX_REVIEW_CARDS") {
            config.daily.max_review_cards = max_review
                .parse()
                .context("Invalid ANKITUI_MAX_REVIEW_CARDS value")?;
        }

        // UI settings
        if let Ok(theme) = std::env::var("ANKITUI_THEME") {
            config.ui.theme = theme;
        }

        if let Ok(mouse_support) = std::env::var("ANKITUI_MOUSE_SUPPORT") {
            config.ui.mouse_support = mouse_support
                .parse()
                .context("Invalid ANKITUI_MOUSE_SUPPORT value (should be true/false)")?;
        }

        // Scheduler settings
        if let Ok(starting_ease) = std::env::var("ANKITUI_STARTING_EASE_FACTOR") {
            config.scheduler.starting_ease_factor = starting_ease
                .parse()
                .context("Invalid ANKITUI_STARTING_EASE_FACTOR value")?;
        }

        if let Ok(max_interval) = std::env::var("ANKITUI_MAX_INTERVAL") {
            config.scheduler.max_interval = max_interval
                .parse()
                .context("Invalid ANKITUI_MAX_INTERVAL value")?;
        }

        // Backup settings
        if let Ok(auto_backup) = std::env::var("ANKITUI_AUTO_BACKUP") {
            config.data.auto_backup = auto_backup
                .parse()
                .context("Invalid ANKITUI_AUTO_BACKUP value (should be true/false)")?;
        }

        if let Ok(backup_count) = std::env::var("ANKITUI_BACKUP_COUNT") {
            config.data.backup_count = backup_count
                .parse()
                .context("Invalid ANKITUI_BACKUP_COUNT value")?;
        }

        Ok(())
    }

    /// Export current configuration to a TOML file
    pub fn export_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content =
            toml::to_string_pretty(self).context("Failed to serialize config for export")?;

        std::fs::write(path, content)
            .with_context(|| format!("Failed to write exported config file: {}", path.display()))?;

        Ok(())
    }

    /// Export configuration with comments explaining each section
    pub fn export_with_comments<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let comments = self.generate_config_comments();
        std::fs::write(path, comments)
            .with_context(|| format!("Failed to write exported config file: {}", path.display()))?;

        Ok(())
    }

    /// Generate commented configuration
    pub fn generate_config_comments(&self) -> String {
        format!(
            r#"# AnkiTUI Configuration File
# Generated on: {}
#
# Configuration priority:
# 1. Config file (this file) - highest priority
# 2. Hardcoded defaults - fallback
# 3. Environment variables (when ANKITUI_ENV_ENABLED=true) - for containers/CI
#
# For available environment variables, see: ankitui config --help-env

[daily]
# Daily study limits
max_new_cards = {}
max_review_cards = {}
day_start_hour = {}
day_end_hour = {}

[scheduler]
# SM-2 Algorithm parameters
starting_ease_factor = {}
max_interval = {}
easy_interval = {}
good_interval = {}

[ui]
# User interface settings
theme = "{}"
mouse_support = {}
show_progress = {}
show_card_counter = {}

[data]
# Data storage and backup settings
auto_backup = {}
backup_count = {}
backup_interval = {}

[shortcuts]
# Keyboard shortcuts
show_answer = "{}"
rate_again = "{}"
rate_hard = "{}"
rate_good = "{}"
rate_easy = "{}"
"#,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            self.daily.max_new_cards,
            self.daily.max_review_cards,
            self.daily.day_start_hour,
            self.daily.day_end_hour,
            self.scheduler.starting_ease_factor,
            self.scheduler.max_interval,
            self.scheduler.easy_interval,
            self.scheduler.good_interval,
            self.ui.theme,
            self.ui.mouse_support,
            self.ui.show_progress,
            self.ui.show_card_counter,
            self.data.auto_backup,
            self.data.backup_count,
            self.data.backup_interval,
            self.shortcuts.show_answer,
            self.shortcuts.rate_again,
            self.shortcuts.rate_hard,
            self.shortcuts.rate_good,
            self.shortcuts.rate_easy,
        )
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        std::fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Get default configuration file path
    pub fn default_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

        Ok(config_dir.join("ankitui").join("config.toml"))
    }

    /// Create a default config file if it doesn't exist
    pub fn ensure_default_exists() -> Result<Self> {
        let path = Self::default_path()?;

        if !path.exists() {
            let default_config = Config::default();
            default_config.save_to_file(&path)?;
            println!("Created default config at: {}", path.display());
            Ok(default_config)
        } else {
            Self::from_file(&path)
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        ConfigValidator::validate(self)
    }
}
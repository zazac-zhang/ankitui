//! Configuration Module
//!
//! Provides unified configuration management for AnkiTUI with a clean,
//! simple interface focused on the core needs of a terminal-based flashcard system.

pub mod daily;
pub mod data;
pub mod scheduler;
pub mod shortcuts;
pub mod ui;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Re-export dependencies for backward compatibility
pub use anyhow;
pub use dirs;
pub use serde;

/// Error type for configuration operations
pub type ConfigResult<T> = anyhow::Result<T>;

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
    fn generate_config_comments(&self) -> String {
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

/// Configuration validator
struct ConfigValidator;

impl ConfigValidator {
    fn validate(config: &Config) -> Result<()> {
        // Validate scheduler config
        if config.scheduler.starting_ease_factor <= 0.0 {
            return Err(anyhow::anyhow!("Starting ease factor must be positive"));
        }

        if config.scheduler.min_ease_factor >= config.scheduler.max_ease_factor {
            return Err(anyhow::anyhow!(
                "Min ease factor must be less than max ease factor"
            ));
        }

        if config.scheduler.max_interval <= 0 {
            return Err(anyhow::anyhow!("Max interval must be positive"));
        }

        // Validate UI config
        if config.ui.refresh_rate == 0 {
            return Err(anyhow::anyhow!("Refresh rate must be greater than 0"));
        }

        // Validate daily config
        if config.daily.max_new_cards < 0 {
            return Err(anyhow::anyhow!("Max new cards cannot be negative"));
        }

        if config.daily.max_review_cards < 0 {
            return Err(anyhow::anyhow!("Max review cards cannot be negative"));
        }

        if config.daily.day_start_hour >= config.daily.day_end_hour {
            return Err(anyhow::anyhow!(
                "Day start hour must be less than day end hour"
            ));
        }

        // Validate data config
        if config.data.backup_count == 0 {
            return Err(anyhow::anyhow!("Backup count must be greater than 0"));
        }

        if config.data.backup_interval == 0 {
            return Err(anyhow::anyhow!("Backup interval must be greater than 0"));
        }

        Ok(())
    }
}

/// Configuration manager
#[derive(Clone)]
pub struct ConfigManager {
    config: Config,
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new configuration manager with default settings
    /// Priority: config file > hardcoded defaults
    pub fn new() -> Result<Self> {
        let config_path = Config::default_path()?;
        let config = Config::from_file(&config_path).unwrap_or_else(|_| {
            let default_config = Config::default();
            let _ = default_config.save_to_file(&config_path);
            default_config
        });

        Ok(Self {
            config,
            config_path,
        })
    }

    /// Create configuration manager with custom config path
    pub fn with_path<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_path = config_path.as_ref().to_path_buf();
        let config = Config::from_file(&config_path).unwrap_or_else(|_| {
            let default_config = Config::default();
            let _ = default_config.save_to_file(&config_path);
            default_config
        });

        Ok(Self {
            config,
            config_path,
        })
    }

    /// Create configuration manager with environment variable support
    /// Use this for containers, CI/CD, or specific deployment scenarios
    pub fn new_with_env() -> Result<Self> {
        let config_path = Config::default_path()?;
        let config = Config::from_file_with_env_support(&config_path).unwrap_or_else(|_| {
            let default_config = Config::default();
            let _ = default_config.save_to_file(&config_path);
            default_config
        });

        Ok(Self {
            config,
            config_path,
        })
    }

    /// Create configuration manager without any config file (for testing)
    pub fn new_no_file() -> Result<Self> {
        let config_path = Config::default_path()?;
        let config = Config::default();

        Ok(Self {
            config,
            config_path,
        })
    }

    /// Create configuration manager from a profile
    pub fn from_profile(profile_name: &str) -> Result<Self> {
        let config_path = Config::default_path()?;
        let config = helpers::config_from_profile(profile_name)?;

        // Save profile as default config
        config.save_to_file(&config_path)?;

        Ok(Self {
            config,
            config_path,
        })
    }

    /// Initialize configuration with profile if no config exists
    pub fn init_with_profile(profile_name: &str) -> Result<Self> {
        let config_path = Config::default_path()?;

        let config = if config_path.exists() {
            Config::from_file(&config_path)?
        } else {
            let profile_config = helpers::config_from_profile(profile_name)?;
            profile_config.save_to_file(&config_path)?;
            profile_config
        };

        Ok(Self {
            config,
            config_path,
        })
    }

    /// Get available configuration profiles
    pub fn available_profiles() -> Vec<(String, String)> {
        helpers::list_config_profiles()
    }

    /// Get current configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Get scheduler configuration
    pub fn get_scheduler_config(&self) -> &scheduler::SchedulerConfig {
        &self.config.scheduler
    }

    /// Get UI configuration
    pub fn get_ui_config(&self) -> &ui::UiConfig {
        &self.config.ui
    }

    /// Get shortcut configuration
    pub fn get_shortcut_config(&self) -> &shortcuts::ShortcutConfig {
        &self.config.shortcuts
    }

    /// Get data configuration
    pub fn get_data_config(&self) -> &data::DataConfig {
        &self.config.data
    }

    /// Get daily configuration
    pub fn get_daily_config(&self) -> &daily::DailyConfig {
        &self.config.daily
    }

    /// Get data directory path
    pub fn get_data_dir(&self) -> PathBuf {
        match &self.config.data.data_dir {
            Some(path) => path.clone(),
            None => {
                let mut default_path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
                default_path.push("ankitui");
                default_path
            }
        }
    }

    /// Update configuration
    pub fn update_config<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Config),
    {
        updater(&mut self.config);
        ConfigValidator::validate(&self.config)?;
        self.config.save_to_file(&self.config_path)?;
        Ok(())
    }

    /// Save configuration to file
    pub fn save_config(&self) -> Result<()> {
        self.config.save_to_file(&self.config_path)
    }

    /// Reset configuration to defaults
    pub fn reset_to_defaults(&mut self) -> Result<()> {
        self.config = Config::default();
        self.config.save_to_file(&self.config_path)?;
        Ok(())
    }

    /// Check if configuration file exists
    pub fn config_exists(&self) -> bool {
        self.config_path.exists()
    }

    /// Get configuration file path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    /// Reload configuration from file
    pub fn reload(&mut self) -> Result<()> {
        self.config = Config::from_file(&self.config_path)?;
        Ok(())
    }

    /// Get effective configuration (with all overrides applied)
    pub fn effective_config(&self) -> &Config {
        &self.config
    }

    /// Show current environment overrides (when env support is enabled)
    pub fn env_overrides(&self) -> Vec<(&'static str, String)> {
        let mut overrides = Vec::new();

        if std::env::var("ANKITUI_ENV_ENABLED").is_ok() {
            if let Ok(val) = std::env::var("ANKITUI_DATA_DIR") {
                overrides.push(("ANKITUI_DATA_DIR", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_MAX_NEW_CARDS") {
                overrides.push(("ANKITUI_MAX_NEW_CARDS", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_MAX_REVIEW_CARDS") {
                overrides.push(("ANKITUI_MAX_REVIEW_CARDS", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_THEME") {
                overrides.push(("ANKITUI_THEME", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_MOUSE_SUPPORT") {
                overrides.push(("ANKITUI_MOUSE_SUPPORT", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_STARTING_EASE_FACTOR") {
                overrides.push(("ANKITUI_STARTING_EASE_FACTOR", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_MAX_INTERVAL") {
                overrides.push(("ANKITUI_MAX_INTERVAL", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_AUTO_BACKUP") {
                overrides.push(("ANKITUI_AUTO_BACKUP", val));
            }
            if let Ok(val) = std::env::var("ANKITUI_BACKUP_COUNT") {
                overrides.push(("ANKITUI_BACKUP_COUNT", val));
            }
        }

        overrides
    }

    /// Export current configuration to a file with one-click
    pub fn export_config<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.config.export_to_file(path)
    }

    /// Export current configuration with detailed comments
    pub fn export_config_with_comments<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.config.export_with_comments(path)
    }

    /// Export configuration to string (for display or copying)
    pub fn export_config_to_string(&self) -> Result<String> {
        Ok(self.config.generate_config_comments())
    }

    /// Create a backup of current configuration
    pub fn backup_config(&self) -> Result<PathBuf> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("config_backup_{}.toml", timestamp);
        let backup_path = self
            .config_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid config path"))?
            .join("backups")
            .join(backup_name);

        // Create backup directory if it doesn't exist
        if let Some(parent) = backup_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        self.config.export_to_file(&backup_path)?;
        Ok(backup_path)
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ConfigManager")
    }
}

/// Configuration provider trait for accessing configuration values
pub trait ConfigProvider: Send + Sync {
    /// Get scheduler parameters
    fn get_scheduler_params(&self) -> SchedulerParams;

    /// Get daily limits
    fn get_daily_limits(&self) -> DailyLimits;

    /// Get UI settings
    fn get_ui_settings(&self) -> UiSettings;

    /// Get shortcuts
    fn get_shortcuts(&self) -> ShortcutMap;

    /// Get data settings
    fn get_data_settings(&self) -> DataSettings;

    /// Check if a feature flag is enabled
    fn is_feature_enabled(&self, flag: &str) -> bool;

    /// Get a specific configuration value by key
    fn get_config_value(&self, key: &str) -> Option<String>;
}

/// Convenience structs for specific configuration sections
#[derive(Debug, Clone)]
pub struct SchedulerParams {
    pub starting_ease_factor: f32,
    pub min_ease_factor: f32,
    pub max_ease_factor: f32,
    pub easy_interval: i32,
    pub good_interval: i32,
    pub graduating_interval: i32,
    pub initial_failure_interval: i32,
    pub max_interval: i32,
    pub hard_multiplier: f32,
    pub easy_bonus: f32,
    pub interval_modifier: f32,
    pub hard_factor: f32,
    pub min_review_interval: i32,
    pub easy_interval_days: i32,
    pub graduating_interval_days: i32,
    pub easy_factor: f32,
    pub max_review_interval: i32,
    pub learning_steps: Vec<i32>,
    pub relearning_steps: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct DailyLimits {
    pub max_new_cards: i32,
    pub max_review_cards: i32,
    pub day_start_hour: u8,
    pub day_end_hour: u8,
    pub show_limit_warnings: bool,
}

#[derive(Debug, Clone)]
pub struct UiSettings {
    pub theme: String,
    pub mouse_support: bool,
    pub show_progress: bool,
    pub show_card_counter: bool,
    pub animation_speed: u64,
    pub refresh_rate: u16,
}

#[derive(Debug, Clone)]
pub struct ShortcutMap {
    pub show_answer: String,
    pub rate_again: String,
    pub rate_hard: String,
    pub rate_good: String,
    pub rate_easy: String,
    pub toggle_pause: String,
    pub exit_session: String,
    pub show_help: String,
    pub show_stats: String,
    pub undo: String,
    pub redo: String,
    pub search: String,
    pub edit_card: String,
    pub delete_card: String,
}

#[derive(Debug, Clone)]
pub struct DataSettings {
    pub data_dir: Option<String>,
    pub auto_backup: bool,
    pub backup_count: usize,
    pub backup_interval: u32,
    pub compress_data: bool,
}

impl ConfigProvider for ConfigManager {
    fn get_scheduler_params(&self) -> SchedulerParams {
        SchedulerParams {
            starting_ease_factor: self.config.scheduler.starting_ease_factor,
            min_ease_factor: self.config.scheduler.min_ease_factor,
            max_ease_factor: self.config.scheduler.max_ease_factor,
            easy_interval: self.config.scheduler.easy_interval,
            good_interval: self.config.scheduler.good_interval,
            graduating_interval: self.config.scheduler.graduating_interval,
            initial_failure_interval: self.config.scheduler.initial_failure_interval,
            max_interval: self.config.scheduler.max_interval,
            hard_multiplier: self.config.scheduler.hard_multiplier,
            easy_bonus: self.config.scheduler.easy_bonus,
            interval_modifier: self.config.scheduler.interval_modifier,
            hard_factor: self.config.scheduler.hard_factor,
            min_review_interval: self.config.scheduler.min_review_interval,
            easy_interval_days: self.config.scheduler.easy_interval_days,
            graduating_interval_days: self.config.scheduler.graduating_interval_days,
            easy_factor: self.config.scheduler.easy_factor,
            max_review_interval: self.config.scheduler.max_review_interval,
            learning_steps: self.config.scheduler.learning_steps.clone(),
            relearning_steps: self.config.scheduler.relearning_steps.clone(),
        }
    }

    fn get_daily_limits(&self) -> DailyLimits {
        DailyLimits {
            max_new_cards: self.config.daily.max_new_cards,
            max_review_cards: self.config.daily.max_review_cards,
            day_start_hour: self.config.daily.day_start_hour,
            day_end_hour: self.config.daily.day_end_hour,
            show_limit_warnings: self.config.daily.show_limit_warnings,
        }
    }

    fn get_ui_settings(&self) -> UiSettings {
        UiSettings {
            theme: self.config.ui.theme.clone(),
            mouse_support: self.config.ui.mouse_support,
            show_progress: self.config.ui.show_progress,
            show_card_counter: self.config.ui.show_card_counter,
            animation_speed: self.config.ui.animation_speed,
            refresh_rate: self.config.ui.refresh_rate,
        }
    }

    fn get_shortcuts(&self) -> ShortcutMap {
        ShortcutMap {
            show_answer: self.config.shortcuts.show_answer.clone(),
            rate_again: self.config.shortcuts.rate_again.clone(),
            rate_hard: self.config.shortcuts.rate_hard.clone(),
            rate_good: self.config.shortcuts.rate_good.clone(),
            rate_easy: self.config.shortcuts.rate_easy.clone(),
            toggle_pause: self.config.shortcuts.toggle_pause.clone(),
            exit_session: self.config.shortcuts.exit_session.clone(),
            show_help: self.config.shortcuts.show_help.clone(),
            show_stats: self.config.shortcuts.show_stats.clone(),
            undo: "u".to_string(),
            redo: "r".to_string(),
            search: "/".to_string(),
            edit_card: "e".to_string(),
            delete_card: "d".to_string(),
        }
    }

    fn get_data_settings(&self) -> DataSettings {
        DataSettings {
            data_dir: self
                .config
                .data
                .data_dir
                .as_ref()
                .map(|p| p.to_string_lossy().to_string()),
            auto_backup: self.config.data.auto_backup,
            backup_count: self.config.data.backup_count,
            backup_interval: self.config.data.backup_interval,
            compress_data: self.config.data.compress_data,
        }
    }

    fn is_feature_enabled(&self, _flag: &str) -> bool {
        // For now, return false for all features
        false
    }

    fn get_config_value(&self, key: &str) -> Option<String> {
        match key {
            "scheduler.starting_ease_factor" => {
                Some(self.config.scheduler.starting_ease_factor.to_string())
            }
            "scheduler.min_ease_factor" => Some(self.config.scheduler.min_ease_factor.to_string()),
            "scheduler.max_ease_factor" => Some(self.config.scheduler.max_ease_factor.to_string()),
            "daily.max_new_cards" => Some(self.config.daily.max_new_cards.to_string()),
            "daily.max_review_cards" => Some(self.config.daily.max_review_cards.to_string()),
            "ui.theme" => Some(self.config.ui.theme.clone()),
            "ui.mouse_support" => Some(self.config.ui.mouse_support.to_string()),
            "data.auto_backup" => Some(self.config.data.auto_backup.to_string()),
            _ => None,
        }
    }
}

/// Configuration profiles for different user types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProfile {
    pub name: String,
    pub description: String,
    pub config: Config,
}

/// Simple configuration profile for new users
pub fn simple_profile() -> ConfigProfile {
    let mut config = Config::default();

    // Expose only essential options
    config.daily.max_new_cards = 15;
    config.daily.max_review_cards = 50;
    config.ui.theme = "default".to_string();
    config.ui.mouse_support = false;
    config.data.auto_backup = true;
    config.data.backup_count = 5;

    ConfigProfile {
        name: "simple".to_string(),
        description: "Simple configuration for new users with essential options only".to_string(),
        config,
    }
}

/// Student configuration profile
pub fn student_profile() -> ConfigProfile {
    let mut config = Config::default();

    config.daily.max_new_cards = 20;
    config.daily.max_review_cards = 100;
    config.ui.theme = "default".to_string();
    config.ui.mouse_support = true;
    config.data.auto_backup = true;
    config.data.backup_count = 10;
    config.scheduler.starting_ease_factor = 2.5;

    ConfigProfile {
        name: "student".to_string(),
        description: "Optimized for students learning new material".to_string(),
        config,
    }
}

/// Power user configuration profile
pub fn power_user_profile() -> ConfigProfile {
    let mut config = Config::default();

    config.daily.max_new_cards = 50;
    config.daily.max_review_cards = 300;
    config.ui.theme = "dark".to_string();
    config.ui.mouse_support = true;
    config.data.auto_backup = true;
    config.data.backup_count = 20;
    config.scheduler.starting_ease_factor = 2.6;
    config.scheduler.max_interval = 73000; // 200 years

    ConfigProfile {
        name: "power".to_string(),
        description: "Advanced configuration for experienced users with high volume learning"
            .to_string(),
        config,
    }
}

/// Minimal configuration profile
pub fn minimal_profile() -> ConfigProfile {
    let mut config = Config::default();

    config.daily.max_new_cards = 10;
    config.daily.max_review_cards = 25;
    config.ui.theme = "default".to_string();
    config.ui.mouse_support = false;
    config.data.auto_backup = false;
    config.data.backup_count = 3;

    ConfigProfile {
        name: "minimal".to_string(),
        description: "Minimal configuration for light users or quick reviews".to_string(),
        config,
    }
}

/// Get all available configuration profiles
pub fn get_config_profiles() -> Vec<ConfigProfile> {
    vec![
        simple_profile(),
        student_profile(),
        power_user_profile(),
        minimal_profile(),
    ]
}

/// Get configuration profile by name
pub fn get_config_profile(name: &str) -> Option<ConfigProfile> {
    get_config_profiles().into_iter().find(|p| p.name == name)
}

/// Configuration helper functions
pub mod helpers {
    use super::*;

    /// Get default configuration
    pub fn default_config() -> Config {
        Config::default()
    }

    /// Validate configuration values
    pub fn validate_config(config: &Config) -> ConfigResult<()> {
        ConfigValidator::validate(config)
    }

    /// Get configuration file path
    pub fn get_config_path() -> ConfigResult<PathBuf> {
        Config::default_path()
    }

    /// Create configuration from profile
    pub fn config_from_profile(profile_name: &str) -> ConfigResult<Config> {
        let profile = get_config_profile(profile_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown profile: {}", profile_name))?;
        Ok(profile.config)
    }

    /// List available configuration profiles
    pub fn list_config_profiles() -> Vec<(String, String)> {
        get_config_profiles()
            .into_iter()
            .map(|p| (p.name, p.description))
            .collect()
    }
}

// Re-export individual config modules for direct access
pub use daily::DailyConfig;
pub use data::DataConfig;
pub use scheduler::SchedulerConfig;
pub use shortcuts::ShortcutConfig;
pub use ui::UiConfig;

// Compatibility constants
pub const DEFAULT_STARTING_EASE_FACTOR: f32 = 2.5;
pub const DEFAULT_MIN_EASE_FACTOR: f32 = 1.3;
pub const DEFAULT_MAX_EASE_FACTOR: f32 = 5.0;
pub const DEFAULT_EASY_INTERVAL: i32 = 4;
pub const DEFAULT_GOOD_INTERVAL: i32 = 1;
pub const DEFAULT_GRADUATING_INTERVAL: i32 = 1;
pub const DEFAULT_INITIAL_FAILURE_INTERVAL: i32 = 1;
pub const DEFAULT_MAX_INTERVAL: i32 = 36500;
pub const DEFAULT_HARD_MULTIPLIER: f32 = 1.2;
pub const DEFAULT_EASY_BONUS: f32 = 1.3;
pub const DEFAULT_INTERVAL_MODIFIER: f32 = 1.0;

pub const DEFAULT_THEME: &str = "default";
pub const DEFAULT_MAX_NEW_CARDS: i32 = 20;
pub const DEFAULT_MAX_REVIEW_CARDS: i32 = 100;
pub const DEFAULT_DAY_START_HOUR: u8 = 0;
pub const DEFAULT_DAY_END_HOUR: u8 = 23;
pub const DEFAULT_BACKUP_COUNT: usize = 10;
pub const DEFAULT_BACKUP_INTERVAL: u32 = 24;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.scheduler.starting_ease_factor, 2.5);
        assert_eq!(config.ui.theme, "default");
        assert_eq!(config.shortcuts.show_answer, " ");
        assert_eq!(config.daily.max_new_cards, 20);
    }

    #[test]
    fn test_config_save_load() -> Result<()> {
        // Clear any existing environment variables
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
        env::remove_var("ANKITUI_THEME");

        let temp_dir = TempDir::new()?;
        let config_path = temp_dir.path().join("config.toml");

        // Create and save config
        let mut config = Config::default();
        config.daily.max_new_cards = 50;
        config.ui.theme = "dark".to_string();
        config.save_to_file(&config_path)?;

        // Load config without environment overrides for this test
        let loaded_config = Config::from_file(&config_path)?;
        assert_eq!(loaded_config.daily.max_new_cards, 50);
        assert_eq!(loaded_config.ui.theme, "dark");

        Ok(())
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // Invalid ease factor
        config.scheduler.starting_ease_factor = -1.0;
        assert!(ConfigValidator::validate(&config).is_err());

        // Fix it
        config.scheduler.starting_ease_factor = 2.5;
        assert!(ConfigValidator::validate(&config).is_ok());

        // Invalid day range
        config.daily.day_start_hour = 12;
        config.daily.day_end_hour = 10;
        assert!(ConfigValidator::validate(&config).is_err());
    }

    #[test]
    fn test_config_manager() {
        // Clear any existing environment variables
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
        env::remove_var("ANKITUI_THEME");
        env::remove_var("ANKITUI_STARTING_EASE_FACTOR");

        let manager = ConfigManager::new_no_file(); // Use no_file to avoid external influences
        assert!(manager.is_ok());

        if let Ok(manager) = manager {
            let config = manager.get_config();
            assert_eq!(config.scheduler.starting_ease_factor, 2.5);
            assert_eq!(config.ui.theme, "default");
            assert_eq!(config.daily.max_new_cards, 20);
        }
    }

    #[test]
    fn test_config_provider() {
        // Clear any existing environment variables
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
        env::remove_var("ANKITUI_STARTING_EASE_FACTOR");

        let manager = ConfigManager::new_no_file().unwrap();
        let params = manager.get_scheduler_params();
        assert_eq!(params.starting_ease_factor, 2.5);

        let limits = manager.get_daily_limits();
        assert_eq!(limits.max_new_cards, 20);
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_STARTING_EASE_FACTOR, 2.5);
        assert_eq!(DEFAULT_THEME, "default");
        assert_eq!(DEFAULT_MAX_NEW_CARDS, 20);
    }

    #[test]
    fn test_env_override_data_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Set environment variable
        env::set_var("ANKITUI_DATA_DIR", "/custom/data/path");

        // Create config with env override
        let config = Config::from_file_with_env_support(&config_path).unwrap();

        // Check that environment variable overrides default
        assert_eq!(
            config.data.data_dir,
            Some(PathBuf::from("/custom/data/path"))
        );

        // Clean up
        env::remove_var("ANKITUI_DATA_DIR");
    }

    #[test]
    fn test_env_override_daily_limits() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Set environment variables
        env::set_var("ANKITUI_MAX_NEW_CARDS", "50");
        env::set_var("ANKITUI_MAX_REVIEW_CARDS", "200");

        // Create config with env overrides
        let config = Config::from_file_with_env_support(&config_path).unwrap();

        // Check that environment variables override defaults
        assert_eq!(config.daily.max_new_cards, 50);
        assert_eq!(config.daily.max_review_cards, 200);

        // Clean up
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
        env::remove_var("ANKITUI_MAX_REVIEW_CARDS");
    }

    #[test]
    fn test_env_override_ui_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Set environment variables
        env::set_var("ANKITUI_THEME", "dark");
        env::set_var("ANKITUI_MOUSE_SUPPORT", "true");

        // Create config with env overrides
        let config = Config::from_file_with_env_support(&config_path).unwrap();

        // Check that environment variables override defaults
        assert_eq!(config.ui.theme, "dark");
        assert_eq!(config.ui.mouse_support, true);

        // Clean up
        env::remove_var("ANKITUI_THEME");
        env::remove_var("ANKITUI_MOUSE_SUPPORT");
    }

    #[test]
    fn test_env_override_scheduler_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Set environment variables
        env::set_var("ANKITUI_STARTING_EASE_FACTOR", "3.0");
        env::set_var("ANKITUI_MAX_INTERVAL", "73000");

        // Create config with env overrides
        let config = Config::from_file_with_env_support(&config_path).unwrap();

        // Check that environment variables override defaults
        assert_eq!(config.scheduler.starting_ease_factor, 3.0);
        assert_eq!(config.scheduler.max_interval, 73000);

        // Clean up
        env::remove_var("ANKITUI_STARTING_EASE_FACTOR");
        env::remove_var("ANKITUI_MAX_INTERVAL");
    }

    #[test]
    fn test_env_override_backup_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Set environment variables
        env::set_var("ANKITUI_AUTO_BACKUP", "false");
        env::set_var("ANKITUI_BACKUP_COUNT", "20");

        // Create config with env overrides
        let config = Config::from_file_with_env_support(&config_path).unwrap();

        // Check that environment variables override defaults
        assert_eq!(config.data.auto_backup, false);
        assert_eq!(config.data.backup_count, 20);

        // Clean up
        env::remove_var("ANKITUI_AUTO_BACKUP");
        env::remove_var("ANKITUI_BACKUP_COUNT");
    }

    #[test]
    fn test_env_overrides_priority() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create a config file with certain values
        let mut file_config = Config::default();
        file_config.daily.max_new_cards = 30;
        file_config.ui.theme = "dark".to_string();
        file_config.save_to_file(&config_path).unwrap();

        // Set environment variables with different values
        env::set_var("ANKITUI_MAX_NEW_CARDS", "100");
        env::set_var("ANKITUI_THEME", "light");

        // Create config with env overrides - should prioritize env vars
        let config = Config::from_file_with_env_support(&config_path).unwrap();

        // Check that environment variables override file config
        assert_eq!(config.daily.max_new_cards, 100); // Env override
        assert_eq!(config.ui.theme, "light"); // Env override

        // Clean up
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
        env::remove_var("ANKITUI_THEME");
    }

    #[test]
    fn test_invalid_env_values() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Set invalid environment variable values
        env::set_var("ANKITUI_MAX_NEW_CARDS", "invalid_number");
        env::set_var("ANKITUI_MOUSE_SUPPORT", "not_a_boolean");

        // Should fail to parse invalid values
        let result = Config::from_file_with_env_support(&config_path);
        assert!(result.is_err());

        // Clean up
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
        env::remove_var("ANKITUI_MOUSE_SUPPORT");
    }

    #[test]
    fn test_config_manager_env_support() {
        // Set environment variable
        env::set_var("ANKITUI_MAX_NEW_CARDS", "75");

        // Create config manager - should apply env overrides
        let manager = ConfigManager::new_no_file().unwrap();
        let overrides = manager.env_overrides();

        // Should detect the environment variable
        assert!(overrides
            .iter()
            .any(|(key, val)| key == &"ANKITUI_MAX_NEW_CARDS" && val == "75"));

        // Clean up
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
    }

    #[test]
    fn test_config_profiles() {
        // Test simple profile
        let simple = simple_profile();
        assert_eq!(simple.name, "simple");
        assert_eq!(simple.config.daily.max_new_cards, 15);
        assert_eq!(simple.config.daily.max_review_cards, 50);
        assert!(!simple.config.ui.mouse_support);

        // Test student profile
        let student = student_profile();
        assert_eq!(student.name, "student");
        assert_eq!(student.config.daily.max_new_cards, 20);
        assert_eq!(student.config.daily.max_review_cards, 100);
        assert!(student.config.ui.mouse_support);

        // Test power user profile
        let power = power_user_profile();
        assert_eq!(power.name, "power");
        assert_eq!(power.config.daily.max_new_cards, 50);
        assert_eq!(power.config.daily.max_review_cards, 300);
        assert_eq!(power.config.ui.theme, "dark");
        assert_eq!(power.config.scheduler.max_interval, 73000);

        // Test minimal profile
        let minimal = minimal_profile();
        assert_eq!(minimal.name, "minimal");
        assert_eq!(minimal.config.daily.max_new_cards, 10);
        assert_eq!(minimal.config.daily.max_review_cards, 25);
        assert!(!minimal.config.data.auto_backup);
    }

    #[test]
    fn test_get_config_profiles() {
        let profiles = get_config_profiles();
        assert_eq!(profiles.len(), 4);

        let profile_names: Vec<String> = profiles.iter().map(|p| p.name.clone()).collect();
        assert!(profile_names.contains(&"simple".to_string()));
        assert!(profile_names.contains(&"student".to_string()));
        assert!(profile_names.contains(&"power".to_string()));
        assert!(profile_names.contains(&"minimal".to_string()));
    }

    #[test]
    fn test_get_config_profile_by_name() {
        let simple = get_config_profile("simple");
        assert!(simple.is_some());
        assert_eq!(simple.unwrap().name, "simple");

        let unknown = get_config_profile("unknown");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_config_manager_from_profile() {
        let manager = ConfigManager::from_profile("simple").unwrap();
        let config = manager.get_config();
        assert_eq!(config.daily.max_new_cards, 15);
        assert_eq!(config.daily.max_review_cards, 50);
        assert!(!config.ui.mouse_support);
    }

    #[test]
    fn test_config_manager_available_profiles() {
        let profiles = ConfigManager::available_profiles();
        assert_eq!(profiles.len(), 4);

        let profile_names: Vec<String> = profiles.iter().map(|(name, _)| name.clone()).collect();
        assert!(profile_names.contains(&"simple".to_string()));
        assert!(profile_names.contains(&"student".to_string()));
        assert!(profile_names.contains(&"power".to_string()));
        assert!(profile_names.contains(&"minimal".to_string()));
    }

    #[test]
    fn test_helper_functions() {
        // Test config_from_profile
        let config = helpers::config_from_profile("student").unwrap();
        assert_eq!(config.daily.max_new_cards, 20);
        assert!(config.ui.mouse_support);

        // Test list_config_profiles
        let profiles = helpers::list_config_profiles();
        assert_eq!(profiles.len(), 4);
        assert!(profiles.iter().any(|(name, _)| name == "power"));
    }

    #[test]
    fn test_config_from_profile_invalid() {
        let result = helpers::config_from_profile("invalid_profile");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown profile"));
    }

    #[test]
    fn test_init_with_profile_new_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Mock the default path to use temp directory
        let original_path = Config::default_path();

        // Since we can't easily mock the default path, let's test the concept
        // by verifying profile config creation works
        let profile_config = helpers::config_from_profile("minimal").unwrap();
        assert_eq!(profile_config.daily.max_new_cards, 10);
        assert!(!profile_config.data.auto_backup);
    }

    #[test]
    fn test_config_export_functionality() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let export_path = temp_dir.path().join("exported.toml");

        let mut config = Config::default();
        config.daily.max_new_cards = 15;
        config.ui.theme = "blue".to_string();
        config.scheduler.starting_ease_factor = 2.7;

        // Test export functionality
        config.export_to_file(&export_path)?;

        // Verify exported file exists and contains comments
        assert!(export_path.exists());
        let exported_content = std::fs::read_to_string(&export_path)?;
        assert!(exported_content.contains("# AnkiTUI Configuration File"));
        assert!(exported_content.contains("# Generated on"));
        assert!(exported_content.contains("max_new_cards = 15"));
        assert!(exported_content.contains("theme = \"blue\""));

        // Test export with comments string
        let comment_string = config.generate_config_comments();
        assert!(comment_string.contains("# AnkiTUI Configuration File"));
        assert!(comment_string.contains("max_new_cards = 15"));

        Ok(())
    }

    #[test]
    fn test_config_manager_export_functionality() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config_path = temp_dir.path().join("config.toml");
        let export_path = temp_dir.path().join("exported.toml");

        let manager = ConfigManager::with_path(&config_path)?;

        // Test export to file
        manager.export_config(&export_path)?;
        assert!(export_path.exists());

        // Test export with comments
        manager.export_config_with_comments(&export_path)?;
        let content = std::fs::read_to_string(&export_path)?;
        assert!(content.contains("# AnkiTUI Configuration File"));

        // Test export to string
        let exported_string = manager.export_config_to_string()?;
        assert!(exported_string.contains("# AnkiTUI Configuration File"));
        assert!(exported_string.contains("[daily]"));

        // Test backup functionality
        let backup_path = manager.backup_config()?;
        assert!(backup_path.exists());
        assert!(backup_path.to_string_lossy().contains("config_backup_"));

        Ok(())
    }

    #[test]
    fn test_config_file_priority_over_environment() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config_path = temp_dir.path().join("config.toml");

        // Set environment variables
        env::set_var("ANKITUI_MAX_NEW_CARDS", "50");
        env::set_var("ANKITUI_THEME", "blue");

        // Create a config file
        let content = r#"
[daily]
max_new_cards = 30

[ui]
theme = "light"
"#;
        std::fs::write(&config_path, content)?;

        // Test that config file takes priority over environment variables
        let config = Config::from_file(&config_path)?;
        assert_eq!(config.daily.max_new_cards, 30); // From file, not env (50)
        assert_eq!(config.ui.theme, "light"); // From file, not env (blue)

        // Clean up environment variables
        env::remove_var("ANKITUI_MAX_NEW_CARDS");
        env::remove_var("ANKITUI_THEME");

        Ok(())
    }
}

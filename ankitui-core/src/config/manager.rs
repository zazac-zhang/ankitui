//! Configuration manager implementation

use anyhow::Result;
use std::path::{Path, PathBuf};

use super::config::Config;
use super::validator::ConfigValidator;

/// Configuration manager
#[derive(Clone)]
pub struct ConfigManager {
    pub config: Config,
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

        Ok(Self { config, config_path })
    }

    /// Create configuration manager with custom config path
    pub fn with_path<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_path = config_path.as_ref().to_path_buf();
        let config = Config::from_file(&config_path).unwrap_or_else(|_| {
            let default_config = Config::default();
            let _ = default_config.save_to_file(&config_path);
            default_config
        });

        Ok(Self { config, config_path })
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

        Ok(Self { config, config_path })
    }

    /// Create configuration manager without any config file (for testing)
    pub fn new_no_file() -> Result<Self> {
        let config_path = Config::default_path()?;
        let config = Config::default();

        Ok(Self { config, config_path })
    }

    /// Create configuration manager from a profile
    pub fn from_profile(profile_name: &str) -> Result<Self> {
        let config_path = Config::default_path()?;
        let config = super::profiles::helpers::config_from_profile(profile_name)?;

        // Save profile as default config
        config.save_to_file(&config_path)?;

        Ok(Self { config, config_path })
    }

    /// Initialize configuration with profile if no config exists
    pub fn init_with_profile(profile_name: &str) -> Result<Self> {
        let config_path = Config::default_path()?;

        let config = if config_path.exists() {
            Config::from_file(&config_path)?
        } else {
            let profile_config = super::profiles::helpers::config_from_profile(profile_name)?;
            profile_config.save_to_file(&config_path)?;
            profile_config
        };

        Ok(Self { config, config_path })
    }

    /// Get available configuration profiles
    pub fn available_profiles() -> Vec<(String, String)> {
        super::profiles::helpers::list_config_profiles()
    }

    /// Get current configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Get scheduler configuration
    pub fn get_scheduler_config(&self) -> &crate::config::scheduler::SchedulerConfig {
        &self.config.scheduler
    }

    /// Get UI configuration
    pub fn get_ui_config(&self) -> &crate::config::ui::UiConfig {
        &self.config.ui
    }

    /// Get shortcut configuration
    pub fn get_shortcut_config(&self) -> &crate::config::shortcuts::ShortcutConfig {
        &self.config.shortcuts
    }

    /// Get data configuration
    pub fn get_data_config(&self) -> &crate::config::data::DataConfig {
        &self.config.data
    }

    /// Get daily configuration
    pub fn get_daily_config(&self) -> &crate::config::daily::DailyConfig {
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

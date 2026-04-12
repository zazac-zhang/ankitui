//! Configuration profiles for different user types

use super::config::Config;
use super::validator::ConfigValidator;
use serde::{Deserialize, Serialize};

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
        description: "Advanced configuration for experienced users with high volume learning".to_string(),
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
    use anyhow::Result;

    /// Get default configuration
    pub fn default_config() -> Config {
        Config::default()
    }

    /// Validate configuration values
    pub fn validate_config(config: &Config) -> anyhow::Result<()> {
        ConfigValidator::validate(config)
    }

    /// Get configuration file path
    pub fn get_config_path() -> super::super::ConfigResult<std::path::PathBuf> {
        Config::default_path()
    }

    /// Create configuration from profile
    pub fn config_from_profile(profile_name: &str) -> Result<Config> {
        let profile =
            get_config_profile(profile_name).ok_or_else(|| anyhow::anyhow!("Unknown profile: {}", profile_name))?;
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

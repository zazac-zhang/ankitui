//! Configuration validation logic

use super::config::Config;
use super::ConfigResult;

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(config: &Config) -> ConfigResult<()> {
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

/// Validate configuration values (convenience function)
pub fn validate(config: &Config) -> ConfigResult<()> {
    ConfigValidator::validate(config)
}
//! Configuration provider trait and related implementations

use super::manager::ConfigManager;

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
            "scheduler.starting_ease_factor" => Some(self.config.scheduler.starting_ease_factor.to_string()),
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

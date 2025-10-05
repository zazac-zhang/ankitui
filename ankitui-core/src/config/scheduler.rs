//! Scheduler Configuration Module
//!
//! Contains SM-2 spaced repetition algorithm parameters and settings

use serde::{Deserialize, Serialize};

/// Scheduler configuration (SM-2 algorithm parameters)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchedulerConfig {
    /// Starting ease factor (2.5 by default)
    pub starting_ease_factor: f32,

    /// Minimum ease factor (1.3 by default)
    pub min_ease_factor: f32,

    /// Maximum ease factor (5.0 by default)
    pub max_ease_factor: f32,

    /// Easy interval (4 days by default)
    pub easy_interval: i32,

    /// Good interval starting point (1 day by default)
    pub good_interval: i32,

    /// Graduating interval (1 day by default)
    pub graduating_interval: i32,

    /// Initial failure interval (1 minute by default)
    pub initial_failure_interval: i32,

    /// Maximum review interval (36500 days by default, ~100 years)
    pub max_interval: i32,

    /// Hard interval multiplier (1.2 by default)
    pub hard_multiplier: f32,

    /// Easy interval bonus (1.3 by default)
    pub easy_bonus: f32,

    /// Interval modifier for all reviews (1.0 by default)
    pub interval_modifier: f32,

    /// Graduation interval for easy cards (4 days by default)
    pub easy_interval4: i32,

    /// Interval factor for easy cards (1.3 by default)
    pub interval_factor: f32,

    /// Learning step intervals in minutes
    pub learning_steps: Vec<i32>,

    /// Relearning step intervals in minutes
    pub relearning_steps: Vec<i32>,

    /// Graduating interval (in days)
    pub graduating_interval_days: i32,

    /// Easy interval bonus (in days)
    pub easy_interval_days: i32,

    /// Minimum interval for reviews
    pub min_review_interval: i32,

    /// Maximum interval for reviews
    pub max_review_interval: i32,

    /// Ease factor for "Again" rating
    pub again_factor: f32,

    /// Ease factor for "Hard" rating
    pub hard_factor: f32,

    /// Ease factor for "Good" rating
    pub good_factor: f32,

    /// Ease factor for "Easy" rating
    pub easy_factor: f32,

    /// Maximum lapse count before burying card
    pub max_lapses: i32,

    /// Interval factor after lapse
    pub lapse_interval_factor: f32,

    /// Minimum ease factor after lapse
    pub min_lapse_ease: f32,

    /// Enable fuzzy intervals
    pub fuzzy_intervals: bool,

    /// Fuzzy interval variance percentage (0-100)
    pub fuzzy_variance: u8,

    /// Enable load balancing for due cards
    pub load_balance: bool,

    /// Load balance window size in days
    pub load_balance_window: u8,

    /// Enable graduating delay
    pub graduating_delay: bool,

    /// Graduating delay in days
    pub graduating_delay_days: i32,

    /// Custom intervals for specific ratings
    pub custom_intervals: CustomIntervals,

    /// Advanced SM-2 settings
    pub advanced: AdvancedSchedulerSettings,
}

/// Custom intervals for specific ratings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomIntervals {
    /// Custom interval for "Again" rating (in minutes)
    pub again_interval: Option<i32>,

    /// Custom interval for "Hard" rating (in days)
    pub hard_interval: Option<i32>,

    /// Custom interval for "Good" rating (in days)
    pub good_interval: Option<i32>,

    /// Custom interval for "Easy" rating (in days)
    pub easy_interval: Option<i32>,

    /// Custom intervals for graduated cards
    pub graduated_intervals: GraduatedIntervals,
}

/// Custom intervals for graduated cards
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraduatedIntervals {
    /// Minimum interval for newly graduated cards
    pub min_interval: Option<i32>,

    /// Maximum interval for newly graduated cards
    pub max_interval: Option<i32>,

    /// Interval growth factor for graduated cards
    pub growth_factor: Option<f32>,
}

/// Advanced SM-2 settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedSchedulerSettings {
    /// Enable SM-2+ algorithm (with improved calculations)
    pub sm2_plus: bool,

    /// Memory retention threshold (0.0-1.0)
    pub retention_threshold: f32,

    /// Difficulty factor scaling
    pub difficulty_scale: f32,

    /// Enable adaptive intervals
    pub adaptive_intervals: bool,

    /// Adaptive interval sensitivity
    pub adaptive_sensitivity: f32,

    /// Enable time-based decay
    pub time_decay: bool,

    /// Time decay factor
    pub time_decay_factor: f32,

    /// Enable context-aware scheduling
    pub context_aware: bool,

    /// Context factors
    pub context_factors: ContextFactors,

    /// Enable performance-based adjustments
    pub performance_adjustments: bool,

    /// Performance tracking settings
    pub performance_tracking: PerformanceTracking,
}

/// Context factors for scheduling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextFactors {
    /// Consider time of day
    pub time_of_day: bool,

    /// Consider day of week
    pub day_of_week: bool,

    /// Consider recent performance
    pub recent_performance: bool,

    /// Consider card difficulty
    pub card_difficulty: bool,

    /// Consider deck performance
    pub deck_performance: bool,
}

/// Performance tracking settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTracking {
    /// Track response time
    pub track_response_time: bool,

    /// Track consecutive correct answers
    pub track_consecutive: bool,

    /// Track recent performance window
    pub recent_window: i32,

    /// Performance weight in interval calculation
    pub performance_weight: f32,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            starting_ease_factor: 2.5,
            min_ease_factor: 1.3,
            max_ease_factor: 5.0,
            easy_interval: 4,
            good_interval: 1,
            graduating_interval: 1,
            initial_failure_interval: 1,
            max_interval: 36500,
            hard_multiplier: 1.2,
            easy_bonus: 1.3,
            interval_modifier: 1.0,
            easy_interval4: 4,
            interval_factor: 1.3,
            learning_steps: vec![1, 6, 10, 14], // 1min, 6min, 10min, 14min
            relearning_steps: vec![10],         // 10min
            graduating_interval_days: 1,
            easy_interval_days: 4,
            min_review_interval: 1,
            max_review_interval: 36500,
            again_factor: 0.0,
            hard_factor: 1.2,
            good_factor: 1.3,
            easy_factor: 2.5,
            max_lapses: 8,
            lapse_interval_factor: 0.5,
            min_lapse_ease: 1.3,
            fuzzy_intervals: false,
            fuzzy_variance: 10,
            load_balance: true,
            load_balance_window: 3,
            graduating_delay: false,
            graduating_delay_days: 1,
            custom_intervals: CustomIntervals::default(),
            advanced: AdvancedSchedulerSettings::default(),
        }
    }
}

impl Default for CustomIntervals {
    fn default() -> Self {
        Self {
            again_interval: None,
            hard_interval: None,
            good_interval: None,
            easy_interval: None,
            graduated_intervals: GraduatedIntervals::default(),
        }
    }
}

impl Default for GraduatedIntervals {
    fn default() -> Self {
        Self {
            min_interval: None,
            max_interval: None,
            growth_factor: None,
        }
    }
}

impl Default for AdvancedSchedulerSettings {
    fn default() -> Self {
        Self {
            sm2_plus: false,
            retention_threshold: 0.9,
            difficulty_scale: 1.0,
            adaptive_intervals: false,
            adaptive_sensitivity: 0.1,
            time_decay: false,
            time_decay_factor: 0.95,
            context_aware: false,
            context_factors: ContextFactors::default(),
            performance_adjustments: false,
            performance_tracking: PerformanceTracking::default(),
        }
    }
}

impl Default for ContextFactors {
    fn default() -> Self {
        Self {
            time_of_day: false,
            day_of_week: false,
            recent_performance: true,
            card_difficulty: true,
            deck_performance: false,
        }
    }
}

impl Default for PerformanceTracking {
    fn default() -> Self {
        Self {
            track_response_time: false,
            track_consecutive: true,
            recent_window: 10,
            performance_weight: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_config_default() {
        let config = SchedulerConfig::default();
        assert_eq!(config.starting_ease_factor, 2.5);
        assert_eq!(config.min_ease_factor, 1.3);
        assert_eq!(config.max_ease_factor, 5.0);
        assert_eq!(config.learning_steps, vec![1, 6, 10, 14]);
        assert_eq!(config.relearning_steps, vec![10]);
    }

    #[test]
    fn test_custom_intervals() {
        let custom_intervals = CustomIntervals {
            again_interval: Some(5),
            good_interval: Some(3),
            graduated_intervals: GraduatedIntervals {
                min_interval: Some(2),
                growth_factor: Some(1.5),
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(custom_intervals.again_interval, Some(5));
        assert_eq!(custom_intervals.good_interval, Some(3));
        assert_eq!(custom_intervals.graduated_intervals.min_interval, Some(2));
    }

    #[test]
    fn test_advanced_scheduler_settings() {
        let advanced = AdvancedSchedulerSettings {
            sm2_plus: true,
            adaptive_intervals: true,
            context_aware: true,
            ..Default::default()
        };

        assert!(advanced.sm2_plus);
        assert!(advanced.adaptive_intervals);
        assert!(advanced.context_aware);
    }

    #[test]
    fn test_fuzzy_intervals() {
        let mut config = SchedulerConfig::default();

        // Enable fuzzy intervals
        config.fuzzy_intervals = true;
        config.fuzzy_variance = 20;

        assert!(config.fuzzy_intervals);
        assert_eq!(config.fuzzy_variance, 20);
    }

    #[test]
    fn test_load_balancing() {
        let config = SchedulerConfig::default();

        assert!(config.load_balance);
        assert_eq!(config.load_balance_window, 3);
    }
}

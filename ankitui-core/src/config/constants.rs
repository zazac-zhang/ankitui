//! Configuration constants and default values

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
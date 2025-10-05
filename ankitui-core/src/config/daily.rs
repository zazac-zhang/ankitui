//! Daily Configuration Module
//!
//! Contains daily limits, goals, reminders, and schedule settings

use serde::{Deserialize, Serialize};

/// Daily limits and preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyConfig {
    /// Maximum new cards per day
    pub max_new_cards: i32,

    /// Maximum review cards per day
    pub max_review_cards: i32,

    /// Start of day (hour in 24h format)
    pub day_start_hour: u8,

    /// End of day (hour in 24h format)
    pub day_end_hour: u8,

    /// Show daily limit warnings
    pub show_limit_warnings: bool,

    /// Study reminder settings
    pub reminder: ReminderConfig,

    /// Study goals
    pub goals: StudyGoals,

    /// Daily schedule settings
    pub schedule: DailySchedule,

    /// Learning statistics
    pub statistics: DailyStatistics,

    /// Notifications
    pub notifications: NotificationConfig,

    /// Break settings
    pub breaks: BreakConfig,

    /// Motivation settings
    pub motivation: MotivationConfig,
}

/// Reminder configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReminderConfig {
    /// Enable daily reminders
    pub enabled: bool,

    /// Reminder time (hour in 24h format)
    pub hour: u8,

    /// Reminder time (minute)
    pub minute: u8,

    /// Reminder message
    pub message: String,

    /// Multiple reminders
    pub multiple_reminders: Vec<TimeReminder>,

    /// Smart reminders
    pub smart_reminders: SmartReminderConfig,

    /// Reminder sound
    pub sound_enabled: bool,

    /// Sound file path
    pub sound_file: Option<String>,

    /// Snooze settings
    pub snooze: SnoozeConfig,

    /// Reminder frequency
    pub frequency: ReminderFrequency,

    /// Reminder channels
    pub channels: Vec<String>,
}

/// Time-based reminder
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeReminder {
    /// Reminder name
    pub name: String,

    /// Hour in 24h format
    pub hour: u8,

    /// Minute
    pub minute: u8,

    /// Days of week
    pub days: Vec<String>,

    /// Reminder message
    pub message: String,

    /// Enabled status
    pub enabled: bool,

    /// Priority
    pub priority: u8,
}

/// Smart reminder configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmartReminderConfig {
    /// Enable smart reminders
    pub enabled: bool,

    /// Minimum cards due for reminder
    pub min_cards_due: i32,

    /// Maximum study streak days
    pub max_streak_days: u32,

    /// Adaptive timing
    pub adaptive_timing: bool,

    /// Learning pattern analysis
    pub pattern_analysis: bool,

    /// Productivity tracking
    pub productivity_tracking: bool,

    /// Context awareness
    pub context_aware: bool,

    /// Machine learning based timing
    pub ml_timing: bool,

    /// Weather-based adjustments
    pub weather_adjustments: bool,

    /// Work schedule integration
    pub work_schedule: bool,
}

/// Snooze configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnoozeConfig {
    /// Enable snooze
    pub enabled: bool,

    /// Snooze duration in minutes
    pub duration: u16,

    /// Maximum snoozes
    pub max_snoozes: u8,

    /// Incremental snooze duration
    pub incremental: bool,

    /// Snooze options
    pub options: Vec<u16>,

    /// Auto-dismiss after max snoozes
    pub auto_dismiss: bool,
}

/// Reminder frequency
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReminderFrequency {
    /// Daily reminders
    Daily,
    /// Weekdays only
    Weekdays,
    /// Weekends only
    Weekends,
    /// Custom days
    Custom(Vec<String>),
    /// Every N days
    Interval(u8),
    /// Specific schedule
    Schedule(Vec<TimeReminder>),
}

/// Study goals configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StudyGoals {
    /// Daily study goal in minutes
    pub daily_minutes: u32,

    /// Weekly study goal in minutes
    pub weekly_minutes: u32,

    /// Monthly study goal in minutes
    pub monthly_minutes: u32,

    /// Goal streak tracking
    pub track_streaks: bool,

    /// Card count goals
    pub card_goals: CardGoals,

    /// Performance goals
    pub performance_goals: PerformanceGoals,

    /// Time-based goals
    pub time_goals: TimeGoals,

    /// Progress tracking
    pub progress_tracking: ProgressTracking,

    /// Goal rewards
    pub rewards: RewardConfig,

    /// Goal difficulty settings
    pub difficulty: GoalDifficulty,
}

/// Card count goals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardGoals {
    /// Daily new cards goal
    pub daily_new_cards: u32,

    /// Daily review cards goal
    pub daily_review_cards: u32,

    /// Daily total cards goal
    pub daily_total_cards: u32,

    /// Weekly cards goal
    pub weekly_cards: u32,

    /// Monthly cards goal
    pub monthly_cards: u32,

    /// Accuracy goals
    pub accuracy_goal: f32,

    /// Speed goals (cards per minute)
    pub speed_goal: f32,
}

/// Performance goals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceGoals {
    /// Target retention rate (0.0-1.0)
    pub retention_rate: f32,

    /// Target accuracy percentage
    pub accuracy_percentage: f32,

    /// Target ease factor
    pub target_ease_factor: f32,

    /// Maximum lapse rate
    pub max_lapse_rate: f32,

    /// Target mature card percentage
    pub mature_card_percentage: f32,

    /// Learning efficiency goal
    pub learning_efficiency: f32,

    /// Consistency goal
    pub consistency_goal: f32,

    /// Improvement goals
    pub improvement_goals: ImprovementGoals,
}

/// Improvement goals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImprovementGoals {
    /// Weekly improvement target
    pub weekly_improvement: f32,

    /// Monthly improvement target
    pub monthly_improvement: f32,

    /// Performance tracking
    pub track_performance: bool,

    /// Benchmark against self
    pub self_benchmark: bool,

    /// Benchmark against others
    pub peer_benchmark: bool,
}

/// Time-based goals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeGoals {
    /// Daily study time goal in minutes
    pub daily_time: u32,

    /// Weekly study time goal in minutes
    pub weekly_time: u32,

    /// Monthly study time goal in minutes
    pub monthly_time: u32,

    /// Maximum study session length
    pub max_session_length: u32,

    /// Minimum session length
    pub min_session_length: u32,

    /// Preferred session times
    pub preferred_times: Vec<TimeRange>,

    /// Time tracking granularity
    pub tracking_granularity: u8,
}

/// Time range
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start hour
    pub start_hour: u8,

    /// Start minute
    pub start_minute: u8,

    /// End hour
    pub end_hour: u8,

    /// End minute
    pub end_minute: u8,

    /// Days of week
    pub days: Vec<String>,

    /// Priority level
    pub priority: u8,
}

/// Progress tracking configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressTracking {
    /// Enable progress tracking
    pub enabled: bool,

    /// Track detailed statistics
    pub detailed_stats: bool,

    /// Track learning curves
    pub learning_curves: bool,

    /// Track forgetting curves
    pub forgetting_curves: bool,

    /// Track session performance
    pub session_performance: bool,

    /// Track long-term trends
    pub long_term_trends: bool,

    /// Track mood/energy levels
    pub mood_tracking: bool,

    /// Track study environment
    pub environment_tracking: bool,

    /// Export progress data
    pub export_data: bool,

    /// Progress report frequency
    pub report_frequency: String,
}

/// Reward configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RewardConfig {
    /// Enable rewards
    pub enabled: bool,

    /// Daily rewards
    pub daily_rewards: Vec<Reward>,

    /// Weekly rewards
    pub weekly_rewards: Vec<Reward>,

    /// Monthly rewards
    pub monthly_rewards: Vec<Reward>,

    /// Milestone rewards
    pub milestone_rewards: Vec<Reward>,

    /// Reward system type
    pub reward_system: String,

    /// Reward notifications
    pub reward_notifications: bool,
}

/// Reward structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reward {
    /// Reward name
    pub name: String,

    /// Reward description
    pub description: String,

    /// Reward type
    pub reward_type: String,

    /// Reward value
    pub value: String,

    /// Icon or image
    pub icon: Option<String>,

    /// Rarity level
    pub rarity: String,

    /// Unlock condition
    pub unlock_condition: String,
}

/// Goal difficulty settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalDifficulty {
    /// Difficulty level
    pub level: String,

    /// Auto-adjust difficulty
    pub auto_adjust: bool,

    /// Adjustment frequency
    pub adjustment_frequency: String,

    /// Performance threshold for adjustment
    pub performance_threshold: f32,

    /// Maximum difficulty level
    pub max_difficulty: u8,

    /// Minimum difficulty level
    pub min_difficulty: u8,
}

/// Daily schedule configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailySchedule {
    /// Enable schedule
    pub enabled: bool,

    /// Study sessions
    pub study_sessions: Vec<StudySession>,

    /// Break periods
    pub break_periods: Vec<BreakPeriod>,

    /// Flexibility settings
    pub flexibility: FlexibilitySettings,

    /// Time zone settings
    pub timezone: TimezoneSettings,

    /// Holiday settings
    pub holidays: HolidaySettings,

    /// Work/school integration
    pub work_integration: WorkIntegration,
}

/// Study session configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StudySession {
    /// Session name
    pub name: String,

    /// Start time
    pub start_time: String,

    /// End time
    pub end_time: String,

    /// Days of week
    pub days: Vec<String>,

    /// Session type
    pub session_type: String,

    /// Target cards for session
    pub target_cards: u32,

    /// Session priority
    pub priority: u8,

    /// Enabled status
    pub enabled: bool,

    /// Session requirements
    pub requirements: Vec<String>,

    /// Session rewards
    pub rewards: Vec<String>,
}

/// Break period configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakPeriod {
    /// Break name
    pub name: String,

    /// Start time
    pub start_time: String,

    /// End time
    pub end_time: String,

    /// Break type
    pub break_type: String,

    /// Duration in minutes
    pub duration: u16,

    /// Activities during break
    pub activities: Vec<String>,

    /// Mandatory status
    pub mandatory: bool,

    /// Days of week
    pub days: Vec<String>,
}

/// Flexibility settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexibilitySettings {
    /// Allow session time adjustments
    pub allow_time_adjustments: bool,

    /// Maximum adjustment in minutes
    pub max_adjustment: u16,

    /// Allow session skipping
    pub allow_skipping: bool,

    /// Automatic rescheduling
    pub auto_reschedule: bool,

    /// Rescheduling preferences
    pub rescheduling_preferences: Vec<String>,

    /// Buffer time between sessions
    pub buffer_time: u8,

    /// Minimum session notice
    pub min_session_notice: u16,
}

/// Time zone settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimezoneSettings {
    /// Time zone identifier
    pub timezone: String,

    /// Auto-detect timezone
    pub auto_detect: bool,

    /// Daylight saving time handling
    pub dst_handling: String,

    /// Travel mode
    pub travel_mode: bool,

    /// Automatic adjustment for travel
    pub auto_travel_adjustment: bool,
}

/// Holiday settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HolidaySettings {
    /// Automatically detect holidays
    pub auto_detect: bool,

    /// Holiday calendar
    pub holiday_calendar: Vec<Holiday>,

    /// Study on holidays
    pub study_on_holidays: bool,

    /// Reduced goals on holidays
    pub reduced_goals_holidays: bool,

    /// Holiday goal reduction percentage
    pub holiday_reduction_percentage: u8,

    /// Custom holiday rules
    pub custom_rules: Vec<HolidayRule>,
}

/// Holiday structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Holiday {
    /// Holiday name
    pub name: String,

    /// Holiday date
    pub date: String,

    /// Holiday type
    pub holiday_type: String,

    /// Study allowed
    pub study_allowed: bool,

    /// Goal adjustment
    pub goal_adjustment: f32,

    /// Notes
    pub notes: Option<String>,
}

/// Holiday rule
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HolidayRule {
    /// Rule name
    pub name: String,

    /// Rule condition
    pub condition: String,

    /// Rule action
    pub action: String,

    /// Rule parameters
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Work integration settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkIntegration {
    /// Enable work integration
    pub enabled: bool,

    /// Work calendar integration
    pub calendar_integration: bool,

    /// Work hours
    pub work_hours: Vec<TimeRange>,

    /// Commute time
    pub commute_time: u16,

    /// Study during commute
    pub study_during_commute: bool,

    /// Study at work
    pub study_at_work: bool,

    /// Work stress adaptation
    pub stress_adaptation: bool,

    /// Productivity tracking
    pub productivity_tracking: bool,
}

/// Daily statistics configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyStatistics {
    /// Enable daily statistics
    pub enabled: bool,

    /// Track detailed metrics
    pub detailed_metrics: bool,

    /// Track session performance
    pub session_performance: bool,

    /// Track card performance
    pub card_performance: bool,

    /// Track time distribution
    pub time_distribution: bool,

    /// Track accuracy trends
    pub accuracy_trends: bool,

    /// Export statistics
    pub export_enabled: bool,

    /// Export format
    pub export_format: String,

    /// Export frequency
    pub export_frequency: String,

    /// Data retention days
    pub data_retention_days: u32,
}

/// Notification configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Enable notifications
    pub enabled: bool,

    /// Notification channels
    pub channels: Vec<NotificationChannel>,

    /// Notification priorities
    pub priorities: NotificationPriorities,

    /// Quiet hours
    pub quiet_hours: QuietHours,

    /// Notification content
    pub content: NotificationContent,

    /// Notification frequency limits
    pub frequency_limits: FrequencyLimits,
}

/// Notification channel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel type
    pub channel_type: String,

    /// Channel enabled
    pub enabled: bool,

    /// Channel configuration
    pub configuration: std::collections::HashMap<String, serde_json::Value>,

    /// Notification types for this channel
    pub notification_types: Vec<String>,

    /// Channel priority
    pub priority: u8,
}

/// Notification priorities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationPriorities {
    /// High priority notifications
    pub high: Vec<String>,

    /// Medium priority notifications
    pub medium: Vec<String>,

    /// Low priority notifications
    pub low: Vec<String>,

    /// Priority escalation rules
    pub escalation_rules: Vec<EscalationRule>,
}

/// Escalation rule
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationRule {
    /// Rule name
    pub name: String,

    /// Trigger condition
    pub trigger: String,

    /// Escalation action
    pub action: String,

    /// Escalation delay in minutes
    pub delay: u16,

    /// Maximum escalations
    pub max_escalations: u8,
}

/// Quiet hours configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuietHours {
    /// Enable quiet hours
    pub enabled: bool,

    /// Start time
    pub start_time: String,

    /// End time
    pub end_time: String,

    /// Days of week
    pub days: Vec<String>,

    /// Emergency exceptions
    pub emergency_exceptions: Vec<String>,

    /// Timezone handling
    pub timezone_handling: String,
}

/// Notification content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationContent {
    /// Include statistics
    pub include_stats: bool,

    /// Include motivational messages
    pub include_motivation: bool,

    /// Include tips
    pub include_tips: bool,

    /// Include progress indicators
    pub include_progress: bool,

    /// Custom message templates
    pub message_templates: std::collections::HashMap<String, String>,

    /// Emoji usage
    pub emoji_usage: bool,

    /// Personalization
    pub personalization: bool,
}

/// Frequency limits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrequencyLimits {
    /// Maximum notifications per hour
    pub max_per_hour: u8,

    /// Maximum notifications per day
    pub max_per_day: u16,

    /// Minimum interval between notifications
    pub min_interval_minutes: u16,

    /// Batching enabled
    pub batching_enabled: bool,

    /// Batch size
    pub batch_size: u8,

    /// Batch interval in minutes
    pub batch_interval: u16,
}

/// Break configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakConfig {
    /// Enable breaks
    pub enabled: bool,

    /// Break strategy
    pub strategy: String,

    /// Break intervals
    pub intervals: Vec<BreakInterval>,

    /// Break activities
    pub activities: Vec<BreakActivity>,

    /// Break reminders
    pub reminders: BreakReminders,

    /// Break analytics
    pub analytics: BreakAnalytics,
}

/// Break interval
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakInterval {
    /// Interval name
    pub name: String,

    /// Study duration before break
    pub study_duration: u16,

    /// Break duration
    pub break_duration: u16,

    /// Maximum consecutive sessions
    pub max_consecutive: u8,

    /// Interval type
    pub interval_type: String,

    /// Conditions
    pub conditions: Vec<String>,

    /// Priority
    pub priority: u8,
}

/// Break activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakActivity {
    /// Activity name
    pub name: String,

    /// Activity type
    pub activity_type: String,

    /// Duration in minutes
    pub duration: u16,

    /// Description
    pub description: String,

    /// Equipment needed
    pub equipment: Vec<String>,

    /// Difficulty level
    pub difficulty: String,

    /// Benefits
    pub benefits: Vec<String>,
}

/// Break reminders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakReminders {
    /// Enable reminders
    pub enabled: bool,

    /// Reminder timing
    pub timing: u16, // minutes before break

    /// Reminder message
    pub message: String,

    /// Snooze options
    pub snooze_options: Vec<u16>,

    /// Sound enabled
    pub sound_enabled: bool,

    /// Visual indicators
    pub visual_indicators: bool,
}

/// Break analytics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakAnalytics {
    /// Track break effectiveness
    pub track_effectiveness: bool,

    /// Track break adherence
    pub track_adherence: bool,

    /// Track post-break performance
    pub track_post_performance: bool,

    /// Analyze optimal break times
    pub analyze_optimal_times: bool,

    /// Personalized recommendations
    pub personalized_recommendations: bool,
}

/// Motivation configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotivationConfig {
    /// Enable motivation system
    pub enabled: bool,

    /// Motivation sources
    pub sources: Vec<MotivationSource>,

    /// Message frequency
    pub message_frequency: String,

    /// Message timing
    pub message_timing: Vec<String>,

    /// Personalization
    pub personalization: MotivationPersonalization,

    /// Motivation analytics
    pub analytics: MotivationAnalytics,
}

/// Motivation source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotivationSource {
    /// Source name
    pub name: String,

    /// Source type
    pub source_type: String,

    /// Messages
    pub messages: Vec<MotivationMessage>,

    /// Enabled status
    pub enabled: bool,

    /// Weight in selection
    pub weight: f32,

    /// Conditions
    pub conditions: Vec<String>,
}

/// Motivation message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotivationMessage {
    /// Message text
    pub text: String,

    /// Message category
    pub category: String,

    /// Message tone
    pub tone: String,

    /// Context requirements
    pub context_requirements: Vec<String>,

    /// Usage count
    pub usage_count: u32,

    /// Effectiveness rating
    pub effectiveness_rating: f32,
}

/// Motivation personalization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotivationPersonalization {
    /// Learn user preferences
    pub learn_preferences: bool,

    /// Adapt message content
    pub adapt_content: bool,

    /// Consider learning style
    pub learning_style: bool,

    /// Consider personality
    pub personality: bool,

    /// Consider current mood
    pub current_mood: bool,

    /// Consider performance history
    pub performance_history: bool,
}

/// Motivation analytics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotivationAnalytics {
    /// Track message effectiveness
    pub track_effectiveness: bool,

    /// Track engagement
    pub track_engagement: bool,

    /// Track performance impact
    pub track_performance_impact: bool,

    /// A/B testing
    pub ab_testing: bool,

    /// Feedback collection
    pub feedback_collection: bool,
}

impl Default for DailyConfig {
    fn default() -> Self {
        Self {
            max_new_cards: 20,
            max_review_cards: 100,
            day_start_hour: 0,
            day_end_hour: 23,
            show_limit_warnings: true,
            reminder: ReminderConfig::default(),
            goals: StudyGoals::default(),
            schedule: DailySchedule::default(),
            statistics: DailyStatistics::default(),
            notifications: NotificationConfig::default(),
            breaks: BreakConfig::default(),
            motivation: MotivationConfig::default(),
        }
    }
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            hour: 9,
            minute: 0,
            message: "Time to review your cards!".to_string(),
            multiple_reminders: Vec::new(),
            smart_reminders: SmartReminderConfig::default(),
            sound_enabled: false,
            sound_file: None,
            snooze: SnoozeConfig::default(),
            frequency: ReminderFrequency::Daily,
            channels: vec!["app".to_string()],
        }
    }
}

impl Default for SmartReminderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_cards_due: 5,
            max_streak_days: 30,
            adaptive_timing: false,
            pattern_analysis: false,
            productivity_tracking: false,
            context_aware: false,
            ml_timing: false,
            weather_adjustments: false,
            work_schedule: false,
        }
    }
}

impl Default for SnoozeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 10,
            max_snoozes: 3,
            incremental: false,
            options: vec![5, 10, 15, 30],
            auto_dismiss: false,
        }
    }
}

impl Default for StudyGoals {
    fn default() -> Self {
        Self {
            daily_minutes: 15,
            weekly_minutes: 105,  // 15 minutes * 7 days
            monthly_minutes: 450, // 15 minutes * 30 days
            track_streaks: true,
            card_goals: CardGoals::default(),
            performance_goals: PerformanceGoals::default(),
            time_goals: TimeGoals::default(),
            progress_tracking: ProgressTracking::default(),
            rewards: RewardConfig::default(),
            difficulty: GoalDifficulty::default(),
        }
    }
}

impl Default for CardGoals {
    fn default() -> Self {
        Self {
            daily_new_cards: 20,
            daily_review_cards: 100,
            daily_total_cards: 120,
            weekly_cards: 840,   // 120 * 7
            monthly_cards: 3600, // 120 * 30
            accuracy_goal: 0.85,
            speed_goal: 10.0, // cards per minute
        }
    }
}

impl Default for PerformanceGoals {
    fn default() -> Self {
        Self {
            retention_rate: 0.85,
            accuracy_percentage: 85.0,
            target_ease_factor: 2.5,
            max_lapse_rate: 0.15,
            mature_card_percentage: 0.70,
            learning_efficiency: 0.80,
            consistency_goal: 0.90,
            improvement_goals: ImprovementGoals::default(),
        }
    }
}

impl Default for ImprovementGoals {
    fn default() -> Self {
        Self {
            weekly_improvement: 0.02,  // 2%
            monthly_improvement: 0.08, // 8%
            track_performance: true,
            self_benchmark: true,
            peer_benchmark: false,
        }
    }
}

impl Default for TimeGoals {
    fn default() -> Self {
        Self {
            daily_time: 15,
            weekly_time: 105,
            monthly_time: 450,
            max_session_length: 60,
            min_session_length: 5,
            preferred_times: Vec::new(),
            tracking_granularity: 1, // 1 minute
        }
    }
}

impl Default for ProgressTracking {
    fn default() -> Self {
        Self {
            enabled: true,
            detailed_stats: true,
            learning_curves: true,
            forgetting_curves: true,
            session_performance: true,
            long_term_trends: true,
            mood_tracking: false,
            environment_tracking: false,
            export_data: false,
            report_frequency: "weekly".to_string(),
        }
    }
}

impl Default for RewardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            daily_rewards: Vec::new(),
            weekly_rewards: Vec::new(),
            monthly_rewards: Vec::new(),
            milestone_rewards: Vec::new(),
            reward_system: "points".to_string(),
            reward_notifications: true,
        }
    }
}

impl Default for GoalDifficulty {
    fn default() -> Self {
        Self {
            level: "moderate".to_string(),
            auto_adjust: true,
            adjustment_frequency: "weekly".to_string(),
            performance_threshold: 0.75,
            max_difficulty: 5,
            min_difficulty: 1,
        }
    }
}

impl Default for DailySchedule {
    fn default() -> Self {
        Self {
            enabled: false,
            study_sessions: Vec::new(),
            break_periods: Vec::new(),
            flexibility: FlexibilitySettings::default(),
            timezone: TimezoneSettings::default(),
            holidays: HolidaySettings::default(),
            work_integration: WorkIntegration::default(),
        }
    }
}

impl Default for FlexibilitySettings {
    fn default() -> Self {
        Self {
            allow_time_adjustments: true,
            max_adjustment: 30, // 30 minutes
            allow_skipping: true,
            auto_reschedule: true,
            rescheduling_preferences: vec!["later_today".to_string(), "tomorrow".to_string()],
            buffer_time: 5,         // 5 minutes
            min_session_notice: 15, // 15 minutes
        }
    }
}

impl Default for TimezoneSettings {
    fn default() -> Self {
        Self {
            timezone: "UTC".to_string(),
            auto_detect: true,
            dst_handling: "automatic".to_string(),
            travel_mode: false,
            auto_travel_adjustment: false,
        }
    }
}

impl Default for HolidaySettings {
    fn default() -> Self {
        Self {
            auto_detect: false,
            holiday_calendar: Vec::new(),
            study_on_holidays: true,
            reduced_goals_holidays: true,
            holiday_reduction_percentage: 50,
            custom_rules: Vec::new(),
        }
    }
}

impl Default for WorkIntegration {
    fn default() -> Self {
        Self {
            enabled: false,
            calendar_integration: false,
            work_hours: Vec::new(),
            commute_time: 0,
            study_during_commute: false,
            study_at_work: false,
            stress_adaptation: false,
            productivity_tracking: false,
        }
    }
}

impl Default for DailyStatistics {
    fn default() -> Self {
        Self {
            enabled: true,
            detailed_metrics: true,
            session_performance: true,
            card_performance: true,
            time_distribution: true,
            accuracy_trends: true,
            export_enabled: false,
            export_format: "csv".to_string(),
            export_frequency: "monthly".to_string(),
            data_retention_days: 365,
        }
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            channels: Vec::new(),
            priorities: NotificationPriorities::default(),
            quiet_hours: QuietHours::default(),
            content: NotificationContent::default(),
            frequency_limits: FrequencyLimits::default(),
        }
    }
}

impl Default for NotificationPriorities {
    fn default() -> Self {
        Self {
            high: vec!["study_reminder".to_string(), "goal_achievement".to_string()],
            medium: vec!["daily_summary".to_string(), "weekly_report".to_string()],
            low: vec![
                "motivational_message".to_string(),
                "tip_of_the_day".to_string(),
            ],
            escalation_rules: Vec::new(),
        }
    }
}

impl Default for QuietHours {
    fn default() -> Self {
        Self {
            enabled: false,
            start_time: "22:00".to_string(),
            end_time: "08:00".to_string(),
            days: vec![
                "monday".to_string(),
                "tuesday".to_string(),
                "wednesday".to_string(),
                "thursday".to_string(),
                "friday".to_string(),
            ],
            emergency_exceptions: vec!["critical_error".to_string(), "data_loss".to_string()],
            timezone_handling: "local".to_string(),
        }
    }
}

impl Default for NotificationContent {
    fn default() -> Self {
        Self {
            include_stats: true,
            include_motivation: true,
            include_tips: false,
            include_progress: true,
            message_templates: std::collections::HashMap::new(),
            emoji_usage: true,
            personalization: true,
        }
    }
}

impl Default for FrequencyLimits {
    fn default() -> Self {
        Self {
            max_per_hour: 5,
            max_per_day: 20,
            min_interval_minutes: 15,
            batching_enabled: false,
            batch_size: 3,
            batch_interval: 30,
        }
    }
}

impl Default for BreakConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: "pomodoro".to_string(),
            intervals: vec![BreakInterval {
                name: "Pomodoro".to_string(),
                study_duration: 25,
                break_duration: 5,
                max_consecutive: 4,
                interval_type: "pomodoro".to_string(),
                conditions: Vec::new(),
                priority: 1,
            }],
            activities: Vec::new(),
            reminders: BreakReminders::default(),
            analytics: BreakAnalytics::default(),
        }
    }
}

impl Default for BreakReminders {
    fn default() -> Self {
        Self {
            enabled: true,
            timing: 1,
            message: "Time for a break!".to_string(),
            snooze_options: vec![5, 10],
            sound_enabled: true,
            visual_indicators: true,
        }
    }
}

impl Default for BreakAnalytics {
    fn default() -> Self {
        Self {
            track_effectiveness: false,
            track_adherence: true,
            track_post_performance: false,
            analyze_optimal_times: false,
            personalized_recommendations: false,
        }
    }
}

impl Default for MotivationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sources: Vec::new(),
            message_frequency: "daily".to_string(),
            message_timing: vec!["session_start".to_string(), "goal_achievement".to_string()],
            personalization: MotivationPersonalization::default(),
            analytics: MotivationAnalytics::default(),
        }
    }
}

impl Default for MotivationPersonalization {
    fn default() -> Self {
        Self {
            learn_preferences: true,
            adapt_content: true,
            learning_style: false,
            personality: false,
            current_mood: false,
            performance_history: true,
        }
    }
}

impl Default for MotivationAnalytics {
    fn default() -> Self {
        Self {
            track_effectiveness: false,
            track_engagement: false,
            track_performance_impact: false,
            ab_testing: false,
            feedback_collection: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_config_default() {
        let config = DailyConfig::default();
        assert_eq!(config.max_new_cards, 20);
        assert_eq!(config.max_review_cards, 100);
        assert_eq!(config.day_start_hour, 0);
        assert_eq!(config.day_end_hour, 23);
        assert!(config.show_limit_warnings);
    }

    #[test]
    fn test_reminder_config() {
        let reminder = ReminderConfig::default();
        assert!(!reminder.enabled);
        assert_eq!(reminder.hour, 9);
        assert_eq!(reminder.minute, 0);
        assert_eq!(reminder.message, "Time to review your cards!");
    }

    #[test]
    fn test_study_goals() {
        let goals = StudyGoals::default();
        assert_eq!(goals.daily_minutes, 15);
        assert_eq!(goals.weekly_minutes, 105);
        assert_eq!(goals.monthly_minutes, 450);
        assert!(goals.track_streaks);
    }

    #[test]
    fn test_card_goals() {
        let card_goals = CardGoals::default();
        assert_eq!(card_goals.daily_new_cards, 20);
        assert_eq!(card_goals.daily_review_cards, 100);
        assert_eq!(card_goals.accuracy_goal, 0.85);
        assert_eq!(card_goals.speed_goal, 10.0);
    }

    #[test]
    fn test_break_config() {
        let breaks = BreakConfig::default();
        assert!(!breaks.enabled);
        assert_eq!(breaks.strategy, "pomodoro");
        assert_eq!(breaks.intervals.len(), 1);
        assert_eq!(breaks.intervals[0].study_duration, 25);
        assert_eq!(breaks.intervals[0].break_duration, 5);
    }

    #[test]
    fn test_notification_config() {
        let notifications = NotificationConfig::default();
        assert!(!notifications.enabled);
        assert_eq!(notifications.quiet_hours.start_time, "22:00");
        assert_eq!(notifications.quiet_hours.end_time, "08:00");
    }

    #[test]
    fn test_motivation_config() {
        let motivation = MotivationConfig::default();
        assert!(motivation.enabled);
        assert_eq!(motivation.message_frequency, "daily");
        assert!(motivation.personalization.learn_preferences);
    }
}

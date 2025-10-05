//! TUI Visualization Components
//!
//! ASCII charts and visual representations for statistics and progress

use ankitui_core::{DeckStatistics, DifficultyDistribution};

/// Create ASCII bar chart for card distribution
pub fn create_card_distribution_chart(stats: &DeckStatistics) -> String {
    let total = stats.total_cards;
    if total == 0 {
        return "No cards to display".to_string();
    }

    let new_percent = (stats.new_cards as f32 / total as f32 * 20.0).round() as usize;
    let learning_percent = (stats.learning_cards as f32 / total as f32 * 20.0).round() as usize;
    let review_percent = (stats.review_cards as f32 / total as f32 * 20.0).round() as usize;
    let relearning_percent = (stats.relearning_cards as f32 / total as f32 * 20.0).round() as usize;

    format!(
        "Card Distribution\n\
         ┌───────────────────────────────────┐\n\
         │ New:       {} ({:>3.1}%) │\n\
         │ Learning:  {} ({:>3.1}%) │\n\
         │ Review:    {} ({:>3.1}%) │\n\
         │ Relearning:{} ({:>3.1}%) │\n\
         └───────────────────────────────────┘",
        "█".repeat(new_percent.max(1)),
        (stats.new_cards as f32 / total as f32 * 100.0),
        "█".repeat(learning_percent.max(1)),
        (stats.learning_cards as f32 / total as f32 * 100.0),
        "█".repeat(review_percent.max(1)),
        (stats.review_cards as f32 / total as f32 * 100.0),
        "█".repeat(relearning_percent.max(1)),
        (stats.relearning_cards as f32 / total as f32 * 100.0)
    )
}

/// Create ASCII progress gauge for retention rate
pub fn create_retention_gauge(retention_rate: f32) -> String {
    let percentage = retention_rate * 100.0;
    let filled_bars = (percentage / 5.0).round() as usize;
    let empty_bars = 20 - filled_bars;

    let color = if percentage >= 90.0 {
        "🟢"
    } else if percentage >= 75.0 {
        "🟡"
    } else {
        "🔴"
    };

    format!(
        "Memory Retention: {:.1}% {}\n\
         [{}{}]",
        percentage,
        color,
        "█".repeat(filled_bars),
        "░".repeat(empty_bars)
    )
}

/// Create difficulty distribution chart
pub fn create_difficulty_chart(distribution: &DifficultyDistribution) -> String {
    let total = distribution.very_easy
        + distribution.easy
        + distribution.normal
        + distribution.hard
        + distribution.very_hard;

    if total == 0 {
        return "No difficulty data available".to_string();
    }

    format!(
        "Difficulty Distribution\n\
         ┌───────────────────────────────────┐\n\
         │ Very Easy: {} ({:>3.1}%) │\n\
         │ Easy:      {} ({:>3.1}%) │\n\
         │ Normal:    {} ({:>3.1}%) │\n\
         │ Hard:      {} ({:>3.1}%) │\n\
         │ Very Hard: {} ({:>3.1}%) │\n\
         └───────────────────────────────────┘",
        "█".repeat(
            (((distribution.very_easy as f32 / total as f32) * 20.0).round() as usize).max(1)
        ),
        (distribution.very_easy as f32 / total as f32 * 100.0),
        "█".repeat((((distribution.easy as f32 / total as f32) * 20.0).round() as usize).max(1)),
        (distribution.easy as f32 / total as f32 * 100.0),
        "█".repeat((((distribution.normal as f32 / total as f32) * 20.0).round() as usize).max(1)),
        (distribution.normal as f32 / total as f32 * 100.0),
        "█".repeat((((distribution.hard as f32 / total as f32) * 20.0).round() as usize).max(1)),
        (distribution.hard as f32 / total as f32 * 100.0),
        "█".repeat(
            (((distribution.very_hard as f32 / total as f32) * 20.0).round() as usize).max(1)
        ),
        (distribution.very_hard as f32 / total as f32 * 100.0)
    )
}

/// Create study streak visualization
pub fn create_streak_chart(days: u32, longest: u32) -> String {
    if days == 0 {
        return "No study streak yet. Start learning today! 📚".to_string();
    }

    let current_streak_bars = (days as f32 / longest.max(days) as f32 * 10.0).round() as usize;
    let longest_streak_bars = 10;

    format!(
        "Study Streak\n\
         Current: {} days {}\n\
         [{}{}]\n\
         Longest: {} days\n\
         [{}]",
        days,
        if days >= 7 {
            "🔥"
        } else if days >= 3 {
            "⭐"
        } else {
            "📖"
        },
        "█".repeat(current_streak_bars),
        "░".repeat(10 - current_streak_bars),
        longest,
        "█".repeat(longest_streak_bars)
    )
}

/// Create learning progress timeline
pub fn create_progress_timeline(cards_today: u32, reviews_today: u32) -> String {
    let total_activity = cards_today + reviews_today;
    if total_activity == 0 {
        return "No activity today. Start a review session! 🎯".to_string();
    }

    let learning_bars = (cards_today as f32 / total_activity as f32 * 10.0).round() as usize;
    let review_bars = 10 - learning_bars;

    format!(
        "Today's Activity\n\
         ┌─────────────────┐\n\
         │ New Cards: {}    │\n\
         │ Reviews:   {}    │\n\
         └─────────────────┘\n\
         Learning: [{}{}]\n\
         Review:   [{}{}]",
        cards_today,
        reviews_today,
        "█".repeat(learning_bars),
        "░".repeat(10 - learning_bars),
        "█".repeat(review_bars),
        "░".repeat(10 - review_bars)
    )
}

/// Get comprehensive statistics visualization
pub fn get_statistics_visualization(stats: &DeckStatistics) -> String {
    let mut visualization = String::new();

    // Card distribution chart
    visualization.push_str(&create_card_distribution_chart(stats));
    visualization.push_str("\n\n");

    // Retention gauge
    visualization.push_str(&create_retention_gauge(stats.retention_rate));
    visualization.push_str("\n\n");

    // Difficulty distribution
    visualization.push_str(&create_difficulty_chart(&stats.difficulty_distribution));
    visualization.push_str("\n\n");

    // Study streak
    visualization.push_str(&create_streak_chart(
        stats.study_streak_days as u32,
        stats.longest_study_streak as u32,
    ));
    visualization.push_str("\n\n");

    // Today's progress
    visualization.push_str(&create_progress_timeline(
        stats.cards_learned_today as u32,
        stats.cards_reviewed_today as u32,
    ));

    visualization
}

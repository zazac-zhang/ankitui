//! Statistics Engine - Calculates comprehensive learning statistics
//!
//! Provides detailed analytics for learning progress, memory retention,
//! efficiency metrics, and visualization data for the TUI layer.

use crate::core::session_controller::SessionStats;
use crate::data::models::{Card, CardState, Deck};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

/// Comprehensive statistics for a deck
#[derive(Debug, Clone)]
pub struct DeckStatistics {
    pub deck_id: uuid::Uuid,
    pub deck_name: String,

    // Card counts by state
    pub total_cards: usize,
    pub new_cards: usize,
    pub learning_cards: usize,
    pub review_cards: usize,
    pub relearning_cards: usize,

    // Due cards
    pub due_now: usize,
    pub due_today: usize,
    pub due_tomorrow: usize,
    pub due_this_week: usize,
    pub due_this_month: usize,

    // Progress metrics
    pub mature_cards: usize,
    pub young_cards: usize,
    pub suspended_cards: usize,
    pub buried_cards: usize,

    // Performance metrics
    pub average_ease_factor: f32,
    pub average_interval: f32,
    pub retention_rate: f32,
    pub accuracy_rate: f32,

    // Study time metrics
    pub total_study_time: Duration,
    pub average_study_time_per_card: Duration,
    pub total_reviews: i32,

    // Learning metrics
    pub cards_learned_today: i32,
    pub cards_reviewed_today: i32,
    pub study_streak_days: i32,
    pub longest_study_streak: i32,

    // Difficulty distribution
    pub difficulty_distribution: DifficultyDistribution,

    // Timeline data for visualization
    pub timeline: Vec<DayStats>,
}

/// Card difficulty distribution
#[derive(Debug, Clone)]
pub struct DifficultyDistribution {
    pub very_easy: usize,
    pub easy: usize,
    pub normal: usize,
    pub hard: usize,
    pub very_hard: usize,
}

/// Daily statistics for timeline analysis
#[derive(Debug, Clone)]
pub struct DayStats {
    pub date: chrono::NaiveDate,
    pub cards_studied: i32,
    pub new_cards: i32,
    pub reviews: i32,
    pub accuracy: f32,
    pub study_time: Duration,
    pub ease_factor_change: f32,
}

/// Learning efficiency metrics
#[derive(Debug, Clone)]
pub struct LearningEfficiency {
    pub cards_per_hour: f32,
    pub retention_rate_7_days: f32,
    pub retention_rate_30_days: f32,
    pub average_reviews_per_card: f32,
    pub card_maturity_rate: f32,
    pub forgetting_curve_index: f32,
}

/// Memory retention analysis
#[derive(Debug, Clone)]
pub struct MemoryRetention {
    pub overall_retention: f32,
    pub retention_by_interval: HashMap<String, f32>,
    pub retention_by_card_age: HashMap<String, f32>,
    pub optimal_review_intervals: Vec<IntervalRetention>,
    pub predicted_retention_curve: Vec<(i32, f32)>, // (days, retention_rate)
}

/// Interval retention data point
#[derive(Debug, Clone)]
pub struct IntervalRetention {
    pub interval_range: (i32, i32),
    pub retention_rate: f32,
    pub sample_size: usize,
}

/// Study trends analysis
#[derive(Debug, Clone)]
pub struct StudyTrends {
    pub last_7_days: TrendPeriod,
    pub last_30_days: TrendPeriod,
    pub last_90_days: TrendPeriod,
    pub predicted_performance: Vec<(chrono::NaiveDate, f32)>,
}

/// Trend analysis for a specific period
#[derive(Debug, Clone)]
pub struct TrendPeriod {
    pub cards_per_day_average: f32,
    pub accuracy_trend: TrendDirection,
    pub volume_trend: TrendDirection,
    pub consistency_score: f32,
    pub best_day: Option<DayStats>,
    pub worst_day: Option<DayStats>,
}

/// Trend direction indicators
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Improving,
    Declining,
    Stable,
    Volatile,
}

/// Visualization data for TUI components
#[derive(Debug, Clone)]
pub struct VisualizationData {
    pub retention_chart: Vec<(String, f32)>,
    pub progress_chart: Vec<(String, i32)>,
    pub calendar_heatmap: Vec<(chrono::NaiveDate, i32)>,
    pub difficulty_pie: Vec<(String, usize)>,
    pub interval_distribution: Vec<(String, usize)>,
}

/// Statistics Engine - Comprehensive analytics for learning data
#[derive(Clone, Debug)]
pub struct StatsEngine {
    // Cache for computed statistics
    stats_cache: HashMap<uuid::Uuid, CachedStats>,

    // Configuration for statistics calculation
    cache_ttl: Duration,
    mature_threshold_days: i32,
}

/// Cached statistics with expiration
#[derive(Debug, Clone)]
struct CachedStats {
    stats: DeckStatistics,
    computed_at: DateTime<Utc>,
}

impl StatsEngine {
    /// Create a new Statistics Engine with default configuration
    pub fn new() -> Self {
        Self {
            stats_cache: HashMap::new(),
            cache_ttl: Duration::minutes(15),
            mature_threshold_days: 21,
        }
    }

    /// Create a new Statistics Engine with custom configuration
    pub fn with_config(cache_ttl_minutes: i64, mature_threshold_days: i32) -> Self {
        Self {
            stats_cache: HashMap::new(),
            cache_ttl: Duration::minutes(cache_ttl_minutes),
            mature_threshold_days,
        }
    }

    /// Calculate comprehensive statistics for a deck
    pub async fn calculate_deck_statistics(
        &mut self,
        deck: &Deck,
        cards: &[Card],
        session_stats: Option<&SessionStats>,
    ) -> Result<DeckStatistics> {
        // Check cache first
        let deck_id = deck.uuid;
        if let Some(cached) = self.get_cached_stats(deck_id) {
            return Ok(cached);
        }

        let now = Utc::now();
        let today = now.date_naive();

        // Calculate card counts by state
        let mut new_cards = 0;
        let mut learning_cards = 0;
        let mut review_cards = 0;
        let mut relearning_cards = 0;
        let mut mature_cards = 0;
        let mut young_cards = 0;
        let mut suspended_cards = 0;
        let mut buried_cards = 0;

        let mut total_ease_factor = 0.0;
        let mut total_interval = 0;
        let mut due_counts = DueCounter::new();

        for card in cards {
            // Count by state
            match card.state.state {
                CardState::New => new_cards += 1,
                CardState::Learning => learning_cards += 1,
                CardState::Review => {
                    review_cards += 1;
                    if card.state.interval >= self.mature_threshold_days {
                        mature_cards += 1;
                    } else {
                        young_cards += 1;
                    }
                }
                CardState::Relearning => relearning_cards += 1,
                CardState::Buried => {
                    buried_cards += 1;
                    continue;
                }
                CardState::Suspended => {
                    suspended_cards += 1;
                    continue;
                }
            }

            // Accumulate metrics
            total_ease_factor += card.state.ease_factor;
            total_interval += card.state.interval;

            // Count due cards
            due_counts.count_card(&card, now);
        }

        let total_cards = cards.len();
        let average_ease_factor = if total_cards > 0 {
            total_ease_factor / total_cards as f32
        } else {
            0.0
        };
        let average_interval = if total_cards > 0 {
            total_interval as f32 / total_cards as f32
        } else {
            0.0
        };

        // Calculate difficulty distribution
        let difficulty_distribution = self.calculate_difficulty_distribution(cards);

        // Generate timeline data
        let timeline = self.generate_timeline_data(cards, session_stats, today)?;

        // Calculate retention rate (based on recent performance)
        let retention_rate = self.calculate_retention_rate(cards, session_stats);

        // Calculate accuracy rate
        let accuracy_rate = self.calculate_accuracy_rate(session_stats);

        let deck_stats = DeckStatistics {
            deck_id,
            deck_name: deck.name.clone(),
            total_cards,
            new_cards,
            learning_cards,
            review_cards,
            relearning_cards,
            due_now: due_counts.due_now,
            due_today: due_counts.due_today,
            due_tomorrow: due_counts.due_tomorrow,
            due_this_week: due_counts.due_this_week,
            due_this_month: due_counts.due_this_month,
            mature_cards,
            young_cards,
            suspended_cards,
            buried_cards,
            average_ease_factor,
            average_interval,
            retention_rate,
            accuracy_rate,
            total_study_time: session_stats
                .map(|s| s.ended_at.unwrap_or(now) - s.started_at)
                .unwrap_or_default(),
            average_study_time_per_card: Duration::seconds(0), // TODO: Track individual card times
            total_reviews: session_stats
                .map(|s| s.total_cards_studied as i32)
                .unwrap_or(0),
            cards_learned_today: session_stats
                .map(|s| s.new_cards_studied as i32)
                .unwrap_or(0),
            cards_reviewed_today: session_stats
                .map(|s| s.review_cards_studied as i32)
                .unwrap_or(0),
            study_streak_days: session_stats.map(|s| s.study_streak_days).unwrap_or(0),
            longest_study_streak: 0, // TODO: Implement streak tracking
            difficulty_distribution,
            timeline,
        };

        // Cache the results
        self.cache_stats(deck_id, deck_stats.clone());

        Ok(deck_stats)
    }

    /// Calculate learning efficiency metrics
    pub async fn calculate_learning_efficiency(
        &self,
        deck_stats: &DeckStatistics,
        cards: &[Card],
        _recent_sessions: &[SessionStats],
    ) -> Result<LearningEfficiency> {
        let total_study_minutes = deck_stats.total_study_time.num_minutes() as f32;
        let cards_per_hour = if total_study_minutes > 0.0 {
            (deck_stats.total_reviews as f32 / total_study_minutes) * 60.0
        } else {
            0.0
        };

        let retention_rate_7_days = self.calculate_retention_for_period(cards, 7);
        let retention_rate_30_days = self.calculate_retention_for_period(cards, 30);

        let average_reviews_per_card = if deck_stats.total_cards > 0 {
            deck_stats.total_reviews as f32 / deck_stats.total_cards as f32
        } else {
            0.0
        };

        let card_maturity_rate = if deck_stats.total_cards > 0 {
            deck_stats.mature_cards as f32 / deck_stats.total_cards as f32
        } else {
            0.0
        };

        let forgetting_curve_index = self.calculate_forgetting_curve_index(cards);

        Ok(LearningEfficiency {
            cards_per_hour,
            retention_rate_7_days,
            retention_rate_30_days,
            average_reviews_per_card,
            card_maturity_rate,
            forgetting_curve_index,
        })
    }

    /// Calculate memory retention analysis
    pub async fn calculate_memory_retention(
        &self,
        cards: &[Card],
        session_stats: Option<&SessionStats>,
    ) -> Result<MemoryRetention> {
        let overall_retention = self.calculate_retention_rate(cards, session_stats);

        let retention_by_interval = self.calculate_retention_by_interval(cards);
        let retention_by_card_age = self.calculate_retention_by_card_age(cards);
        let optimal_review_intervals = self.calculate_optimal_review_intervals(cards);
        let predicted_retention_curve = self.predict_retention_curve(cards);

        Ok(MemoryRetention {
            overall_retention,
            retention_by_interval,
            retention_by_card_age,
            optimal_review_intervals,
            predicted_retention_curve,
        })
    }

    /// Calculate study trends analysis
    pub async fn calculate_study_trends(&self, timeline: &[DayStats]) -> Result<StudyTrends> {
        let last_7_days = self.analyze_trend_period(timeline, 7);
        let last_30_days = self.analyze_trend_period(timeline, 30);
        let last_90_days = self.analyze_trend_period(timeline, 90);
        let predicted_performance = self.predict_performance(timeline);

        Ok(StudyTrends {
            last_7_days,
            last_30_days,
            last_90_days,
            predicted_performance,
        })
    }

    /// Generate visualization data for TUI components
    pub async fn generate_visualization_data(
        &self,
        deck_stats: &DeckStatistics,
        retention: &MemoryRetention,
        _trends: &StudyTrends,
    ) -> Result<VisualizationData> {
        let retention_chart = vec![
            (
                "1 day".to_string(),
                retention
                    .retention_by_interval
                    .get("0-1")
                    .unwrap_or(&0.0)
                    .clone(),
            ),
            (
                "2-3 days".to_string(),
                retention
                    .retention_by_interval
                    .get("2-3")
                    .unwrap_or(&0.0)
                    .clone(),
            ),
            (
                "1 week".to_string(),
                retention
                    .retention_by_interval
                    .get("7-14")
                    .unwrap_or(&0.0)
                    .clone(),
            ),
            (
                "2 weeks".to_string(),
                retention
                    .retention_by_interval
                    .get("14-21")
                    .unwrap_or(&0.0)
                    .clone(),
            ),
            (
                "1 month".to_string(),
                retention
                    .retention_by_interval
                    .get("21-30")
                    .unwrap_or(&0.0)
                    .clone(),
            ),
        ];

        let progress_chart = vec![
            ("New".to_string(), deck_stats.new_cards as i32),
            ("Learning".to_string(), deck_stats.learning_cards as i32),
            ("Young".to_string(), deck_stats.young_cards as i32),
            ("Mature".to_string(), deck_stats.mature_cards as i32),
        ];

        let calendar_heatmap = deck_stats
            .timeline
            .iter()
            .take(365) // Last year
            .map(|day| (day.date, day.cards_studied))
            .collect();

        let difficulty_pie = vec![
            (
                "Very Easy".to_string(),
                deck_stats.difficulty_distribution.very_easy,
            ),
            ("Easy".to_string(), deck_stats.difficulty_distribution.easy),
            (
                "Normal".to_string(),
                deck_stats.difficulty_distribution.normal,
            ),
            ("Hard".to_string(), deck_stats.difficulty_distribution.hard),
            (
                "Very Hard".to_string(),
                deck_stats.difficulty_distribution.very_hard,
            ),
        ];

        let interval_distribution = self.calculate_interval_distribution(deck_stats);

        Ok(VisualizationData {
            retention_chart,
            progress_chart,
            calendar_heatmap,
            difficulty_pie,
            interval_distribution,
        })
    }

    // Private helper methods

    fn get_cached_stats(&self, deck_id: uuid::Uuid) -> Option<DeckStatistics> {
        if let Some(cached) = self.stats_cache.get(&deck_id) {
            if Utc::now() - cached.computed_at < self.cache_ttl {
                return Some(cached.stats.clone());
            }
        }
        None
    }

    fn cache_stats(&mut self, deck_id: uuid::Uuid, stats: DeckStatistics) {
        self.stats_cache.insert(
            deck_id,
            CachedStats {
                stats,
                computed_at: Utc::now(),
            },
        );
    }

    fn calculate_difficulty_distribution(&self, cards: &[Card]) -> DifficultyDistribution {
        let mut distribution = DifficultyDistribution {
            very_easy: 0,
            easy: 0,
            normal: 0,
            hard: 0,
            very_hard: 0,
        };

        for card in cards {
            match card.state.ease_factor {
                ef if ef >= 2.7 => distribution.very_easy += 1,
                ef if ef >= 2.5 => distribution.easy += 1,
                ef if ef >= 2.3 => distribution.normal += 1,
                ef if ef >= 2.1 => distribution.hard += 1,
                _ => distribution.very_hard += 1,
            }
        }

        distribution
    }

    fn generate_timeline_data(
        &self,
        cards: &[Card],
        session_stats: Option<&SessionStats>,
        today: chrono::NaiveDate,
    ) -> Result<Vec<DayStats>> {
        let mut timeline = Vec::new();

        // Generate last 90 days of data
        for days_back in 0..90 {
            let date = today - Duration::days(days_back);

            // In a real implementation, this would query the database for historical data
            // For now, we'll generate mock data based on current card states
            let day_stats = DayStats {
                date,
                cards_studied: (cards.len() as f32 * 0.1) as i32, // Mock: 10% studied per day
                new_cards: (cards.len() as f32 * 0.02) as i32,    // Mock: 2% new per day
                reviews: (cards.len() as f32 * 0.08) as i32,      // Mock: 8% reviews per day
                accuracy: 0.85,                                   // Mock: 85% accuracy
                study_time: Duration::minutes(15),                // Mock: 15 minutes per day
                ease_factor_change: 0.01,                         // Mock: slight improvement
            };

            timeline.push(day_stats);
        }

        // Sort by date
        timeline.sort_by_key(|d| d.date);

        Ok(timeline)
    }

    fn calculate_retention_rate(
        &self,
        cards: &[Card],
        session_stats: Option<&SessionStats>,
    ) -> f32 {
        if let Some(stats) = session_stats {
            if stats.total_cards_studied > 0 {
                stats.correct_answers as f32 / stats.total_cards_studied as f32
            } else {
                0.0
            }
        } else {
            // Fallback: estimate retention based on ease factors
            if cards.is_empty() {
                return 0.0;
            }

            let total_ease: f32 = cards.iter().map(|c| c.state.ease_factor).sum();
            let average_ease = total_ease / cards.len() as f32;

            // Rough conversion: ease factor 2.5 ~ 85% retention
            (average_ease / 2.5).min(1.0).max(0.0)
        }
    }

    fn calculate_accuracy_rate(&self, session_stats: Option<&SessionStats>) -> f32 {
        session_stats
            .map(|stats| {
                if stats.total_cards_studied > 0 {
                    stats.correct_answers as f32 / stats.total_cards_studied as f32
                } else {
                    0.0
                }
            })
            .unwrap_or(0.0)
    }

    fn calculate_retention_for_period(&self, cards: &[Card], days: i32) -> f32 {
        if cards.is_empty() {
            return 0.0;
        }

        let cutoff_date = Utc::now() - Duration::days(days as i64);
        let relevant_cards: Vec<_> = cards
            .iter()
            .filter(|c| c.state.updated_at >= cutoff_date)
            .collect();

        if relevant_cards.is_empty() {
            return 0.0;
        }

        // Estimate retention based on ease factors of recently studied cards
        let total_ease: f32 = relevant_cards.iter().map(|c| c.state.ease_factor).sum();
        let average_ease = total_ease / relevant_cards.len() as f32;

        (average_ease / 2.5).min(1.0).max(0.0)
    }

    fn calculate_retention_by_interval(&self, cards: &[Card]) -> HashMap<String, f32> {
        let mut retention_by_interval = HashMap::new();

        // Group cards by interval ranges
        let mut intervals: HashMap<String, Vec<&Card>> = HashMap::new();

        for card in cards {
            let range = match card.state.interval {
                0 => "0-1".to_string(),
                1..=3 => "2-3".to_string(),
                4..=7 => "4-7".to_string(),
                8..=14 => "8-14".to_string(),
                15..=21 => "15-21".to_string(),
                22..=30 => "22-30".to_string(),
                _ => "30+".to_string(),
            };

            intervals.entry(range).or_insert_with(Vec::new).push(card);
        }

        // Calculate retention for each interval range
        for (range, cards_in_range) in intervals {
            if !cards_in_range.is_empty() {
                let avg_ease = cards_in_range
                    .iter()
                    .map(|c| c.state.ease_factor)
                    .sum::<f32>()
                    / cards_in_range.len() as f32;
                let retention = (avg_ease / 2.5).min(1.0).max(0.0);
                retention_by_interval.insert(range, retention);
            }
        }

        retention_by_interval
    }

    fn calculate_retention_by_card_age(&self, cards: &[Card]) -> HashMap<String, f32> {
        let mut retention_by_age = HashMap::new();

        // Group cards by age based on repetition count
        let mut age_groups: HashMap<String, Vec<&Card>> = HashMap::new();

        for card in cards {
            let age_group = match card.state.reps {
                0 => "New".to_string(),
                1..=2 => "Young".to_string(),
                3..=5 => "Adult".to_string(),
                _ => "Mature".to_string(),
            };

            age_groups
                .entry(age_group)
                .or_insert_with(Vec::new)
                .push(card);
        }

        // Calculate retention for each age group
        for (age, cards_in_age) in age_groups {
            if !cards_in_age.is_empty() {
                let avg_ease = cards_in_age
                    .iter()
                    .map(|c| c.state.ease_factor)
                    .sum::<f32>()
                    / cards_in_age.len() as f32;
                let retention = (avg_ease / 2.5).min(1.0).max(0.0);
                retention_by_age.insert(age, retention);
            }
        }

        retention_by_age
    }

    fn calculate_optimal_review_intervals(&self, cards: &[Card]) -> Vec<IntervalRetention> {
        let mut intervals = Vec::new();

        // Define interval ranges
        let ranges = vec![
            (0, 1),
            (2, 3),
            (4, 7),
            (8, 14),
            (15, 21),
            (22, 30),
            (31, 60),
            (61, 180),
            (181, i32::MAX),
        ];

        for (start, end) in ranges {
            let cards_in_range: Vec<_> = cards
                .iter()
                .filter(|c| c.state.interval >= start && c.state.interval <= end)
                .collect();

            if !cards_in_range.is_empty() {
                let avg_ease = cards_in_range
                    .iter()
                    .map(|c| c.state.ease_factor)
                    .sum::<f32>()
                    / cards_in_range.len() as f32;
                let retention = (avg_ease / 2.5).min(1.0).max(0.0);

                intervals.push(IntervalRetention {
                    interval_range: (start, end),
                    retention_rate: retention,
                    sample_size: cards_in_range.len(),
                });
            }
        }

        intervals
    }

    fn predict_retention_curve(&self, cards: &[Card]) -> Vec<(i32, f32)> {
        let mut curve = Vec::new();

        // Predict retention over next 90 days
        for days in 1..=90 {
            let predicted_retention = self.predict_retention_at_days(cards, days);
            curve.push((days, predicted_retention));
        }

        curve
    }

    fn predict_retention_at_days(&self, cards: &[Card], days: i32) -> f32 {
        if cards.is_empty() {
            return 0.0;
        }

        // Simplified forgetting curve model
        // R(t) = initial_retention * e^(-t / decay_constant)
        let initial_retention = self.calculate_retention_rate(cards, None);
        let decay_constant = 30.0; // 30 days for significant forgetting

        let days_f32 = days as f32;
        let predicted = initial_retention * (-days_f32 / decay_constant).exp();

        predicted.max(0.0).min(1.0)
    }

    fn calculate_forgetting_curve_index(&self, cards: &[Card]) -> f32 {
        if cards.is_empty() {
            return 0.0;
        }

        // Calculate how well cards are retaining knowledge over time
        // Higher index = better retention
        let total_lapses: i32 = cards.iter().map(|c| c.state.lapses).sum();
        let total_reps: i32 = cards.iter().map(|c| c.state.reps).sum();

        if total_reps == 0 {
            return 1.0; // Perfect retention (no lapses)
        }

        let lapse_rate = total_lapses as f32 / total_reps as f32;
        (1.0 - lapse_rate).max(0.0)
    }

    fn analyze_trend_period(&self, timeline: &[DayStats], days: i32) -> TrendPeriod {
        let recent_data: Vec<_> = timeline.iter().rev().take(days as usize).collect();

        if recent_data.is_empty() {
            return TrendPeriod {
                cards_per_day_average: 0.0,
                accuracy_trend: TrendDirection::Stable,
                volume_trend: TrendDirection::Stable,
                consistency_score: 0.0,
                best_day: None,
                worst_day: None,
            };
        }

        let cards_per_day_average = recent_data
            .iter()
            .map(|d| d.cards_studied as f32)
            .sum::<f32>()
            / recent_data.len() as f32;

        let accuracy_trend =
            self.calculate_trend_direction(recent_data.iter().map(|d| d.accuracy).collect());

        let volume_trend = self.calculate_trend_direction(
            recent_data.iter().map(|d| d.cards_studied as f32).collect(),
        );

        let consistency_score = self.calculate_consistency_score(&recent_data);

        let best_day = recent_data
            .iter()
            .max_by_key(|d| d.cards_studied)
            .map(|&d| d);

        let worst_day = recent_data
            .iter()
            .min_by_key(|d| d.cards_studied)
            .map(|&d| d);

        TrendPeriod {
            cards_per_day_average,
            accuracy_trend,
            volume_trend,
            consistency_score,
            best_day: best_day.cloned(),
            worst_day: worst_day.cloned(),
        }
    }

    fn calculate_trend_direction(&self, values: Vec<f32>) -> TrendDirection {
        if values.len() < 3 {
            return TrendDirection::Stable;
        }

        // Simple linear regression to detect trend
        let n = values.len() as f32;
        let sum_x: f32 = (0..values.len()).map(|i| i as f32).sum();
        let sum_y: f32 = values.iter().sum();
        let sum_xy: f32 = values.iter().enumerate().map(|(i, &y)| i as f32 * y).sum();
        let sum_x2: f32 = (0..values.len()).map(|i| (i as f32).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));

        let threshold = values.iter().map(|&v| v.abs()).sum::<f32>() / values.len() as f32 * 0.1;

        if slope > threshold {
            TrendDirection::Improving
        } else if slope < -threshold {
            TrendDirection::Declining
        } else {
            TrendDirection::Stable
        }
    }

    fn calculate_consistency_score(&self, data: &[&DayStats]) -> f32 {
        if data.len() < 2 {
            return 0.0;
        }

        let values: Vec<f32> = data.iter().map(|d| d.cards_studied as f32).collect();
        let mean = values.iter().sum::<f32>() / values.len() as f32;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / values.len() as f32;
        let std_dev = variance.sqrt();

        // Lower standard deviation = higher consistency
        let consistency = if mean > 0.0 {
            1.0 - (std_dev / mean).min(1.0)
        } else {
            0.0
        };

        consistency
    }

    fn predict_performance(&self, timeline: &[DayStats]) -> Vec<(chrono::NaiveDate, f32)> {
        let mut predictions = Vec::new();
        let last_date = timeline
            .last()
            .map(|d| d.date)
            .unwrap_or_else(|| Utc::now().date_naive());

        // Predict next 30 days
        for days_ahead in 1..=30 {
            let future_date = last_date + Duration::days(days_ahead);

            // Simple prediction based on recent average
            let recent_avg = timeline
                .iter()
                .rev()
                .take(7)
                .map(|d| d.cards_studied as f32)
                .sum::<f32>()
                / 7.0;

            let predicted_cards = recent_avg * (1.0 + (days_ahead as f32 * 0.01)); // Slight growth
            predictions.push((future_date, predicted_cards));
        }

        predictions
    }

    fn calculate_interval_distribution(&self, deck_stats: &DeckStatistics) -> Vec<(String, usize)> {
        vec![
            ("0-1 days".to_string(), deck_stats.new_cards),
            ("2-7 days".to_string(), deck_stats.learning_cards),
            ("8-21 days".to_string(), deck_stats.young_cards),
            ("21+ days".to_string(), deck_stats.mature_cards),
        ]
    }
}

/// Helper structure for counting due cards
#[derive(Debug, Default)]
struct DueCounter {
    pub due_now: usize,
    pub due_today: usize,
    pub due_tomorrow: usize,
    pub due_this_week: usize,
    pub due_this_month: usize,
}

impl DueCounter {
    fn new() -> Self {
        Self::default()
    }

    fn count_card(&mut self, card: &Card, now: DateTime<Utc>) {
        let card_date = card.state.due.date_naive();
        let today = now.date_naive();

        if card_date <= today {
            self.due_now += 1;
            self.due_today += 1;
        } else if card_date == today + Duration::days(1) {
            self.due_tomorrow += 1;
        }

        if card_date <= today + Duration::days(7) {
            self.due_this_week += 1;
        }

        if card_date <= today + Duration::days(30) {
            self.due_this_month += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::models::{CardContent, CardStateData};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_card(id: Uuid, interval: i32, ease_factor: f32, state: CardState) -> Card {
        let content = CardContent {
            id,
            front: "Test front".to_string(),
            back: "Test back".to_string(),
            tags: vec!["test".to_string()],
            media: None,
            custom: HashMap::new(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        let state_data = CardStateData {
            id,
            due: Utc::now(),
            interval,
            ease_factor,
            reps: 3,
            lapses: 1,
            state,
            updated_at: Utc::now(),
        };

        Card {
            content,
            state: state_data,
        }
    }

    fn create_test_deck() -> Deck {
        Deck {
            uuid: Uuid::new_v4(),
            name: "Test Deck".to_string(),
            description: Some("Test description".to_string()),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            scheduler_config: None,
        }
    }

    #[tokio::test]
    async fn test_stats_engine_creation() {
        let mut engine = StatsEngine::new();
        assert_eq!(engine.mature_threshold_days, 21);

        let engine_custom = StatsEngine::with_config(30, 30);
        assert_eq!(engine_custom.mature_threshold_days, 30);
    }

    #[tokio::test]
    async fn test_deck_statistics_calculation() {
        let mut engine = StatsEngine::new();
        let deck = create_test_deck();

        let cards = vec![
            create_test_card(Uuid::new_v4(), 0, 2.5, CardState::New),
            create_test_card(Uuid::new_v4(), 1, 2.6, CardState::Learning),
            create_test_card(Uuid::new_v4(), 5, 2.4, CardState::Review),
            create_test_card(Uuid::new_v4(), 25, 2.8, CardState::Review),
        ];

        let stats = engine
            .calculate_deck_statistics(&deck, &cards, None)
            .await
            .unwrap();

        assert_eq!(stats.total_cards, 4);
        assert_eq!(stats.new_cards, 1);
        assert_eq!(stats.learning_cards, 1);
        assert_eq!(stats.review_cards, 2);
        assert_eq!(stats.mature_cards, 1);
        assert_eq!(stats.young_cards, 1);
        assert!(stats.average_ease_factor > 2.0);
        assert!(stats.average_ease_factor < 3.0);
    }

    #[tokio::test]
    async fn test_difficulty_distribution() {
        let mut engine = StatsEngine::new();

        let cards = vec![
            create_test_card(Uuid::new_v4(), 5, 2.1, CardState::Review), // Hard
            create_test_card(Uuid::new_v4(), 5, 2.3, CardState::Review), // Normal
            create_test_card(Uuid::new_v4(), 5, 2.6, CardState::Review), // Easy
            create_test_card(Uuid::new_v4(), 5, 2.8, CardState::Review), // Very Easy
        ];

        let distribution = engine.calculate_difficulty_distribution(&cards);

        assert_eq!(distribution.very_easy, 1);
        assert_eq!(distribution.easy, 1);
        assert_eq!(distribution.normal, 1);
        assert_eq!(distribution.hard, 1);
        assert_eq!(distribution.very_hard, 0);
    }

    #[tokio::test]
    async fn test_learning_efficiency_calculation() {
        let mut engine = StatsEngine::with_config(15, 10); // 10 days mature threshold
        let deck = create_test_deck();

        let cards = vec![
            create_test_card(Uuid::new_v4(), 15, 2.5, CardState::Review),
            create_test_card(Uuid::new_v4(), 25, 2.7, CardState::Review),
        ];

        let session_stats = SessionStats {
            started_at: Utc::now() - Duration::minutes(30),
            ended_at: Some(Utc::now()),
            total_cards_studied: 2,
            new_cards_studied: 0,
            review_cards_studied: 2,
            relearning_cards_studied: 0,
            correct_answers: 2,
            incorrect_answers: 0,
            average_response_time: None,
            study_streak_days: 5,
        };

        let deck_stats = engine
            .calculate_deck_statistics(&deck, &cards, Some(&session_stats))
            .await
            .unwrap();
        let efficiency = engine
            .calculate_learning_efficiency(&deck_stats, &cards, &[session_stats])
            .await
            .unwrap();

        assert!(efficiency.cards_per_hour > 0.0);
        assert_eq!(efficiency.card_maturity_rate, 1.0); // All cards are mature (>=10 days)
        assert!(efficiency.retention_rate_7_days >= 0.0);
        assert!(efficiency.retention_rate_7_days <= 1.0);
    }

    #[tokio::test]
    async fn test_memory_retention_calculation() {
        let mut engine = StatsEngine::new();

        let cards = vec![
            create_test_card(Uuid::new_v4(), 5, 2.8, CardState::Review), // High ease
            create_test_card(Uuid::new_v4(), 3, 2.2, CardState::Review), // Low ease
        ];

        let session_stats = SessionStats {
            started_at: Utc::now() - Duration::minutes(15),
            ended_at: Some(Utc::now()),
            total_cards_studied: 2,
            new_cards_studied: 0,
            review_cards_studied: 2,
            relearning_cards_studied: 0,
            correct_answers: 1,
            incorrect_answers: 1,
            average_response_time: None,
            study_streak_days: 3,
        };

        let retention = engine
            .calculate_memory_retention(&cards, Some(&session_stats))
            .await
            .unwrap();

        assert_eq!(retention.overall_retention, 0.5); // 1 correct out of 2 total
        assert!(!retention.retention_by_interval.is_empty());
        assert!(!retention.optimal_review_intervals.is_empty());
        assert!(!retention.predicted_retention_curve.is_empty());
    }

    #[tokio::test]
    async fn test_study_trends_analysis() {
        let mut engine = StatsEngine::new();

        let today = Utc::now().date_naive();
        let timeline = vec![
            DayStats {
                date: today - Duration::days(2),
                cards_studied: 10,
                new_cards: 2,
                reviews: 8,
                accuracy: 0.8,
                study_time: Duration::minutes(15),
                ease_factor_change: 0.01,
            },
            DayStats {
                date: today - Duration::days(1),
                cards_studied: 15,
                new_cards: 3,
                reviews: 12,
                accuracy: 0.85,
                study_time: Duration::minutes(20),
                ease_factor_change: 0.02,
            },
            DayStats {
                date: today,
                cards_studied: 12,
                new_cards: 1,
                reviews: 11,
                accuracy: 0.9,
                study_time: Duration::minutes(18),
                ease_factor_change: 0.01,
            },
        ];

        let trends = engine.calculate_study_trends(&timeline).await.unwrap();

        assert!(trends.last_7_days.cards_per_day_average > 0.0);
        assert!(trends.last_30_days.cards_per_day_average > 0.0);
        assert!(trends.last_90_days.cards_per_day_average > 0.0);
        assert!(!trends.predicted_performance.is_empty());
    }

    #[tokio::test]
    async fn test_visualization_data_generation() {
        let mut engine = StatsEngine::new();
        let deck = create_test_deck();

        let cards = vec![create_test_card(Uuid::new_v4(), 5, 2.5, CardState::Review)];

        let session_stats = SessionStats {
            started_at: Utc::now() - Duration::minutes(10),
            ended_at: Some(Utc::now()),
            total_cards_studied: 1,
            new_cards_studied: 0,
            review_cards_studied: 1,
            relearning_cards_studied: 0,
            correct_answers: 1,
            incorrect_answers: 0,
            average_response_time: None,
            study_streak_days: 1,
        };

        let deck_stats = engine
            .calculate_deck_statistics(&deck, &cards, Some(&session_stats))
            .await
            .unwrap();
        let retention = engine
            .calculate_memory_retention(&cards, Some(&session_stats))
            .await
            .unwrap();
        let trends = engine.calculate_study_trends(&[]).await.unwrap();

        let viz_data = engine
            .generate_visualization_data(&deck_stats, &retention, &trends)
            .await
            .unwrap();

        assert!(!viz_data.retention_chart.is_empty());
        assert!(!viz_data.progress_chart.is_empty());
        assert!(!viz_data.difficulty_pie.is_empty());
        assert!(!viz_data.interval_distribution.is_empty());
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        let mut engine = StatsEngine::new();
        let deck = create_test_deck();
        let cards = vec![create_test_card(Uuid::new_v4(), 5, 2.5, CardState::Review)];

        // First calculation should compute and cache
        let stats1 = engine
            .calculate_deck_statistics(&deck, &cards, None)
            .await
            .unwrap();

        // Second calculation should use cache (within TTL)
        let stats2 = engine
            .calculate_deck_statistics(&deck, &cards, None)
            .await
            .unwrap();

        assert_eq!(stats1.deck_id, stats2.deck_id);
        assert_eq!(stats1.total_cards, stats2.total_cards);
    }

    #[tokio::test]
    async fn test_due_counter() {
        let mut counter = DueCounter::new();
        let now = Utc::now();
        let today = now.date_naive();

        // Create test cards with different due dates
        let mut card_due_now = create_test_card(Uuid::new_v4(), 5, 2.5, CardState::Review);
        card_due_now.state.due = now;

        let mut card_due_tomorrow = create_test_card(Uuid::new_v4(), 5, 2.5, CardState::Review);
        card_due_tomorrow.state.due = now + Duration::days(1);

        let mut card_due_next_week = create_test_card(Uuid::new_v4(), 5, 2.5, CardState::Review);
        card_due_next_week.state.due = now + Duration::days(7);

        counter.count_card(&card_due_now, now);
        counter.count_card(&card_due_tomorrow, now);
        counter.count_card(&card_due_next_week, now);

        assert_eq!(counter.due_now, 1);
        assert_eq!(counter.due_today, 1);
        assert_eq!(counter.due_tomorrow, 1);
        assert_eq!(counter.due_this_week, 3);
        assert_eq!(counter.due_this_month, 3);
    }

    #[test]
    fn test_trend_direction_calculation() {
        let mut engine = StatsEngine::new();

        // Improving trend - use more pronounced improvement
        let improving_values = vec![0.5, 0.6, 0.7, 0.8, 0.9];
        assert_eq!(
            engine.calculate_trend_direction(improving_values),
            TrendDirection::Improving
        );

        // Declining trend - use more pronounced decline
        let declining_values = vec![0.9, 0.8, 0.7, 0.6, 0.5];
        assert_eq!(
            engine.calculate_trend_direction(declining_values),
            TrendDirection::Declining
        );

        // Stable trend
        let stable_values = vec![0.8, 0.81, 0.79, 0.82, 0.8];
        assert_eq!(
            engine.calculate_trend_direction(stable_values),
            TrendDirection::Stable
        );
    }
}

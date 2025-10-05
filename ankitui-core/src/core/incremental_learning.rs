//! Incremental Learning System
//!
//! Advanced learning algorithms and queue management for optimized card presentation

use crate::core::{Rating, Scheduler};
use crate::data::models::{Card, CardState};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

/// Incremental learning configuration
#[derive(Debug, Clone)]
pub struct IncrementalLearningConfig {
    /// Maximum number of new cards to introduce per session
    pub max_new_cards_per_session: usize,
    /// Maximum number of review cards per session
    pub max_review_cards_per_session: usize,
    /// Interval between introducing new cards (in reviews)
    pub new_card_spacing: usize,
    /// Enable automatic burying of related cards
    pub auto_bury_related: bool,
    /// Enable dynamic difficulty adjustment
    pub dynamic_difficulty: bool,
    /// Weight for easy cards in queue selection
    pub easy_card_weight: f32,
    /// Weight for hard cards in queue selection
    pub hard_card_weight: f32,
    /// Minimum retention rate before adjusting difficulty
    pub min_retention_rate: f32,
}

impl Default for IncrementalLearningConfig {
    fn default() -> Self {
        Self {
            max_new_cards_per_session: 20,
            max_review_cards_per_session: 100,
            new_card_spacing: 5,
            auto_bury_related: true,
            dynamic_difficulty: true,
            easy_card_weight: 0.7,
            hard_card_weight: 1.5,
            min_retention_rate: 0.85,
        }
    }
}

/// Learning queue management
#[derive(Debug)]
pub struct LearningQueue {
    /// New cards queue
    new_cards: VecDeque<Uuid>,
    /// Review cards queue
    review_cards: VecDeque<Uuid>,
    /// Learning cards queue (short intervals)
    learning_cards: VecDeque<Uuid>,
    /// Relearning cards queue
    relearning_cards: VecDeque<Uuid>,
    /// Current session statistics
    session_stats: SessionStats,
    /// Configuration
    config: IncrementalLearningConfig,
    /// Card priority scores for dynamic ordering
    priority_scores: HashMap<Uuid, f32>,
}

/// Session learning statistics
#[derive(Debug, Clone, Default)]
pub struct SessionStats {
    /// Total cards studied
    pub total_studied: usize,
    /// New cards studied
    pub new_cards_studied: usize,
    /// Review cards studied
    pub review_cards_studied: usize,
    /// Learning cards studied
    pub learning_cards_studied: usize,
    /// Correct responses
    pub correct_responses: usize,
    /// Total responses
    pub total_responses: usize,
    /// Average response time in seconds
    pub avg_response_time: f32,
    /// Current retention rate
    pub retention_rate: f32,
    /// Cards buried this session
    pub cards_buried: usize,
    /// Cards suspended this session
    pub cards_suspended: usize,
    /// Session start time
    pub session_start: DateTime<Utc>,
}

/// Card priority factors
#[derive(Debug, Clone)]
pub struct CardPriority {
    /// Base priority from due time
    pub due_priority: f32,
    /// Priority from difficulty (ease factor)
    pub difficulty_priority: f32,
    /// Priority from card state
    pub state_priority: f32,
    /// Priority from user performance history
    pub performance_priority: f32,
    /// Final calculated priority
    pub final_priority: f32,
}

impl LearningQueue {
    /// Create a new learning queue
    pub fn new(config: IncrementalLearningConfig) -> Self {
        Self {
            new_cards: VecDeque::new(),
            review_cards: VecDeque::new(),
            learning_cards: VecDeque::new(),
            relearning_cards: VecDeque::new(),
            session_stats: SessionStats::default(),
            config,
            priority_scores: HashMap::new(),
        }
    }

    /// Initialize the learning queue from available cards
    pub fn initialize(&mut self, cards: &[Card], now: DateTime<Utc>) -> Result<()> {
        self.clear_queues();

        let mut new_queue = Vec::new();
        let mut review_queue = Vec::new();
        let mut learning_queue = Vec::new();
        let mut relearning_queue = Vec::new();

        for card in cards {
            if !Scheduler::is_reviewable(card) {
                continue;
            }

            match card.state.state {
                CardState::New => {
                    new_queue.push(card.clone());
                }
                CardState::Review => {
                    if card.state.due <= now {
                        review_queue.push(card.clone());
                    }
                }
                CardState::Learning => {
                    if card.state.due <= now {
                        learning_queue.push(card.clone());
                    }
                }
                CardState::Relearning => {
                    if card.state.due <= now {
                        relearning_queue.push(card.clone());
                    }
                }
                CardState::Buried | CardState::Suspended => {
                    // Skip these cards
                }
            }
        }

        // Sort and limit queues based on configuration
        self.sort_and_limit_new_cards(&mut new_queue, now)?;
        self.sort_review_cards(&mut review_queue, now);
        self.sort_learning_cards(&mut learning_queue, now);
        self.sort_relearning_cards(&mut relearning_queue, now);

        // Add to queues
        for card in new_queue {
            self.new_cards.push_back(card.content.id);
        }
        for card in review_queue {
            self.review_cards.push_back(card.content.id);
        }
        for card in learning_queue {
            self.learning_cards.push_back(card.content.id);
        }
        for card in relearning_queue {
            self.relearning_cards.push_back(card.content.id);
        }

        self.session_stats.session_start = now;

        Ok(())
    }

    /// Get the next card to study
    pub fn get_next_card(&mut self, available_cards: &[Card]) -> Option<Uuid> {
        // Priority order: relearning > learning > review > new
        if let Some(card_id) = self.relearning_cards.pop_front() {
            return Some(card_id);
        }

        if let Some(card_id) = self.learning_cards.pop_front() {
            return Some(card_id);
        }

        if let Some(card_id) = self.review_cards.pop_front() {
            return Some(card_id);
        }

        // Check if we should add more new cards based on spacing
        if self.should_add_new_card() {
            if let Some(card_id) = self.new_cards.pop_front() {
                return Some(card_id);
            }
        }

        None
    }

    /// Check if we should add a new card based on spacing configuration
    fn should_add_new_card(&self) -> bool {
        if self.session_stats.new_cards_studied >= self.config.max_new_cards_per_session {
            return false;
        }

        if self.session_stats.total_studied == 0 {
            return true;
        }

        // Check spacing: every N reviews, introduce a new card
        let reviews_since_last_new =
            self.session_stats.review_cards_studied + self.session_stats.learning_cards_studied;

        reviews_since_last_new >= self.config.new_card_spacing
    }

    /// Process card review result
    pub fn process_review_result(
        &mut self,
        card_id: Uuid,
        card_state: CardState,
        rating: Rating,
        response_time_ms: u32,
    ) -> ReviewResult {
        self.session_stats.total_studied += 1;
        self.session_stats.total_responses += 1;

        // Update appropriate counter based on card state
        match card_state {
            CardState::New => self.session_stats.new_cards_studied += 1,
            CardState::Review => self.session_stats.review_cards_studied += 1,
            CardState::Learning | CardState::Relearning => {
                self.session_stats.learning_cards_studied += 1
            }
            CardState::Buried | CardState::Suspended => {
                // These shouldn't be in the learning queue
            }
        }

        let is_correct = matches!(rating, Rating::Good | Rating::Easy);
        if is_correct {
            self.session_stats.correct_responses += 1;
        }

        // Update retention rate
        self.session_stats.retention_rate =
            self.session_stats.correct_responses as f32 / self.session_stats.total_responses as f32;

        // Update average response time
        let new_time = response_time_ms as f32 / 1000.0;
        let total_time = self.session_stats.avg_response_time
            * (self.session_stats.total_responses - 1) as f32
            + new_time;
        self.session_stats.avg_response_time =
            total_time / self.session_stats.total_responses as f32;

        // Determine if card should be requeued
        let should_requeue = self.should_requeue_card(card_id, rating);

        ReviewResult {
            rating,
            is_correct,
            should_requeue,
            session_progress: self.get_session_progress(),
        }
    }

    /// Determine if a card should be requeued based on rating and session state
    fn should_requeue_card(&self, _card_id: Uuid, rating: Rating) -> bool {
        match rating {
            Rating::Again => {
                // Card should be requeued after some other cards
                true
            }
            Rating::Hard => {
                // Might requeue depending on difficulty settings
                self.config.dynamic_difficulty
            }
            Rating::Good | Rating::Easy => {
                // Don't requeue immediately
                false
            }
        }
    }

    /// Get current session progress
    pub fn get_session_progress(&self) -> LearningSessionProgress {
        LearningSessionProgress {
            total_cards: self.session_stats.total_studied,
            new_cards: self.session_stats.new_cards_studied,
            review_cards: self.session_stats.review_cards_studied,
            retention_rate: self.session_stats.retention_rate,
            remaining_cards: self.get_remaining_card_count(),
            estimated_time_remaining: self.estimate_time_remaining(),
        }
    }

    /// Get remaining card count
    fn get_remaining_card_count(&self) -> usize {
        self.new_cards.len()
            + self.review_cards.len()
            + self.learning_cards.len()
            + self.relearning_cards.len()
    }

    /// Estimate remaining time in minutes
    fn estimate_time_remaining(&self) -> u32 {
        let remaining_cards = self.get_remaining_card_count();
        let avg_time_per_card = if self.session_stats.total_studied > 0 {
            self.session_stats.avg_response_time / 60.0 // Convert to minutes
        } else {
            0.5 // Default estimate: 30 seconds per card
        };

        (remaining_cards as f32 * avg_time_per_card).ceil() as u32
    }

    /// Clear all queues
    fn clear_queues(&mut self) {
        self.new_cards.clear();
        self.review_cards.clear();
        self.learning_cards.clear();
        self.relearning_cards.clear();
        self.priority_scores.clear();
    }

    /// Sort and limit new cards based on priority
    fn sort_and_limit_new_cards(
        &mut self,
        cards: &mut Vec<Card>,
        _now: DateTime<Utc>,
    ) -> Result<()> {
        // Calculate priority scores for new cards
        for card in &*cards {
            let priority = self.calculate_new_card_priority(card);
            self.priority_scores.insert(card.content.id, priority);
        }

        // Sort by priority (higher priority first)
        cards.sort_by(|a, b| {
            let priority_a = self.priority_scores.get(&a.content.id).unwrap_or(&0.0);
            let priority_b = self.priority_scores.get(&b.content.id).unwrap_or(&0.0);
            priority_b
                .partial_cmp(priority_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit to configured maximum
        cards.truncate(self.config.max_new_cards_per_session);

        Ok(())
    }

    /// Sort review cards by priority
    fn sort_review_cards(&mut self, cards: &mut Vec<Card>, _now: DateTime<Utc>) {
        cards.sort_by(|a, b| {
            // Sort by due date first (earlier due first)
            a.state.due.cmp(&b.state.due).then_with(|| {
                // Then by ease factor (harder cards first)
                a.state
                    .ease_factor
                    .partial_cmp(&b.state.ease_factor)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });
    }

    /// Sort learning cards by priority
    fn sort_learning_cards(&mut self, cards: &mut Vec<Card>, _now: DateTime<Utc>) {
        cards.sort_by(|a, b| {
            // Sort by due date first
            a.state.due.cmp(&b.state.due)
        });
    }

    /// Sort relearning cards by priority
    fn sort_relearning_cards(&mut self, cards: &mut Vec<Card>, _now: DateTime<Utc>) {
        cards.sort_by(|a, b| {
            // Sort by due date first
            a.state.due.cmp(&b.state.due)
        });
    }

    /// Calculate priority score for a new card
    fn calculate_new_card_priority(&self, card: &Card) -> f32 {
        let mut priority = 1.0;

        // Consider card tags for priority
        if card.content.tags.contains(&"important".to_string()) {
            priority += 0.5;
        }

        // Consider card creation date (newer cards might get priority)
        let days_since_creation = (Utc::now() - card.content.created_at).num_days();
        if days_since_creation < 7 {
            priority += 0.2; // Recent cards get slight priority
        }

        priority
    }

    /// Get session statistics
    pub fn get_session_stats(&self) -> &SessionStats {
        &self.session_stats
    }

    /// Reset session statistics
    pub fn reset_session(&mut self) {
        self.session_stats = SessionStats::default();
    }

    /// Get configuration
    pub fn get_config(&self) -> &IncrementalLearningConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: IncrementalLearningConfig) {
        self.config = config;
    }
}

/// Result of a card review
#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub rating: Rating,
    pub is_correct: bool,
    pub should_requeue: bool,
    pub session_progress: LearningSessionProgress,
}

/// Learning session progress information
#[derive(Debug, Clone)]
pub struct LearningSessionProgress {
    pub total_cards: usize,
    pub new_cards: usize,
    pub review_cards: usize,
    pub retention_rate: f32,
    pub remaining_cards: usize,
    pub estimated_time_remaining: u32, // in minutes
}

impl LearningSessionProgress {
    /// Get completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.total_cards + self.remaining_cards == 0 {
            100.0
        } else {
            (self.total_cards as f32 / (self.total_cards + self.remaining_cards) as f32) * 100.0
        }
    }

    /// Check if session is complete
    pub fn is_complete(&self) -> bool {
        self.remaining_cards == 0
    }
}

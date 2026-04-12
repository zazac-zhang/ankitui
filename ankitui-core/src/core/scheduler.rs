//! Scheduler - SM-2 Algorithm Implementation
//!
//! Implements the SuperMemo 2 spaced repetition algorithm for card scheduling

use crate::config::ConfigProvider;
use crate::data::models::{Card, CardState};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};

/// Rating quality for card reviews
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
    /// Again (0) - Card review failed
    Again = 0,
    /// Hard (1) - Card review with difficulty
    Hard = 1,
    /// Good (2) - Card review successful
    Good = 2,
    /// Easy (3) - Card review very easy
    Easy = 3,
}

impl Rating {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Rating::Again),
            1 => Some(Rating::Hard),
            2 => Some(Rating::Good),
            3 => Some(Rating::Easy),
            _ => None,
        }
    }
}

/// Card queue types for scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardQueue {
    New,
    Learning,
    Review,
    Relearning,
    Buried,
    Suspended,
}

pub struct Scheduler {
    config_provider: Option<Box<dyn ConfigProvider>>,
}

impl std::fmt::Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scheduler")
            .field("config_provider", &self.config_provider.is_some())
            .finish()
    }
}

impl Clone for Scheduler {
    fn clone(&self) -> Self {
        Self {
            config_provider: None, // Cannot clone ConfigProvider trait object
        }
    }
}

impl Scheduler {
    /// Create a new scheduler with configuration provider
    pub fn new(config_provider: Option<Box<dyn ConfigProvider>>) -> Self {
        Self { config_provider }
    }

    /// Create a scheduler without configuration provider (uses defaults)
    pub fn new_with_defaults() -> Self {
        Self { config_provider: None }
    }

    /// Update configuration provider
    pub fn update_config(&mut self, config_provider: Box<dyn ConfigProvider>) {
        self.config_provider = Some(config_provider);
    }

    /// Get scheduler parameters from config provider or defaults
    fn get_scheduler_params(&self) -> crate::config::SchedulerParams {
        if let Some(ref provider) = self.config_provider {
            provider.get_scheduler_params()
        } else {
            crate::config::SchedulerParams {
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
                hard_factor: 1.2,
                min_review_interval: 1,
                easy_interval_days: 4,
                graduating_interval_days: 1,
                easy_factor: 2.5,
                max_review_interval: 36500,
                learning_steps: vec![1, 6, 10, 14],
                relearning_steps: vec![10],
            }
        }
    }

    /// Get the next card due for review from a deck
    pub fn get_next_card<'a>(&self, cards: &'a [Card], now: DateTime<Utc>) -> Option<&'a Card> {
        cards
            .iter()
            .filter(|card| self.is_card_due(card, now))
            .min_by_key(|card| card.state.due)
    }

    /// Check if a card is due for review
    pub fn is_card_due(&self, card: &Card, now: DateTime<Utc>) -> bool {
        match card.state.state {
            CardState::New => true, // New cards are always available
            CardState::Learning | CardState::Relearning => card.state.due <= now,
            CardState::Review => card.state.due <= now,
            CardState::Buried => false,    // Buried cards are not available
            CardState::Suspended => false, // Suspended cards are never available
        }
    }

    /// Update card state based on user rating
    pub fn update_card(&self, card: &mut Card, rating: Rating, now: DateTime<Utc>) -> Result<()> {
        match rating {
            Rating::Again => self.process_again(card, now),
            Rating::Hard => self.process_hard(card, now),
            Rating::Good => self.process_good(card, now),
            Rating::Easy => self.process_easy(card, now),
        }

        card.state.updated_at = now;
        Ok(())
    }

    /// Get cards due for review within a time window
    pub fn get_due_cards<'a>(&self, cards: &'a [Card], now: DateTime<Utc>, limit: Option<usize>) -> Vec<&'a Card> {
        let mut due_cards: Vec<&'a Card> = cards.iter().filter(|card| self.is_card_due(card, now)).collect();

        // Sort by due date (earliest first)
        due_cards.sort_by_key(|card| card.state.due);

        // Apply limit if specified
        if let Some(limit) = limit {
            due_cards.truncate(limit);
        }

        due_cards
    }

    /// Get cards in specific state
    pub fn get_cards_by_state<'a>(&self, cards: &'a [Card], state: CardState) -> Vec<&'a Card> {
        cards.iter().filter(|card| card.state.state == state).collect()
    }

    /// Convert CardState to CardQueue
    pub fn state_to_queue(state: CardState) -> CardQueue {
        match state {
            CardState::New => CardQueue::New,
            CardState::Learning => CardQueue::Learning,
            CardState::Review => CardQueue::Review,
            CardState::Relearning => CardQueue::Relearning,
            CardState::Buried => CardQueue::Buried,
            CardState::Suspended => CardQueue::Suspended,
        }
    }

    /// Convert CardQueue to CardState
    pub fn queue_to_state(queue: CardQueue) -> CardState {
        match queue {
            CardQueue::New => CardState::New,
            CardQueue::Learning => CardState::Learning,
            CardQueue::Review => CardState::Review,
            CardQueue::Relearning => CardState::Relearning,
            CardQueue::Buried => CardState::Buried,
            CardQueue::Suspended => CardState::Suspended,
        }
    }

    /// Get cards in specific queue
    pub fn get_cards_by_queue<'a>(&self, cards: &'a [Card], queue: CardQueue) -> Vec<&'a Card> {
        cards
            .iter()
            .filter(|card| Self::state_to_queue(card.state.state) == queue)
            .collect()
    }

    /// Check if a card should be included in review (not buried or suspended)
    pub fn is_reviewable(card: &Card) -> bool {
        !matches!(card.state.state, CardState::Buried | CardState::Suspended)
    }

    /// Get daily limits for learning
    pub fn get_daily_limits(&self) -> (i32, i32) {
        if let Some(ref provider) = self.config_provider {
            let limits = provider.get_daily_limits();
            (limits.max_new_cards, limits.max_review_cards)
        } else {
            (20, 200) // Default limits
        }
    }

    /// Calculate next review time based on interval
    fn calculate_next_review(&self, interval_days: i32, now: DateTime<Utc>) -> DateTime<Utc> {
        now + Duration::days(interval_days as i64)
    }

    // SM-2 Algorithm implementation for each rating

    fn process_again(&self, card: &mut Card, now: DateTime<Utc>) {
        card.state.lapses += 1;
        card.state.reps = 0;

        match card.state.state {
            CardState::New | CardState::Review => {
                // Card moves back to learning
                card.state.state = CardState::Learning;
                card.state.interval = 0;
                card.state.due = now + Duration::minutes(1); // Show again in 1 minute
            }
            CardState::Learning | CardState::Relearning => {
                // Reset learning progress
                card.state.interval = 0;
                card.state.due = now + Duration::minutes(1);
            }
            CardState::Buried | CardState::Suspended => {
                // These cards should not be in learning
                // Move them to learning state if they get an "Again" rating
                card.state.state = CardState::Learning;
                card.state.interval = 0;
                card.state.due = now + Duration::minutes(1);
            }
        }

        // Reduce ease factor
        card.state.ease_factor = (card.state.ease_factor - 0.2).max(self.get_scheduler_params().min_ease_factor);
    }

    fn process_hard(&self, card: &mut Card, now: DateTime<Utc>) {
        card.state.reps += 1;
        let params = self.get_scheduler_params();

        match card.state.state {
            CardState::New => {
                card.state.state = CardState::Learning;
                card.state.interval = 0;
                // Use first learning step for hard new cards
                let learning_steps = self.get_learning_steps(&params);
                if let Some(first_step) = learning_steps.first() {
                    card.state.due = now + Duration::minutes(*first_step as i64);
                } else {
                    card.state.due = now + Duration::minutes(1);
                }
            }
            CardState::Learning | CardState::Relearning => {
                // For learning cards, extend current interval or use next step
                let learning_steps = if card.state.state == CardState::Learning {
                    self.get_learning_steps(&params)
                } else {
                    self.get_relearning_steps(&params)
                };

                // Check if we should extend current learning or graduate
                if (card.state.reps as usize) < learning_steps.len() {
                    // Stay in learning phase, but extend interval with hard multiplier
                    if let Some(current_step) = learning_steps.get((card.state.reps - 1) as usize) {
                        let extended_interval = (*current_step as f32 * params.hard_multiplier) as i32;
                        card.state.due = now + Duration::minutes(extended_interval as i64);
                    } else {
                        // Graduate if we've run out of steps
                        self.graduate_card(card, now, &params);
                    }
                } else {
                    // Graduate to review with hard penalty
                    self.graduate_card(card, now, &params);
                    // Apply hard factor to the graduated interval
                    card.state.interval = (card.state.interval as f32 * params.hard_factor) as i32;
                    card.state.due = self.calculate_next_review(card.state.interval, now);
                }
            }
            CardState::Review => {
                // For review cards, decrease ease factor and apply hard multiplier
                card.state.ease_factor = (card.state.ease_factor - 0.15).max(params.min_ease_factor);
                card.state.interval = ((card.state.interval as f32) * params.hard_factor) as i32;
                card.state.interval = card.state.interval.max(params.min_review_interval);
                card.state.due = self.calculate_next_review(card.state.interval, now);
            }
            CardState::Buried | CardState::Suspended => {
                // These cards should not be in learning
                // Move them to learning state if they get a "Hard" rating
                card.state.state = CardState::Learning;
                card.state.interval = 1;
                card.state.due = now + Duration::days(1);
            }
        }
    }

    fn process_good(&self, card: &mut Card, now: DateTime<Utc>) {
        card.state.reps += 1;

        match card.state.state {
            CardState::New => {
                card.state.state = CardState::Learning;
                card.state.interval = 0;
                // Use first learning step
                let params = self.get_scheduler_params();
                let learning_steps = self.get_learning_steps(&params);
                if let Some(first_step) = learning_steps.first() {
                    card.state.due = now + Duration::minutes(*first_step as i64);
                } else {
                    card.state.due = now + Duration::minutes(1);
                }
            }
            CardState::Learning | CardState::Relearning => {
                let params = self.get_scheduler_params();
                let learning_steps = if card.state.state == CardState::Learning {
                    self.get_learning_steps(&params)
                } else {
                    self.get_relearning_steps(&params)
                };

                // Check if we should graduate to review
                if (card.state.reps as usize) < learning_steps.len() {
                    // Still in learning phase, use next step
                    if let Some(step) = learning_steps.get(card.state.reps as usize) {
                        card.state.interval = 0; // Learning cards have interval 0
                        card.state.due = now + Duration::minutes(*step as i64);
                    } else {
                        // Graduate to review
                        self.graduate_card(card, now, &params);
                    }
                } else {
                    // Graduate to review
                    self.graduate_card(card, now, &params);
                }
            }
            CardState::Review => {
                // Standard SM-2 interval calculation for review cards
                if card.state.interval == 0 {
                    card.state.interval = 1;
                } else {
                    card.state.interval = (card.state.interval as f32 * card.state.ease_factor) as i32;
                }
                card.state.due = self.calculate_next_review(card.state.interval, now);
            }
            CardState::Buried | CardState::Suspended => {
                // These cards should not be in learning
                // Move them to learning state if they get a "Good" rating
                card.state.state = CardState::Learning;
                card.state.interval = 0;
                card.state.due = now + Duration::minutes(1);
            }
        }
    }

    fn process_easy(&self, card: &mut Card, now: DateTime<Utc>) {
        card.state.reps += 1;
        let params = self.get_scheduler_params();

        match card.state.state {
            CardState::New => {
                card.state.state = CardState::Review;
                // Use easy_interval_days if configured, otherwise fall back to easy_interval
                let easy_interval = if params.easy_interval_days > 0 {
                    params.easy_interval_days
                } else {
                    params.easy_interval
                };
                card.state.interval = easy_interval;
                card.state.due = self.calculate_next_review(card.state.interval, now);

                // Increase ease factor for easy reviews using easy_factor
                card.state.ease_factor = (card.state.ease_factor + 0.15).min(params.max_ease_factor);
            }
            CardState::Learning | CardState::Relearning => {
                // Graduate directly to review with maximum bonus
                card.state.state = CardState::Review;

                // Start with graduating interval and apply maximum bonuses
                let base_interval = if params.graduating_interval_days > 0 {
                    params.graduating_interval_days
                } else {
                    params.graduating_interval
                };

                // Apply interval modifier, easy bonus, and an additional easy graduation bonus
                let easy_graduation_interval =
                    ((base_interval as f32) * params.interval_modifier * params.easy_bonus * params.easy_factor) as i32;

                // Ensure minimum easy interval but respect maximum
                card.state.interval = easy_graduation_interval
                    .max(params.easy_interval)
                    .min(params.max_review_interval);

                card.state.due = self.calculate_next_review(card.state.interval, now);

                // Increase ease factor
                card.state.ease_factor += 0.15;
            }
            CardState::Review => {
                // Easy bonus for review cards
                let params = self.get_scheduler_params();
                card.state.interval = (card.state.interval as f32 * card.state.ease_factor * params.easy_bonus) as i32;
                card.state.due = self.calculate_next_review(card.state.interval, now);

                // Increase ease factor
                card.state.ease_factor += 0.15;
            }
            CardState::Buried | CardState::Suspended => {
                // These cards should not be in learning
                // Move them directly to review state if they get an "Easy" rating
                card.state.state = CardState::Review;
                card.state.interval = self.get_scheduler_params().easy_interval;
                card.state.due = self.calculate_next_review(card.state.interval, now);
                card.state.ease_factor += 0.15;
            }
        }
    }

    /// Schedule a card with rating and return updated card
    pub fn schedule_card(&self, card: &Card, rating: Rating) -> Card {
        let mut updated_card = card.clone();
        let now = Utc::now();

        // Use existing update_card logic
        let _ = self.update_card(&mut updated_card, rating, now);

        updated_card
    }

    /// Get learning steps from configuration
    fn get_learning_steps(&self, params: &crate::config::SchedulerParams) -> Vec<i32> {
        // Return learning steps from configuration, or defaults if empty
        if params.learning_steps.is_empty() {
            vec![1, 6, 10, 14] // Default: 1min, 6min, 10min, 14min
        } else {
            params.learning_steps.clone()
        }
    }

    /// Get relearning steps from configuration
    fn get_relearning_steps(&self, params: &crate::config::SchedulerParams) -> Vec<i32> {
        // Return relearning steps from configuration, or defaults if empty
        if params.relearning_steps.is_empty() {
            vec![10] // Default: 10min
        } else {
            params.relearning_steps.clone()
        }
    }

    /// Graduate a card from learning to review state
    fn graduate_card(&self, card: &mut Card, now: DateTime<Utc>, params: &crate::config::SchedulerParams) {
        card.state.state = CardState::Review;

        // Set initial interval based on graduation settings
        // Use graduating_interval_days from config for more intuitive configuration
        let base_interval = if params.graduating_interval_days > 0 {
            params.graduating_interval_days
        } else {
            params.graduating_interval
        };

        // Apply interval modifier for fine-tuning
        card.state.interval = (base_interval as f32 * params.interval_modifier).max(1.0) as i32;

        // Apply easy bonus if this was an easy graduation (few repetitions)
        if card.state.reps <= 2 {
            card.state.interval = ((card.state.interval as f32) * params.easy_bonus) as i32;
        }

        // Ensure minimum interval for review cards
        card.state.interval = card.state.interval.max(params.min_review_interval.max(1));

        // Calculate next due date
        card.state.due = self.calculate_next_review(card.state.interval, now);
    }

    /// Reset a card back to New state
    pub fn reset_card(&self, card: &Card) -> Card {
        let mut updated_card = card.clone();
        let now = Utc::now();

        // Reset card to initial state
        updated_card.state.state = CardState::New;
        updated_card.state.interval = 0;
        updated_card.state.reps = 0;
        updated_card.state.lapses = 0;
        updated_card.state.ease_factor = self.get_scheduler_params().starting_ease_factor;
        updated_card.state.due = now;
        updated_card.state.updated_at = now;

        updated_card
    }

    /// Get daily statistics for a deck
    pub fn get_daily_stats(&self, cards: &[Card], now: DateTime<Utc>) -> DailyStats {
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let tomorrow_start = today_start + Duration::days(1);

        let mut new_count = 0;
        let mut learning_count = 0;
        let mut review_count = 0;
        let mut due_today = 0;
        let mut due_tomorrow = 0;

        for card in cards {
            if self.is_card_due(card, now) {
                due_today += 1;
            }

            // Count cards due tomorrow
            if card.state.due >= tomorrow_start && card.state.due < tomorrow_start + Duration::days(1) {
                due_tomorrow += 1;
            }

            match card.state.state {
                CardState::New => new_count += 1,
                CardState::Learning | CardState::Relearning => learning_count += 1,
                CardState::Review => review_count += 1,
                CardState::Buried | CardState::Suspended => {
                    // These cards are not active in learning
                }
            }
        }

        DailyStats {
            new_cards: new_count,
            learning_cards: learning_count,
            review_cards: review_count,
            due_today,
            due_tomorrow,
        }
    }
}

/// Daily statistics for a deck
#[derive(Debug, Clone)]
pub struct DailyStats {
    pub new_cards: i32,
    pub learning_cards: i32,
    pub review_cards: i32,
    pub due_today: i32,
    pub due_tomorrow: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::models::{CardContent, CardStateData};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_card() -> Card {
        let content = CardContent {
            id: Uuid::new_v4(),
            front: "Question".to_string(),
            back: "Answer".to_string(),
            tags: vec![],
            media: None,
            custom: std::collections::HashMap::new(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        };

        Card::new(content)
    }

    #[test]
    fn test_new_card_scheduling() {
        let scheduler = Scheduler::new(None);
        let mut card = create_test_card();
        let now = Utc::now();

        // Test Again rating on new card
        scheduler.update_card(&mut card, Rating::Again, now).unwrap();
        assert_eq!(card.state.state, CardState::Learning);
        assert_eq!(card.state.interval, 0);
        assert!(card.state.due > now);
        assert!(card.state.due <= now + Duration::minutes(2));

        // Test Good rating on new card
        let mut card = create_test_card();
        scheduler.update_card(&mut card, Rating::Good, now).unwrap();
        assert_eq!(card.state.state, CardState::Learning);
        assert_eq!(card.state.interval, 1);
        assert!(card.state.due >= now + Duration::days(1));
    }

    #[test]
    fn test_ease_factor_adjustment() {
        let scheduler = Scheduler::new(None);
        let mut card = create_test_card();
        let now = Utc::now();

        let initial_ease = card.state.ease_factor;

        // Test Again reduces ease factor
        scheduler.update_card(&mut card, Rating::Again, now).unwrap();
        assert!(card.state.ease_factor < initial_ease);

        // Test Easy increases ease factor
        let mut card = create_test_card();
        scheduler.update_card(&mut card, Rating::Easy, now).unwrap();
        assert!(card.state.ease_factor > initial_ease);
    }

    #[test]
    fn test_learning_progression() {
        let scheduler = Scheduler::new(None);
        let mut card = create_test_card();
        let now = Utc::now();

        // Move to learning state
        scheduler.update_card(&mut card, Rating::Good, now).unwrap();
        assert_eq!(card.state.state, CardState::Learning);
        assert_eq!(card.state.reps, 1);

        // Second good review
        scheduler.update_card(&mut card, Rating::Good, now).unwrap();
        assert_eq!(card.state.reps, 2);

        // Third good review should graduate to review
        scheduler.update_card(&mut card, Rating::Good, now).unwrap();
        assert_eq!(card.state.state, CardState::Review);
        assert!(card.state.interval > 1);
    }

    #[test]
    fn test_daily_stats() {
        let scheduler = Scheduler::new(None);
        let now = Utc::now();

        // Create cards in different states
        let cards = vec![
            create_test_card(), // New
            create_test_card(), // New
        ];

        let stats = scheduler.get_daily_stats(&cards, now);
        assert_eq!(stats.new_cards, 2);
        assert_eq!(stats.learning_cards, 0);
        assert_eq!(stats.review_cards, 0);
        assert!(stats.due_today >= 2); // New cards are always due
    }
}

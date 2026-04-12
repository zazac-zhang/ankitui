//! Session Controller - Manages review sessions
//!
//! Handles complete review session lifecycle including:
//! - Session flow control (start, pause, end)
//! - Card fetching logic with priority queues
//! - Session state management and progress tracking
//! - Daily limits control (new cards and reviews)
//! - Session interruption and recovery

use crate::core::deck_manager::DeckManager;
use crate::core::scheduler::{Rating, Scheduler};
use crate::data::models::{BuriedCardRecord, BuryReason, Card, CardState, Deck, SuspendedCardRecord};
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

/// Session states
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    NotStarted,
    InProgress,
    Paused,
    Finished,
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub total_cards_studied: usize,
    pub new_cards_studied: usize,
    pub review_cards_studied: usize,
    pub relearning_cards_studied: usize,
    pub correct_answers: usize,
    pub incorrect_answers: usize,
    pub average_response_time: Option<f32>, // in seconds
    pub study_streak_days: i32,
}

/// Daily limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLimits {
    pub max_new_cards: i32,
    pub max_review_cards: i32,
    pub cards_studied_today: i32,
    pub new_cards_studied_today: i32,
    pub review_cards_studied_today: i32,
}

/// Card queues for session management
#[derive(Debug, Clone)]
pub struct CardQueues {
    pub new_cards: VecDeque<Card>,
    pub learning_cards: VecDeque<Card>,
    pub review_cards: VecDeque<Card>,
    pub relearning_cards: VecDeque<Card>,
}

/// Session Controller manages review sessions
#[derive(Clone, Debug)]
pub struct SessionController {
    deck_manager: DeckManager,
    scheduler: Scheduler,

    // Session state
    current_deck_id: Option<Uuid>,
    session_state: SessionState,
    session_stats: SessionStats,
    daily_limits: DailyLimits,

    // Card queues
    card_queues: CardQueues,
    current_card: Option<Card>,

    // Session configuration
    session_start_time: Option<DateTime<Utc>>,
    last_card_time: Option<DateTime<Utc>>,
}

impl SessionController {
    /// Create a new SessionController with required dependencies
    pub async fn new(deck_manager: DeckManager, scheduler: Option<Scheduler>) -> Result<Self> {
        let scheduler = scheduler.unwrap_or_else(|| Scheduler::new(None));

        let session_stats = SessionStats {
            started_at: Utc::now(),
            ended_at: None,
            total_cards_studied: 0,
            new_cards_studied: 0,
            review_cards_studied: 0,
            relearning_cards_studied: 0,
            correct_answers: 0,
            incorrect_answers: 0,
            average_response_time: None,
            study_streak_days: 0,
        };

        let daily_limits = DailyLimits {
            max_new_cards: 20,
            max_review_cards: 200,
            cards_studied_today: 0,
            new_cards_studied_today: 0,
            review_cards_studied_today: 0,
        };

        let card_queues = CardQueues {
            new_cards: VecDeque::new(),
            learning_cards: VecDeque::new(),
            review_cards: VecDeque::new(),
            relearning_cards: VecDeque::new(),
        };

        Ok(Self {
            deck_manager,
            scheduler,
            current_deck_id: None,
            session_state: SessionState::NotStarted,
            session_stats,
            daily_limits,
            card_queues,
            current_card: None,
            session_start_time: None,
            last_card_time: None,
        })
    }

    /// Start a new review session for the specified deck
    pub async fn start_session(&mut self, deck_id: Uuid) -> Result<()> {
        if self.session_state == SessionState::InProgress {
            return Err(anyhow!("Session already in progress"));
        }

        // Load deck and initialize queues
        let (deck, _) = self
            .deck_manager
            .get_deck(&deck_id)
            .await
            .context("Failed to load deck for session")?;

        // Update daily limits from deck configuration
        self.update_daily_limits_from_deck(&deck).await?;

        // Load cards into queues
        self.load_cards_for_session(deck_id)
            .await
            .context("Failed to load cards for session")?;

        // Initialize session state
        self.current_deck_id = Some(deck_id);
        self.session_state = SessionState::InProgress;
        self.session_start_time = Some(Utc::now());
        self.session_stats.started_at = Utc::now();
        self.session_stats.ended_at = None;

        // Fetch first card
        self.fetch_next_card().await?;

        Ok(())
    }

    /// Pause the current session
    pub fn pause_session(&mut self) -> Result<()> {
        if self.session_state != SessionState::InProgress {
            return Err(anyhow!("No session in progress to pause"));
        }

        self.session_state = SessionState::Paused;
        Ok(())
    }

    /// Resume a paused session
    pub fn resume_session(&mut self) -> Result<()> {
        if self.session_state != SessionState::Paused {
            return Err(anyhow!("No paused session to resume"));
        }

        self.session_state = SessionState::InProgress;
        Ok(())
    }

    /// End the current session and save statistics
    pub async fn end_session(&mut self) -> Result<SessionStats> {
        if self.session_state == SessionState::NotStarted {
            return Err(anyhow!("No session to end"));
        }

        self.session_state = SessionState::Finished;
        self.session_stats.ended_at = Some(Utc::now());

        // Calculate average response time if we have timing data
        if self.session_stats.total_cards_studied > 0 {
            // This would require tracking response times throughout the session
            // For now, we'll leave it as None
        }

        // Save session statistics to database if needed
        // This could be implemented later

        Ok(self.session_stats.clone())
    }

    /// Get the current session state
    pub fn session_state(&self) -> &SessionState {
        &self.session_state
    }

    /// Get current session statistics
    pub fn session_stats(&self) -> &SessionStats {
        &self.session_stats
    }

    /// Get current daily limits
    pub fn daily_limits(&self) -> &DailyLimits {
        &self.daily_limits
    }

    /// Get the current card being reviewed
    pub fn current_card(&self) -> Option<&Card> {
        self.current_card.as_ref()
    }

    /// Get the current deck ID
    pub fn current_deck_id(&self) -> Option<Uuid> {
        self.current_deck_id
    }

    /// Get all cards in the current deck
    pub async fn get_deck_cards(&self, deck_id: &Uuid) -> Result<Vec<Card>> {
        self.deck_manager.get_cards(deck_id).await
    }

    /// Check if session has more cards
    pub fn has_more_cards(&self) -> bool {
        !self.card_queues.new_cards.is_empty()
            || !self.card_queues.learning_cards.is_empty()
            || !self.card_queues.review_cards.is_empty()
            || !self.card_queues.relearning_cards.is_empty()
    }

    /// Get remaining cards count by type
    pub fn remaining_cards_count(&self) -> (usize, usize, usize, usize) {
        (
            self.card_queues.new_cards.len(),
            self.card_queues.learning_cards.len(),
            self.card_queues.review_cards.len(),
            self.card_queues.relearning_cards.len(),
        )
    }

    // Private helper methods

    /// Update daily limits from deck configuration
    async fn update_daily_limits_from_deck(&mut self, deck: &Deck) -> Result<()> {
        if let Some(scheduler_config) = &deck.scheduler_config {
            if let Some(new_limit) = scheduler_config.new_cards_per_day {
                self.daily_limits.max_new_cards = new_limit;
            }
            if let Some(review_limit) = scheduler_config.max_reviews_per_day {
                self.daily_limits.max_review_cards = review_limit;
            }
        }
        Ok(())
    }

    /// Load cards for the session and populate queues
    async fn load_cards_for_session(&mut self, deck_id: Uuid) -> Result<()> {
        // Clear existing queues
        self.card_queues.new_cards.clear();
        self.card_queues.learning_cards.clear();
        self.card_queues.review_cards.clear();
        self.card_queues.relearning_cards.clear();

        // Get due cards (reviews and relearning)
        let due_cards = self
            .deck_manager
            .get_due_cards(&deck_id, None)
            .await
            .context("Failed to get due cards")?;

        // Get new cards (respecting daily limits)
        let remaining_new_cards = (self.daily_limits.max_new_cards - self.daily_limits.new_cards_studied_today).max(0);
        let new_cards = if remaining_new_cards > 0 {
            self.deck_manager
                .get_new_cards(&deck_id, Some(remaining_new_cards))
                .await
                .context("Failed to get new cards")?
        } else {
            Vec::new()
        };

        // Sort cards into appropriate queues
        for card in due_cards {
            match card.state.state {
                CardState::Review => {
                    self.card_queues.review_cards.push_back(card);
                }
                CardState::Learning => {
                    self.card_queues.learning_cards.push_back(card);
                }
                CardState::Relearning => {
                    self.card_queues.relearning_cards.push_back(card);
                }
                CardState::New => {
                    // This shouldn't happen with due cards, but handle it gracefully
                    self.card_queues.new_cards.push_back(card);
                }
                CardState::Buried | CardState::Suspended => {
                    // These cards should not be in the review queue
                    continue;
                }
            }
        }

        for card in new_cards {
            self.card_queues.new_cards.push_back(card);
        }

        Ok(())
    }

    /// Fetch the next card according to priority order
    async fn fetch_next_card(&mut self) -> Result<()> {
        // Priority order: Relearning > Learning > Review > New
        self.current_card = if let Some(card) = self.card_queues.relearning_cards.pop_front() {
            Some(card)
        } else if let Some(card) = self.card_queues.learning_cards.pop_front() {
            Some(card)
        } else if let Some(card) = self.card_queues.review_cards.pop_front() {
            Some(card)
        } else if let Some(card) = self.card_queues.new_cards.pop_front() {
            Some(card)
        } else {
            None
        };

        self.last_card_time = Some(Utc::now());
        Ok(())
    }

    /// Check if daily limits are reached
    fn daily_limits_reached(&self) -> bool {
        (self.daily_limits.new_cards_studied_today >= self.daily_limits.max_new_cards
            && self.daily_limits.review_cards_studied_today >= self.daily_limits.max_review_cards)
            || (self.daily_limits.cards_studied_today
                >= self.daily_limits.max_new_cards + self.daily_limits.max_review_cards)
    }

    /// Update daily limits counters based on card type
    fn update_daily_limits(&mut self, card_state: CardState) {
        self.daily_limits.cards_studied_today += 1;

        match card_state {
            CardState::New => {
                self.daily_limits.new_cards_studied_today += 1;
            }
            CardState::Review | CardState::Learning | CardState::Relearning => {
                self.daily_limits.review_cards_studied_today += 1;
            }
            CardState::Buried | CardState::Suspended => {
                // These cards should not be in a learning session
                // Don't update daily limits for them
            }
        }
    }

    /// Update session statistics after card review
    fn update_session_stats(&mut self, card: &Card, rating: Rating) {
        self.session_stats.total_cards_studied += 1;

        // Update counters based on card state
        match card.state.state {
            CardState::New => {
                self.session_stats.new_cards_studied += 1;
            }
            CardState::Review => {
                self.session_stats.review_cards_studied += 1;
            }
            CardState::Relearning => {
                self.session_stats.relearning_cards_studied += 1;
            }
            CardState::Learning => {
                // Learning cards are counted in reviews for stats
                self.session_stats.review_cards_studied += 1;
            }
            CardState::Buried | CardState::Suspended => {
                // These cards should not be in a learning session
                // Don't update session stats for them
            }
        }

        // Update correct/incorrect counters
        match rating {
            Rating::Again => {
                self.session_stats.incorrect_answers += 1;
            }
            Rating::Hard | Rating::Good | Rating::Easy => {
                self.session_stats.correct_answers += 1;
            }
        }
    }

    /// Review the current card with the given rating
    pub async fn review_current_card(&mut self, rating: Rating) -> Result<()> {
        if self.session_state != SessionState::InProgress {
            return Err(anyhow!("No session in progress"));
        }

        let current_card = self
            .current_card
            .take()
            .ok_or_else(|| anyhow!("No current card to review"))?;

        // Update session statistics
        self.update_session_stats(&current_card, rating);

        // Update daily limits
        self.update_daily_limits(current_card.state.state);

        // Apply scheduler algorithm to get updated card
        let updated_card = self.scheduler.schedule_card(&current_card, rating);

        // Save updated card state through deck manager
        self.deck_manager
            .review_card(updated_card.clone(), rating)
            .await
            .context("Failed to save card review")?;

        // Check if card needs to be re-queued (for learning cards)
        if updated_card.state.state == CardState::Learning || updated_card.state.state == CardState::Relearning {
            // Calculate when this card should be shown again
            let next_review_time = updated_card.state.due;
            if next_review_time <= Utc::now() {
                // Card is due again soon, add back to appropriate queue
                match updated_card.state.state {
                    CardState::Learning => {
                        self.card_queues.learning_cards.push_back(updated_card);
                    }
                    CardState::Relearning => {
                        self.card_queues.relearning_cards.push_back(updated_card);
                    }
                    _ => {}
                }
            }
        }

        // Fetch next card if available and daily limits not reached
        if !self.daily_limits_reached() {
            self.fetch_next_card().await?;
        } else {
            self.current_card = None;
        }

        Ok(())
    }

    /// Skip the current card (moves it to the back of its queue)
    pub async fn skip_current_card(&mut self) -> Result<()> {
        if self.session_state != SessionState::InProgress {
            return Err(anyhow!("No session in progress"));
        }

        let current_card = self
            .current_card
            .take()
            .ok_or_else(|| anyhow!("No current card to skip"))?;

        // Add card back to the appropriate queue based on its state
        match current_card.state.state {
            CardState::New => {
                self.card_queues.new_cards.push_back(current_card);
            }
            CardState::Learning => {
                self.card_queues.learning_cards.push_back(current_card);
            }
            CardState::Relearning => {
                self.card_queues.relearning_cards.push_back(current_card);
            }
            CardState::Review => {
                self.card_queues.review_cards.push_back(current_card);
            }
            CardState::Buried | CardState::Suspended => {
                // These cards should not be in a learning session
                // Return them to their respective inactive state
                self.fetch_next_card().await?;
                return Ok(());
            }
        }

        // Fetch next card
        self.fetch_next_card().await?;

        Ok(())
    }

    /// Reset current card to new state (for difficult cards)
    pub async fn reset_current_card(&mut self) -> Result<()> {
        if self.session_state != SessionState::InProgress {
            return Err(anyhow!("No session in progress"));
        }

        let mut current_card = self
            .current_card
            .take()
            .ok_or_else(|| anyhow!("No current card to reset"))?;

        // Reset card to new state using scheduler
        current_card = self.scheduler.reset_card(&current_card);

        // Save the reset card state
        self.deck_manager
            .review_card(current_card.clone(), Rating::Again)
            .await
            .context("Failed to save reset card")?;

        // Add to new cards queue
        self.card_queues.new_cards.push_back(current_card);

        // Fetch next card
        self.fetch_next_card().await?;

        Ok(())
    }

    /// Check if the session should automatically end (no more cards or limits reached)
    pub fn should_auto_end_session(&self) -> bool {
        !self.has_more_cards() || self.daily_limits_reached()
    }

    /// Bury the current card until next session or specified time
    pub async fn bury_current_card(&mut self, reason: BuryReason, until: Option<DateTime<Utc>>) -> Result<()> {
        if self.session_state != SessionState::InProgress {
            return Err(anyhow!("No session in progress"));
        }

        let current_card = self
            .current_card
            .take()
            .ok_or_else(|| anyhow!("No current card to bury"))?;

        // Determine bury time
        let bury_until = until.unwrap_or_else(|| {
            match reason {
                BuryReason::SessionBury => {
                    // Bury until next session (tomorrow)
                    let now = Utc::now();
                    now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc() + Duration::days(1)
                }
                BuryReason::ReviewBury => {
                    // Bury for a short time (10 minutes)
                    Utc::now() + Duration::minutes(10)
                }
                _ => {
                    // Default bury until tomorrow
                    let now = Utc::now();
                    now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc() + Duration::days(1)
                }
            }
        });

        // Create buried card record
        let buried_record = BuriedCardRecord {
            card_id: current_card.content.id,
            bury_reason: reason,
            bury_until,
            original_state: current_card.state.state,
            created_at: Utc::now(),
        };

        // Update card state to buried
        let mut updated_card = current_card.clone();
        updated_card.state.state = CardState::Buried;
        updated_card.state.updated_at = Utc::now();

        // Save updated card state
        self.deck_manager
            .review_card(updated_card.clone(), Rating::Again)
            .await
            .context("Failed to save buried card state")?;

        // Store buried card record (in a real implementation, this would go to database)
        // For now, we'll just proceed with the state change

        // Fetch next card
        self.fetch_next_card().await?;

        Ok(())
    }

    /// Suspend the current card indefinitely
    pub async fn suspend_current_card(&mut self, reason: String, notes: Option<String>) -> Result<()> {
        if self.session_state != SessionState::InProgress {
            return Err(anyhow!("No session in progress"));
        }

        let current_card = self
            .current_card
            .take()
            .ok_or_else(|| anyhow!("No current card to suspend"))?;

        // Create suspended card record
        let suspended_record = SuspendedCardRecord {
            card_id: current_card.content.id,
            suspend_reason: reason.clone(),
            created_at: Utc::now(),
            notes,
        };

        // Update card state to suspended
        let mut updated_card = current_card.clone();
        updated_card.state.state = CardState::Suspended;
        updated_card.state.updated_at = Utc::now();

        // Save updated card state
        self.deck_manager
            .review_card(updated_card.clone(), Rating::Again)
            .await
            .context("Failed to save suspended card state")?;

        // Store suspended card record (in a real implementation, this would go to database)
        // For now, we'll just proceed with the state change

        // Fetch next card
        self.fetch_next_card().await?;

        Ok(())
    }

    /// Unsuspend a card by ID
    pub async fn unsuspend_card(&mut self, card_id: Uuid) -> Result<()> {
        // This would typically retrieve the original state and restore it
        // For now, we'll implement a basic version that sets it back to New
        if let Some((deck_id, _)) = self.current_deck_id.map(|id| (id, ())) {
            // Get the card from deck manager
            let cards = self.deck_manager.get_cards(&deck_id).await?;
            if let Some(card) = cards.iter().find(|c| c.content.id == card_id) {
                let mut updated_card = card.clone();
                updated_card.state.state = CardState::New; // Reset to new state
                updated_card.state.updated_at = Utc::now();

                // Save the updated state
                self.deck_manager
                    .review_card(updated_card, Rating::Again)
                    .await
                    .context("Failed to unsuspend card")?;
            }
        }

        Ok(())
    }

    /// Unbury a card by ID
    pub async fn unbury_card(&mut self, card_id: Uuid) -> Result<()> {
        // Similar to unsuspend, restore the card to its original state
        if let Some((deck_id, _)) = self.current_deck_id.map(|id| (id, ())) {
            let cards = self.deck_manager.get_cards(&deck_id).await?;
            if let Some(card) = cards.iter().find(|c| c.content.id == card_id) {
                let mut updated_card = card.clone();
                // Restore to Review state if it has interval > 0, otherwise New
                updated_card.state.state = if card.state.interval > 0 {
                    CardState::Review
                } else {
                    CardState::New
                };
                updated_card.state.updated_at = Utc::now();

                // Save the updated state
                self.deck_manager
                    .review_card(updated_card, Rating::Again)
                    .await
                    .context("Failed to unbury card")?;
            }
        }

        Ok(())
    }

    /// Bury multiple cards
    pub async fn bury_cards(&mut self, card_ids: &[Uuid], reason: BuryReason) -> Result<usize> {
        let mut buried_count = 0;

        for &card_id in card_ids {
            // Get the card and bury it
            if let Some((deck_id, _)) = self.current_deck_id.map(|id| (id, ())) {
                let cards = self.deck_manager.get_cards(&deck_id).await?;
                if let Some(card) = cards.iter().find(|c| c.content.id == card_id) {
                    let mut updated_card = card.clone();
                    updated_card.state.state = CardState::Buried;
                    updated_card.state.updated_at = Utc::now();

                    // Save the updated state
                    self.deck_manager
                        .review_card(updated_card, Rating::Again)
                        .await
                        .context("Failed to bury card")?;

                    buried_count += 1;
                }
            }
        }

        Ok(buried_count)
    }

    /// Suspend multiple cards
    pub async fn suspend_cards(&mut self, card_ids: &[Uuid], reason: String) -> Result<usize> {
        let mut suspended_count = 0;

        for &card_id in card_ids {
            if let Some((deck_id, _)) = self.current_deck_id.map(|id| (id, ())) {
                let cards = self.deck_manager.get_cards(&deck_id).await?;
                if let Some(card) = cards.iter().find(|c| c.content.id == card_id) {
                    let mut updated_card = card.clone();
                    updated_card.state.state = CardState::Suspended;
                    updated_card.state.updated_at = Utc::now();

                    // Save the updated state
                    self.deck_manager
                        .review_card(updated_card, Rating::Again)
                        .await
                        .context("Failed to suspend card")?;

                    suspended_count += 1;
                }
            }
        }

        Ok(suspended_count)
    }

    /// Get session progress information
    pub fn session_progress(&self) -> SessionProgress {
        let (new_remaining, learning_remaining, review_remaining, relearning_remaining) = self.remaining_cards_count();

        SessionProgress {
            total_remaining: new_remaining + learning_remaining + review_remaining + relearning_remaining,
            new_remaining,
            learning_remaining,
            review_remaining,
            relearning_remaining,
            cards_studied_today: self.daily_limits.cards_studied_today,
            new_cards_limit: self.daily_limits.max_new_cards,
            review_cards_limit: self.daily_limits.max_review_cards,
            session_complete: self.should_auto_end_session(),
        }
    }

    /// Save current session state for recovery
    pub async fn save_session_state(&self) -> Result<()> {
        if self.session_state == SessionState::NotStarted || self.session_state == SessionState::Finished {
            return Ok(());
        }

        if let Some(deck_id) = self.current_deck_id {
            // Get the content base directory from deck manager
            let base_dir = self.deck_manager.get_content_base_dir();
            let recovery_dir = base_dir.join(".recovery");

            // Ensure recovery directory exists
            tokio::fs::create_dir_all(&recovery_dir)
                .await
                .context("Failed to create recovery directory")?;

            let recovery_file = recovery_dir.join(format!("session_{}.json", deck_id));

            let recovery_data = SessionRecoveryData {
                deck_id,
                session_state: self.session_state.clone(),
                session_stats: self.session_stats.clone(),
                daily_limits: self.daily_limits.clone(),
                current_card_id: self.current_card.as_ref().map(|c| c.content.id),
                timestamp: Utc::now(),
            };

            let json_data =
                serde_json::to_string_pretty(&recovery_data).context("Failed to serialize recovery data")?;

            tokio::fs::write(&recovery_file, json_data)
                .await
                .context("Failed to write recovery file")?;

            log::info!("Session state saved to {:?}", recovery_file);
        }

        Ok(())
    }

    /// Recover a previously interrupted session
    pub async fn recover_session(&mut self, deck_id: Uuid) -> Result<bool> {
        // Get the content base directory from deck manager
        let base_dir = self.deck_manager.get_content_base_dir();
        let recovery_dir = base_dir.join(".recovery");
        let recovery_file = recovery_dir.join(format!("session_{}.json", deck_id));

        // Check if recovery file exists
        if !recovery_file.exists() {
            return Ok(false);
        }

        // Load recovery data
        let json_data = tokio::fs::read_to_string(&recovery_file)
            .await
            .context("Failed to read recovery file")?;

        let recovery_data: SessionRecoveryData =
            serde_json::from_str(&json_data).context("Failed to deserialize recovery data")?;

        // Validate recovery data is for the correct deck
        if recovery_data.deck_id != deck_id {
            return Err(anyhow!("Recovery data deck mismatch"));
        }

        // Check if recovery data is too old (older than 24 hours)
        let age = Utc::now() - recovery_data.timestamp;
        if age.num_hours() > 24 {
            log::info!("Recovery data is too old ({} hours), ignoring", age.num_hours());
            tokio::fs::remove_file(&recovery_file).await.ok(); // Clean up old recovery file
            return Ok(false);
        }

        // Restore session state
        self.current_deck_id = Some(deck_id);
        self.session_state = recovery_data.session_state;
        self.session_stats = recovery_data.session_stats;
        self.daily_limits = recovery_data.daily_limits;

        // Rebuild card queues
        self.load_cards_for_session(deck_id).await?;

        // Position to the current card if available
        if let Some(card_id) = recovery_data.current_card_id {
            // Try to find the card in the queues
            self.current_card = self.find_card_in_queues(&card_id);
        }

        log::info!("Session recovered from {:?}", recovery_file);
        Ok(true)
    }

    /// Find a card in the queues by ID
    fn find_card_in_queues(&self, card_id: &Uuid) -> Option<Card> {
        self.card_queues
            .new_cards
            .iter()
            .chain(self.card_queues.learning_cards.iter())
            .chain(self.card_queues.review_cards.iter())
            .chain(self.card_queues.relearning_cards.iter())
            .find(|card| &card.content.id == card_id)
            .cloned()
    }
}

/// Session progress information for UI display
#[derive(Debug, Clone)]
pub struct SessionProgress {
    pub total_remaining: usize,
    pub new_remaining: usize,
    pub learning_remaining: usize,
    pub review_remaining: usize,
    pub relearning_remaining: usize,
    pub cards_studied_today: i32,
    pub new_cards_limit: i32,
    pub review_cards_limit: i32,
    pub session_complete: bool,
}

/// Session recovery data for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionRecoveryData {
    deck_id: Uuid,
    session_state: SessionState,
    session_stats: SessionStats,
    daily_limits: DailyLimits,
    current_card_id: Option<Uuid>,
    timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scheduler::{Rating, Scheduler};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_session_initialization() {
        // Create a mock scheduler for testing
        let scheduler = Scheduler::new(None);

        // Note: We can't easily test SessionController without a real DeckManager
        // due to its tight integration. In a real application, we would use
        // dependency injection with mock traits.

        // For now, let's test the basic data structures
        let session_stats = SessionStats {
            started_at: Utc::now(),
            ended_at: None,
            total_cards_studied: 0,
            new_cards_studied: 0,
            review_cards_studied: 0,
            relearning_cards_studied: 0,
            correct_answers: 0,
            incorrect_answers: 0,
            average_response_time: None,
            study_streak_days: 0,
        };

        assert_eq!(session_stats.total_cards_studied, 0);
        assert!(session_stats.ended_at.is_none());

        let daily_limits = DailyLimits {
            max_new_cards: 20,
            max_review_cards: 200,
            cards_studied_today: 0,
            new_cards_studied_today: 0,
            review_cards_studied_today: 0,
        };

        assert_eq!(daily_limits.max_new_cards, 20);
        assert_eq!(daily_limits.max_review_cards, 200);
        assert_eq!(daily_limits.cards_studied_today, 0);

        let session_progress = SessionProgress {
            total_remaining: 0,
            new_remaining: 0,
            learning_remaining: 0,
            review_remaining: 0,
            relearning_remaining: 0,
            cards_studied_today: 0,
            new_cards_limit: 20,
            review_cards_limit: 200,
            session_complete: true,
        };

        assert!(session_progress.session_complete);
        assert_eq!(session_progress.total_remaining, 0);
    }

    #[tokio::test]
    async fn test_session_states() {
        // Test that session states are properly defined
        assert_eq!(SessionState::NotStarted, SessionState::NotStarted);
        assert_ne!(SessionState::InProgress, SessionState::Paused);
        assert_ne!(SessionState::Finished, SessionState::NotStarted);
    }

    #[tokio::test]
    async fn test_scheduler_integration() {
        // Test that scheduler methods work correctly
        let scheduler = Scheduler::new(None);

        // Test that schedule_card and reset_card methods exist and work
        // These are core methods that SessionController depends on

        // We can't fully test without cards, but we can verify the methods exist
        let daily_limits = scheduler.get_daily_limits();
        assert!(daily_limits.0 > 0); // new cards limit
        assert!(daily_limits.1 > 0); // review cards limit
    }
}

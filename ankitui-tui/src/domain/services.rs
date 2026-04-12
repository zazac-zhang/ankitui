//! Domain Services - Core business logic coordination
//!
//! Provides high-level service interfaces that coordinate between Core package
//! and TUI application layer

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::utils::error::{TuiResult, TuiError};
use crate::domain::viewmodels::StudySessionStats;
use ankitui_core::{DeckManager, SessionController};
use ankitui_core::core::Rating;
use ankitui_core::data::{Card, CardContent, Deck};
use ankitui_core::core::DeckStats;
use uuid::Uuid;

/// Deck Service - High-level deck operations
pub struct DeckService {
    deck_manager: Arc<DeckManager>,
}

impl DeckService {
    pub fn new(deck_manager: Arc<DeckManager>) -> Self {
        Self { deck_manager }
    }

    /// Create a new deck
    pub async fn create_deck(&self, name: String, description: Option<String>) -> TuiResult<Uuid> {
        self.deck_manager
            .create_deck(name, description, None)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Get all decks with their cards
    pub async fn get_all_decks(&self) -> TuiResult<Vec<(Deck, Vec<Card>)>> {
        self.deck_manager
            .get_all_decks()
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Get deck statistics
    pub async fn get_deck_statistics(&self, deck_uuid: &Uuid) -> TuiResult<DeckStats> {
        self.deck_manager
            .get_deck_statistics(deck_uuid)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Get a specific deck
    pub async fn get_deck(&self, deck_uuid: &Uuid) -> TuiResult<(Deck, Vec<Card>)> {
        self.deck_manager
            .get_deck(deck_uuid)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Add cards to deck
    pub async fn add_cards(&self, deck_uuid: &Uuid, cards: Vec<CardContent>) -> TuiResult<()> {
        self.deck_manager
            .add_cards(deck_uuid, cards)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Delete a deck
    pub async fn delete_deck(&self, deck_uuid: &Uuid) -> TuiResult<()> {
        self.deck_manager
            .delete_deck(deck_uuid)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }
}

/// Study Service - Manages study sessions and card reviews
pub struct StudyService {
    session_controller: Arc<Mutex<SessionController>>,
    deck_manager: Arc<DeckManager>,
}

impl StudyService {
    pub fn new(session_controller: Arc<Mutex<SessionController>>, deck_manager: Arc<DeckManager>) -> Self {
        Self {
            session_controller,
            deck_manager,
        }
    }

    /// Get due cards for a deck
    pub async fn get_due_cards(&self, deck_uuid: &Uuid, limit: Option<i32>) -> TuiResult<Vec<Card>> {
        self.deck_manager
            .get_due_cards(deck_uuid, limit)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Get next card to study
    pub async fn get_next_card(&self, deck_uuid: &Uuid) -> TuiResult<Option<Card>> {
        self.deck_manager
            .get_next_card(deck_uuid)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Start a study session
    pub async fn start_session(&mut self, deck_uuid: Uuid) -> TuiResult<()> {
        let mut controller = self.session_controller.lock().await;
        controller
            .start_session(deck_uuid)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// End the current study session
    pub async fn end_session(&mut self) -> TuiResult<StudySessionStats> {
        let mut controller = self.session_controller.lock().await;
        let stats = controller
            .end_session()
            .await
            .map_err(|e| TuiError::Core(e.to_string()))?;

        Ok(StudySessionStats {
            cards_studied: stats.total_cards_studied,
            total_cards_studied: stats.total_cards_studied,
            new_cards: stats.new_cards_studied,
            review_cards: stats.review_cards_studied,
            correct_answers: stats.correct_answers,
            average_time_seconds: stats.average_response_time.unwrap_or(0.0),
            started_at: stats.started_at,
            ended_at: stats.ended_at,
        })
    }

    /// Rate current card
    pub async fn rate_current_card(&mut self, rating: Rating) -> TuiResult<()> {
        let mut controller = self.session_controller.lock().await;
        controller
            .review_current_card(rating)
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }

    /// Skip current card
    pub async fn skip_current_card(&mut self) -> TuiResult<()> {
        let mut controller = self.session_controller.lock().await;
        controller
            .skip_current_card()
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }
}

/// Statistics Service - Handles learning statistics and analytics
pub struct StatisticsService {
    deck_manager: Arc<DeckManager>,
}

impl StatisticsService {
    pub fn new(deck_manager: Arc<DeckManager>) -> Self {
        Self { deck_manager }
    }

    /// Get global statistics
    pub async fn get_global_statistics(&self) -> TuiResult<ankitui_core::data::sync_adapter::GlobalStats> {
        self.deck_manager
            .get_global_statistics()
            .await
            .map_err(|e| TuiError::Core(e.to_string()))
    }
}
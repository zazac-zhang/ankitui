//! Card State Manager
//!
//! Provides high-level management for card states including bury/suspend operations

use crate::data::models::{Card, CardState};
use crate::data::StateStore;
use anyhow::Result;
use uuid::Uuid;

/// Card state management operations
pub struct CardStateManager {
    state_store: StateStore,
}

impl CardStateManager {
    /// Create a new card state manager
    pub async fn new(state_store: StateStore) -> Self {
        Self { state_store }
    }

    /// Get a reference to the underlying state store
    pub fn state_store(&self) -> &StateStore {
        &self.state_store
    }

    /// Get a mutable reference to the underlying state store
    pub fn state_store_mut(&mut self) -> &mut StateStore {
        &mut self.state_store
    }

    /// Bury a card temporarily
    /// The card will not appear in reviews until the next session
    pub async fn bury_card(&self, card_id: &Uuid) -> Result<()> {
        self.state_store.bury_card(card_id).await
    }

    /// Suspend a card permanently
    /// The card will never appear in reviews unless explicitly unsuspended
    pub async fn suspend_card(&self, card_id: &Uuid) -> Result<()> {
        self.state_store.suspend_card(card_id).await
    }

    /// Unbury a single card, restoring it to a specific state
    pub async fn unbury_card(&self, card_id: &Uuid, target_state: CardState) -> Result<()> {
        self.state_store.unbury_card(card_id, target_state).await
    }

    /// Unsuspend a single card, restoring it to a specific state
    pub async fn unsuspend_card(&self, card_id: &Uuid, target_state: CardState) -> Result<()> {
        self.state_store.unsuspend_card(card_id, target_state).await
    }

    /// Unbury all cards in a deck (called at the end of a study session)
    pub async fn unbury_all_deck_cards(&self, deck_cards: &[Uuid]) -> Result<usize> {
        self.state_store.unbury_all_cards(deck_cards).await
    }

    /// Get all buried cards for a deck
    pub async fn get_buried_cards(&self, deck_cards: &[Uuid]) -> Result<Vec<Uuid>> {
        self.state_store.get_buried_cards(deck_cards).await
    }

    /// Get all suspended cards for a deck
    pub async fn get_suspended_cards(&self, deck_cards: &[Uuid]) -> Result<Vec<Uuid>> {
        self.state_store.get_suspended_cards(deck_cards).await
    }

    /// Bury a list of cards
    pub async fn bury_cards(&self, card_ids: &[Uuid]) -> Result<()> {
        for card_id in card_ids {
            self.bury_card(card_id).await?;
        }
        Ok(())
    }

    /// Suspend a list of cards
    pub async fn suspend_cards(&self, card_ids: &[Uuid]) -> Result<()> {
        for card_id in card_ids {
            self.suspend_card(card_id).await?;
        }
        Ok(())
    }

    /// Get state summary for cards in a deck
    pub async fn get_deck_state_summary(&self, deck_cards: &[Uuid]) -> Result<DeckStateSummary> {
        let buried = self.get_buried_cards(deck_cards).await?;
        let suspended = self.get_suspended_cards(deck_cards).await?;

        let total_cards = deck_cards.len();
        let buried_count = buried.len();
        let suspended_count = suspended.len();
        let active_count = total_cards - buried_count - suspended_count;

        Ok(DeckStateSummary {
            total_cards,
            active_cards: active_count,
            buried_cards: buried_count,
            suspended_cards: suspended_count,
            buried_card_ids: buried,
            suspended_card_ids: suspended,
        })
    }

    /// Filter cards to only include active (non-buried, non-suspended) cards
    pub async fn filter_active_cards<'a>(&self, cards: &'a [Card]) -> Result<Vec<&'a Card>> {
        let buried_cards = self
            .get_buried_cards(&cards.iter().map(|c| c.content.id).collect::<Vec<_>>())
            .await?;
        let suspended_cards = self
            .get_suspended_cards(&cards.iter().map(|c| c.content.id).collect::<Vec<_>>())
            .await?;

        let buried_set: std::collections::HashSet<_> = buried_cards.into_iter().collect();
        let suspended_set: std::collections::HashSet<_> = suspended_cards.into_iter().collect();

        Ok(cards
            .iter()
            .filter(|card| !buried_set.contains(&card.content.id) && !suspended_set.contains(&card.content.id))
            .collect())
    }

    /// Check if a card is currently buried
    pub async fn is_card_buried(&self, card_id: &Uuid, deck_cards: &[Uuid]) -> Result<bool> {
        let buried_cards = self.get_buried_cards(deck_cards).await?;
        Ok(buried_cards.contains(card_id))
    }

    /// Check if a card is currently suspended
    pub async fn is_card_suspended(&self, card_id: &Uuid, deck_cards: &[Uuid]) -> Result<bool> {
        let suspended_cards = self.get_suspended_cards(deck_cards).await?;
        Ok(suspended_cards.contains(card_id))
    }

    /// Get card status with detailed information
    pub async fn get_card_status(&self, card_id: &Uuid, deck_cards: &[Uuid]) -> Result<CardStatus> {
        if self.is_card_suspended(card_id, deck_cards).await? {
            Ok(CardStatus::Suspended)
        } else if self.is_card_buried(card_id, deck_cards).await? {
            Ok(CardStatus::Buried)
        } else {
            // Check if card exists in deck and get its actual state
            if deck_cards.contains(card_id) {
                if let Some(card_state) = self.state_store.load_card_state(card_id).await? {
                    match card_state.state {
                        CardState::Suspended => Ok(CardStatus::Suspended),
                        CardState::Buried => Ok(CardStatus::Buried),
                        _ => Ok(CardStatus::Active),
                    }
                } else {
                    Ok(CardStatus::NotFound)
                }
            } else {
                Ok(CardStatus::NotFound)
            }
        }
    }

    /// Batch operation to bury multiple cards at once
    pub async fn batch_bury_cards(&self, card_ids: &[Uuid]) -> Result<BurySuspendResult> {
        let mut success_count = 0;
        let mut failed_cards = Vec::new();

        for card_id in card_ids {
            match self.bury_card(card_id).await {
                Ok(_) => success_count += 1,
                Err(e) => failed_cards.push((*card_id, e.to_string())),
            }
        }

        Ok(BurySuspendResult {
            total_cards: card_ids.len(),
            success_count,
            failed_cards,
        })
    }

    /// Batch operation to suspend multiple cards at once
    pub async fn batch_suspend_cards(&self, card_ids: &[Uuid]) -> Result<BurySuspendResult> {
        let mut success_count = 0;
        let mut failed_cards = Vec::new();

        for card_id in card_ids {
            match self.suspend_card(card_id).await {
                Ok(_) => success_count += 1,
                Err(e) => failed_cards.push((*card_id, e.to_string())),
            }
        }

        Ok(BurySuspendResult {
            total_cards: card_ids.len(),
            success_count,
            failed_cards,
        })
    }
}

/// Summary of card states in a deck
#[derive(Debug, Clone)]
pub struct DeckStateSummary {
    pub total_cards: usize,
    pub active_cards: usize,
    pub buried_cards: usize,
    pub suspended_cards: usize,
    pub buried_card_ids: Vec<Uuid>,
    pub suspended_card_ids: Vec<Uuid>,
}

/// Card status for UI display
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardStatus {
    Active,
    Buried,
    Suspended,
    NotFound,
}

/// Result of batch bury/suspend operations
#[derive(Debug, Clone)]
pub struct BurySuspendResult {
    pub total_cards: usize,
    pub success_count: usize,
    pub failed_cards: Vec<(Uuid, String)>,
}

impl BurySuspendResult {
    /// Check if the operation was completely successful
    pub fn is_success(&self) -> bool {
        self.failed_cards.is_empty()
    }

    /// Get the success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_cards == 0 {
            100.0
        } else {
            (self.success_count as f64 / self.total_cards as f64) * 100.0
        }
    }
}

impl DeckStateSummary {
    /// Get the percentage of active cards
    pub fn active_percentage(&self) -> f64 {
        if self.total_cards == 0 {
            100.0
        } else {
            (self.active_cards as f64 / self.total_cards as f64) * 100.0
        }
    }

    /// Get the percentage of buried cards
    pub fn buried_percentage(&self) -> f64 {
        if self.total_cards == 0 {
            0.0
        } else {
            (self.buried_cards as f64 / self.total_cards as f64) * 100.0
        }
    }

    /// Get the percentage of suspended cards
    pub fn suspended_percentage(&self) -> f64 {
        if self.total_cards == 0 {
            0.0
        } else {
            (self.suspended_cards as f64 / self.total_cards as f64) * 100.0
        }
    }
}

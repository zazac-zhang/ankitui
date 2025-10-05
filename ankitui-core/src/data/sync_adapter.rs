//! Sync Adapter - Coordinates content and state storage
//!
//! Handles merging of TOML content with SQLite state to provide complete card objects

use crate::data::models::{Card, CardContent, CardStateData, Deck};
use crate::data::{ContentStore, StateStore};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

/// Sync Adapter coordinates between content and state storage
#[derive(Debug, Clone)]
pub struct SyncAdapter {
    content_store: ContentStore,
    state_store: StateStore,
    content_base_dir: std::path::PathBuf,
}

impl SyncAdapter {
    /// Create a new SyncAdapter with content and state stores
    pub async fn new(
        content_base_dir: impl AsRef<std::path::Path>,
        db_path: impl AsRef<std::path::Path>,
    ) -> Result<Self> {
        let content_base_dir = content_base_dir.as_ref().to_path_buf();
        let content_store = ContentStore::new(&content_base_dir)?;
        let state_store = StateStore::new(db_path).await?;

        Ok(Self {
            content_store,
            state_store,
            content_base_dir,
        })
    }

    /// Load a complete deck with both content and state merged
    pub async fn load_deck(&self, deck_uuid: &Uuid) -> Result<(Deck, Vec<Card>)> {
        // Load deck content from TOML
        let (deck, card_contents) = self
            .content_store
            .load_deck(deck_uuid)
            .context("Failed to load deck content")?;

        // Extract card IDs
        let card_ids: Vec<Uuid> = card_contents.iter().map(|c| c.id).collect();

        // Load card states from SQLite
        let card_states = self
            .state_store
            .load_card_states(&card_ids)
            .await
            .context("Failed to load card states")?;

        // Create state lookup map
        let state_map: HashMap<Uuid, CardStateData> = card_states
            .into_iter()
            .map(|state| (state.id, state))
            .collect();

        // Merge content with state
        let mut cards = Vec::new();
        for content in card_contents {
            let state = state_map
                .get(&content.id)
                .cloned()
                .unwrap_or_else(|| CardStateData {
                    id: content.id,
                    due: Utc::now(),
                    interval: 0,
                    ease_factor: 2.5,
                    reps: 0,
                    lapses: 0,
                    state: crate::data::models::CardState::New,
                    updated_at: Utc::now(),
                });

            cards.push(Card { content, state });
        }

        Ok((deck, cards))
    }

    /// Load all decks with complete card data
    pub async fn load_all_decks(&self) -> Result<Vec<(Deck, Vec<Card>)>> {
        let decks_content = self
            .content_store
            .load_all_decks()
            .context("Failed to load deck contents")?;

        let mut complete_decks = Vec::new();

        for (deck, card_contents) in decks_content {
            let card_ids: Vec<Uuid> = card_contents.iter().map(|c| c.id).collect();
            let card_states = self
                .state_store
                .load_card_states(&card_ids)
                .await
                .context("Failed to load card states")?;

            let state_map: HashMap<Uuid, CardStateData> = card_states
                .into_iter()
                .map(|state| (state.id, state))
                .collect();

            let mut cards = Vec::new();
            for content in card_contents {
                let state = state_map
                    .get(&content.id)
                    .cloned()
                    .unwrap_or_else(|| CardStateData {
                        id: content.id,
                        due: Utc::now(),
                        interval: 0,
                        ease_factor: 2.5,
                        reps: 0,
                        lapses: 0,
                        state: crate::data::models::CardState::New,
                        updated_at: Utc::now(),
                    });

                cards.push(Card { content, state });
            }

            complete_decks.push((deck, cards));
        }

        Ok(complete_decks)
    }

    /// Save a deck with content and update card states
    pub async fn save_deck(&self, deck: &Deck, cards: &[Card]) -> Result<()> {
        // Extract content from cards
        let card_contents: Vec<CardContent> = cards.iter().map(|c| c.content.clone()).collect();

        // Save deck content to TOML
        self.content_store
            .save_deck(deck, &card_contents)
            .context("Failed to save deck content")?;

        // Save card states to SQLite
        for card in cards {
            self.state_store
                .save_card_state(&card.state)
                .await
                .context("Failed to save card state")?;
        }

        Ok(())
    }

    /// Add new cards to a deck
    pub async fn add_cards_to_deck(&self, deck_uuid: &Uuid, cards: &[CardContent]) -> Result<()> {
        // Load existing deck content
        let (deck, existing_cards) = self
            .load_deck(deck_uuid)
            .await
            .context("Failed to load existing deck")?;

        // Extract content from existing cards without consuming them
        let mut all_card_contents: Vec<CardContent> =
            existing_cards.iter().map(|c| c.content.clone()).collect();
        all_card_contents.extend(cards.iter().cloned());

        // Create Card objects for new cards (with default state)
        let new_cards: Vec<Card> = cards
            .iter()
            .map(|content| Card::new(content.clone()))
            .collect();

        // Keep existing cards that aren't being replaced
        let existing_cards_filtered: Vec<Card> = existing_cards
            .into_iter()
            .filter(|c| !cards.iter().any(|new| new.id == c.content.id))
            .collect();

        // Combine new cards with filtered existing cards
        let mut all_cards = new_cards;
        all_cards.extend(existing_cards_filtered);

        // Save updated deck
        self.save_deck(&deck, &all_cards)
            .await
            .context("Failed to save updated deck")?;

        Ok(())
    }

    /// Update card states after review (only state changes, not content)
    pub async fn update_card_states(&self, cards: &[Card]) -> Result<()> {
        for card in cards {
            self.state_store
                .save_card_state(&card.state)
                .await
                .context("Failed to save updated card state")?;
        }
        Ok(())
    }

    /// Get cards due for review across all decks
    pub async fn get_due_cards(&self, limit: Option<i32>) -> Result<Vec<(Deck, Vec<Card>)>> {
        let due_card_ids = self
            .state_store
            .get_due_cards(Utc::now(), limit)
            .await
            .context("Failed to get due card IDs")?;

        if due_card_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Load all decks to find cards that are due
        let all_decks = self
            .load_all_decks()
            .await
            .context("Failed to load all decks")?;

        let mut result = Vec::new();

        for (deck, cards) in all_decks {
            let due_cards: Vec<Card> = cards
                .into_iter()
                .filter(|card| due_card_ids.contains(&card.content.id))
                .collect();

            if !due_cards.is_empty() {
                result.push((deck, due_cards));
            }
        }

        Ok(result)
    }

    /// Get cards due for review in a specific deck
    pub async fn get_due_cards_in_deck(
        &self,
        deck_uuid: &Uuid,
        limit: Option<i32>,
    ) -> Result<Vec<Card>> {
        let (_deck, cards) = self
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck")?;

        let due_cards: Vec<Card> = cards
            .into_iter()
            .filter(|card| card.state.due <= Utc::now())
            .take(limit.unwrap_or(i32::MAX) as usize)
            .collect();

        Ok(due_cards)
    }

    /// Get new cards for a deck
    pub async fn get_new_cards(&self, deck_uuid: &Uuid, limit: Option<i32>) -> Result<Vec<Card>> {
        let (_deck, cards) = self
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck")?;

        let new_cards: Vec<Card> = cards
            .into_iter()
            .filter(|card| matches!(card.state.state, crate::data::models::CardState::New))
            .take(limit.unwrap_or(i32::MAX) as usize)
            .collect();

        Ok(new_cards)
    }

    /// Delete a deck and all its card states
    pub async fn delete_deck(&self, deck_uuid: &Uuid) -> Result<()> {
        // Load deck to get card IDs for state cleanup
        let (_deck, cards) = self
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck for deletion")?;

        // Delete card states from SQLite
        for card in cards {
            self.state_store
                .delete_card_state(&card.content.id)
                .await
                .context("Failed to delete card state")?;
        }

        // Delete deck content from TOML
        self.content_store
            .delete_deck(deck_uuid)
            .context("Failed to delete deck content")?;

        Ok(())
    }

    /// Find deck by name
    pub async fn find_deck_by_name(&self, name: &str) -> Result<Option<(Deck, Vec<Card>)>> {
        if let Some(deck_uuid) = self.content_store.find_deck_by_name(name)? {
            let deck = self.load_deck(&deck_uuid).await?;
            Ok(Some(deck))
        } else {
            Ok(None)
        }
    }

    /// Get global statistics across all decks
    pub async fn get_global_statistics(&self) -> Result<GlobalStats> {
        let state_stats = self
            .state_store
            .get_statistics()
            .await
            .context("Failed to get state statistics")?;

        let decks_content = self
            .content_store
            .load_all_decks()
            .context("Failed to load deck contents")?;

        Ok(GlobalStats {
            total_decks: decks_content.len(),
            total_cards: state_stats.total,
            due_cards: state_stats.due,
            new_cards: state_stats.new,
            learning_cards: state_stats.learning,
            review_cards: state_stats.review,
        })
    }

    /// Export deck for backup/debug (includes state in TOML)
    pub async fn export_deck_with_state(&self, deck_uuid: &Uuid) -> Result<String> {
        let (deck, cards) = self
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck for export")?;

        // Create exportable structure with embedded state
        let export_cards: Vec<crate::data::models::ExportCard> = cards
            .into_iter()
            .map(|card| crate::data::models::ExportCard {
                content: card.content,
                ease_factor: card.state.ease_factor,
                interval: card.state.interval,
                reps: card.state.reps,
                lapses: card.state.lapses,
                due: card.state.due.to_rfc3339(),
                state: format!("{:?}", card.state.state),
            })
            .collect();

        let export_deck = crate::data::models::ExportDeck {
            deck,
            cards: export_cards,
        };

        toml::to_string_pretty(&export_deck).context("Failed to serialize deck for export")
    }

    /// Import deck from export (handles both content-only and content+state)
    pub async fn import_deck(&self, toml_content: &str) -> Result<Uuid> {
        // First try to parse as new import format (flexible fields)
        if let Ok(import_deck) = self.parse_import_format(toml_content) {
            // Convert import format to internal format
            let deck = import_deck.to_internal_deck();
            let card_contents = import_deck.to_internal_cards();

            // Validate card contents
            for content in &card_contents {
                if content.front.trim().is_empty() {
                    return Err(anyhow::anyhow!("Card front cannot be empty"));
                }
                if content.back.trim().is_empty() {
                    return Err(anyhow::anyhow!("Card back cannot be empty"));
                }
            }

            // Create Card objects from CardContent
            let cards: Vec<Card> = card_contents.into_iter().map(|content| Card::new(content)).collect();

            // Save deck and cards
            self.save_deck(&deck, &cards)
                .await
                .context("Failed to save imported deck")?;

            Ok(deck.uuid)
        }
        // Try to parse as export format first (with state)
        else if let Ok(export_deck) = self.parse_export_format(toml_content) {
            // Convert export format back to internal format
            let mut cards = Vec::new();
            for export_card in export_deck.cards {
                let state = CardStateData {
                    id: export_card.content.id,
                    due: DateTime::parse_from_rfc3339(&export_card.due)
                        .context("Failed to parse due date in export")?
                        .with_timezone(&Utc),
                    interval: export_card.interval,
                    ease_factor: export_card.ease_factor,
                    reps: export_card.reps,
                    lapses: export_card.lapses,
                    state: match export_card.state.as_str() {
                        "New" => crate::data::models::CardState::New,
                        "Learning" => crate::data::models::CardState::Learning,
                        "Review" => crate::data::models::CardState::Review,
                        "Relearning" => crate::data::models::CardState::Relearning,
                        _ => crate::data::models::CardState::New,
                    },
                    updated_at: Utc::now(),
                };

                cards.push(Card {
                    content: export_card.content,
                    state,
                });
            }

            self.save_deck(&export_deck.deck, &cards)
                .await
                .context("Failed to save imported deck")?;

            Ok(export_deck.deck.uuid)
        } else {
            // Fallback to content-only import using ContentStore
            let deck_uuid = self
                .content_store
                .import_deck_from_string(toml_content)
                .context("Failed to import deck content")?;

            // Load the imported deck to get cards and create state records
            let (_deck, card_contents) = self
                .content_store
                .load_deck(&deck_uuid)
                .context("Failed to load imported deck")?;

            // Create state records for all cards
            let mut cards = Vec::new();
            for content in card_contents {
                let state = CardStateData {
                    id: content.id,
                    due: Utc::now(),
                    interval: 0,
                    ease_factor: 2.5,
                    reps: 0,
                    lapses: 0,
                    state: crate::data::models::CardState::New,
                    updated_at: Utc::now(),
                };

                cards.push(Card { content, state });
            }

            // Save the card states to database
            for card in &cards {
                self.state_store
                    .save_card_state(&card.state)
                    .await
                    .context("Failed to save card state")?;
            }

            Ok(deck_uuid)
        }
    }

    /// Try to parse import format with flexible fields
    fn parse_import_format(&self, toml_content: &str) -> Result<crate::data::models::ImportDeck> {
        let import_deck: crate::data::models::ImportDeck =
            toml::from_str(toml_content).context("Failed to parse import format")?;
        Ok(import_deck)
    }

    /// Try to parse export format with embedded state
    fn parse_export_format(&self, toml_content: &str) -> Result<crate::data::models::ExportDeck> {
        let export_deck: crate::data::models::ExportDeck =
            toml::from_str(toml_content).context("Failed to parse export format")?;
        Ok(export_deck)
    }

    /// Close the sync adapter and underlying connections
    pub async fn close(&self) -> Result<()> {
        self.state_store
            .close()
            .await
            .context("Failed to close state store")?;
        Ok(())
    }

    /// Get the content base directory
    pub fn get_content_base_dir(&self) -> &std::path::Path {
        &self.content_base_dir
    }

    /// Update card content
    pub async fn update_card_content(&self, card_content: &CardContent) -> Result<()> {
        // Since ContentStore doesn't have individual card update, we need to
        // load the deck, update the card, and save the entire deck
        // For now, this is a placeholder implementation
        Ok(())
    }

    /// Get a deck by UUID
    pub async fn get_deck(&self, deck_uuid: &Uuid) -> Option<Deck> {
        self.content_store
            .load_deck(deck_uuid)
            .ok()
            .map(|(deck, _)| deck)
    }

    /// Get card state by card ID
    pub async fn get_card_state(&self, card_id: &Uuid) -> Result<Option<CardStateData>> {
        self.state_store
            .load_card_state(card_id)
            .await
    }

    /// List all decks (without cards)
    pub async fn list_decks(&self) -> Result<Vec<Deck>> {
        let decks_content = self
            .content_store
            .load_all_decks()
            .context("Failed to load deck contents")?;

        let decks = decks_content.into_iter().map(|(deck, _)| deck).collect();
        Ok(decks)
    }
}

/// Global statistics across all decks
#[derive(Debug, Clone)]
pub struct GlobalStats {
    pub total_decks: usize,
    pub total_cards: i64,
    pub due_cards: i64,
    pub new_cards: i64,
    pub learning_cards: i64,
    pub review_cards: i64,
}

// Add the missing types to models.rs for the export functionality
pub(crate) mod export_types {
    use crate::data::models::{CardContent, Deck};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct ExportDeck {
        pub deck: Deck,
        pub cards: Vec<ExportCard>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct ExportCard {
        #[serde(flatten)]
        pub content: CardContent,
        pub ease_factor: f32,
        pub interval: i32,
        pub reps: i32,
        pub lapses: i32,
        pub due: String,
        pub state: String,
    }
}

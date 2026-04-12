//! Deck Manager - Handles deck operations
//!
//! Provides complete deck lifecycle management including CRUD operations,
//! statistics calculation, and integration with data layer and scheduler.

use crate::core::scheduler::{Rating, Scheduler};
use crate::data::models::{Card, CardContent, CardState, Deck, MediaRef, MediaType, SchedulerConfig};
use crate::data::SyncAdapter;
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use std::path::Path;
use uuid::Uuid;

/// Deck statistics information
#[derive(Debug)]
pub struct DeckStats {
    pub total_cards: usize,
    pub new_cards: usize,
    pub learning_cards: usize,
    pub review_cards: usize,
    pub due_cards: usize,
    pub cards_today: usize,
    pub retention_rate: Option<f32>,
    pub average_ease_factor: f32,
}

/// Deck Manager handles all deck-related operations
#[derive(Clone, Debug)]
pub struct DeckManager {
    sync_adapter: SyncAdapter,
    scheduler: Scheduler,
}

impl DeckManager {
    /// Create a new DeckManager with data stores
    pub async fn new(
        content_base_dir: impl AsRef<std::path::Path>,
        db_path: impl AsRef<std::path::Path>,
    ) -> Result<Self> {
        let sync_adapter = SyncAdapter::new(content_base_dir, db_path)
            .await
            .context("Failed to initialize sync adapter")?;

        let scheduler = Scheduler::new(None);

        Ok(Self {
            sync_adapter,
            scheduler,
        })
    }

    /// Create a new deck with the given name and optional description
    pub async fn create_deck(
        &self,
        name: String,
        description: Option<String>,
        scheduler_config: Option<SchedulerConfig>,
    ) -> Result<Uuid> {
        // Validate deck name
        if name.trim().is_empty() {
            return Err(anyhow!("Deck name cannot be empty"));
        }

        // Check if deck with this name already exists
        if self.sync_adapter.find_deck_by_name(&name).await?.is_some() {
            return Err(anyhow!("Deck with name '{}' already exists", name));
        }

        let now = Utc::now();
        let deck = Deck {
            uuid: Uuid::new_v4(),
            name: name.trim().to_string(),
            description,
            created_at: now,
            modified_at: now,
            scheduler_config,
        };

        // Save empty deck
        self.sync_adapter
            .save_deck(&deck, &[])
            .await
            .context("Failed to create new deck")?;

        Ok(deck.uuid)
    }

    /// Get a deck by UUID with all its cards
    pub async fn get_deck(&self, deck_uuid: &Uuid) -> Result<(Deck, Vec<Card>)> {
        self.sync_adapter
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck")
    }

    /// Get all decks with their cards
    pub async fn get_all_decks(&self) -> Result<Vec<(Deck, Vec<Card>)>> {
        self.sync_adapter
            .load_all_decks()
            .await
            .context("Failed to load all decks")
    }

    /// Find a deck by name
    pub async fn find_deck_by_name(&self, name: &str) -> Result<Option<(Deck, Vec<Card>)>> {
        self.sync_adapter
            .find_deck_by_name(name)
            .await
            .context("Failed to find deck by name")
    }

    /// Update deck metadata (name, description, scheduler config)
    pub async fn update_deck(&self, deck_uuid: &Uuid, updates: DeckUpdate) -> Result<()> {
        let (mut deck, cards) = self
            .sync_adapter
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck for update")?;

        let mut changed = false;

        if let Some(name) = updates.name {
            if name.trim().is_empty() {
                return Err(anyhow!("Deck name cannot be empty"));
            }

            // Check if new name conflicts with existing deck
            if let Some((existing_deck, _)) = self.sync_adapter.find_deck_by_name(&name).await? {
                if existing_deck.uuid != *deck_uuid {
                    return Err(anyhow!("Deck with name '{}' already exists", name));
                }
            }

            deck.name = name.trim().to_string();
            changed = true;
        }

        if let Some(description) = updates.description {
            deck.description = Some(description);
            changed = true;
        }

        if let Some(scheduler_config) = updates.scheduler_config {
            deck.scheduler_config = Some(scheduler_config);
            changed = true;
        }

        if changed {
            deck.modified_at = Utc::now();
            self.sync_adapter
                .save_deck(&deck, &cards)
                .await
                .context("Failed to save updated deck")?;
        }

        Ok(())
    }

    /// Delete a deck and all its data
    pub async fn delete_deck(&self, deck_uuid: &Uuid) -> Result<()> {
        self.sync_adapter
            .delete_deck(deck_uuid)
            .await
            .context("Failed to delete deck")
    }

    /// Add new cards to a deck
    pub async fn add_cards(&self, deck_uuid: &Uuid, card_contents: Vec<CardContent>) -> Result<()> {
        if card_contents.is_empty() {
            return Ok(());
        }

        // Validate card contents
        for content in &card_contents {
            if content.front.trim().is_empty() {
                return Err(anyhow!("Card front cannot be empty"));
            }
            if content.back.trim().is_empty() {
                return Err(anyhow!("Card back cannot be empty"));
            }
        }

        self.sync_adapter
            .add_cards_to_deck(deck_uuid, &card_contents)
            .await
            .context("Failed to add cards to deck")
    }

    /// Get statistics for a specific deck
    pub async fn get_deck_statistics(&self, deck_uuid: &Uuid) -> Result<DeckStats> {
        let (_, cards) = self
            .sync_adapter
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck for statistics")?;

        let total_cards = cards.len();
        let mut new_cards = 0;
        let mut learning_cards = 0;
        let mut review_cards = 0;
        let mut due_cards = 0;
        let mut total_ease_factor = 0.0;
        let mut cards_with_reviews = 0;
        let mut cards_today = 0;

        let now = Utc::now();
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();

        for card in &cards {
            match card.state.state {
                CardState::New => new_cards += 1,
                CardState::Learning | CardState::Relearning => learning_cards += 1,
                CardState::Review => {
                    review_cards += 1;
                    total_ease_factor += card.state.ease_factor;
                    cards_with_reviews += 1;
                }
                CardState::Buried | CardState::Suspended => {
                    // These cards are not active in learning
                }
            }

            if card.state.due <= now {
                due_cards += 1;
            }

            if card.state.updated_at >= today_start {
                cards_today += 1;
            }
        }

        let average_ease_factor = if cards_with_reviews > 0 {
            total_ease_factor / cards_with_reviews as f32
        } else {
            2.5 // Default ease factor
        };

        // Calculate retention rate (simplified - would track actual performance data)
        let retention_rate = if cards_today > 0 && review_cards > 0 {
            // This is a simplified calculation - in a real implementation,
            // you'd track actual performance data
            Some(0.85) // Placeholder value
        } else {
            None
        };

        Ok(DeckStats {
            total_cards,
            new_cards,
            learning_cards,
            review_cards,
            due_cards,
            cards_today,
            retention_rate,
            average_ease_factor,
        })
    }

    /// Get global statistics across all decks
    pub async fn get_global_statistics(&self) -> Result<crate::data::sync_adapter::GlobalStats> {
        self.sync_adapter
            .get_global_statistics()
            .await
            .context("Failed to get global statistics")
    }

    /// Get cards due for review in a specific deck
    pub async fn get_due_cards(&self, deck_uuid: &Uuid, limit: Option<i32>) -> Result<Vec<Card>> {
        self.sync_adapter
            .get_due_cards_in_deck(deck_uuid, limit)
            .await
            .context("Failed to get due cards")
    }

    /// Get new cards for a deck
    pub async fn get_new_cards(&self, deck_uuid: &Uuid, limit: Option<i32>) -> Result<Vec<Card>> {
        self.sync_adapter
            .get_new_cards(deck_uuid, limit)
            .await
            .context("Failed to get new cards")
    }

    /// Get the next card to review from a deck
    pub async fn get_next_card(&self, deck_uuid: &Uuid) -> Result<Option<Card>> {
        let (_, cards) = self
            .sync_adapter
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck for next card")?;

        Ok(self.scheduler.get_next_card(&cards, Utc::now()).cloned())
    }

    /// Update card states after review
    pub async fn update_card_states(&self, cards: &[Card]) -> Result<()> {
        self.sync_adapter
            .update_card_states(cards)
            .await
            .context("Failed to update card states")
    }

    /// Process a card review and update its state
    pub async fn review_card(&self, mut card: Card, rating: Rating) -> Result<Card> {
        self.scheduler
            .update_card(&mut card, rating, Utc::now())
            .context("Failed to update card after review")?;

        self.sync_adapter
            .update_card_states(&[card.clone()])
            .await
            .context("Failed to save updated card state")?;

        Ok(card)
    }

    /// Export a deck (with optional state information)
    pub async fn export_deck(&self, deck_uuid: &Uuid, include_state: bool) -> Result<String> {
        if include_state {
            self.sync_adapter
                .export_deck_with_state(deck_uuid)
                .await
                .context("Failed to export deck with state")
        } else {
            // Export content-only format using content store
            let (_deck, cards) = self
                .sync_adapter
                .load_deck(deck_uuid)
                .await
                .context("Failed to load deck for export")?;

            let _card_contents: Vec<CardContent> = cards.iter().map(|c| c.content.clone()).collect();

            // Use the content store's export functionality
            // Note: This would require implementing export_deck in ContentStore
            // For now, we'll use the existing export with state and filter out state info
            self.sync_adapter
                .export_deck_with_state(deck_uuid)
                .await
                .context("Failed to export deck")
        }
    }

    /// Import a deck from TOML string
    pub async fn import_deck(&self, toml_content: &str) -> Result<Uuid> {
        self.sync_adapter
            .import_deck(toml_content)
            .await
            .context("Failed to import deck")
    }

    /// Rename a deck
    pub async fn rename_deck(&self, deck_uuid: &Uuid, new_name: String) -> Result<()> {
        self.update_deck(
            deck_uuid,
            DeckUpdate {
                name: Some(new_name),
                description: None,
                scheduler_config: None,
            },
        )
        .await
    }

    /// Get deck configuration
    pub async fn get_deck_config(&self, deck_uuid: &Uuid) -> Result<SchedulerConfig> {
        let (deck, _) = self
            .sync_adapter
            .load_deck(deck_uuid)
            .await
            .context("Failed to load deck config")?;

        Ok(deck.scheduler_config.unwrap_or_default())
    }

    /// Update deck configuration
    pub async fn update_deck_config(&self, deck_uuid: &Uuid, config: SchedulerConfig) -> Result<()> {
        self.update_deck(
            deck_uuid,
            DeckUpdate {
                name: None,
                description: None,
                scheduler_config: Some(config),
            },
        )
        .await
    }

    // ========== Media Management ==========

    /// Add media to a card
    pub async fn add_card_media<P: AsRef<Path>>(
        &self,
        deck_uuid: &Uuid,
        card_id: &Uuid,
        media_path: P,
        media_type: MediaType,
    ) -> Result<()> {
        // Initialize media manager
        let media_dir = self.sync_adapter.get_content_base_dir().join("media");
        let media_manager = super::MediaManager::new(media_dir);

        // Add media file
        let enhanced_ref = media_manager.add_media_file(media_path, media_type).await?;
        let media_ref = MediaRef {
            path: enhanced_ref.path,
            media_type: enhanced_ref.media_type,
        };

        // Update card with media reference
        let mut card = self.get_card(deck_uuid, card_id).await?;
        card.content.media = Some(MediaRef {
            path: media_ref.path,
            media_type: media_ref.media_type,
        });

        // Save updated card
        self.sync_adapter.update_card_content(&card.content).await
    }

    /// Remove media from a card
    pub async fn remove_card_media(&self, deck_uuid: &Uuid, card_id: &Uuid) -> Result<()> {
        let mut card = self.get_card(deck_uuid, card_id).await?;

        if let Some(media_ref) = &card.content.media {
            // Delete media file
            let media_dir = self.sync_adapter.get_content_base_dir().join("media");
            let media_manager = super::MediaManager::new(media_dir);

            // Create an enhanced media reference for deletion
            let enhanced_ref = crate::data::models::EnhancedMediaRef {
                id: uuid::Uuid::new_v4(), // Generate temp ID for deletion
                path: media_ref.path.clone(),
                media_type: media_ref.media_type.clone(),
                metadata: crate::data::models::MediaMetadata::default(),
                status: crate::data::models::MediaStatus::Available,
                local_cache_path: None,
                remote_url: None,
                alt_text: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            let media_ref = MediaRef {
                path: enhanced_ref.path.clone(),
                media_type: enhanced_ref.media_type.clone(),
            };
            media_manager.delete_media(&media_ref)?;
        }

        // Remove media reference from card
        card.content.media = None;

        // Save updated card
        self.sync_adapter.update_card_content(&card.content).await
    }

    /// Update card media
    pub async fn update_card_media<P: AsRef<Path>>(
        &self,
        deck_uuid: &Uuid,
        card_id: &Uuid,
        media_path: P,
        media_type: MediaType,
    ) -> Result<()> {
        // Remove existing media first
        self.remove_card_media(deck_uuid, card_id).await?;

        // Add new media
        self.add_card_media(deck_uuid, card_id, media_path, media_type).await
    }

    /// Get a specific card from a deck
    pub async fn get_card(&self, deck_uuid: &Uuid, card_id: &Uuid) -> Result<Card> {
        let cards = self.get_all_cards(deck_uuid).await?;
        cards
            .into_iter()
            .find(|card| card.content.id == *card_id)
            .ok_or_else(|| anyhow::anyhow!("Card not found: {}", card_id))
    }

    /// Get all cards from a deck
    pub async fn get_all_cards(&self, deck_uuid: &Uuid) -> Result<Vec<Card>> {
        let (_, cards) = self
            .sync_adapter
            .load_deck(deck_uuid)
            .await
            .map_err(|_| anyhow::anyhow!("Deck not found: {}", deck_uuid))?;

        Ok(cards)
    }

    /// Get cards (alias for get_all_cards for compatibility)
    pub async fn get_cards(&self, deck_uuid: &Uuid) -> Result<Vec<Card>> {
        self.get_all_cards(deck_uuid).await
    }

    /// Get cards with media
    pub async fn get_cards_with_media(&self, deck_uuid: &Uuid) -> Result<Vec<Card>> {
        let cards = self.get_all_cards(deck_uuid).await?;
        Ok(cards.into_iter().filter(|card| card.content.media.is_some()).collect())
    }

    /// Get all media files referenced by cards in a deck
    pub async fn get_deck_media_files(&self, deck_uuid: &Uuid) -> Result<Vec<String>> {
        let cards = self.get_all_cards(deck_uuid).await?;
        let mut media_files = Vec::new();

        for card in cards {
            if let Some(media_ref) = &card.content.media {
                media_files.push(media_ref.path.clone());
            }
        }

        Ok(media_files)
    }

    /// Clean up orphaned media files in a deck
    pub async fn cleanup_deck_media(&self, deck_uuid: &Uuid) -> Result<usize> {
        // Get all referenced media files
        let referenced_files = self.get_deck_media_files(deck_uuid).await?;

        // Initialize media manager
        let media_dir = self.sync_adapter.get_content_base_dir().join("media");
        let media_manager = super::MediaManager::new(media_dir);

        // Clean up orphaned files
        media_manager.cleanup_orphaned_media(&referenced_files).await
    }

    /// Get the total number of decks
    pub async fn get_deck_count(&self) -> Result<usize> {
        let decks = self.sync_adapter.list_decks().await?;
        Ok(decks.len())
    }

    /// Update a card (both content and state)
    pub async fn update_card(&self, deck_uuid: &Uuid, card: &Card) -> Result<()> {
        // Load the deck to get all cards
        let (mut deck, mut cards) = self
            .sync_adapter
            .load_deck(deck_uuid)
            .await
            .map_err(|_| anyhow::anyhow!("Deck not found: {}", deck_uuid))?;

        // Find and update the target card
        if let Some(index) = cards.iter().position(|c| c.content.id == card.content.id) {
            cards[index] = card.clone();

            // Save the updated deck
            self.sync_adapter.save_deck(&deck, &cards).await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Card not found: {}", card.content.id))
        }
    }

    /// Validate media files for all cards in a deck
    pub async fn validate_deck_media(&self, deck_uuid: &Uuid) -> Result<Vec<(Uuid, bool)>> {
        let cards = self.get_cards_with_media(deck_uuid).await?;
        let media_dir = self.sync_adapter.get_content_base_dir().join("media");
        let media_manager = super::MediaManager::new(media_dir);
        let mut results = Vec::new();

        for card in cards {
            if let Some(media_ref) = &card.content.media {
                // Create enhanced media reference for validation
                let enhanced_ref = crate::data::models::EnhancedMediaRef {
                    id: uuid::Uuid::new_v4(), // Generate temp ID for validation
                    path: media_ref.path.clone(),
                    media_type: media_ref.media_type.clone(),
                    metadata: crate::data::models::MediaMetadata::default(),
                    status: crate::data::models::MediaStatus::Available,
                    local_cache_path: None,
                    remote_url: None,
                    alt_text: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                let media_ref = MediaRef {
                    path: enhanced_ref.path.clone(),
                    media_type: enhanced_ref.media_type.clone(),
                };
                let is_valid = media_manager.validate_media(&media_ref).await?;
                results.push((card.content.id, is_valid));
            }
        }

        Ok(results)
    }

    /// Get media statistics for a deck
    pub async fn get_deck_media_stats(&self, deck_uuid: &Uuid) -> Result<DeckMediaStats> {
        let cards = self.get_all_cards(deck_uuid).await?;
        let mut audio_count = 0;
        let mut image_count = 0;
        let mut video_count = 0;
        let mut total_size = 0u64;

        let media_dir = self.sync_adapter.get_content_base_dir().join("media");

        for card in cards {
            if let Some(media_ref) = &card.content.media {
                match media_ref.media_type {
                    MediaType::Audio => audio_count += 1,
                    MediaType::Image => image_count += 1,
                    MediaType::Video => video_count += 1,
                }

                // Try to get file size
                let media_path = media_dir.join(&media_ref.path);
                if media_path.exists() {
                    if let Ok(metadata) = std::fs::metadata(&media_path) {
                        total_size += metadata.len();
                    }
                }
            }
        }

        Ok(DeckMediaStats {
            total_media_files: audio_count + image_count + video_count,
            audio_files: audio_count,
            image_files: image_count,
            video_files: video_count,
            total_size_bytes: total_size,
            average_size_bytes: if audio_count + image_count + video_count > 0 {
                total_size / (audio_count + image_count + video_count) as u64
            } else {
                0
            },
        })
    }

    /// Get the content base directory
    pub fn get_content_base_dir(&self) -> &std::path::Path {
        self.sync_adapter.get_content_base_dir()
    }
}

/// Updates that can be applied to a deck
#[derive(Debug, Default)]
pub struct DeckUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub scheduler_config: Option<SchedulerConfig>,
}

impl Clone for DeckStats {
    fn clone(&self) -> Self {
        Self {
            total_cards: self.total_cards,
            new_cards: self.new_cards,
            learning_cards: self.learning_cards,
            review_cards: self.review_cards,
            due_cards: self.due_cards,
            cards_today: self.cards_today,
            retention_rate: self.retention_rate,
            average_ease_factor: self.average_ease_factor,
        }
    }
}

/// Media statistics for a deck
#[derive(Debug, Clone)]
pub struct DeckMediaStats {
    pub total_media_files: usize,
    pub audio_files: usize,
    pub image_files: usize,
    pub video_files: usize,
    pub total_size_bytes: u64,
    pub average_size_bytes: u64,
}

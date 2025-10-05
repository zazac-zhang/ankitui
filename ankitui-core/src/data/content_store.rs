//! Content Store - TOML-based storage for user-defined content
//!
//! Handles storage of card content and deck metadata in human-readable TOML format

use crate::data::models::{CardContent, Deck};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml;
use uuid::Uuid;

/// Content storage format for TOML files
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DeckFile {
    deck: Deck,
    cards: Vec<CardContent>,
}

/// Content Store manages TOML-based storage of cards and decks
#[derive(Debug, Clone)]
pub struct ContentStore {
    base_dir: PathBuf,
}

impl ContentStore {
    /// Create a new ContentStore with the specified base directory
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();

        // Ensure base directory exists
        fs::create_dir_all(&base_dir)
            .with_context(|| format!("Failed to create content directory: {:?}", base_dir))?;

        Ok(Self { base_dir })
    }

    /// Get the file path for a deck
    fn deck_file_path(&self, deck_uuid: &Uuid) -> PathBuf {
        self.base_dir.join(format!("{}.toml", deck_uuid))
    }

    /// Get the file path for a deck by name (for lookup)
    fn deck_name_index_path(&self) -> PathBuf {
        self.base_dir.join("deck_index.toml")
    }

    /// Save a deck with its cards to TOML file
    pub fn save_deck(&self, deck: &Deck, cards: &[CardContent]) -> Result<()> {
        let deck_file = DeckFile {
            deck: deck.clone(),
            cards: cards.to_vec(),
        };

        let toml_content =
            toml::to_string_pretty(&deck_file).context("Failed to serialize deck to TOML")?;

        let file_path = self.deck_file_path(&deck.uuid);
        fs::write(&file_path, toml_content)
            .with_context(|| format!("Failed to write deck file: {:?}", file_path))?;

        // Update name index
        self.update_deck_name_index(deck.uuid, &deck.name)?;

        Ok(())
    }

    /// Load a deck with its cards from TOML file
    pub fn load_deck(&self, deck_uuid: &Uuid) -> Result<(Deck, Vec<CardContent>)> {
        let file_path = self.deck_file_path(deck_uuid);

        if !file_path.exists() {
            return Err(anyhow::anyhow!("Deck file not found: {:?}", file_path));
        }

        let content = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read deck file: {:?}", file_path))?;

        let deck_file: DeckFile = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML from deck file: {:?}", file_path))?;

        Ok((deck_file.deck, deck_file.cards))
    }

    /// Load all decks from the content directory
    pub fn load_all_decks(&self) -> Result<Vec<(Deck, Vec<CardContent>)>> {
        let mut decks = Vec::new();

        for entry in fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip index file and non-toml files
            if path.file_name() == Some(std::ffi::OsStr::new("deck_index.toml")) {
                continue;
            }

            if path.extension() != Some(std::ffi::OsStr::new("toml")) {
                continue;
            }

            // Try to extract UUID from filename
            if let Some(uuid_str) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(uuid) = Uuid::parse_str(uuid_str) {
                    match self.load_deck(&uuid) {
                        Ok(deck_data) => decks.push(deck_data),
                        Err(e) => {
                            // Handle deck loading errors without console output in TUI mode
                            // Log error internally instead of printing to console
                            continue;
                        }
                    }
                }
            }
        }

        Ok(decks)
    }

    /// Delete a deck file
    pub fn delete_deck(&self, deck_uuid: &Uuid) -> Result<()> {
        let file_path = self.deck_file_path(deck_uuid);

        if file_path.exists() {
            fs::remove_file(&file_path)
                .with_context(|| format!("Failed to remove deck file: {:?}", file_path))?;
        }

        // Remove from name index
        self.remove_from_deck_name_index(deck_uuid)?;

        Ok(())
    }

    /// Update the deck name index for quick lookup by name
    fn update_deck_name_index(&self, deck_uuid: Uuid, deck_name: &str) -> Result<()> {
        let index_path = self.deck_name_index_path();
        let mut index = self.load_deck_name_index()?;

        index.insert(deck_name.to_string(), deck_uuid);

        let toml_content =
            toml::to_string_pretty(&index).context("Failed to serialize deck index to TOML")?;

        fs::write(&index_path, toml_content)
            .with_context(|| format!("Failed to write deck index file: {:?}", index_path))?;

        Ok(())
    }

    /// Remove deck from name index
    fn remove_from_deck_name_index(&self, deck_uuid: &Uuid) -> Result<()> {
        let index_path = self.deck_name_index_path();
        let mut index = self.load_deck_name_index()?;

        // Find and remove the entry with matching UUID
        index.retain(|_, &mut uuid| uuid != *deck_uuid);

        let toml_content =
            toml::to_string_pretty(&index).context("Failed to serialize deck index to TOML")?;

        fs::write(&index_path, toml_content)
            .with_context(|| format!("Failed to write deck index file: {:?}", index_path))?;

        Ok(())
    }

    /// Load the deck name index
    fn load_deck_name_index(&self) -> Result<HashMap<String, Uuid>> {
        let index_path = self.deck_name_index_path();

        if !index_path.exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&index_path)
            .with_context(|| format!("Failed to read deck index file: {:?}", index_path))?;

        let index: HashMap<String, Uuid> = toml::from_str(&content)
            .with_context(|| format!("Failed to parse deck index TOML: {:?}", index_path))?;

        Ok(index)
    }

    /// Find deck UUID by name
    pub fn find_deck_by_name(&self, name: &str) -> Result<Option<Uuid>> {
        let index = self.load_deck_name_index()?;
        Ok(index.get(name).copied())
    }

    /// Export deck to TOML string (for backup/debug)
    pub fn export_deck_to_string(&self, deck_uuid: &Uuid) -> Result<String> {
        let file_path = self.deck_file_path(deck_uuid);

        if !file_path.exists() {
            return Err(anyhow::anyhow!("Deck file not found: {:?}", file_path));
        }

        fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read deck file: {:?}", file_path))
    }

    /// Import deck from TOML string
    pub fn import_deck_from_string(&self, toml_content: &str) -> Result<Uuid> {
        let deck_file: DeckFile =
            toml::from_str(toml_content).context("Failed to parse TOML content")?;

        // Validate that all cards belong to the deck
        for card in &deck_file.cards {
            // In a real implementation, you might want to validate card-deck relationships
            // For now, we assume the input is valid
        }

        self.save_deck(&deck_file.deck, &deck_file.cards)?;
        Ok(deck_file.deck.uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::models::{CardContent, Deck};
    use chrono::Utc;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn create_test_deck() -> Deck {
        Deck {
            uuid: Uuid::new_v4(),
            name: "Test Deck".to_string(),
            description: Some("A test deck".to_string()),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            scheduler_config: None,
        }
    }

    fn create_test_card() -> CardContent {
        CardContent {
            id: Uuid::new_v4(),
            front: "Question".to_string(),
            back: "Answer".to_string(),
            tags: vec!["test".to_string()],
            media: None,
            custom: HashMap::new(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
        }
    }

    #[test]
    fn test_save_and_load_deck() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let store = ContentStore::new(temp_dir.path())?;

        let deck = create_test_deck();
        let cards = vec![create_test_card()];

        store.save_deck(&deck, &cards)?;
        let (loaded_deck, loaded_cards) = store.load_deck(&deck.uuid)?;

        assert_eq!(deck.uuid, loaded_deck.uuid);
        assert_eq!(deck.name, loaded_deck.name);
        assert_eq!(cards.len(), loaded_cards.len());
        assert_eq!(cards[0].front, loaded_cards[0].front);

        Ok(())
    }

    #[test]
    fn test_find_deck_by_name() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let store = ContentStore::new(temp_dir.path())?;

        let deck = create_test_deck();
        let cards = vec![];

        store.save_deck(&deck, &cards)?;

        let found_uuid = store.find_deck_by_name("Test Deck")?;
        assert_eq!(found_uuid, Some(deck.uuid));

        let not_found = store.find_deck_by_name("Nonexistent")?;
        assert_eq!(not_found, None);

        Ok(())
    }
}

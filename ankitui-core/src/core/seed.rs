//! Sample deck seeder for onboarding
//!
//! Loads sample decks from TOML files in the `decks/` directory.
//! Each seed deck is imported only if a deck with the same name doesn't already exist.

use crate::DeckManager;
use std::collections::HashSet;
use std::path::Path;

/// Seed file paths relative to the workspace root.
const SEED_FILES: &[&str] = &[
    "decks/linear_algebra.toml",
    "decks/python_basics.toml",
    "decks/dsa.toml",
    "decks/english_vocabulary.toml",
];

/// Extract deck name from TOML content (supports both `[deck]` nested and flat formats)
fn extract_deck_name(content: &str) -> Option<String> {
    // Try `[deck]` nested format first
    if let Some(deck_section) = content.split("[deck]").nth(1) {
        for line in deck_section.lines() {
            let line = line.trim();
            if line.starts_with("name") && line.contains('=') {
                let value = line.split('=').nth(1)?.trim().trim_matches('"');
                return Some(value.to_string());
            }
        }
    }
    // Try flat format: `name = "..."`
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("name") && line.contains('=') && !line.starts_with('#') {
            let value = line.split('=').nth(1)?.trim().trim_matches('"');
            return Some(value.to_string());
        }
    }
    None
}

/// Create sample decks from TOML files, skipping any that already exist by name
pub async fn seed_sample_decks(deck_manager: &DeckManager) -> anyhow::Result<()> {
    // Collect existing deck names
    let decks = deck_manager.get_all_decks().await?;
    let existing_names: HashSet<String> = decks.iter().map(|(d, _)| d.name.clone()).collect();

    let candidates = [
        Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap_or(Path::new(".")),
        Path::new("."),
    ];

    for seed_path in SEED_FILES {
        let mut loaded = false;
        for base in &candidates {
            let full_path = base.join(seed_path);
            if full_path.exists() {
                let content = std::fs::read_to_string(&full_path)?;

                // Skip if a deck with this name already exists
                if let Some(name) = extract_deck_name(&content) {
                    if existing_names.contains(&name) {
                        loaded = true; // Already exists, skip silently
                        break;
                    }
                }

                if let Err(e) = deck_manager.import_deck(&content).await {
                    eprintln!("Warning: failed to import seed deck {}: {}", seed_path, e);
                }
                loaded = true;
                break;
            }
        }
        if !loaded {
            eprintln!("Warning: seed file not found: {}", seed_path);
        }
    }

    Ok(())
}

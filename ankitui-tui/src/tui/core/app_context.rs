//! Application Context
//!
//! Provides shared services and context for UI components

use ankitui_core::Config;
use ankitui_core::{DeckManager, StatsEngine};
use anyhow::Result;
use std::sync::Arc;

/// Application context providing shared services
#[derive(Clone)]
pub struct AppContext {
    /// Deck manager for card operations
    pub deck_manager: Arc<DeckManager>,
    /// Statistics engine
    pub stats_engine: Arc<StatsEngine>,
    /// Application configuration
    pub config: Arc<Config>,
}

impl AppContext {
    /// Create a new application context
    pub fn new(deck_manager: DeckManager, stats_engine: StatsEngine, config: Config) -> Self {
        Self {
            deck_manager: Arc::new(deck_manager),
            stats_engine: Arc::new(stats_engine),
            config: Arc::new(config),
        }
    }

    /// Get deck manager reference
    pub fn deck_manager(&self) -> &DeckManager {
        &self.deck_manager
    }

    /// Get stats engine reference
    pub fn stats_engine(&self) -> &StatsEngine {
        &self.stats_engine
    }

    /// Get config reference
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Component result from handling actions
#[derive(Debug, Clone)]
pub enum ComponentResult {
    /// No state change needed
    None,
    /// Transition to a new app state
    Transition(crate::tui::app::AppState),
    /// Show message to user
    Message(String),
    /// Request data refresh
    Refresh,
    /// Error occurred
    Error(String),
}

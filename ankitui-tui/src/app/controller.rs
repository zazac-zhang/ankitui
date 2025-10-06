//! Application Controller - Coordinates UI and business logic
//!
//! Provides high-level operations that bridge UI components with Core package business logic

use crate::utils::error::{TuiResult, TuiError};
use crate::app::main_app::App;
use crate::ui::state::Screen;
use crate::ui::event::Command;
use ankitui_core::data::CardContent;
use ankitui_core::core::Rating;
use uuid::Uuid;

/// Application Controller - Main business logic coordinator
pub struct AppController<'a> {
    app: &'a mut App,
}

impl<'a> AppController<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }

    /// Navigate to a specific screen
    pub async fn navigate_to_screen(&mut self, screen: Screen) -> TuiResult<()> {
        let screen_clone = screen.clone();
        self.app.navigate_to(screen).await?;
        self.update_state_for_screen(screen_clone).await
    }

    /// Load and display all decks
    pub async fn load_all_decks(&mut self) -> TuiResult<Vec<(ankitui_core::data::models::Deck, Vec<ankitui_core::data::models::Card>)>> {
        self.app.deck_service().get_all_decks().await
    }

    /// Create a new deck
    pub async fn create_deck(&mut self, name: String, description: Option<String>) -> TuiResult<Uuid> {
        let deck_id = self.app.deck_service().create_deck(name.clone(), description).await?;

        // Update UI state
        {
            let mut state = self.app.state_store.write().await;
            state.add_system_message(format!("Deck '{}' created successfully", name));
        }

        Ok(deck_id)
    }

    /// Delete a deck
    pub async fn delete_deck(&mut self, deck_id: Uuid) -> TuiResult<()> {
        // Get deck name for message
        let deck_name = if let Ok((deck, _)) = self.app.deck_service().get_deck(&deck_id).await {
            deck.name
        } else {
            "Unknown".to_string()
        };

        self.app.deck_service().delete_deck(&deck_id).await?;

        // Update UI state
        {
            let mut state = self.app.state_store.write().await;
            state.add_system_message(format!("Deck '{}' deleted successfully", deck_name));
        }

        Ok(())
    }

    /// Start a study session for a deck
    pub async fn start_study_session(&mut self, deck_id: Uuid) -> TuiResult<()> {
        // Navigate to study screen
        self.navigate_to_screen(Screen::StudySession).await?;

        // Start the session using service layer
        self.app.study_service_mut().start_session(deck_id).await?;

        // Load initial cards
        self.load_next_card().await?;

        {
            let mut state = self.app.state_store.write().await;
            state.add_system_message("Study session started".to_string());
        }

        Ok(())
    }

    /// End current study session
    pub async fn end_study_session(&mut self) -> TuiResult<()> {
        if let Ok(stats) = self.app.study_service_mut().end_session().await {
            {
                let mut state = self.app.state_store.write().await;
                state.add_system_message(format!(
                    "Study session ended. Studied {} cards in {:.1} minutes",
                    stats.cards_studied,
                    stats.average_time_seconds
                ));
            }

            // Navigate back to deck selection
            self.navigate_to_screen(Screen::DeckSelection).await?;
        }

        Ok(())
    }

    /// Rate the current card
    pub async fn rate_current_card(&mut self, rating: Rating) -> TuiResult<()> {
        self.app.study_service_mut().rate_current_card(rating).await?;

        // Load next card
        self.load_next_card().await?;

        Ok(())
    }

    /// Skip current card
    pub async fn skip_current_card(&mut self) -> TuiResult<()> {
        self.app.study_service_mut().skip_current_card().await?;

        // Load next card
        self.load_next_card().await?;

        Ok(())
    }

    /// Add new cards to a deck
    pub async fn add_cards(&mut self, deck_id: Uuid, cards: Vec<CardContent>) -> TuiResult<()> {
        self.app.deck_service().add_cards(&deck_id, cards).await?;

        {
            let mut state = self.app.state_store.write().await;
            state.add_system_message("Cards added successfully".to_string());
        }

        Ok(())
    }

    /// Load deck statistics
    pub async fn load_deck_statistics(&mut self, deck_id: Uuid) -> TuiResult<ankitui_core::core::DeckStats> {
        self.app.deck_service().get_deck_statistics(&deck_id).await
    }

    /// Load global statistics
    pub async fn load_global_statistics(&mut self) -> TuiResult<ankitui_core::data::sync_adapter::GlobalStats> {
        self.app.statistics_service().get_global_statistics().await
    }

    /// Handle UI commands
    pub async fn handle_command(&mut self, command: Command) -> TuiResult<()> {
        use crate::ui::event::CommandType;

        match &command.command_type {
            CommandType::NavigateTo(screen) => {
                self.navigate_to_screen(screen.clone()).await?;
            }

            CommandType::CreateDeck(name, description) => {
                self.create_deck(name.clone(), description.clone()).await?;
            }

            CommandType::DeleteDeck(deck_id) => {
                self.delete_deck(*deck_id).await?;
            }

            CommandType::StartStudySession(deck_id, _) => {
                self.start_study_session(*deck_id).await?;
            }

            CommandType::EndStudySession => {
                self.end_study_session().await?;
            }

            CommandType::RateCurrentCard(rating) => {
                // Convert UI rating to Core rating
                let core_rating = match rating {
                    crate::domain::CardRating::Again => Rating::Again,
                    crate::domain::CardRating::Hard => Rating::Hard,
                    crate::domain::CardRating::Good => Rating::Good,
                    crate::domain::CardRating::Easy => Rating::Easy,
                };
                self.rate_current_card(core_rating).await?;
            }

            CommandType::SkipCurrentCard => {
                self.skip_current_card().await?;
            }

            CommandType::LoadDecks => {
                let _decks = self.load_all_decks().await?;
                {
                    let mut state = self.app.state_store.write().await;
                    state.add_system_message("Decks loaded successfully".to_string());
                }
            }

            CommandType::LoadStatistics(deck_id) => {
                let _stats = self.load_deck_statistics(*deck_id).await?;
                self.navigate_to_screen(Screen::Statistics).await?;
            }

            CommandType::RefreshStatistics => {
                let _stats = self.load_global_statistics().await?;
                {
                    let mut state = self.app.state_store.write().await;
                    state.add_system_message("Statistics refreshed".to_string());
                }
            }

            _ => {
                // Handle other commands as needed
            }
        }

        Ok(())
    }

    // Private helper methods

    async fn load_next_card(&mut self) -> TuiResult<()> {
        // Get current deck from navigation state
        let current_deck_id = self.app.navigator().current_deck();

        if let Some(deck_id) = current_deck_id {
            if let Ok(Some(_card)) = self.app.study_service().get_next_card(&deck_id).await {
                // Update state with current card
                {
                    let mut state = self.app.state_store.write().await;
                    state.set_current_card_study(true);
                }
            } else {
                // No more cards available
                {
                    let mut state = self.app.state_store.write().await;
                    state.set_current_card_study(false);
                    state.add_system_message("No more cards to study".to_string());
                }
            }
        }

        Ok(())
    }

    async fn update_state_for_screen(&mut self, screen: Screen) -> TuiResult<()> {
        {
            let mut state = self.app.state_store.write().await;
            state.set_current_screen(screen.clone());

            // Update UI state based on screen
            match screen {
                Screen::DeckSelection => {
                    state.set_loading(false);
                }
                Screen::StudySession => {
                    // Study session specific setup
                }
                Screen::Statistics => {
                    // Statistics specific setup
                }
                _ => {}
            }
        }

        Ok(())
    }
}

// UI Rating enum for command handling
pub enum UiRating {
    Again,
    Hard,
    Good,
    Easy,
}
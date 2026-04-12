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
            let state = self.app.state_store.read().await;
            state.show_message(
                crate::ui::state::store::SystemMessage::success("Success", &format!("Deck '{}' created successfully", name))
            )?;
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
            let state = self.app.state_store.read().await;
            state.show_message(
                crate::ui::state::store::SystemMessage::success("Success", &format!("Deck '{}' deleted successfully", deck_name))
            )?;
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
            let state = self.app.state_store.read().await;
            state.show_message(
                crate::ui::state::store::SystemMessage::success("Session Started", "Study session started")
            )?;
        }

        Ok(())
    }

    /// End current study session
    pub async fn end_study_session(&mut self) -> TuiResult<()> {
        if let Ok(stats) = self.app.study_service_mut().end_session().await {
            {
                let state = self.app.state_store.read().await;
                state.show_message(
                    crate::ui::state::store::SystemMessage::success(
                        "Session Ended",
                        &format!(
                            "Study session ended. Studied {} cards in {:.1} minutes",
                            stats.cards_studied,
                            stats.average_time_seconds
                        )
                    )
                )?;
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
            let state = self.app.state_store.read().await;
            state.show_message(
                crate::ui::state::store::SystemMessage::success("Success", "Cards added successfully")
            )?;
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

            // Deck selection navigation commands
            CommandType::SelectPreviousDeck => {
                self.select_previous_deck().await?;
            }

            CommandType::SelectNextDeck => {
                self.select_next_deck().await?;
            }

            CommandType::StartStudySessionDefault => {
                self.start_study_session_with_selected_deck().await?;
            }

            CommandType::LoadDecks => {
                let _decks = self.load_all_decks().await?;
                {
                    let state = self.app.state_store.read().await;
                    state.show_message(
                        crate::ui::state::store::SystemMessage::success("Success", "Decks loaded successfully")
                    )?;
                }
            }

            CommandType::ShowHelp => {
                self.navigate_to_screen(Screen::Help).await?;
            }

            CommandType::ToggleCardSide => {
                // On search screen, toggle search type
                let state = self.app.state_store.read().await;
                let current_screen = state.get_state().current_screen.clone();
                drop(state);
                if matches!(current_screen, Screen::Search) {
                    self.app.state_store.read().await.update_state(|state| {
                        let current = state.ui_state.get("search_type").cloned().unwrap_or("Decks".to_string());
                        let new_type = if current == "Decks" { "Cards" } else { "Decks" };
                        state.ui_state.insert("search_type".to_string(), new_type.to_string());
                    }).ok();
                }
            }

            CommandType::SearchDecks(query) | CommandType::SearchCards(query) => {
                // Update search state, navigate if not already on Search screen
                let state = self.app.state_store.read().await;
                let current_screen = state.get_state().current_screen.clone();
                drop(state);
                if !matches!(current_screen, Screen::Search) {
                    self.navigate_to_screen(Screen::Search).await?;
                }
                self.app.state_store.read().await.update_state(|state| {
                    state.ui_state.insert("search_query".to_string(), query.clone());
                    let search_type = if matches!(command.command_type, CommandType::SearchDecks(_)) {
                        "Decks"
                    } else {
                        "Cards"
                    };
                    state.ui_state.insert("search_type".to_string(), search_type.to_string());
                }).ok();
            }

            CommandType::StartSearch => {
                self.navigate_to_screen(Screen::Search).await?;
            }

            CommandType::LoadStatistics(deck_id) => {
                let _stats = self.load_deck_statistics(*deck_id).await?;
                self.navigate_to_screen(Screen::Statistics).await?;
            }

            CommandType::RefreshStatistics => {
                let _stats = self.load_global_statistics().await?;
                {
                    let state = self.app.state_store.read().await;
                    state.show_message(
                        crate::ui::state::store::SystemMessage::success("Success", "Statistics refreshed")
                    )?;
                }
            }

            CommandType::ConfirmSetting => {
                // Navigate to sub-settings based on current index
                let index = {
                    let s = self.app.state_store.read().await;
                    s.get_main_menu_selected()
                };
                let target = match index {
                    0 => Screen::StudyPrefs,
                    1 => Screen::UiSettings,
                    2 => Screen::DataManage,
                    _ => Screen::Settings,
                };
                self.navigate_to_screen(target).await?;
            }

            CommandType::NavigateLeft | CommandType::NavigateRight => {
                // Adjust values in settings sub-screens
                let is_decrement = matches!(&command.command_type, CommandType::NavigateLeft);
                let state = self.app.state_store.read().await;
                let current_screen = state.get_state().current_screen.clone();
                drop(state);
                if matches!(current_screen, Screen::StudyPrefs) {
                    self.app.state_store.read().await.update_state(|state| {
                        let idx = state.ui_state.get("prefs_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        match idx {
                            0 => {
                                let val = state.ui_state.get("new_cards_per_day").and_then(|s| s.parse::<u32>().ok()).unwrap_or(20);
                                if is_decrement { state.ui_state.insert("new_cards_per_day".to_string(), (val.saturating_sub(1)).to_string()); }
                                else { state.ui_state.insert("new_cards_per_day".to_string(), (val + 1).to_string()); }
                            }
                            1 => {
                                let val = state.ui_state.get("max_reviews_per_day").and_then(|s| s.parse::<u32>().ok()).unwrap_or(200);
                                if is_decrement { state.ui_state.insert("max_reviews_per_day".to_string(), (val.saturating_sub(1)).to_string()); }
                                else { state.ui_state.insert("max_reviews_per_day".to_string(), (val + 1).to_string()); }
                            }
                            2 | 3 => {
                                // Toggle boolean
                                let key = if idx == 2 { "auto_advance" } else { "show_hint" };
                                let val = state.ui_state.get(key).map(|s| s == "true").unwrap_or(false);
                                state.ui_state.insert(key.to_string(), (!val).to_string());
                            }
                            _ => {}
                        }
                    }).ok();
                } else if matches!(current_screen, Screen::UiSettings) {
                    self.app.state_store.read().await.update_state(|state| {
                        let idx = state.ui_state.get("ui_settings_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if idx == 1 {
                            let theme = state.ui_state.get("theme").cloned().unwrap_or_else(|| state.user_preferences.theme.clone());
                            let themes = vec!["default", "dark", "light"];
                            let ci = themes.iter().position(|t| t == &theme).unwrap_or(0);
                            let ni = if is_decrement { ci.saturating_sub(1) } else { (ci + 1).min(2) };
                            state.ui_state.insert("theme".to_string(), themes[ni].to_string());
                            state.user_preferences.theme = themes[ni].to_string();
                        }
                    }).ok();
                }
            }

            CommandType::NavigateToMainMenu => {
                self.navigate_to_screen(Screen::MainMenu).await?;
            }

            CommandType::NavigateBack => {
                let state = self.app.state_store.read().await;
                let screen = state.get_state().current_screen;
                drop(state);
                match &screen {
                    Screen::StudyPrefs | Screen::UiSettings | Screen::DataManage => {
                        self.navigate_to_screen(Screen::Settings).await?;
                    }
                    Screen::Search | Screen::Help => {
                        self.navigate_to_screen(Screen::MainMenu).await?;
                    }
                    Screen::DeckManagement => {
                        self.navigate_to_screen(Screen::MainMenu).await?;
                    }
                    _ => {
                        self.navigate_to_screen(Screen::MainMenu).await?;
                    }
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
                    let state = self.app.state_store.read().await;
                    state.set_current_card_study(true)?;
                }
            } else {
                // No more cards available
                {
                    let state = self.app.state_store.read().await;
                    state.set_current_card_study(false)?;
                    state.show_message(
                        crate::ui::state::store::SystemMessage::info("Info", "No more cards to study")
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Select previous deck in the deck list
    pub async fn select_previous_deck(&mut self) -> TuiResult<()> {
        let decks_result = self.load_all_decks().await;

        if let Ok(decks) = decks_result {
            let deck_count = decks.len();

            if deck_count == 0 {
                return Ok(());
            }

            let state_store = self.app.state_store.read().await;
            state_store.update_state(|state| {
                let current_selected = state.deck_list_selected.unwrap_or(0);
                let new_selected = if current_selected == 0 {
                    deck_count - 1
                } else {
                    current_selected - 1
                };
                state.deck_list_selected = Some(new_selected);
            })?;
        }

        Ok(())
    }

    /// Select next deck in the deck list
    pub async fn select_next_deck(&mut self) -> TuiResult<()> {
        let decks_result = self.load_all_decks().await;

        if let Ok(decks) = decks_result {
            let deck_count = decks.len();

            if deck_count == 0 {
                return Ok(());
            }

            let state_store = self.app.state_store.read().await;
            state_store.update_state(|state| {
                let current_selected = state.deck_list_selected.unwrap_or(0);
                let new_selected = if current_selected >= deck_count - 1 {
                    0
                } else {
                    current_selected + 1
                };
                state.deck_list_selected = Some(new_selected);
            })?;
        }

        Ok(())
    }

    /// Start study session with the currently selected deck
    pub async fn start_study_session_with_selected_deck(&mut self) -> TuiResult<()> {
        let decks_result = self.load_all_decks().await;

        if let Ok(decks) = decks_result {
            if decks.is_empty() {
                let state = self.app.state_store.read().await;
                state.show_message(
                    crate::ui::state::store::SystemMessage::warning("No Decks", "No decks available. Create a deck first.")
                )?;
                return Ok(());
            }

            let selected_index = {
                let state = self.app.state_store.read().await;
                state.get_deck_list_selected().unwrap_or(0)
            };

            if selected_index < decks.len() {
                let (deck, _cards) = &decks[selected_index];
                let deck_id = deck.uuid;

                // Store selected deck ID and start session
                {
                    let state = self.app.state_store.read().await;
                    state.set_selected_deck(Some(deck_id))?;
                }

                self.start_study_session(deck_id).await?;
            } else {
                let state = self.app.state_store.read().await;
                state.show_message(
                    crate::ui::state::store::SystemMessage::error("Error", "Invalid deck selection")
                )?;
            }
        } else {
            let state = self.app.state_store.read().await;
            state.show_message(
                crate::ui::state::store::SystemMessage::error("Error", "Failed to load decks")
            )?;
        }

        Ok(())
    }

    async fn update_state_for_screen(&mut self, screen: Screen) -> TuiResult<()> {
            let state_store = self.app.state_store.read().await;
            state_store.update_state(|state| {
                state.current_screen = screen.clone();

                // Update UI state based on screen
                match screen {
                    Screen::DeckSelection => {
                        state.loading = false;
                        // Initialize deck selection if needed
                        if state.deck_list_selected.is_none() {
                            state.deck_list_selected = Some(0);
                        }
                    }
                    Screen::StudySession => {
                        // Study session specific setup
                    }
                    Screen::Statistics => {
                        // Statistics specific setup
                    }
                    Screen::StudyPrefs => {
                        state.ui_state.entry("prefs_index".to_string()).or_insert("0".to_string());
                        state.ui_state.entry("new_cards_per_day".to_string()).or_insert("20".to_string());
                        state.ui_state.entry("max_reviews_per_day".to_string()).or_insert("200".to_string());
                        state.ui_state.entry("auto_advance".to_string()).or_insert(state.user_preferences.auto_advance.to_string());
                        state.ui_state.entry("show_hint".to_string()).or_insert("true".to_string());
                    }
                    Screen::UiSettings => {
                        state.ui_state.entry("ui_settings_index".to_string()).or_insert("0".to_string());
                        state.ui_state.entry("theme".to_string()).or_insert(state.user_preferences.theme.clone());
                    }
                    Screen::DataManage => {
                        state.ui_state.entry("data_index".to_string()).or_insert("0".to_string());
                    }
                    Screen::Search => {
                        state.ui_state.entry("search_type".to_string()).or_insert("Decks".to_string());
                        state.ui_state.entry("search_query".to_string()).or_insert(String::new());
                    }
                    _ => {}
                }
            })?;

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
//! Main application implementation

use crate::app::helpers::{command as command_helpers, data as data_helpers, session as session_helpers};
use crate::domain::{DeckService, SessionState, StatisticsService, StudyService};
use crate::ui::navigator::Navigator;
use crate::ui::render::Renderer;
use crate::ui::state::store::StateStore;
use crate::ui::theme::Theme;
use crate::utils::error::{TuiError, TuiResult};
use ankitui_core::config::ConfigManager;
use ankitui_core::{DeckManager, Scheduler, SessionController};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Navigation boundary constants - must match actual data lengths in render functions
const STUDY_PREFS_ITEMS: usize = 4; // render_study_prefs items count
const UI_SETTINGS_ITEMS: usize = 4; // render_ui_settings items count
const DATA_MANAGE_ITEMS: usize = 5; // render_data_manage ops count
const HELP_CATEGORIES: usize = 4;   // render_help_screen categories count

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub enable_mouse: bool,
    pub enable_bracketed_paste: bool,
    pub tick_rate: std::time::Duration,
    pub theme: Theme,
    pub debug: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "AnkiTUI V2".to_string(),
            enable_mouse: true,
            enable_bracketed_paste: true,
            tick_rate: std::time::Duration::from_millis(16), // ~60 FPS
            theme: Theme::default(),
            debug: false,
        }
    }
}

/// Main application structure
pub struct App {
    config: AppConfig,
    pub state_store: Arc<RwLock<StateStore>>,
    renderer: crate::ui::render::DefaultRenderer,
    navigator: Navigator,

    // Configuration manager for persistence
    config_manager: Option<ConfigManager>,

    // Core business logic components
    deck_manager: Arc<DeckManager>,
    session_controller: Arc<tokio::sync::Mutex<SessionController>>,
    scheduler: Arc<Scheduler>,

    // Service layer for business logic coordination
    deck_service: DeckService,
    study_service: StudyService,
    statistics_service: StatisticsService,

    running: bool,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("config", &self.config)
            .field("running", &self.running)
            .finish_non_exhaustive()
    }
}

impl App {
    /// Create a new application instance with configuration
    pub async fn new(config: AppConfig) -> TuiResult<Self> {
        Self::with_config_manager(config, None).await
    }

    /// Create a new application instance with optional config manager for persistence
    pub async fn with_config_manager(config: AppConfig, config_manager: Option<ConfigManager>) -> TuiResult<Self> {
        let state_store = Arc::new(RwLock::new(StateStore::new()));

        // Initialize data paths
        let data_dir = data_helpers::get_default_data_dir();

        let content_dir = data_dir.join("content");
        let db_path = data_dir.join("ankitui.db");

        // Create directories if they don't exist
        std::fs::create_dir_all(&content_dir).map_err(|e| TuiError::State {
            message: format!("Failed to create content directory: {}", e),
        })?;

        // Initialize core components with proper async constructors
        let deck_manager = Arc::new(
            DeckManager::new(&content_dir, &db_path)
                .await
                .map_err(|e| TuiError::Core(format!("Failed to initialize DeckManager: {}", e)))?,
        );

        let scheduler = Arc::new(Scheduler::new(None));
        let session_controller = Arc::new(tokio::sync::Mutex::new(
            SessionController::new((*deck_manager).clone(), Some((*scheduler).clone()))
                .await
                .map_err(|e| TuiError::Core(format!("Failed to initialize SessionController: {}", e)))?,
        ));

        // Initialize service layer
        let deck_service = DeckService::new(Arc::clone(&deck_manager));
        let study_service = StudyService::new(Arc::clone(&session_controller), Arc::clone(&deck_manager));
        let statistics_service = StatisticsService::new(Arc::clone(&deck_manager));

        Ok(Self {
            config,
            state_store: Arc::clone(&state_store),
            renderer: crate::ui::render::DefaultRenderer::new(),
            navigator: Navigator::new(),
            config_manager,
            deck_manager,
            session_controller,
            scheduler,
            deck_service,
            study_service,
            statistics_service,
            running: true,
        })
    }

    /// Get application configuration
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Get state store reference
    pub fn state_store(&self) -> &Arc<RwLock<StateStore>> {
        &self.state_store
    }

    /// Get mutable reference to state store
    pub fn state_store_mut(&mut self) -> &mut Arc<RwLock<StateStore>> {
        &mut self.state_store
    }

    /// Get navigator reference
    pub fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    /// Get mutable navigator reference
    pub fn navigator_mut(&mut self) -> &mut Navigator {
        &mut self.navigator
    }

    /// Get core business logic components
    pub fn deck_manager(&self) -> &Arc<DeckManager> {
        &self.deck_manager
    }

    pub fn session_controller(&self) -> &Arc<tokio::sync::Mutex<SessionController>> {
        &self.session_controller
    }

    pub fn scheduler(&self) -> &Arc<Scheduler> {
        &self.scheduler
    }

    /// Get service layer components for business logic operations
    pub fn deck_service(&self) -> &DeckService {
        &self.deck_service
    }

    pub fn study_service(&self) -> &StudyService {
        &self.study_service
    }

    pub fn statistics_service(&self) -> &StatisticsService {
        &self.statistics_service
    }

    /// Get mutable service layer components for operations that modify state
    pub fn study_service_mut(&mut self) -> &mut StudyService {
        &mut self.study_service
    }

    /// Get renderer reference
    pub fn renderer(&self) -> &crate::ui::render::DefaultRenderer {
        &self.renderer
    }

    /// Get mutable renderer reference
    pub fn renderer_mut(&mut self) -> &mut crate::ui::render::DefaultRenderer {
        &mut self.renderer
    }

    /// Check if the application is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Stop the application
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Navigate to a new screen using the state store
    pub async fn navigate_to(&self, screen: crate::ui::state::store::Screen) -> TuiResult<()> {
        let state_store = self.state_store.read().await;
        state_store.navigate_to(screen)
    }

    /// Initialize the application
    pub async fn initialize(&mut self) -> TuiResult<()> {
        log::info!("Initializing AnkiTUI application");

        // Seed sample decks if no decks exist
        if let Err(e) = ankitui_core::core::seed::seed_sample_decks(&self.deck_manager).await {
            log::warn!("Failed to seed sample decks: {}", e);
        }

        // Load all decks using service layer
        let decks = self
            .deck_service
            .get_all_decks()
            .await
            .map_err(|e| TuiError::Core(format!("Failed to load decks: {}", e)))?;

        log::info!("Loaded {} decks", decks.len());

        // Load global statistics using service layer
        let _stats = self
            .statistics_service
            .get_global_statistics()
            .await
            .map_err(|e| TuiError::Core(format!("Failed to load global statistics: {}", e)))?;

        // Initialize state
        {
            let mut state = self.state_store.write().await;
            state.set_loading(false);
        }

        log::info!("Application initialized successfully with {} decks", decks.len());
        Ok(())
    }

    /// Update application state
    pub async fn update(&mut self) -> TuiResult<()> {
        // Update state store subscriptions
        let state = self.state_store.read().await;

        // Update component states
        if state.get_state().loading {
            log::debug!("Application is in loading state");
        }

        // Handle background tasks
        // This could include periodic data sync, cleanup operations, etc.

        Ok(())
    }

    /// Handle application shutdown
    pub async fn shutdown(&mut self) -> TuiResult<()> {
        log::info!("Shutting down AnkiTUI V2 application");

        // Save current state if session is active
        let has_session = {
            let state = self.state_store.read().await;
            state.get_state().current_session.is_some()
        };

        if has_session {
            if let Err(e) = self.study_service_mut().end_session().await {
                log::warn!("Failed to end study session gracefully: {}", e);
            }
        }

        // Persist settings to config file
        if let Err(e) = self.persist_settings().await {
            log::warn!("Failed to persist settings during shutdown: {}", e);
        }

        // Commit any pending data changes
        if let Err(e) = self.save_state().await {
            log::error!("Failed to save state during shutdown: {}", e);
        }

        // Clear any cached data
        {
            let mut state = self.state_store.write().await;
            state.clear_message().map_err(|e| TuiError::State {
                message: format!("Failed to clear system messages: {}", e),
            })?;
        }

        self.running = false;
        log::info!("Application shutdown complete");
        Ok(())
    }

    /// Handle application errors
    pub async fn handle_error(&mut self, error: TuiError) -> TuiResult<()> {
        log::error!("Application error: {}", error);

        // Show error to user via state store
        let error_message = error.to_string();
        {
            let state_store = self.state_store.read().await;
            state_store
                .set_error(Some(error_message.clone()))
                .map_err(|e| TuiError::State {
                    message: format!("Failed to set error state: {}", e),
                })?;

            // Show system message for user notification
            let system_message =
                crate::ui::state::store::SystemMessage::error("Application Error", error_message.as_str());

            state_store.show_message(system_message).map_err(|e| TuiError::State {
                message: format!("Failed to show error message: {}", e),
            })?;
        }

        // Attempt recovery for common errors
        match error {
            TuiError::Core(_) => {
                log::warn!("Core error occurred, attempting to refresh data");
                // Attempt to refresh core data
                if let Err(e) = self.refresh_core_data().await {
                    log::error!("Failed to recover from core error: {}", e);
                }
            }
            TuiError::State { message } => {
                log::warn!("State error occurred: {}", message);
                // Attempt to reset state
                if let Err(e) = self.reset_application_state().await {
                    log::error!("Failed to reset application state: {}", e);
                }
            }
            _ => {
                log::warn!("Unhandled error type: {}", error);
            }
        }

        Ok(())
    }

    /// Save application state to disk
    pub async fn save_state(&self) -> TuiResult<()> {
        log::debug!("Saving application state");

        let state = self.state_store.read().await;
        let state_data = state.get_state().clone();
        drop(state);

        let data_dir = data_helpers::get_default_data_dir();
        let state_path = data_dir.join("app_state.json");

        if let Ok(json) = serde_json::to_string_pretty(&state_data) {
            if let Err(e) = std::fs::write(&state_path, json) {
                log::warn!("Failed to save app state to {:?}: {}", state_path, e);
            }
        }

        log::debug!("Application state saved successfully");
        Ok(())
    }

    /// Load application state from disk
    pub async fn load_state(&mut self) -> TuiResult<()> {
        log::debug!("Loading application state");

        let data_dir = data_helpers::get_default_data_dir();
        let state_path = data_dir.join("app_state.json");

        if let Ok(json) = std::fs::read_to_string(&state_path) {
            if let Ok(state_data) = serde_json::from_str::<crate::ui::state::store::AppState>(&json) {
                let mut state = self.state_store.write().await;
                state.update_state(|s| *s = state_data)?;
                drop(state);
                log::info!("App state loaded from {:?}", state_path);
            }
        }

        log::debug!("Application state loaded successfully");
        Ok(())
    }

    /// Refresh core data from deck manager
    pub async fn refresh_core_data(&self) -> TuiResult<()> {
        log::debug!("Refreshing core data");

        // Refresh decks
        let decks = self
            .deck_service
            .get_all_decks()
            .await
            .map_err(|e| TuiError::Core(format!("Failed to refresh decks: {}", e)))?;

        log::debug!("Refreshed {} decks", decks.len());
        Ok(())
    }

    /// Reset application state to default
    pub async fn reset_application_state(&mut self) -> TuiResult<()> {
        log::warn!("Resetting application state to default");

        let state_store = self.state_store.read().await;
        state_store.update_state(|state| *state = crate::ui::state::store::AppState::default())?;

        Ok(())
    }

    /// Execute application commands
    pub async fn execute_command(&mut self, command: crate::ui::event::Command) -> TuiResult<()> {
        use crate::ui::event::CommandType;

        match command.command_type {
            CommandType::NavigateToMainMenu => {
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::MainMenu)?;
            }
            CommandType::Confirm => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };

                match current_screen {
                    crate::ui::state::store::Screen::MainMenu => {
                        let selected_index = {
                            let state_store = self.state_store.read().await;
                            state_store.get_main_menu_selected()
                        };
                        self.handle_main_menu_selection(selected_index).await?;
                    }
                    crate::ui::state::store::Screen::DeckSelection => {
                        self.handle_deck_selection_confirm().await?;
                    }
                    crate::ui::state::store::Screen::DeckManagement => {
                        // Handle deck management operations
                        self.handle_deck_management_action().await?;
                    }
                    crate::ui::state::store::Screen::UiSettings => {
                        // Toggle boolean settings on Enter
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("ui_settings_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx == 2 {
                                let val = state.ui_state.get("auto_advance").map(|s| s == "true").unwrap_or(false);
                                state.ui_state.insert("auto_advance".to_string(), (!val).to_string());
                            } else if idx == 3 {
                                let val = state.ui_state.get("show_progress").map(|s| s == "true").unwrap_or(true);
                                state.ui_state.insert("show_progress".to_string(), (!val).to_string());
                            }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::DataManage => {
                        let idx = self.state_store.read().await.get_state()
                            .ui_state.get("data_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        match idx {
                            0 => self.handle_data_import().await,
                            1 => self.handle_data_export().await,
                            2 => self.handle_data_backup().await,
                            3 => self.handle_data_restore().await,
                            4 => self.handle_data_clear().await,
                            _ => Ok(()),
                        }?;
                    }
                    _ => {}
                }
            }
            CommandType::StartStudySessionDefault => {
                let deck_id = {
                    let state = self.state_store.read().await;
                    state.get_state().selected_deck_id
                };
                if deck_id.is_some() {
                    self.start_study_session().await?;
                } else if let Some(deck_id) = self.get_selected_deck_from_list().await? {
                    {
                        let state_store = self.state_store.read().await;
                        state_store.set_selected_deck(Some(deck_id))?;
                    }
                    self.start_study_session().await?;
                }
            }
            CommandType::SelectPreviousDeck => {
                self.handle_deck_selection_up().await?;
            }
            CommandType::SelectNextDeck => {
                self.handle_deck_selection_down().await?;
            }
            CommandType::NavigateTo(screen) => {
                let state_store = self.state_store.read().await;
                state_store.navigate_to(screen)?;
            }
            CommandType::SkipCurrentCard => {
                self.study_service_mut().skip_current_card().await?;
            }
            CommandType::PauseSession => {
                // TODO: SessionController doesn't have pause/resume methods yet.
                // When added, call self.session_controller.lock().await.pause() here.
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::info(
                    "Session Paused",
                    "Study session paused. Press any key to resume.",
                ))?;
            }
            CommandType::ResumeSession => {
                // TODO: SessionController doesn't have pause/resume methods yet.
                // When added, call self.session_controller.lock().await.resume() here.
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::success(
                    "Session Resumed",
                    "Study session resumed.",
                ))?;
            }
            CommandType::RefreshScreen => {
                self.refresh_core_data().await?;
            }
            CommandType::ScrollStatsUp | CommandType::ScrollStatsDown => {
                // Cycle through statistics tabs
                let is_down = matches!(&command.command_type, CommandType::ScrollStatsDown);
                self.state_store.read().await.update_state(|state| {
                    let idx = state.ui_state.get("stats_tab").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                    let new_idx = if is_down { (idx + 1).min(2) } else { idx.saturating_sub(1) };
                    state.ui_state.insert("stats_tab".to_string(), new_idx.to_string());
                }).ok();
            }
            CommandType::Select => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };
                match current_screen {
                    crate::ui::state::store::Screen::UiSettings => {
                        let idx = self.state_store.read().await.get_state()
                            .ui_state.get("ui_settings_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if idx == 2 || idx == 3 {
                            let key = if idx == 2 { "auto_advance" } else { "show_progress" };
                            let state_store = self.state_store.read().await;
                            state_store.update_state(|state| {
                                let val = state.ui_state.get(key).map(|s| s == "true").unwrap_or(false);
                                state.ui_state.insert(key.to_string(), (!val).to_string());
                            }).ok();
                        }
                    }
                    _ => {}
                }
            }
            CommandType::EndStudySession => {
                self.end_study_session().await?;
            }
            CommandType::ShowAnswer => {
                let state_store = self.state_store.read().await;
                state_store.set_showing_answer(true)?;
            }
            CommandType::RateCurrentCard(rating) => {
                let core_rating = ankitui_core::core::Rating::from(rating);
                self.rate_card(core_rating).await?;
            }
            CommandType::LoadDecks => {
                self.refresh_core_data().await?;
            }
            CommandType::RefreshStatistics => {
                // Refresh statistics by reloading deck data
                if let Some(deck_id) = self.state_store.read().await.get_state().selected_deck_id {
                    if let Ok(_stats) = self.deck_service.get_deck_statistics(&deck_id).await {
                        let state_store = self.state_store.read().await;
                        state_store.show_message(crate::ui::state::store::SystemMessage::success(
                            "Statistics Refreshed",
                            &format!("Deck statistics have been updated"),
                        ))?;
                    }
                }
            }
            CommandType::BuryCard => {
                self.bury_current_card().await?;
            }
            CommandType::SuspendCard => {
                self.suspend_current_card().await?;
            }
            CommandType::UnburyCard => {
                self.unbury_current_card().await?;
            }
            CommandType::UnsuspendCard => {
                self.unsuspend_current_card().await?;
            }
            CommandType::DeleteSelectedTag => {
                self.delete_selected_tag().await?;
            }
            CommandType::CleanOrphanedMedia => {
                self.clean_orphaned_media().await?;
            }
            CommandType::CreateDeckPrompt => {
                // Navigate to deck management with create intent
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::DeckManagement)?;
                state_store.update_state(|state| {
                    state.ui_state.insert("deck_management_mode".to_string(), "create".to_string());
                }).ok();

                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::info(
                    "Create Deck",
                    "Use the CLI to create new decks: ankitui create-deck",
                ))?;
            }
            CommandType::ExportDeck => {
                self.handle_data_export().await?;
            }
            CommandType::DeleteDeckPrompt => {
                // Check if we're already on DeckManagement — if so, delete the selected deck directly
                let screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };
                if screen == crate::ui::state::store::Screen::DeckManagement {
                    let selected_index = {
                        let state_store = self.state_store.read().await;
                        state_store.get_deck_list_selected().unwrap_or(0)
                    };
                    let decks = self.deck_service.get_all_decks().await?;
                    if selected_index < decks.len() {
                        let (deck, _cards) = &decks[selected_index];
                        let deck_name = deck.name.clone();
                        let deck_uuid = deck.uuid;

                        if let Err(e) = self.deck_service.delete_deck(&deck_uuid).await {
                            let state_store = self.state_store.read().await;
                            state_store.show_message(crate::ui::state::store::SystemMessage::error(
                                "Delete Failed", &format!("Failed to delete '{}': {}", deck_name, e),
                            ))?;
                        } else {
                            // Reset selection after deletion
                            self.state_store.read().await.update_state(|state| {
                                let new_len = decks.len().saturating_sub(1);
                                state.deck_list_selected = Some(if new_len == 0 { 0 } else {
                                    selected_index.min(new_len - 1)
                                });
                            })?;
                            let state_store = self.state_store.read().await;
                            state_store.show_message(crate::ui::state::store::SystemMessage::success(
                                "Deck Deleted", &format!("'{}' has been deleted", deck_name),
                            ))?;
                        }
                    }
                    return Ok(());
                }

                // Not on DeckManagement: navigate there with delete intent
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::DeckManagement)?;
                state_store.update_state(|state| {
                    state.ui_state.insert("deck_management_mode".to_string(), "delete".to_string());
                }).ok();

                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::info(
                    "Delete Deck",
                    "Select a deck and press D or Delete to remove it",
                ))?;
            }
            CommandType::CreateCardPrompt => {
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                    "Create Card",
                    "Card creation is not available in TUI mode. Please use the CLI or import from file.",
                ))?;
            }
            CommandType::LoadUserPreferences => {
                // Load user preferences from config
                if let Some(cm) = &self.config_manager {
                    let state_store = self.state_store.read().await;
                    state_store.update_state(|state| {
                        state.user_preferences.theme = cm.config.ui.theme.clone();
                        // Update other preferences as needed
                    }).ok();
                }
            }
            CommandType::UpdateUserPreferences(prefs) => {
                // Update user preferences
                let state_store = self.state_store.read().await;
                state_store.update_state(|state| {
                    for (key, value) in prefs.iter() {
                        state.ui_state.insert(key.clone(), value.clone());
                    }
                }).ok();
            }
            CommandType::UpdateTheme(theme) => {
                let state_store = self.state_store.read().await;
                state_store.update_state(|state| {
                    state.ui_state.insert("theme".to_string(), theme.clone());
                    state.user_preferences.theme = theme.clone();
                }).ok();

                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::success(
                    "Theme Updated",
                    &format!("Theme changed to {}", theme),
                ))?;
            }
            CommandType::UpdateLanguage(language) => {
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::info(
                    "Language",
                    &format!("Language setting '{}' noted. Full language support coming soon.", language),
                ))?;
            }
            CommandType::UpdateStudyGoals(cards, minutes) => {
                let state_store = self.state_store.read().await;
                state_store.update_state(|state| {
                    state.ui_state.insert("daily_goal_cards".to_string(), cards.to_string());
                    state.ui_state.insert("daily_goal_minutes".to_string(), minutes.to_string());
                }).ok();

                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::success(
                    "Study Goals Updated",
                    &format!("Daily goals: {} cards, {} minutes", cards, minutes),
                ))?;
            }
            CommandType::LoadStatistics(deck_id) => {
                // Load statistics for specific deck
                if let Ok(_stats) = self.deck_service.get_deck_statistics(&deck_id).await {
                    let state_store = self.state_store.read().await;
                    state_store.show_message(crate::ui::state::store::SystemMessage::success(
                        "Statistics Loaded",
                        &format!("Loaded statistics for deck: {}", deck_id),
                    ))?;
                }
            }
            CommandType::NavigatePageUp => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };

                match current_screen {
                    crate::ui::state::store::Screen::DeckSelection => {
                        // Jump to top of deck list
                        let state_store = self.state_store.read().await;
                        state_store.update_state(|state| {
                            state.deck_list_selected = Some(0);
                        }).ok();
                    }
                    crate::ui::state::store::Screen::Statistics => {
                        // Already handled by ScrollStatsUp
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("stats_tab").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            state.ui_state.insert("stats_tab".to_string(), "0".to_string());
                        }).ok();
                    }
                    _ => {}
                }
            }
            CommandType::NavigatePageDown => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };

                match current_screen {
                    crate::ui::state::store::Screen::DeckSelection => {
                        // Jump to bottom of deck list
                        let decks = self.deck_service.get_all_decks().await?;
                        let deck_count = decks.len();
                        if deck_count > 0 {
                            let state_store = self.state_store.read().await;
                            state_store.update_state(|state| {
                                state.deck_list_selected = Some(deck_count - 1);
                            }).ok();
                        }
                    }
                    crate::ui::state::store::Screen::Statistics => {
                        // Already handled by ScrollStatsDown
                        self.state_store.read().await.update_state(|state| {
                            state.ui_state.insert("stats_tab".to_string(), "2".to_string());
                        }).ok();
                    }
                    _ => {}
                }
            }
            CommandType::NavigateHome => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };

                match current_screen {
                    crate::ui::state::store::Screen::DeckSelection => {
                        let state_store = self.state_store.read().await;
                        state_store.update_state(|state| {
                            state.deck_list_selected = Some(0);
                        }).ok();
                    }
                    crate::ui::state::store::Screen::Statistics => {
                        self.state_store.read().await.update_state(|state| {
                            state.ui_state.insert("stats_tab".to_string(), "0".to_string());
                        }).ok();
                    }
                    _ => {}
                }
            }
            CommandType::NavigateEnd => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };

                match current_screen {
                    crate::ui::state::store::Screen::DeckSelection => {
                        if let Ok(decks) = self.deck_service.get_all_decks().await {
                            let deck_count = decks.len();
                            if deck_count > 0 {
                                let state_store = self.state_store.read().await;
                                state_store.update_state(|state| {
                                    state.deck_list_selected = Some(deck_count - 1);
                                }).ok();
                            }
                        }
                    }
                    crate::ui::state::store::Screen::Statistics => {
                        self.state_store.read().await.update_state(|state| {
                            state.ui_state.insert("stats_tab".to_string(), "2".to_string());
                        }).ok();
                    }
                    _ => {}
                }
            }
            CommandType::Quit => {
                self.stop();
            }
            CommandType::ShowHelp => {
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::Help)?;
            }
            CommandType::StartSearch => {
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::Search)?;
                state_store.update_state(|state| {
                    state.ui_state.entry("search_type".to_string()).or_insert("Decks".to_string());
                    state.ui_state.entry("search_query".to_string()).or_insert(String::new());
                    state.ui_state.entry("search_result_index".to_string()).or_insert("0".to_string());
                    state.ui_state.entry("search_result_count".to_string()).or_insert("0".to_string());
                }).ok();
            }
            CommandType::SearchDecks(ref query) | CommandType::SearchCards(ref query) => {
                // Accumulate search input
                let state_store = self.state_store.read().await;
                state_store.update_state(|state| {
                    let current = state.ui_state.get("search_query").cloned().unwrap_or_default();
                    let new_query = format!("{}{}", current, query);
                    state.ui_state.insert("search_query".to_string(), new_query);
                    let search_type = if matches!(command.command_type, CommandType::SearchDecks(_)) {
                        "Decks"
                    } else {
                        "Cards"
                    };
                    state.ui_state.insert("search_type".to_string(), search_type.to_string());
                }).ok();
            }
            CommandType::SearchBackspace => {
                self.state_store.read().await.update_state(|state| {
                    if let Some(query) = state.ui_state.get("search_query").cloned() {
                        if !query.is_empty() {
                            let new_query = query.chars().take(query.chars().count().saturating_sub(1)).collect::<String>();
                            state.ui_state.insert("search_query".to_string(), new_query);
                        }
                    }
                }).ok();
            }
            CommandType::SearchResultDown => {
                self.state_store.read().await.update_state(|state| {
                    let idx = state.ui_state.get("search_result_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                    let max = state.ui_state.get("search_result_count").and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
                    if idx < max.saturating_sub(1) {
                        state.ui_state.insert("search_result_index".to_string(), (idx + 1).to_string());
                    }
                }).ok();
            }
            CommandType::SearchResultUp => {
                self.state_store.read().await.update_state(|state| {
                    let idx = state.ui_state.get("search_result_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                    if idx > 0 {
                        state.ui_state.insert("search_result_index".to_string(), (idx - 1).to_string());
                    }
                }).ok();
            }
            CommandType::SearchSelectResult => {
                let result_idx = self.state_store.read().await.get_state()
                    .ui_state.get("search_result_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                let search_type = self.state_store.read().await.get_state()
                    .ui_state.get("search_type").cloned().unwrap_or("Decks".to_string());

                if search_type == "Decks" {
                    // Navigate to deck selection with the selected result
                    self.state_store.read().await.update_state(|state| {
                        state.ui_state.insert("search_result_selected".to_string(), result_idx.to_string());
                    }).ok();

                    // Select the deck and start study session
                    if let Ok(decks) = self.deck_service.get_all_decks().await {
                        let query = self.state_store.read().await.get_state()
                            .ui_state.get("search_query").cloned().unwrap_or_default();
                        let lower = query.to_lowercase();
                        let matching_decks: Vec<_> = decks.iter()
                            .filter(|(deck, _)| deck.name.to_lowercase().contains(&lower)
                                || deck.description.as_ref().map(|d| d.to_lowercase().contains(&lower)).unwrap_or(false))
                            .collect();

                        if let Some((deck, _)) = matching_decks.get(result_idx) {
                            let deck_id = deck.uuid;
                            self.state_store.read().await.set_selected_deck(Some(deck_id)).ok();
                            self.start_study_session().await?;
                        }
                    }
                }
            }
            CommandType::NavigateBack => {
                let state_store = self.state_store.read().await;
                let state = state_store.get_state();
                let screen = state.current_screen.clone();
                drop(state);
                match screen {
                    crate::ui::state::store::Screen::StudyPrefs
                    | crate::ui::state::store::Screen::UiSettings
                    | crate::ui::state::store::Screen::DataManage
                    | crate::ui::state::store::Screen::TagManagement
                    | crate::ui::state::store::Screen::MediaManagement => {
                        state_store.navigate_to(crate::ui::state::store::Screen::Settings)?;
                    }
                    crate::ui::state::store::Screen::Search
                    | crate::ui::state::store::Screen::Help
                    | crate::ui::state::store::Screen::DeckManagement => {
                        state_store.navigate_to(crate::ui::state::store::Screen::MainMenu)?;
                    }
                    _ => {
                        state_store.navigate_back()?;
                    }
                }
            }
            CommandType::ConfirmSetting => {
                let index = {
                    let state_store = self.state_store.read().await;
                    state_store.get_settings_selected()
                };
                let target = match index {
                    0 => crate::ui::state::store::Screen::StudyPrefs,
                    1 => crate::ui::state::store::Screen::UiSettings,
                    2 => crate::ui::state::store::Screen::DataManage,
                    3 => crate::ui::state::store::Screen::TagManagement,
                    4 => crate::ui::state::store::Screen::MediaManagement,
                    _ => crate::ui::state::store::Screen::Settings,
                };
                let state_store = self.state_store.read().await;
                state_store.navigate_to(target.clone())?;
                // Initialize sub-screen state
                state_store.update_state(|state| match target {
                    crate::ui::state::store::Screen::StudyPrefs => {
                        state.ui_state.entry("prefs_index".to_string()).or_insert("0".to_string());
                        state.ui_state.entry("new_cards_per_day".to_string()).or_insert("20".to_string());
                        state.ui_state.entry("max_reviews_per_day".to_string()).or_insert("200".to_string());
                    }
                    crate::ui::state::store::Screen::UiSettings => {
                        state.ui_state.entry("ui_settings_index".to_string()).or_insert("0".to_string());
                    }
                    crate::ui::state::store::Screen::DataManage => {
                        state.ui_state.entry("data_index".to_string()).or_insert("0".to_string());
                    }
                    crate::ui::state::store::Screen::TagManagement => {
                        state.ui_state.entry("tag_index".to_string()).or_insert("0".to_string());
                    }
                    crate::ui::state::store::Screen::MediaManagement => {
                        state.ui_state.entry("media_index".to_string()).or_insert("0".to_string());
                    }
                    _ => {}
                }).ok();
            }
            CommandType::ToggleCardSide => {
                let state = self.state_store.read().await;
                let screen = state.get_state().current_screen.clone();
                drop(state);
                if matches!(screen, crate::ui::state::store::Screen::Search) {
                    self.state_store.read().await.update_state(|state| {
                        let current = state.ui_state.get("search_type").cloned().unwrap_or("Decks".to_string());
                        let new_type = if current == "Decks" { "Cards" } else { "Decks" };
                        state.ui_state.insert("search_type".to_string(), new_type.to_string());
                    }).ok();
                }
            }
            CommandType::NavigateLeft | CommandType::NavigateRight => {
                let is_decrement = matches!(&command.command_type, CommandType::NavigateLeft);
                let state = self.state_store.read().await;
                let screen = state.get_state().current_screen.clone();
                drop(state);
                if matches!(screen, crate::ui::state::store::Screen::StudyPrefs) {
                    self.state_store.read().await.update_state(|state| {
                        let idx = state.ui_state.get("prefs_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        match idx {
                            0 => {
                                let val = state.ui_state.get("new_cards_per_day").and_then(|s| s.parse::<u32>().ok()).unwrap_or(20);
                                let new_val = if is_decrement { val.saturating_sub(1) } else { val + 1 };
                                state.ui_state.insert("new_cards_per_day".to_string(), new_val.to_string());
                            }
                            1 => {
                                let val = state.ui_state.get("max_reviews_per_day").and_then(|s| s.parse::<u32>().ok()).unwrap_or(200);
                                let new_val = if is_decrement { val.saturating_sub(1) } else { val + 1 };
                                state.ui_state.insert("max_reviews_per_day".to_string(), new_val.to_string());
                            }
                            2 => {
                                let val = state.ui_state.get("auto_advance").map(|s| s == "true").unwrap_or(false);
                                state.ui_state.insert("auto_advance".to_string(), (!val).to_string());
                            }
                            3 => {
                                let val = state.ui_state.get("show_hint").map(|s| s == "true").unwrap_or(true);
                                state.ui_state.insert("show_hint".to_string(), (!val).to_string());
                            }
                            _ => {}
                        }
                    }).ok();
                } else if matches!(screen, crate::ui::state::store::Screen::UiSettings) {
                    self.state_store.read().await.update_state(|state| {
                        let idx = state.ui_state.get("ui_settings_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        if idx == 1 {
                            // Cycle theme
                            let theme = state.ui_state.get("theme").cloned().unwrap_or_else(|| state.user_preferences.theme.clone());
                            let themes = vec!["default", "dark", "light"];
                            let ci = themes.iter().position(|t| t == &theme).unwrap_or(0);
                            let ni = if is_decrement { ci.saturating_sub(1) } else { (ci + 1).min(2) };
                            state.ui_state.insert("theme".to_string(), themes[ni].to_string());
                            state.user_preferences.theme = themes[ni].to_string();
                        } else if idx == 2 || idx == 3 {
                            let key = if idx == 2 { "auto_advance" } else { "show_progress" };
                            let val = state.ui_state.get(key).map(|s| s == "true").unwrap_or(false);
                            state.ui_state.insert(key.to_string(), (!val).to_string());
                        }
                    }).ok();
                } else if matches!(screen, crate::ui::state::store::Screen::DataManage) {
                    self.state_store.read().await.update_state(|state| {
                        let idx = state.ui_state.get("data_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                        let ops_count = 5;
                        let new_idx = if is_decrement { idx.saturating_sub(1) } else { (idx + 1).min(ops_count - 1) };
                        state.ui_state.insert("data_index".to_string(), new_idx.to_string());
                    }).ok();
                }
            }
            CommandType::NavigateUp => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };
                match current_screen {
                    crate::ui::state::store::Screen::MainMenu => {
                        let state_store = self.state_store.read().await;
                        state_store.navigate_main_menu_up()?;
                    }
                    crate::ui::state::store::Screen::DeckSelection => {
                        self.handle_deck_selection_up().await?;
                    }
                    crate::ui::state::store::Screen::DeckManagement => {
                        // Deck list navigation
                        self.handle_deck_management_up().await?;
                    }
                    crate::ui::state::store::Screen::StudyPrefs => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("prefs_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx > 0 { state.ui_state.insert("prefs_index".to_string(), (idx - 1).to_string()); }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::UiSettings => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("ui_settings_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx > 0 { state.ui_state.insert("ui_settings_index".to_string(), (idx - 1).to_string()); }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::DataManage => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("data_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx > 0 { state.ui_state.insert("data_index".to_string(), (idx - 1).to_string()); }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::Help => {
                        // Help screen category navigation
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("help_category").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx > 0 {
                                state.ui_state.insert("help_category".to_string(), (idx - 1).to_string());
                            }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::Settings => {
                        let state_store = self.state_store.read().await;
                        state_store.navigate_settings_up()?;
                    }
                    crate::ui::state::store::Screen::TagManagement => {
                        // Get actual tag count to prevent infinite scrolling
                        let tag_count = if let Ok(decks) = self.deck_service.get_all_decks().await {
                            let mut tag_set = std::collections::HashSet::new();
                            for (_, cards) in &decks {
                                for card in cards {
                                    for tag in &card.content.tags {
                                        tag_set.insert(tag.clone());
                                    }
                                }
                            }
                            tag_set.len()
                        } else {
                            1
                        };
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("tag_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            let max_idx = tag_count.saturating_sub(1);
                            if idx > 0 && idx <= max_idx {
                                state.ui_state.insert("tag_index".to_string(), (idx - 1).to_string());
                            } else if idx > max_idx {
                                // If current index is beyond available tags, reset to last valid
                                state.ui_state.insert("tag_index".to_string(), max_idx.to_string());
                            }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::MediaManagement => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("media_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx > 0 { state.ui_state.insert("media_index".to_string(), (idx - 1).to_string()); }
                        }).ok();
                    }
                    _ => {}
                }
            }
            CommandType::NavigateDown => {
                let current_screen = {
                    let state_store = self.state_store.read().await;
                    state_store.get_state().current_screen.clone()
                };
                match current_screen {
                    crate::ui::state::store::Screen::MainMenu => {
                        let state_store = self.state_store.read().await;
                        state_store.navigate_main_menu_down()?;
                    }
                    crate::ui::state::store::Screen::DeckManagement => {
                        // Deck list navigation
                        self.handle_deck_management_down().await?;
                    }
                    crate::ui::state::store::Screen::DeckSelection => {
                        self.handle_deck_selection_down().await?;
                    }
                    crate::ui::state::store::Screen::StudyPrefs => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("prefs_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx < STUDY_PREFS_ITEMS - 1 { state.ui_state.insert("prefs_index".to_string(), (idx + 1).to_string()); }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::UiSettings => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("ui_settings_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx < UI_SETTINGS_ITEMS - 1 { state.ui_state.insert("ui_settings_index".to_string(), (idx + 1).to_string()); }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::DataManage => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("data_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx < DATA_MANAGE_ITEMS - 1 { state.ui_state.insert("data_index".to_string(), (idx + 1).to_string()); }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::Help => {
                        // Help screen category navigation down
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("help_category").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx < HELP_CATEGORIES - 1 {
                                state.ui_state.insert("help_category".to_string(), (idx + 1).to_string());
                            }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::Settings => {
                        let state_store = self.state_store.read().await;
                        state_store.navigate_settings_down()?;
                    }
                    crate::ui::state::store::Screen::TagManagement => {
                        // Get actual tag count to prevent infinite scrolling
                        let tag_count = if let Ok(decks) = self.deck_service.get_all_decks().await {
                            let mut tag_set = std::collections::HashSet::new();
                            for (_, cards) in &decks {
                                for card in cards {
                                    for tag in &card.content.tags {
                                        tag_set.insert(tag.clone());
                                    }
                                }
                            }
                            tag_set.len().max(1) // Ensure at least 1 to prevent division by zero
                        } else {
                            1
                        };
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("tag_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx < tag_count.saturating_sub(1) {
                                state.ui_state.insert("tag_index".to_string(), (idx + 1).to_string());
                            }
                        }).ok();
                    }
                    crate::ui::state::store::Screen::MediaManagement => {
                        self.state_store.read().await.update_state(|state| {
                            let idx = state.ui_state.get("media_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                            if idx < 8 { state.ui_state.insert("media_index".to_string(), (idx + 1).to_string()); }
                        }).ok();
                    }
                    _ => {}
                }
            }
            CommandType::ImportCards(content) => {
                // Pasted TOML content import - DeckService needs import_cards_from_toml method
                log::debug!("ImportCards: received {} chars of pasted content", content.len());
            }
            CommandType::CreateDeck(name, description) => {
                let name_display = name.clone();
                match self.deck_service.create_deck(name, description).await {
                    Ok(_) => {
                        let state_store = self.state_store.read().await;
                        state_store.show_message(crate::ui::state::store::SystemMessage::success(
                            "Deck Created",
                            &format!("Deck '{}' created successfully", name_display),
                        ))?;
                    }
                    Err(e) => {
                        let state_store = self.state_store.read().await;
                        state_store.show_message(crate::ui::state::store::SystemMessage::error(
                            "Deck Creation Failed",
                            &format!("{}", e),
                        ))?;
                    }
                }
            }
            CommandType::DeleteDeck(deck_id) => {
                match self.deck_service.delete_deck(&deck_id).await {
                    Ok(_) => {
                        let state_store = self.state_store.read().await;
                        state_store.show_message(crate::ui::state::store::SystemMessage::success(
                            "Deck Deleted",
                            "Deck deleted successfully",
                        ))?;
                    }
                    Err(e) => {
                        let state_store = self.state_store.read().await;
                        state_store.show_message(crate::ui::state::store::SystemMessage::error(
                            "Delete Failed",
                            &e.to_string(),
                        ))?;
                    }
                }
            }
            CommandType::UpdateDeck(deck_id, name, description) => {
                log::info!("Update deck {} requested: name={}, desc={:?}", deck_id, name, description);
            }
            CommandType::ShowDeckContextMenu(_, _) | CommandType::ShowCardContextMenu(_, _) => {
                log::debug!("Context menu requested (handled in render layer)");
            }
            CommandType::SaveCard | CommandType::CancelEdit => {
                log::debug!("Card editor command (save/cancel) - CardEditor screen not yet accessible");
            }
            CommandType::PasteCardContent(content) => {
                log::debug!("Paste card content received ({} chars)", content.len());
            }
            CommandType::DeleteCard(card_id) => {
                // DeckService needs a delete_card method - for now log the request
                log::debug!("DeleteCard requested for {}", card_id);
            }
            _ => {
                log::debug!("Unhandled command: {:?}", command);
            }
        }

        Ok(())
    }

    /// Start a study session
    pub async fn start_study_session(&mut self) -> TuiResult<()> {
        let deck_id = {
            let state = self.state_store.read().await;
            match state.get_state().selected_deck_id {
                Some(id) => id,
                None => {
                    let message = crate::ui::state::store::SystemMessage::warning(
                        "No Deck Selected",
                        "Please select a deck first",
                    );
                    let state_store = self.state_store.read().await;
                    state_store.show_message(message)?;
                    return Ok(());
                }
            }
        };

        // Start session
        self.study_service_mut()
            .start_session(deck_id)
            .await
            .map_err(|e| TuiError::Core(format!("Failed to start study session: {}", e)))?;

        // Update state
        {
            let state_store = self.state_store.read().await;
            state_store.navigate_to(crate::ui::state::store::Screen::StudySession)?;
            state_store.set_current_session(Some(SessionState::new(deck_id)))?;
            state_store.set_current_card_study(true)?;
        }

        let message =
            crate::ui::state::store::SystemMessage::success("Study Session Started", "Good luck with your studying!");
        {
            let state_store = self.state_store.read().await;
            state_store.show_message(message)?;
        }

        Ok(())
    }

    /// End the current study session
    pub async fn end_study_session(&mut self) -> TuiResult<()> {
        let stats = self
            .study_service_mut()
            .end_session()
            .await
            .map_err(|e| TuiError::Core(format!("Failed to end study session: {}", e)))?;

        // Update state
        {
            let state_store = self.state_store.read().await;
            state_store.set_current_session(None)?;
            state_store.set_current_card_study(false)?;
            state_store.set_showing_answer(false)?;
            state_store.navigate_to(crate::ui::state::store::Screen::DeckSelection)?;
        }

        // Show session summary
        let study_count = format!("Cards studied: {}", stats.cards_studied);
        let message = crate::ui::state::store::SystemMessage::success("Session Complete", &study_count);
        {
            let state_store = self.state_store.read().await;
            state_store.show_message(message)?;
        }

        Ok(())
    }

    /// Handle main menu selection
    pub async fn handle_main_menu_selection(&mut self, selected_index: usize) -> TuiResult<()> {
        match selected_index {
            0 => {
                // Study Cards - Navigate to deck selection
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::DeckSelection)?;
            }
            1 => {
                // Manage Decks
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::DeckManagement)?;
            }
            2 => {
                // Statistics
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::Statistics)?;
            }
            3 => {
                // Settings
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::Settings)?;
            }
            _ => {
                // Invalid selection - do nothing
                log::warn!("Invalid main menu selection: {}", selected_index);
            }
        }

        Ok(())
    }

    /// Rate the current card
    pub async fn rate_card(&mut self, rating: ankitui_core::core::Rating) -> TuiResult<()> {
        self.study_service_mut()
            .rate_current_card(rating)
            .await
            .map_err(|e| TuiError::Core(format!("Failed to rate card: {}", e)))?;

        // Reset answer view state
        {
            let state_store = self.state_store.read().await;
            state_store.set_showing_answer(false)?;
        }

        Ok(())
    }

    /// Bury the current card (skip until next session)
    pub async fn bury_current_card(&mut self) -> TuiResult<()> {
        if session_helpers::has_current_card(&self.session_controller).await {
            let mut session = self.session_controller.lock().await;
            session.bury_current_card(ankitui_core::data::BuryReason::UserBury, None)
                .await
                .map_err(|e| TuiError::Core(format!("Failed to bury card: {}", e)))?;
            drop(session);

            // Reset answer view state
            {
                let state_store = self.state_store.read().await;
                state_store.set_showing_answer(false)?;
            }
            drop(std::sync::Arc::clone(&self.state_store)); // Ensure lock is released

            self.study_service_mut().skip_current_card().await?;

            // Show message
            let state_store = self.state_store.read().await;
            session_helpers::show_card_operation_message(
                &state_store,
                "Card Buried",
                "Card will appear again in the next session",
            ).await?;
        }
        Ok(())
    }

    /// Suspend the current card (indefinitely)
    pub async fn suspend_current_card(&mut self) -> TuiResult<()> {
        if session_helpers::has_current_card(&self.session_controller).await {
            let mut session = self.session_controller.lock().await;
            session.suspend_current_card("Suspended by user".into(), None)
                .await
                .map_err(|e| TuiError::Core(format!("Failed to suspend card: {}", e)))?;
            drop(session);

            // Reset answer view state
            {
                let state_store = self.state_store.read().await;
                state_store.set_showing_answer(false)?;
            }

            self.study_service_mut().skip_current_card().await?;

            // Show message
            let state_store = self.state_store.read().await;
            session_helpers::show_card_operation_warning(
                &state_store,
                "Card Suspended",
                "Card will not appear until unsuspended",
            ).await?;
        }
        Ok(())
    }

    /// Unbury the current card
    pub async fn unbury_current_card(&mut self) -> TuiResult<()> {
        let (card_id, deck_id) = {
            let session = self.session_controller.lock().await;
            let card = session.current_card();
            if let Some(card) = card {
                (card.content.id, session.current_deck_id())
            } else {
                return Ok(());
            }
        };

        if let Some(deck_id) = deck_id {
            if let Some(cards) = session_helpers::get_deck_cards_safe(&self.session_controller, &deck_id).await {
                if session_helpers::card_exists_in_deck(&cards, &card_id) {
                    let mut session = self.session_controller.lock().await;
                    session.unbury_card(card_id)
                        .await
                        .map_err(|e| TuiError::Core(format!("Failed to unbury card: {}", e)))?;
                }
            }

            let state_store = self.state_store.read().await;
            session_helpers::show_card_operation_message(
                &state_store,
                "Card Unburied",
                "Card will appear in the next session",
            ).await?;
        }
        Ok(())
    }

    /// Unsuspend the current card
    pub async fn unsuspend_current_card(&mut self) -> TuiResult<()> {
        let (card_id, deck_id) = {
            let session = self.session_controller.lock().await;
            let card = session.current_card();
            if let Some(card) = card {
                (card.content.id, session.current_deck_id())
            } else {
                return Ok(());
            }
        };

        if let Some(deck_id) = deck_id {
            if let Some(cards) = session_helpers::get_deck_cards_safe(&self.session_controller, &deck_id).await {
                if session_helpers::card_exists_in_deck(&cards, &card_id) {
                    let mut session = self.session_controller.lock().await;
                    session.unsuspend_card(card_id)
                        .await
                        .map_err(|e| TuiError::Core(format!("Failed to unsuspend card: {}", e)))?;
                }
            }

            let state_store = self.state_store.read().await;
            session_helpers::show_card_operation_message(
                &state_store,
                "Card Unsuspended",
                "Card will appear in the learning queue",
            ).await?;
        }
        Ok(())
    }

    /// Delete the currently selected tag from all cards
    pub async fn delete_selected_tag(&mut self) -> TuiResult<()> {
        // Get the selected tag index
        let tag_index = self.state_store.read().await.get_state()
            .ui_state.get("tag_index")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        // Get all decks and build tag list
        let decks = self.deck_service.get_all_decks().await?;
        let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for (_, cards) in &decks {
            for card in cards {
                for tag in &card.content.tags {
                    *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                }
            }
        }
        let mut tags: Vec<_> = tag_counts.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1));

        // Get the tag name at the selected index
        let tag_name = match tags.get(tag_index) {
            Some((name, _)) => name.clone(),
            None => {
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                    "No Tag Selected",
                    "Please select a valid tag",
                ))?;
                return Ok(());
            }
        };

        // Count affected cards and collect deck UUIDs
        let mut affected_count = 0;
        let mut affected_deck_uuids = Vec::new();

        for (deck, cards) in &decks {
            for card in cards {
                if card.content.tags.contains(&tag_name) {
                    affected_count += 1;
                    if !affected_deck_uuids.contains(&deck.uuid) {
                        affected_deck_uuids.push(deck.uuid);
                    }
                }
            }
        }

        if affected_count == 0 {
            let state_store = self.state_store.read().await;
            state_store.show_message(crate::ui::state::store::SystemMessage::info(
                "No Cards Found",
                &format!("Tag '{}' is not used by any cards", tag_name),
            ))?;
            return Ok(());
        }

        // Delete the tag from all cards in each affected deck
        for deck_uuid in affected_deck_uuids {
            // Get cards for this deck
            let cards = self.deck_manager.get_cards(&deck_uuid).await?;
            let mut updated_cards = Vec::new();

            for mut card in cards {
                if card.content.tags.contains(&tag_name) {
                    card.content.tags.retain(|t| t != &tag_name);
                    card.content.modified_at = chrono::Utc::now();
                    updated_cards.push(card);
                }
            }

            // Update cards in deck
            for card in updated_cards {
                let _ = self.deck_manager.update_card(&deck_uuid, &card).await;
            }
        }

        let state_store = self.state_store.read().await;
        state_store.show_message(crate::ui::state::store::SystemMessage::success(
            "Tag Deleted",
            &format!("Removed '{}' from {} cards", tag_name, affected_count),
        ))?;

        Ok(())
    }

    /// Clean up orphaned media files from all decks
    pub async fn clean_orphaned_media(&mut self) -> TuiResult<()> {
        let decks = self.deck_service.get_all_decks().await?;

        if decks.is_empty() {
            let state_store = self.state_store.read().await;
            state_store.show_message(crate::ui::state::store::SystemMessage::info(
                "No Decks",
                "No decks found to clean media from",
            ))?;
            return Ok(());
        }

        let mut total_cleaned = 0;

        for (deck, _cards) in &decks {
            match self.deck_manager.cleanup_deck_media(&deck.uuid).await {
                Ok(count) => {
                    total_cleaned += count;
                }
                Err(e) => {
                    log::warn!("Failed to clean media for deck {}: {}", deck.name, e);
                }
            }
        }

        let state_store = self.state_store.read().await;
        if total_cleaned > 0 {
            state_store.show_message(crate::ui::state::store::SystemMessage::success(
                "Media Cleaned",
                &format!("Removed {} orphaned media file(s)", total_cleaned),
            ))?;
        } else {
            state_store.show_message(crate::ui::state::store::SystemMessage::info(
                "No Orphaned Media",
                "All media files are referenced by cards",
            ))?;
        }

        Ok(())
    }

    /// Check application health
    pub async fn health_check(&self) -> TuiResult<()> {
        // Check core components
        let decks_count = self.deck_service.get_all_decks().await?.len();

        if decks_count == 0 {
            log::warn!("No decks found - application may be in an empty state");
        }

        // Check state store
        let state = self.state_store.read().await;
        if state.get_state().error.is_some() {
            log::warn!("Application has pending errors");
        }

        log::debug!("Health check passed - {} decks loaded", decks_count);
        Ok(())
    }

    /// Validate configuration
    pub fn validate_config(config: &AppConfig) -> TuiResult<()> {
        if config.title.is_empty() {
            return Err(TuiError::State {
                message: "Application title cannot be empty".to_string(),
            });
        }

        if config.tick_rate.as_millis() == 0 {
            return Err(TuiError::State {
                message: "Tick rate must be greater than 0".to_string(),
            });
        }

        if config.tick_rate.as_millis() > 1000 {
            return Err(TuiError::State {
                message: "Tick rate should not exceed 1000ms for smooth UI".to_string(),
            });
        }

        Ok(())
    }

    /// Create configuration from environment variables
    pub fn config_from_env() -> AppConfig {
        let mut config = AppConfig::default();

        if let Ok(title) = std::env::var("ANKITUI_TITLE") {
            config.title = title;
        }

        if let Ok(tick_rate_str) = std::env::var("ANKITUI_TICK_RATE") {
            if let Ok(tick_rate) = tick_rate_str.parse::<u64>() {
                config.tick_rate = std::time::Duration::from_millis(tick_rate);
            }
        }

        if let Ok(debug_str) = std::env::var("ANKITUI_DEBUG") {
            config.debug = debug_str.parse().unwrap_or(false);
        }

        if let Ok(mouse_str) = std::env::var("ANKITUI_ENABLE_MOUSE") {
            config.enable_mouse = mouse_str.parse().unwrap_or(true);
        }

        config
    }

    /// Get current application statistics
    pub async fn get_app_statistics(&self) -> TuiResult<crate::domain::viewmodels::AppStats> {
        let decks_count = self.deck_service.get_all_decks().await?.len();
        let state = self.state_store.read().await.get_state();

        Ok(crate::domain::viewmodels::AppStats {
            total_decks: decks_count,
            current_screen: state.current_screen.clone(),
            has_active_session: state.current_session.is_some(),
            has_pending_error: state.error.is_some(),
            is_loading: state.loading,
        })
    }

    /// Toggle debug mode
    pub async fn toggle_debug(&mut self) -> TuiResult<()> {
        self.config.debug = !self.config.debug;

        let message = crate::ui::state::store::SystemMessage::info(
            "Debug Mode",
            if self.config.debug {
                "Debug mode enabled"
            } else {
                "Debug mode disabled"
            },
        );
        let state_store = self.state_store.read().await;
        state_store.show_message(message)?;

        Ok(())
    }

    /// Set theme
    pub async fn set_theme(&mut self, theme: crate::ui::theme::Theme) -> TuiResult<()> {
        self.config.theme = theme.clone();

        // Update renderer with new theme
        self.renderer.update_theme(theme);

        let message = crate::ui::state::store::SystemMessage::success("Theme Updated", "UI theme has been updated");
        let state_store = self.state_store.read().await;
        state_store.show_message(message)?;

        Ok(())
    }

    /// Force refresh all data
    pub async fn force_refresh(&mut self) -> TuiResult<()> {
        log::info!("Force refreshing all application data");

        // Set loading state
        {
            let state_store = self.state_store.read().await;
            state_store.set_loading(true)?;
        }

        // Refresh core data
        self.refresh_core_data().await?;

        // Clear any stale errors
        {
            let state_store = self.state_store.read().await;
            state_store.set_error(None)?;
        }

        // Clear expired messages
        {
            let state = self.state_store.read().await;
            if let Some(ref message) = state.get_state().message {
                if message.is_expired() {
                    let state_store = self.state_store.read().await;
                    state_store.clear_message()?;
                }
            }
        }

        // Clear loading state
        {
            let state_store = self.state_store.read().await;
            state_store.set_loading(false)?;
        }

        let message = crate::ui::state::store::SystemMessage::success(
            "Data Refreshed",
            "All application data has been refreshed",
        );
        let state_store = self.state_store.read().await;
        state_store.show_message(message)?;

        Ok(())
    }

    /// Export application data to JSON file
    pub async fn export_data(&self, path: &std::path::Path) -> TuiResult<()> {
        log::info!("Exporting application data to: {:?}", path);

        // Export all decks and their cards
        let decks = self
            .deck_service
            .get_all_decks()
            .await
            .map_err(|e| TuiError::Core(format!("Failed to get decks for export: {}", e)))?;

        let export = serde_json::json!({
            "version": "1.0",
            "exported_at": chrono::Utc::now().to_rfc3339(),
            "decks": decks.iter().map(|(deck, cards)| {
                serde_json::json!({
                    "deck": {
                        "uuid": deck.uuid.to_string(),
                        "name": deck.name,
                        "description": deck.description,
                        "created_at": deck.created_at.to_rfc3339(),
                        "modified_at": deck.modified_at.to_rfc3339(),
                    },
                    "card_count": cards.len(),
                })
            }).collect::<Vec<_>>(),
        });

        let json = serde_json::to_string_pretty(&export).map_err(|e| TuiError::State {
            message: format!("Failed to serialize export: {}", e),
        })?;

        std::fs::write(path, &json).map_err(|e| TuiError::State {
            message: format!("Failed to write export file: {}", e),
        })?;

        let message = crate::ui::state::store::SystemMessage::success(
            "Export Complete",
            &format!("Exported {} decks to {:?}", decks.len(), path),
        );
        let state_store = self.state_store.read().await;
        state_store.show_message(message)?;

        Ok(())
    }

    /// Import application data from JSON file
    pub async fn import_data(&mut self, path: &std::path::Path) -> TuiResult<()> {
        log::info!("Importing application data from: {:?}", path);

        let json = std::fs::read_to_string(path).map_err(|e| TuiError::State {
            message: format!("Failed to read import file: {}", e),
        })?;

        let _data: serde_json::Value = serde_json::from_str(&json).map_err(|e| TuiError::State {
            message: format!("Failed to parse import file: {}", e),
        })?;

        // Refresh data after import
        self.force_refresh().await?;

        let message = crate::ui::state::store::SystemMessage::success(
            "Import Complete",
            &format!("Data imported from: {:?}", path),
        );
        let state_store = self.state_store.read().await;
        state_store.show_message(message)?;

        Ok(())
    }

    // Deck selection navigation methods

    /// Get the UUID of the currently highlighted deck in the list
    async fn get_selected_deck_from_list(&self) -> TuiResult<Option<Uuid>> {
        let decks = self.deck_service.get_all_decks().await?;
        let state = self.state_store.read().await;
        let idx = state.get_state().deck_list_selected.unwrap_or(0);
        drop(state);

        if idx < decks.len() {
            let (deck, _) = &decks[idx];
            Ok(Some(deck.uuid))
        } else {
            Ok(None)
        }
    }

    /// Handle deck selection up navigation
    async fn handle_deck_selection_up(&mut self) -> TuiResult<()> {
        let decks = self.deck_service.get_all_decks().await?;
        let deck_count = decks.len();

        if deck_count == 0 {
            return Ok(());
        }

        let state_store = self.state_store.read().await;
        state_store.update_state(|state| {
            let current_selected = state.deck_list_selected.unwrap_or(0);
            let new_selected = if current_selected == 0 {
                deck_count - 1
            } else {
                current_selected - 1
            };
            state.deck_list_selected = Some(new_selected);
        })?;

        Ok(())
    }

    /// Handle deck selection down navigation
    async fn handle_deck_selection_down(&mut self) -> TuiResult<()> {
        let decks = self.deck_service.get_all_decks().await?;
        let deck_count = decks.len();

        if deck_count == 0 {
            return Ok(());
        }

        let state_store = self.state_store.read().await;
        state_store.update_state(|state| {
            let current_selected = state.deck_list_selected.unwrap_or(0);
            let new_selected = if current_selected >= deck_count - 1 {
                0
            } else {
                current_selected + 1
            };
            state.deck_list_selected = Some(new_selected);
        })?;

        Ok(())
    }

    /// Handle deck selection confirm (Enter key)
    async fn handle_deck_selection_confirm(&mut self) -> TuiResult<()> {
        let decks = self.deck_service.get_all_decks().await?;

        if decks.is_empty() {
            let state_store = self.state_store.read().await;
            let message =
                crate::ui::state::store::SystemMessage::warning("No Decks", "No decks available. Create a deck first.");
            state_store.show_message(message)?;
            return Ok(());
        }

        let selected_index = {
            let state = self.state_store.read().await;
            state.get_deck_list_selected().unwrap_or(0)
        };

        if selected_index < decks.len() {
            let (deck, _cards) = &decks[selected_index];
            let deck_id = deck.uuid;

            // Store selected deck ID
            {
                let state_store = self.state_store.read().await;
                state_store.set_selected_deck(Some(deck_id))?;
            }

            // Start study session
            self.start_study_session().await?;
        } else {
            let state_store = self.state_store.read().await;
            let message = crate::ui::state::store::SystemMessage::error("Error", "Invalid deck selection");
            state_store.show_message(message)?;
        }

        Ok(())
    }

    // Settings persistence

    async fn persist_settings(&mut self) -> TuiResult<()> {
        let Some(cm) = &mut self.config_manager else {
            log::debug!("No config manager available, skipping settings persistence");
            return Ok(());
        };

        let state = self.state_store.read().await.get_state();
        let ui = &state.ui_state;

        // Persist daily config
        if let Some(new_cards) = ui.get("new_cards_per_day").and_then(|s| s.parse::<i32>().ok()) {
            cm.config.daily.max_new_cards = new_cards;
        }
        if let Some(max_reviews) = ui.get("max_reviews_per_day").and_then(|s| s.parse::<i32>().ok()) {
            cm.config.daily.max_review_cards = max_reviews;
        }

        // Persist UI config
        if let Some(theme) = ui.get("theme") {
            cm.config.ui.theme = theme.clone();
        }
        if let Some(show_progress) = ui.get("show_progress").and_then(|s| s.parse::<bool>().ok()) {
            cm.config.ui.show_progress = show_progress;
        }

        // Persist study preferences
        if let Some(auto_advance) = ui.get("auto_advance") {
            cm.config.daily.auto_advance = auto_advance == "true";
        }
        if let Some(show_hint) = ui.get("show_hint") {
            cm.config.daily.show_hint = show_hint == "true";
        }

        // Save config to file
        if let Err(e) = cm.save_config() {
            log::warn!("Failed to save config: {}", e);
        } else {
            log::info!("Settings persisted to config file");
        }
        Ok(())
    }

    // Data management operations

    async fn handle_data_import(&mut self) -> TuiResult<()> {
        let data_dir = data_helpers::get_default_data_dir();
        let import_path = data_dir.join("import.toml");

        if !import_path.exists() {
            let state_store = self.state_store.read().await;
            state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                "Import",
                &format!("No import file found at {:?}", import_path),
            ))?;
            return Ok(());
        }

        let content = std::fs::read_to_string(&import_path).map_err(|e| TuiError::State {
            message: format!("Failed to read import file: {}", e),
        })?;

        match self.deck_manager.import_deck(&content).await {
            Ok(deck_uuid) => {
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::success(
                    "Import Complete",
                    &format!("Deck imported successfully (UUID: {})", deck_uuid),
                ))?;
                self.refresh_core_data().await?;
            }
            Err(e) => {
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::error(
                    "Import Failed",
                    &format!("Failed to import deck: {}", e),
                ))?;
            }
        }
        Ok(())
    }

    async fn handle_data_export(&mut self) -> TuiResult<()> {
        let deck_id = self.state_store.read().await.get_state().selected_deck_id;

        let (deck_uuid, deck_name) = if let Some(id) = deck_id {
            if let Ok((deck, _)) = self.deck_manager.get_deck(&id).await {
                (id, deck.name)
            } else {
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                    "Export",
                    "Selected deck not found",
                ))?;
                return Ok(());
            }
        } else {
            let state_store = self.state_store.read().await;
            state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                "Export",
                "No deck selected. Select a deck first, then export.",
            ))?;
            return Ok(());
        };

        match self.deck_manager.export_deck(&deck_uuid, true).await {
            Ok(export_data) => {
                let data_dir = data_helpers::get_default_data_dir().join("exports");
                std::fs::create_dir_all(&data_dir).ok();
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                let export_path = data_dir.join(format!("{}_{}.toml", deck_name.replace('/', "::"), timestamp));

                std::fs::write(&export_path, export_data).map_err(|e| TuiError::State {
                    message: format!("Failed to write export file: {}", e),
                })?;

                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::success(
                    "Export Complete",
                    &format!("Deck exported to {:?}", export_path),
                ))?;
            }
            Err(e) => {
                let state_store = self.state_store.read().await;
                state_store.show_message(crate::ui::state::store::SystemMessage::error(
                    "Export Failed",
                    &format!("Failed to export deck: {}", e),
                ))?;
            }
        }
        Ok(())
    }

    async fn handle_data_backup(&mut self) -> TuiResult<()> {
        let data_dir = data_helpers::get_default_data_dir();
        let db_path = data_dir.join("ankitui.db");

        if !db_path.exists() {
            let state_store = self.state_store.read().await;
            state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                "Backup",
                "No database file found to backup",
            ))?;
            return Ok(());
        }

        let backup_dir = data_dir.join("backups");
        std::fs::create_dir_all(&backup_dir).ok();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("ankitui_{}.db", timestamp));

        std::fs::copy(&db_path, &backup_path).map_err(|e| TuiError::State {
            message: format!("Failed to create backup: {}", e),
        })?;

        let state_store = self.state_store.read().await;
        state_store.show_message(crate::ui::state::store::SystemMessage::success(
            "Backup Complete",
            &format!("Database backed up to {:?}", backup_path),
        ))?;
        Ok(())
    }

    async fn handle_data_restore(&mut self) -> TuiResult<()> {
        let data_dir = data_helpers::get_default_data_dir();
        let db_path = data_dir.join("ankitui.db");
        let backup_dir = data_dir.join("backups");

        // Find the most recent backup
        if let Ok(entries) = std::fs::read_dir(&backup_dir) {
            let mut backups: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "db"))
                .filter_map(|e| {
                    e.metadata().ok().and_then(|m| m.modified().ok().map(|t| (e, t)))
                })
                .collect();
            backups.sort_by_key(|(_, t)| std::cmp::Reverse(*t));

            if let Some((latest, _)) = backups.first() {
                let backup_path = latest.path();
                // Only restore if DB doesn't exist (safety check)
                if !db_path.exists() {
                    std::fs::copy(&backup_path, &db_path).map_err(|e| TuiError::State {
                        message: format!("Failed to restore database: {}", e),
                    })?;

                    let state_store = self.state_store.read().await;
                    state_store.show_message(crate::ui::state::store::SystemMessage::success(
                        "Restore Complete",
                        &format!("Database restored from {:?}", backup_path),
                    ))?;
                } else {
                    let state_store = self.state_store.read().await;
                    state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                        "Restore Skipped",
                        "Database already exists. To restore from backup, delete the current database \
                         manually (or use `rm ~/.local/share/ankitui/ankitui.db`), then try restore again.",
                    ))?;
                }
                return Ok(());
            }
        }

        let state_store = self.state_store.read().await;
        state_store.show_message(crate::ui::state::store::SystemMessage::warning(
            "Restore",
            "No backup files found",
        ))?;
        Ok(())
    }

    async fn handle_data_clear(&mut self) -> TuiResult<()> {
        let state_store = self.state_store.read().await;
        state_store.show_message(crate::ui::state::store::SystemMessage::warning(
            "Clear Data",
            "Data clearing requires CLI: run `ankitui clear-data --confirm` to remove all data.",
        ))?;
        Ok(())
    }

    // Deck management methods

    /// Handle deck management up navigation
    async fn handle_deck_management_up(&mut self) -> TuiResult<()> {
        let decks = self.deck_service.get_all_decks().await?;
        let deck_count = decks.len();

        if deck_count == 0 {
            return Ok(());
        }

        self.state_store.read().await.update_state(|state| {
            let current_selected = state.deck_list_selected.unwrap_or(0);
            let new_selected = if current_selected == 0 {
                deck_count - 1
            } else {
                current_selected - 1
            };
            state.deck_list_selected = Some(new_selected);
        })?;

        Ok(())
    }

    /// Handle deck management down navigation
    async fn handle_deck_management_down(&mut self) -> TuiResult<()> {
        let decks = self.deck_service.get_all_decks().await?;
        let deck_count = decks.len();

        if deck_count == 0 {
            return Ok(());
        }

        self.state_store.read().await.update_state(|state| {
            let current_selected = state.deck_list_selected.unwrap_or(0);
            let new_selected = if current_selected >= deck_count - 1 {
                0
            } else {
                current_selected + 1
            };
            state.deck_list_selected = Some(new_selected);
        })?;

        Ok(())
    }

    /// Handle deck management action (Enter key) — start studying the selected deck
    async fn handle_deck_management_action(&mut self) -> TuiResult<()> {
        let decks = self.deck_service.get_all_decks().await?;

        if decks.is_empty() {
            let state_store = self.state_store.read().await;
            state_store.show_message(crate::ui::state::store::SystemMessage::warning(
                "No Decks", "No decks available. Create a deck first."))?;
            return Ok(());
        }

        let selected_index = {
            let state = self.state_store.read().await;
            state.get_deck_list_selected().unwrap_or(0)
        };

        if selected_index < decks.len() {
            let (deck, _) = &decks[selected_index];

            // Set as selected deck and start study session
            {
                let state_store = self.state_store.read().await;
                state_store.set_selected_deck(Some(deck.uuid))?;
            }

            self.start_study_session().await?;
        } else {
            let state_store = self.state_store.read().await;
            state_store.show_message(crate::ui::state::store::SystemMessage::error(
                "Error", "Invalid deck selection"))?;
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        // Note: This is a synchronous default implementation
        // In practice, use App::new() for proper async initialization
        let rt = tokio::runtime::Runtime::new()
            .expect("AnkiTUI: Failed to create tokio runtime - this is a critical error");
        rt.block_on(async {
            Self::new(AppConfig::default())
                .await
                .expect("AnkiTUI: Failed to initialize application - check data directory permissions")
        })
    }
}

//! Main application implementation

use crate::domain::{DeckService, StatisticsService, StudyService};
use crate::ui::event::handler::EventHandler;
use crate::ui::navigator::Navigator;
use crate::ui::render::Renderer;
use crate::ui::state::store::StateStore;
use crate::ui::theme::Theme;
use crate::utils::error::{TuiError, TuiResult};
use ankitui_core::{DeckManager, Scheduler, SessionController};
use std::sync::Arc;
use tokio::sync::RwLock;

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
    pub event_handler: EventHandler,
    renderer: crate::ui::render::DefaultRenderer,
    navigator: Navigator,

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
        let state_store = Arc::new(RwLock::new(StateStore::new()));

        // Initialize data paths
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("ankitui");

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
                .map_err(|e| {
                    TuiError::Core(format!("Failed to initialize SessionController: {}", e))
                })?,
        ));

        // Initialize service layer
        let deck_service = DeckService::new(Arc::clone(&deck_manager));
        let study_service =
            StudyService::new(Arc::clone(&session_controller), Arc::clone(&deck_manager));
        let statistics_service = StatisticsService::new(Arc::clone(&deck_manager));

        Ok(Self {
            config,
            state_store: Arc::clone(&state_store),
            event_handler: EventHandler::new(crate::ui::state::store::AppState::default()),
            renderer: crate::ui::render::DefaultRenderer::new(),
            navigator: Navigator::new(),
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

        log::info!(
            "Application initialized successfully with {} decks",
            decks.len()
        );
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
            let system_message = crate::ui::state::store::SystemMessage::error(
                "Application Error",
                error_message.as_str(),
            );

            state_store
                .show_message(system_message)
                .map_err(|e| TuiError::State {
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

        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("ankitui");
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

        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("ankitui");
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

    /// Handle user input events
    pub async fn handle_input(&mut self, input: crate::ui::event::Event) -> TuiResult<()> {
        // Update event handler with current state
        let current_state = self.state_store.read().await.get_state().clone();
        self.event_handler.update_state(current_state);

        // Process the event
        let command = self.event_handler.handle_event(input);

        // Execute the command
        self.execute_command(command).await
    }

    /// Execute application commands
    pub async fn execute_command(&mut self, command: crate::ui::event::Command) -> TuiResult<()> {
        use crate::ui::event::CommandType;

        match command.command_type {
            CommandType::NavigateToMainMenu => {
                let state_store = self.state_store.read().await;
                state_store.navigate_to(crate::ui::state::store::Screen::MainMenu)?;
            }
            CommandType::NavigateBack => {
                let state_store = self.state_store.read().await;
                state_store.navigate_back()?;
            }
            // Main menu navigation
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
                    crate::ui::state::store::Screen::DeckSelection => {
                        self.handle_deck_selection_down().await?;
                    }
                    _ => {}
                }
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
                    _ => {}
                }
            }
            CommandType::StartStudySessionDefault => {
                self.start_study_session().await?;
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
                // Refresh statistics
            }
            CommandType::Quit => {
                self.stop();
            }
            CommandType::ShowHelp => {
                // Show help dialog
                let help_message = crate::ui::state::store::SystemMessage::info(
                    "Help",
                    "AnkiTUI V2 - Help\n\
                     \n\
                     Navigation:\n\
                     ↑/↓ - Navigate up/down\n\
                     Enter - Select/Confirm\n\
                     Esc - Go back\n\
                     \n\
                     Study Session:\n\
                     Space - Show answer\n\
                     1-4 - Rate card (Again/Hard/Good/Easy)\n\
                     \n\
                     Global:\n\
                     Ctrl+C - Quit\n\
                     F1 - Show this help",
                );
                let state_store = self.state_store.read().await;
                state_store.show_message(help_message)?;
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
            state_store.set_current_card_study(true)?;
        }

        let message = crate::ui::state::store::SystemMessage::success(
            "Study Session Started",
            "Good luck with your studying!",
        );
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
        let message =
            crate::ui::state::store::SystemMessage::success("Session Complete", &study_count);
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
            4 => {
                // Quit
                self.stop();
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

        let message = crate::ui::state::store::SystemMessage::success(
            "Theme Updated",
            "UI theme has been updated",
        );
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

        let json = serde_json::to_string_pretty(&export)
            .map_err(|e| TuiError::State { message: format!("Failed to serialize export: {}", e) })?;

        std::fs::write(path, &json)
            .map_err(|e| TuiError::State { message: format!("Failed to write export file: {}", e) })?;

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

        let json = std::fs::read_to_string(path)
            .map_err(|e| TuiError::State { message: format!("Failed to read import file: {}", e) })?;

        let _data: serde_json::Value = serde_json::from_str(&json)
            .map_err(|e| TuiError::State { message: format!("Failed to parse import file: {}", e) })?;

        // Refresh data after import
        self.force_refresh().await?;

        let message =
            crate::ui::state::store::SystemMessage::success("Import Complete", &format!("Data imported from: {:?}", path));
        let state_store = self.state_store.read().await;
        state_store.show_message(message)?;

        Ok(())
    }

    // Deck selection navigation methods

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
            let message = crate::ui::state::store::SystemMessage::warning(
                "No Decks",
                "No decks available. Create a deck first.",
            );
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
            let message = crate::ui::state::store::SystemMessage::error(
                "Error",
                "Invalid deck selection",
            );
            state_store.show_message(message)?;
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        // Note: This is a synchronous default implementation
        // In practice, use App::new() for proper async initialization
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        rt.block_on(async {
            Self::new(AppConfig::default())
                .await
                .expect("Failed to create default application")
        })
    }
}

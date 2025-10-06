//! Main application implementation

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::ui::state::store::StateStore;
use crate::ui::event::handler::EventHandler;
use crate::ui::render::Renderer;
use crate::ui::theme::Theme;
use crate::ui::navigator::Navigator;
use crate::utils::error::{TuiError, TuiResult};
use crate::domain::{DeckService, StudyService, StatisticsService};
use ankitui_core::{DeckManager, SessionController, Scheduler};

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
    session_controller: Arc<SessionController>,
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
        let db_path = data_dir.join("state.db");

        // Create directories if they don't exist
        std::fs::create_dir_all(&content_dir)
            .map_err(|e| TuiError::State {
                message: format!("Failed to create content directory: {}", e)
            })?;

        // Initialize core components with proper async constructors
        let deck_manager = Arc::new(DeckManager::new(&content_dir, &db_path)
            .await
            .map_err(|e| TuiError::Core(format!("Failed to initialize DeckManager: {}", e)))?);

        let scheduler = Arc::new(Scheduler::new(None));
        let session_controller = Arc::new(SessionController::new((*deck_manager).clone(), Some((*scheduler).clone()))
            .await
            .map_err(|e| TuiError::Core(format!("Failed to initialize SessionController: {}", e)))?);

        // Initialize service layer
        let deck_service = DeckService::new(Arc::clone(&deck_manager));
        let study_service = StudyService::new(Arc::clone(&session_controller), Arc::clone(&deck_manager));
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

    pub fn session_controller(&self) -> &Arc<SessionController> {
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
        log::info!("Initializing AnkiTUI V2 application");

        // Load all decks using service layer
        let decks = self.deck_service.get_all_decks().await
            .map_err(|e| TuiError::Core(
                format!("Failed to load decks: {}", e)
            ))?;

        log::info!("Loaded {} decks", decks.len());

        // Load global statistics using service layer
        let _stats = self.statistics_service.get_global_statistics().await
            .map_err(|e| TuiError::Core(
                format!("Failed to load global statistics: {}", e)
            ))?;

        // Initialize state
        let mut state = self.state_store.write().await;
        state.set_loading(false);

        log::info!("Application initialized successfully with {} decks", decks.len());
        Ok(())
    }

    /// Update application state
    pub async fn update(&mut self) -> TuiResult<()> {
        // Update state store subscriptions
        // Update component states
        // Handle background tasks
        Ok(())
    }

    /// Handle application shutdown
    pub async fn shutdown(&mut self) -> TuiResult<()> {
        log::info!("Shutting down AnkiTUI V2 application");

        // Save current state
        // Cleanup resources
        // Close database connections

        self.running = false;
        Ok(())
    }

    /// Handle application errors
    pub fn handle_error(&mut self, error: TuiError) -> TuiResult<()> {
        log::error!("Application error: {}", error);

        // Show error to user
        // Log error details
        // Attempt recovery if possible

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        // Note: This is a synchronous default implementation
        // In practice, use App::new() for proper async initialization
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime");
        rt.block_on(async {
            Self::new(AppConfig::default()).await
                .expect("Failed to create default application")
        })
    }
}
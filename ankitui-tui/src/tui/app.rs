//! Main TUI Application
//!
//! Main application state and UI management with modern architecture

use super::components::Components;
use super::core::event_handler::Action;
use super::core::{ActionDispatcher, AppContext, ComponentRegistry, RenderingManager};
use ankitui_core::ConfigManager;
use ankitui_core::{DeckManager, SessionController, StatsEngine};
use anyhow::Result;

/// Application states
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    MainMenu,
    DeckSelection,
    DeckManagement,
    Learning,
    CardReview,
    Statistics,
    Settings,
    Help,
    ConfirmExit,
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppState::MainMenu => write!(f, "Main Menu"),
            AppState::DeckSelection => write!(f, "Deck Selection"),
            AppState::DeckManagement => write!(f, "Deck Management"),
            AppState::Learning => write!(f, "Learning"),
            AppState::CardReview => write!(f, "Card Review"),
            AppState::Statistics => write!(f, "Statistics"),
            AppState::Settings => write!(f, "Settings"),
            AppState::Help => write!(f, "Help"),
            AppState::ConfirmExit => write!(f, "Confirm Exit"),
        }
    }
}

/// Main TUI application with modern architecture
pub struct App {
    /// Current application state
    state: AppState,
    /// Previous state (for navigation back)
    previous_state: Option<AppState>,
    /// Legacy UI components (being phased out)
    components: Components,
    /// Application context for dependency injection
    app_context: AppContext,
    /// Event-driven action dispatcher (owns component registry)
    action_dispatcher: ActionDispatcher,
    /// Advanced rendering manager
    rendering_manager: RenderingManager,
    /// Deck manager instance
    deck_manager: DeckManager,
    /// Session controller for card reviews
    session_controller: Option<SessionController>,
    /// Statistics engine
    stats_engine: StatsEngine,
    /// Configuration manager
    config_manager: ConfigManager,
    /// Should the application quit?
    should_quit: bool,
    /// Error message to display (if any)
    error_message: Option<String>,
    /// Success message to display (if any)
    success_message: Option<String>,
}

impl App {
    /// Create a new application instance
    pub fn new(deck_manager: DeckManager, config_manager: ConfigManager) -> Result<Self> {
        let components = Components::new();
        let stats_engine = StatsEngine::new();
        let config = config_manager.get_config();

        // Initialize modern architecture components
        let app_context =
            AppContext::new(deck_manager.clone(), stats_engine.clone(), config.clone());
        let mut component_registry = ComponentRegistry::new();

        // Register legacy components with the new registry for backward compatibility
        // TODO: Eventually migrate all components to the new system
        let action_dispatcher = ActionDispatcher::new(component_registry, app_context.clone());
        let rendering_manager = RenderingManager::new()?;

        Ok(Self {
            state: AppState::MainMenu,
            previous_state: None,
            components,
            app_context,
            action_dispatcher,
            rendering_manager,
            deck_manager,
            session_controller: None,
            stats_engine,
            config_manager,
            should_quit: false,
            error_message: None,
            success_message: None,
        })
    }

    /// Get the current application state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get current error message
    pub fn error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    /// Get current success message
    pub fn success_message(&self) -> Option<&String> {
        self.success_message.as_ref()
    }

    /// Clear messages
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }

    /// Set an error message
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }

    /// Set a success message
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }

    /// Handle user actions using modern event-driven architecture
    pub async fn handle_action(&mut self, action: Action) -> Result<()> {
        // Clear any existing messages when new action is taken
        if action != Action::Help {
            self.clear_messages();
        }

        // Handle Quit action directly since it's application-level
        if action == Action::Quit {
            self.should_quit = true;
            return Ok(());
        }

        // Use ActionDispatcher for event-driven processing
        if let Some(new_state) = self
            .action_dispatcher
            .handle_action(action.clone(), self.state.clone())
            .await?
        {
            // Update state if dispatcher suggests a change
            if new_state != self.state {
                self.previous_state = Some(self.state.clone());
                self.state = new_state;

                // Sync component registry with new state
                if let Ok(mut registry) = self.action_dispatcher.component_registry().lock() {
                    registry.set_current_component(self.state.clone());
                }
            }
        }
        Ok(())
    }

    /// Update application data (refresh from managers)
    pub async fn update(&mut self) -> Result<()> {
        // Refresh deck list with more detailed error handling
        match self.deck_manager.get_all_decks().await {
            Ok(decks) => {
                // Convert (Deck, Vec<Card>) to Vec<Deck>
                let deck_only: Vec<_> = decks.into_iter().map(|(deck, _cards)| deck).collect();
                self.components.update_decks(deck_only.clone());

                // Show helpful message if no decks are available
                if deck_only.is_empty() && self.state == AppState::MainMenu {
                    // Only set this message once to avoid spam
                    if self.error_message().is_none() && self.success_message().is_none() {
                        self.set_success(
                            "Welcome! Create your first deck to get started learning.".to_string(),
                        );
                    }
                }
            }
            Err(e) => {
                self.set_error(format!("Failed to load decks: {}", e));
            }
        }

        // Update session data if active
        if let Some(session) = &self.session_controller {
            let progress = session.session_progress();
            self.components.update_session_progress(progress.clone());
            // Also update Study component with current session progress
            self.components.set_session_progress(Some(progress));

            // Check if session has run out of cards but hasn't been ended
            if !session.has_more_cards() && self.state == AppState::CardReview {
                if self.error_message().is_none() && self.success_message().is_none() {
                    self.set_success("All cards reviewed! Session complete.".to_string());
                }
            }
        }

        Ok(())
    }

    /// Load statistics for all decks
    async fn load_statistics(&mut self) -> Result<()> {
        match self.deck_manager.get_all_decks().await {
            Ok(decks) => {
                if decks.is_empty() {
                    // No decks available, don't set statistics
                    return Ok(());
                }

                // Initialize stats component with the stats engine
                self.components.set_stats_engine(self.stats_engine.clone());

                // Pass real deck and card data to the statistics component
                self.components.update_deck_statistics(decks).await;
            }
            Err(e) => {
                self.set_error(format!("Failed to load statistics: {}", e));
            }
        }
        Ok(())
    }

    /// Get reference to components
    pub fn components(&self) -> &Components {
        &self.components
    }

    /// Get mutable reference to components
    pub fn components_mut(&mut self) -> &mut Components {
        &mut self.components
    }

    /// Get reference to deck manager
    pub fn deck_manager(&self) -> &DeckManager {
        &self.deck_manager
    }

    /// Get reference to session controller
    pub fn session_controller(&self) -> Option<&SessionController> {
        self.session_controller.as_ref()
    }

    /// Get reference to config manager
    pub fn config_manager(&self) -> &ConfigManager {
        &self.config_manager
    }

    /// Get reference to stats engine
    pub fn stats_engine(&self) -> &StatsEngine {
        &self.stats_engine
    }

    /// Render the application UI using modern rendering system
    pub fn render<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut ratatui::Terminal<B>,
    ) -> Result<()> {
        terminal.draw(|f| {
            if let Ok(mut registry) = self.action_dispatcher.component_registry().lock() {
                self.rendering_manager
                    .render::<B>(f, &self.state, &mut registry, &self.app_context)
                    .unwrap_or_else(|e| {
                        // Don't print to stderr in TUI mode, it breaks the interface
                        // Use proper error handling instead
                    });
            }
        })?;
        Ok(())
    }
}

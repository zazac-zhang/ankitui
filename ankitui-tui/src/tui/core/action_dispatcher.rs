//! Action Dispatcher
//!
//! Event-driven action handling system to replace centralized action management

use super::app_context::{AppContext, ComponentResult};
use super::component_registry::{ComponentRegistry, AutonomousComponent};
use super::event_bus::{AppEvent, EventBus, global_event_bus, publish_global};
use super::event_handler::Action;
use crate::tui::app::AppState;
use anyhow::Result;
use std::sync::{Arc, Mutex};

/// Action dispatcher for event-driven handling
pub struct ActionDispatcher {
    /// Event bus for publishing events
    event_bus: Arc<EventBus>,
    /// Component registry for component interactions
    component_registry: Arc<Mutex<ComponentRegistry>>,
    /// Application context
    app_context: AppContext,
}

impl ActionDispatcher {
    /// Create new action dispatcher
    pub fn new(component_registry: ComponentRegistry, app_context: AppContext) -> Self {
        Self {
            event_bus: Arc::new(EventBus::new()),
            component_registry: Arc::new(Mutex::new(component_registry)),
            app_context,
        }
    }

    /// Handle an action using event-driven approach
    pub async fn handle_action(&mut self, action: Action, current_state: AppState) -> Result<Option<AppState>> {
        // Convert action to event
        let event = self.action_to_event(action.clone(), &current_state)?;

        // Publish the event
        self.event_bus.publish(event.clone())?;

        // Handle the action using component registry
        let result = self.handle_action_with_components(action, &current_state).await?;

        // Publish result event
        if let Some(ref new_state) = result {
            let result_event = AppEvent::StateChanged {
                old_state: format!("{:?}", current_state),
                new_state: format!("{:?}", new_state),
            };
            self.event_bus.publish(result_event)?;
        }

        Ok(result)
    }

    /// Convert action to event
    fn action_to_event(&self, action: Action, current_state: &AppState) -> Result<AppEvent> {
        match action {
            Action::Select => Ok(AppEvent::UserAction {
                action: "select".to_string(),
                target: format!("{:?}", current_state),
            }),
            Action::Cancel => Ok(AppEvent::UserAction {
                action: "cancel".to_string(),
                target: format!("{:?}", current_state),
            }),
            Action::Up | Action::Down => Ok(AppEvent::KeyPressed {
                key: format!("{:?}", action),
                modifiers: vec![],
            }),
            Action::Quit => Ok(AppEvent::ApplicationExiting),
            Action::Help => Ok(AppEvent::NavigateTo {
                destination: "help".to_string(),
            }),
            // Add more action-to-event mappings as needed
            _ => Ok(AppEvent::UserAction {
                action: format!("{:?}", action),
                target: format!("{:?}", current_state),
            }),
        }
    }

    /// Handle action using component registry
    async fn handle_action_with_components(
        &mut self,
        action: Action,
        current_state: &AppState,
    ) -> Result<Option<AppState>> {
        let mut registry = self.component_registry.lock().unwrap();

        // Set current component based on state
        registry.set_current_component(current_state.clone());

        // Try to handle with current component
        if let Some(result) = registry.with_current_component(|component| {
            // Use regular component handling for now
            component.handle_action(action.clone())
        }) {
            // Component handled the action, return the result
            return result.map(|state| {
                // Convert component result to app state if needed
                match state {
                    Some(new_state) => Some(new_state),
                    None => None,
                }
            });
        }

        // Fallback to state-based handling only if no current component
        self.fallback_state_handling(action, current_state)
    }

    /// Fallback state-based handling for backward compatibility
    fn fallback_state_handling(&self, action: Action, current_state: &AppState) -> Result<Option<AppState>> {
        match current_state {
            AppState::MainMenu => self.handle_main_menu_fallback(action),
            AppState::DeckSelection => self.handle_deck_selection_fallback(action),
            AppState::DeckManagement => self.handle_deck_management_fallback(action),
            AppState::Learning => self.handle_learning_fallback(action),
            AppState::CardReview => self.handle_card_review_fallback(action),
            AppState::Statistics => self.handle_statistics_fallback(action),
            AppState::Settings => self.handle_settings_fallback(action),
            AppState::Help => self.handle_help_fallback(action),
            AppState::ConfirmExit => self.handle_confirm_exit_fallback(action),
        }
    }

    // Fallback handlers for each state
    fn handle_main_menu_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Quit => Ok(Some(AppState::ConfirmExit)),
            Action::Help => Ok(Some(AppState::Help)),
            _ => Ok(None),
        }
    }

    fn handle_deck_selection_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Help => Ok(Some(AppState::Help)),
            _ => Ok(None),
        }
    }

    fn handle_deck_management_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Help => Ok(Some(AppState::Help)),
            _ => Ok(None),
        }
    }

    fn handle_learning_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Help => Ok(Some(AppState::Help)),
            _ => Ok(None),
        }
    }

    fn handle_card_review_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Help => Ok(Some(AppState::Help)),
            _ => Ok(None),
        }
    }

    fn handle_statistics_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Help => Ok(Some(AppState::Help)),
            _ => Ok(None),
        }
    }

    fn handle_settings_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Help => Ok(Some(AppState::Help)),
            _ => Ok(None),
        }
    }

    fn handle_help_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            _ => Ok(None),
        }
    }

    fn handle_confirm_exit_fallback(&self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => Ok(Some(AppState::MainMenu)),
            Action::Select => {
                // Publish exit event
                let _ = self.event_bus.publish(AppEvent::ApplicationExiting);
                Ok(None) // Let application handle quitting
            }
            _ => Ok(None),
        }
    }

    /// Register event handlers for common patterns
    pub fn register_common_handlers(&self) {
        // Register navigation handler
        self.event_bus.register_handler(
            vec!["navigate_to".to_string()],
            |event| {
                if let AppEvent::NavigateTo { destination } = event {
                    // Handle navigation without console output in TUI mode
                    // In a full implementation, this would trigger state changes
                }
                Ok(())
            }
        );

        // Register error handler
        self.event_bus.register_handler(
            vec!["error_occurred".to_string()],
            |event| {
                if let AppEvent::ErrorOccurred { error, context } = event {
                    // Handle errors without console output in TUI mode
                    // Store error for display in UI instead
                }
                Ok(())
            }
        );

        // Register state change handler
        self.event_bus.register_handler(
            vec!["state_changed".to_string()],
            |event| {
                if let AppEvent::StateChanged { old_state, new_state } = event {
                    // Handle state changes without console output in TUI mode
                }
                Ok(())
            }
        );
    }

    /// Get event bus reference
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Get component registry reference
    pub fn component_registry(&self) -> Arc<Mutex<ComponentRegistry>> {
        self.component_registry.clone()
    }
}


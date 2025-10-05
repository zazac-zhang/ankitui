//! State Manager
//!
//! Centralized state management with clean transitions

use super::event_handler::Action;
use crate::tui::app::AppState;
use anyhow::Result;
use std::collections::HashMap;

/// Render context for components
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Current application state
    pub state: AppState,
    /// Additional context data
    pub data: HashMap<String, String>,
    /// Whether current state has focus
    pub focused: bool,
}

/// Centralized state manager
pub struct StateManager {
    current_state: AppState,
    previous_state: Option<AppState>,
    state_history: Vec<AppState>,
    render_context: RenderContext,
}

impl StateManager {
    /// Create a new state manager
    pub fn new() -> Self {
        let render_context = RenderContext {
            state: AppState::MainMenu,
            data: HashMap::new(),
            focused: true,
        };

        Self {
            current_state: AppState::MainMenu,
            previous_state: None,
            state_history: Vec::new(),
            render_context,
        }
    }

    /// Get current state
    pub fn current_state(&self) -> AppState {
        self.current_state.clone()
    }

    /// Get previous state
    pub fn previous_state(&self) -> Option<AppState> {
        self.previous_state.clone()
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: AppState) {
        self.previous_state = Some(self.current_state.clone());

        // Add to history (keep last 10 states)
        self.state_history.push(self.current_state.clone());
        if self.state_history.len() > 10 {
            self.state_history.remove(0);
        }

        self.current_state = new_state.clone();
        self.render_context.state = new_state;
    }

    /// Go back to previous state
    pub fn go_back(&mut self) -> bool {
        if let Some(prev_state) = self.previous_state.clone() {
            self.transition_to(prev_state);
            true
        } else {
            false
        }
    }

    /// Handle action and determine state transition
    pub fn handle_action(&mut self, action: Action, _component_registry: &super::component_registry::ComponentRegistry) -> Result<AppState> {
        let new_state = match (self.current_state.clone(), action) {
            // Main menu navigation
            (AppState::MainMenu, Action::Select) => {
                match self.get_menu_selection() {
                    0 => AppState::Learning,      // Start Learning
                    1 => AppState::DeckSelection, // Deck Management
                    2 => AppState::Statistics,   // Statistics
                    3 => AppState::Settings,     // Settings
                    4 => AppState::Help,         // Help
                    5 => AppState::ConfirmExit,         // Quit
                    _ => AppState::MainMenu,
                }
            }

            // Global navigation
            (_, Action::Quit) => AppState::ConfirmExit,
            (_, Action::Cancel) => AppState::MainMenu,

            // Help navigation
            (_, Action::Help) => AppState::Help,

            // Learning navigation
            (AppState::Learning, Action::BackTab) => AppState::DeckSelection,

            // Deck selection navigation
            (AppState::DeckSelection, Action::Select) => AppState::Learning,
            (AppState::DeckSelection, Action::Cancel) => AppState::MainMenu,

            // Statistics navigation
            (AppState::Statistics, Action::Cancel) => AppState::MainMenu,

            // Settings navigation
            (AppState::Settings, Action::Cancel) => AppState::MainMenu,

            // Help navigation
            (AppState::Help, Action::Cancel) => {
                self.go_back();
                self.current_state.clone()
            }

            // No state change
            _ => self.current_state.clone(),
        };

        if new_state != self.current_state {
            self.transition_to(new_state.clone());
        }

        Ok(new_state)
    }

    /// Get render context for components
    pub fn get_render_context(&self) -> RenderContext {
        self.render_context.clone()
    }

    /// Update render context data
    pub fn update_context_data(&mut self, key: String, value: String) {
        self.render_context.data.insert(key, value);
    }

    /// Get menu selection (simplified - in real implementation would track this)
    fn get_menu_selection(&self) -> usize {
        // For now, return 0. In real implementation, this would be tracked in state
        0
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.render_context.focused = focused;
    }
}

/// Error types for state management
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Invalid state transition from {from} to {to}")]
    InvalidTransition { from: AppState, to: AppState },
    #[error("State history is empty")]
    EmptyHistory,
    #[error("Action not allowed in current state {state}")]
    ActionNotAllowed { state: AppState },
}


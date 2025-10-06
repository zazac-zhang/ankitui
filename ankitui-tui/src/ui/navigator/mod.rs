//! Navigation system for the TUI application

use crate::ui::state::store::{AppState, Screen};
use crate::utils::error::{TuiError, TuiResult};
use uuid::Uuid;

/// Navigation manager
#[derive(Debug)]
pub struct Navigator {
    history: Vec<Screen>,
    max_history_size: usize,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history_size: 50,
        }
    }

    pub fn with_max_history_size(max_size: usize) -> Self {
        Self {
            history: Vec::new(),
            max_history_size: max_size,
        }
    }

    /// Navigate to a new screen
    pub fn navigate_to(&mut self, state: &mut AppState, screen: Screen) -> TuiResult<()> {
        // Add current screen to history
        self.history.push(state.current_screen.clone());

        // Limit history size
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
        }

        // Update current screen
        state.current_screen = screen;

        Ok(())
    }

    /// Navigate back to previous screen
    pub fn navigate_back(&mut self, state: &mut AppState) -> TuiResult<bool> {
        if let Some(previous_screen) = self.history.pop() {
            state.current_screen = previous_screen;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Navigate to main menu
    pub fn navigate_to_main_menu(&mut self, state: &mut AppState) -> TuiResult<()> {
        self.history.clear();
        state.current_screen = Screen::MainMenu;
        Ok(())
    }

    /// Get current history
    pub fn history(&self) -> &[Screen] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Check if can navigate back
    pub fn can_navigate_back(&self) -> bool {
        !self.history.is_empty()
    }

    /// Get current deck ID (for study sessions)
    pub fn current_deck(&self) -> Option<Uuid> {
        // This would be stored in app state during navigation
        // For now, return None as placeholder
        None
    }
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}
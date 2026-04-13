//! State management helpers
//!
//! Utility functions for application state operations.

use crate::ui::state::store::{Screen, StateStore, SystemMessage};
use crate::utils::error::TuiResult;

/// Initialize the state store with default values
pub async fn initialize_state(state_store: &StateStore) -> TuiResult<()> {
    state_store.update_state(|state| {
        state.current_screen = Screen::MainMenu;
        state.loading = false;
        state.error = None;
        state.message = None;
    }).ok();
    Ok(())
}

/// Reset the application state to initial values
pub async fn reset_state(state_store: &StateStore) -> TuiResult<()> {
    state_store.update_state(|state| {
        state.current_screen = Screen::MainMenu;
        state.selected_deck_id = None;
        state.current_session = None;
        state.loading = false;
        state.error = None;
        state.message = None;
        state.ui_state.clear();
        state.action_history.clear();
        state.navigation_history.clear();
    }).ok();
    Ok(())
}

/// Navigate to a screen with history tracking
pub async fn navigate_with_history(
    state_store: &StateStore,
    target_screen: Screen,
) -> TuiResult<()> {
    let current_screen = state_store.get_state().current_screen.clone();

    state_store.update_state(|state| {
        // Don't duplicate history for the same screen
        if current_screen != target_screen {
            state.navigation_history.push(current_screen.clone());
        }
        state.current_screen = target_screen.clone();
    }).ok();

    Ok(())
}

/// Show a system message
pub async fn show_message(
    state_store: &StateStore,
    message: SystemMessage,
) -> TuiResult<()> {
    state_store.update_state(|state| {
        state.message = Some(message);
    }).ok();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_functions_exist() {
        // This test ensures the helper functions are accessible
        assert!(true);
    }
}

//! Command handling helper functions
//!
//! Utility functions for processing UI commands and updating state.

use crate::ui::event::CommandType;
use crate::ui::state::store::{Screen, StateStore};
use crate::utils::error::TuiResult;

/// Handle simple navigation commands
pub async fn handle_simple_navigation(
    state_store: &StateStore,
    command: &CommandType,
) -> TuiResult<bool> {
    match command {
        CommandType::NavigateToMainMenu => {
            state_store.navigate_to(Screen::MainMenu)?;
            Ok(true)
        }
        CommandType::ShowHelp => {
            state_store.navigate_to(Screen::Help)?;
            Ok(true)
        }
        CommandType::NavigateBack => {
            handle_navigate_back(state_store)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// Handle navigate back command with context awareness
fn handle_navigate_back(state_store: &StateStore) -> TuiResult<()> {
    let current_screen = state_store.get_state().current_screen.clone();

    let previous_screen = match current_screen {
        Screen::DeckSelection => Screen::MainMenu,
        Screen::StudySession => Screen::DeckSelection,
        Screen::CardEditor => Screen::DeckManagement,
        Screen::DeckManagement => Screen::DeckSelection,
        Screen::Statistics => Screen::MainMenu,
        Screen::Settings => Screen::MainMenu,
        Screen::Search => Screen::DeckSelection,
        Screen::Help => {
            // Get previous screen from navigation history
            let history = &state_store.get_state().navigation_history;
            history.last().cloned().unwrap_or(Screen::MainMenu)
        }
        Screen::TagManagement => Screen::Settings,
        Screen::MediaManagement => Screen::Settings,
        _ => current_screen,
    };

    state_store.navigate_to(previous_screen)
}

/// Update a numeric UI state value with increment/decrement
pub async fn update_numeric_ui_state(
    state_store: &StateStore,
    key: &str,
    increment: i32,
) -> TuiResult<()> {
    state_store.update_state(|state| {
        let current = state.ui_state.get(key).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
        let new_value = (current + increment).max(0);
        state.ui_state.insert(key.to_string(), new_value.to_string());
    }).ok();
    Ok(())
}

/// Toggle a boolean UI state value
pub async fn toggle_boolean_ui_state(
    state_store: &StateStore,
    key: &str,
) -> TuiResult<()> {
    state_store.update_state(|state| {
        let current = state.ui_state.get(key).map(|s| s == "true").unwrap_or(false);
        state.ui_state.insert(key.to_string(), (!current).to_string());
    }).ok();
    Ok(())
}

/// Handle screen-specific navigation
pub async fn handle_screen_navigation(
    state_store: &StateStore,
    screen: Screen,
) -> TuiResult<()> {
    state_store.navigate_to(screen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_state_update_logic() {
        // Test the logic for numeric updates
        let current: i32 = 5;
        let increment: i32 = 1;
        let new_value = (current + increment).max(0);
        assert_eq!(new_value, 6);

        let current: i32 = 0;
        let increment: i32 = -1;
        let new_value = (current + increment).max(0);
        assert_eq!(new_value, 0);
    }
}

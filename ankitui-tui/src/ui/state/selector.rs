//! State selectors for efficient state reading

use crate::domain::*;
use crate::ui::state::store::{AppState, Screen};
use std::collections::HashMap;

/// Trait for state selectors
pub trait StateSelector<T> {
    fn select(&self, state: &AppState) -> T;
}

/// Select current screen
pub struct CurrentScreenSelector;

impl StateSelector<Screen> for CurrentScreenSelector {
    fn select(&self, state: &AppState) -> Screen {
        state.current_screen.clone()
    }
}

/// Select selected deck ID
pub struct SelectedDeckIdSelector;

impl StateSelector<Option<uuid::Uuid>> for SelectedDeckIdSelector {
    fn select(&self, state: &AppState) -> Option<uuid::Uuid> {
        state.selected_deck_id
    }
}

/// Select current session
pub struct CurrentSessionSelector;

impl StateSelector<Option<SessionState>> for CurrentSessionSelector {
    fn select(&self, state: &AppState) -> Option<SessionState> {
        state.current_session.clone()
    }
}

/// Select user preferences
pub struct UserPreferencesSelector;

impl StateSelector<UserPreferences> for UserPreferencesSelector {
    fn select(&self, state: &AppState) -> UserPreferences {
        state.user_preferences.clone()
    }
}

/// Select loading state
pub struct LoadingStateSelector;

impl StateSelector<bool> for LoadingStateSelector {
    fn select(&self, state: &AppState) -> bool {
        state.loading
    }
}

/// Select error message
pub struct ErrorMessageSelector;

impl StateSelector<Option<String>> for ErrorMessageSelector {
    fn select(&self, state: &AppState) -> Option<String> {
        state.error.clone()
    }
}

/// Select system message
pub struct SystemMessageSelector;

impl StateSelector<Option<crate::ui::state::store::SystemMessage>> for SystemMessageSelector {
    fn select(&self, state: &AppState) -> Option<crate::ui::state::store::SystemMessage> {
        state.message.clone()
    }
}

/// Select navigation history
pub struct NavigationHistorySelector;

impl StateSelector<Vec<Screen>> for NavigationHistorySelector {
    fn select(&self, state: &AppState) -> Vec<Screen> {
        state.navigation_history.clone()
    }
}

/// Select study session progress
pub struct StudyProgressSelector;

#[derive(Debug, Clone)]
pub struct StudyProgress {
    pub cards_studied: usize,
    pub total_cards: usize,
    pub progress_percentage: f64,
    pub remaining_new: u32,
    pub remaining_review: u32,
    pub session_duration_minutes: f64,
    pub is_complete: bool,
}

impl StateSelector<Option<StudyProgress>> for StudyProgressSelector {
    fn select(&self, state: &AppState) -> Option<StudyProgress> {
        state.current_session.as_ref().map(|session| StudyProgress {
            cards_studied: session.current_position() as usize,
            total_cards: session.total_cards,
            progress_percentage: session.progress_percentage() as f64,
            remaining_new: session.remaining_new as u32,
            remaining_review: session.remaining_review as u32,
            session_duration_minutes: session.session_duration_minutes() as f64,
            is_complete: session.is_complete(),
        })
    }
}

/// Select deck selection state
pub struct DeckSelectionStateSelector;

#[derive(Debug, Clone)]
pub struct DeckSelectionState {
    pub selected_deck_id: Option<uuid::Uuid>,
    pub can_start_study: bool,
    pub can_manage_deck: bool,
}

impl StateSelector<DeckSelectionState> for DeckSelectionStateSelector {
    fn select(&self, state: &AppState) -> DeckSelectionState {
        DeckSelectionState {
            selected_deck_id: state.selected_deck_id,
            can_start_study: state.selected_deck_id.is_some(),
            can_manage_deck: state.selected_deck_id.is_some(),
        }
    }
}

/// Select main menu state
pub struct MainMenuStateSelector;

#[derive(Debug, Clone)]
pub struct MainMenuState {
    pub user_name: String,
    pub has_decks: bool,
    pub has_active_session: bool,
}

impl StateSelector<MainMenuState> for MainMenuStateSelector {
    fn select(&self, state: &AppState) -> MainMenuState {
        MainMenuState {
            user_name: state.user_preferences.display_name.clone(),
            has_decks: state.deck_count > 0,
            has_active_session: state.current_session.is_some(),
        }
    }
}

/// Utility functions for state selection
pub fn select_current_screen(state: &AppState) -> Screen {
    CurrentScreenSelector.select(state)
}

pub fn select_selected_deck_id(state: &AppState) -> Option<uuid::Uuid> {
    SelectedDeckIdSelector.select(state)
}

pub fn select_current_session(state: &AppState) -> Option<SessionState> {
    CurrentSessionSelector.select(state)
}

pub fn select_user_preferences(state: &AppState) -> UserPreferences {
    UserPreferencesSelector.select(state)
}

pub fn select_loading_state(state: &AppState) -> bool {
    LoadingStateSelector.select(state)
}

pub fn select_error_message(state: &AppState) -> Option<String> {
    ErrorMessageSelector.select(state)
}

pub fn select_system_message(state: &AppState) -> Option<crate::ui::state::store::SystemMessage> {
    SystemMessageSelector.select(state)
}

pub fn select_study_progress(state: &AppState) -> Option<StudyProgress> {
    StudyProgressSelector.select(state)
}

pub fn select_deck_selection_state(state: &AppState) -> DeckSelectionState {
    DeckSelectionStateSelector.select(state)
}

pub fn select_main_menu_state(state: &AppState) -> MainMenuState {
    MainMenuStateSelector.select(state)
}

/// Memoization cache for expensive selectors
#[derive(Debug, Default)]
pub struct SelectorCache {
    cache: HashMap<String, (chrono::DateTime<chrono::Utc>, Vec<u8>)>,
}

impl SelectorCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_compute<T, F, S>(&mut self, key: &str, compute: F, state: &AppState) -> T
    where
        T: serde::Serialize + for<'a> serde::Deserialize<'a>,
        F: FnOnce(&AppState) -> T,
        S: StateSelector<T>,
    {
        let now = chrono::Utc::now();

        // Check cache validity
        if let Some((timestamp, cached_data)) = self.cache.get(key) {
            let age = now - *timestamp;
            if age.num_seconds() < 1 { // Cache for 1 second
                if let Ok(cached_value) = bincode::deserialize::<T>(cached_data) {
                    return cached_value;
                }
            }
        }

        // Compute new value
        let value = compute(state);

        // Cache the result
        if let Ok(serialized) = bincode::serialize(&value) {
            self.cache.insert(key.to_string(), (now, serialized));
        }

        value
    }

    pub fn invalidate(&mut self) {
        self.cache.clear();
    }

    pub fn invalidate_key(&mut self, key: &str) {
        self.cache.remove(key);
    }
}
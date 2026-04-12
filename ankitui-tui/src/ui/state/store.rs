//! Centralized state store for the TUI application

use crate::domain::*;
use crate::utils::error::{TuiError, TuiResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub current_screen: Screen,
    pub selected_deck_id: Option<Uuid>,
    pub current_session: Option<SessionState>,
    pub user_preferences: UserPreferences,
    pub navigation_history: Vec<Screen>,
    pub loading: bool,
    pub error: Option<String>,
    pub message: Option<SystemMessage>,

    // Contextual state for better event handling
    pub sub_state: String,
    pub ui_state: HashMap<String, String>,
    pub last_action: Option<String>,
    pub action_history: Vec<String>,

    // Menu navigation state
    pub main_menu_selected: usize,
    pub deck_list_selected: Option<usize>,
    pub settings_selected: usize,

    // Cached deck count
    pub deck_count: usize,
}

/// UI screen types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
    MainMenu,
    DeckSelection,
    StudySession,
    Statistics,
    Settings,
    CardEditor,
    DeckManagement,
    Search,
    Help,
    StudyPrefs,
    UiSettings,
    DataManage,
}

/// System message for user notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    pub id: Uuid,
    pub level: MessageLevel,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration: Option<Duration>,
    pub auto_dismiss: bool,
}

/// Message severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Message display duration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Duration {
    Short,    // 2 seconds
    Medium,   // 5 seconds
    Long,     // 10 seconds
    Infinite, // Until dismissed
}

impl SystemMessage {
    pub fn new(level: MessageLevel, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            level,
            title,
            content,
            created_at: chrono::Utc::now(),
            duration: Some(Duration::Medium),
            auto_dismiss: true,
        }
    }

    pub fn info<S: Into<String>>(title: S, content: S) -> Self {
        Self::new(MessageLevel::Info, title.into(), content.into())
    }

    pub fn success<S: Into<String>>(title: S, content: S) -> Self {
        Self::new(MessageLevel::Success, title.into(), content.into())
    }

    pub fn warning<S: Into<String>>(title: S, content: S) -> Self {
        Self::new(MessageLevel::Warning, title.into(), content.into())
    }

    pub fn error<S: Into<String>>(title: S, content: S) -> Self {
        Self::new(MessageLevel::Error, title.into(), content.into())
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn persistent(mut self) -> Self {
        self.auto_dismiss = false;
        self.duration = Some(Duration::Infinite);
        self
    }

    pub fn is_expired(&self) -> bool {
        if !self.auto_dismiss {
            return false;
        }

        let duration_ms = match self.duration {
            Some(Duration::Short) => 2000,
            Some(Duration::Medium) => 5000,
            Some(Duration::Long) => 10000,
            Some(Duration::Infinite) => return false,
            None => 5000,
        };

        let elapsed_ms = (chrono::Utc::now() - self.created_at).num_milliseconds();
        elapsed_ms >= duration_ms as i64
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: Screen::MainMenu,
            selected_deck_id: None,
            current_session: None,
            user_preferences: UserPreferences::new("User".to_string()),
            navigation_history: Vec::new(),
            loading: false,
            error: None,
            message: None,
            sub_state: "".to_string(),
            ui_state: HashMap::new(),
            last_action: None,
            action_history: Vec::new(),
            main_menu_selected: 0,
            deck_list_selected: None,
            settings_selected: 0,
            deck_count: 0,
        }
    }
}

/// State subscription for reactive updates
#[derive(Clone)]
pub struct StateSubscription {
    pub id: Uuid,
    pub callback: Arc<dyn Fn(&AppState) + Send + Sync>,
}

impl std::fmt::Debug for StateSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateSubscription")
            .field("id", &self.id)
            .field("callback", &"<callback>")
            .finish()
    }
}

/// Centralized state store
pub struct StateStore {
    state: Arc<RwLock<AppState>>,
    subscriptions: Arc<RwLock<HashMap<Uuid, StateSubscription>>>,
}

impl std::fmt::Debug for StateStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateStore")
            .field("subscription_count", &self.subscriptions.read().unwrap().len())
            .finish()
    }
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::default())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get current state
    pub fn get_state(&self) -> AppState {
        self.state.read().unwrap().clone()
    }

    /// Update state with a mutation function
    pub fn update_state<F>(&self, mutator: F) -> TuiResult<()>
    where
        F: FnOnce(&mut AppState),
    {
        let mut state = self.state.write().unwrap();
        mutator(&mut state);

        // Notify subscribers
        self.notify_subscribers(&state);

        Ok(())
    }

    /// Navigate to a new screen
    pub fn navigate_to(&self, screen: Screen) -> TuiResult<()> {
        self.update_state(|state| {
            state.navigation_history.push(state.current_screen.clone());
            state.current_screen = screen;
        })
    }

    /// Navigate back to previous screen
    pub fn navigate_back(&self) -> TuiResult<()> {
        self.update_state(|state| {
            if let Some(previous_screen) = state.navigation_history.pop() {
                state.current_screen = previous_screen;
            }
        })
    }

    /// Set loading state
    pub fn set_loading(&self, loading: bool) -> TuiResult<()> {
        self.update_state(|state| {
            state.loading = loading;
        })
    }

    /// Set current screen
    pub fn set_current_screen(&mut self, screen: Screen) {
        self.update_state(|state| {
            state.current_screen = screen;
        })
        .unwrap();
    }

    /// Set error message
    pub fn set_error(&self, error: Option<String>) -> TuiResult<()> {
        self.update_state(|state| {
            state.error = error;
        })
    }

    /// Show system message
    pub fn show_message(&self, message: SystemMessage) -> TuiResult<()> {
        self.update_state(|state| {
            state.message = Some(message);
        })
    }

    /// Clear system message
    pub fn clear_message(&self) -> TuiResult<()> {
        self.update_state(|state| {
            state.message = None;
        })
    }

    /// Set selected deck
    pub fn set_selected_deck(&self, deck_id: Option<Uuid>) -> TuiResult<()> {
        self.update_state(|state| {
            state.selected_deck_id = deck_id;
        })
    }

    /// Set current session
    pub fn set_current_session(&self, session: Option<SessionState>) -> TuiResult<()> {
        self.update_state(|state| {
            state.current_session = session;
        })
    }

    // Menu navigation methods

    /// Get main menu selected index
    pub fn get_main_menu_selected(&self) -> usize {
        self.state.read().unwrap().main_menu_selected
    }

    /// Set main menu selected index
    pub fn set_main_menu_selected(&self, index: usize) -> TuiResult<()> {
        self.update_state(|state| {
            state.main_menu_selected = index;
        })
    }

    /// Navigate main menu up
    pub fn navigate_main_menu_up(&self) -> TuiResult<()> {
        self.update_state(|state| {
            if state.main_menu_selected > 0 {
                state.main_menu_selected -= 1;
            } else {
                state.main_menu_selected = 4; // Wrap to last item (Quit)
            }
        })
    }

    /// Navigate main menu down
    pub fn navigate_main_menu_down(&self) -> TuiResult<()> {
        self.update_state(|state| {
            if state.main_menu_selected < 4 {
                state.main_menu_selected += 1;
            } else {
                state.main_menu_selected = 0; // Wrap to first item
            }
        })
    }

    /// Get deck list selected index
    pub fn get_deck_list_selected(&self) -> Option<usize> {
        self.state.read().unwrap().deck_list_selected
    }

    /// Set deck list selected index
    pub fn set_deck_list_selected(&self, index: Option<usize>) -> TuiResult<()> {
        self.update_state(|state| {
            state.deck_list_selected = index;
        })
    }

    /// Get settings menu selected index
    pub fn get_settings_selected(&self) -> usize {
        self.state.read().unwrap().settings_selected
    }

    /// Set settings menu selected index
    pub fn set_settings_selected(&self, index: usize) -> TuiResult<()> {
        self.update_state(|state| {
            state.settings_selected = index;
        })
    }

    /// Navigate settings menu up
    pub fn navigate_settings_up(&self) -> TuiResult<()> {
        self.update_state(|state| {
            if state.settings_selected > 0 {
                state.settings_selected -= 1;
            }
        })
    }

    /// Navigate settings menu down (6 items total)
    pub fn navigate_settings_down(&self) -> TuiResult<()> {
        self.update_state(|state| {
            if state.settings_selected < 5 {
                state.settings_selected += 1;
            }
        })
    }

    /// Update user preferences
    pub fn update_user_preferences(&self, preferences: UserPreferences) -> TuiResult<()> {
        self.update_state(|state| {
            state.user_preferences = preferences;
        })
    }

    // Context-aware state methods

    /// Set sub-state for contextual event handling
    pub fn set_sub_state(&self, sub_state: String) -> TuiResult<()> {
        self.update_state(|state| {
            state.sub_state = sub_state;
        })
    }

    /// Get current sub-state
    pub fn get_sub_state(&self) -> String {
        self.state.read().unwrap().sub_state.clone()
    }

    /// Set UI state value
    pub fn set_ui_state(&self, key: String, value: String) -> TuiResult<()> {
        self.update_state(|state| {
            state.ui_state.insert(key, value);
        })
    }

    /// Get UI state value
    pub fn get_ui_state(&self, key: &str) -> Option<String> {
        self.state.read().unwrap().ui_state.get(key).cloned()
    }

    /// Record action for context
    pub fn record_action(&self, action: String) -> TuiResult<()> {
        self.update_state(|state| {
            state.last_action = Some(action.clone());
            state.action_history.push(action);

            // Keep only last 50 actions
            if state.action_history.len() > 50 {
                state.action_history.remove(0);
            }
        })
    }

    /// Get last action
    pub fn get_last_action(&self) -> Option<String> {
        self.state.read().unwrap().last_action.clone()
    }

    /// Add system message
    pub fn add_system_message(&self, message: String) {
        let _ = self.show_message(SystemMessage::info("Info", message.as_str()));
    }

    /// Set current card study state
    pub fn set_current_card_study(&self, is_studying: bool) -> TuiResult<()> {
        self.update_state(|state| {
            if is_studying {
                state.set_ui_state("card_study_mode".to_string(), "active".to_string());
            } else {
                state.set_ui_state("card_study_mode".to_string(), "inactive".to_string());
            }
        })
    }

    /// Show answer state
    pub fn set_showing_answer(&self, showing: bool) -> TuiResult<()> {
        self.update_state(|state| {
            if showing {
                state.set_ui_state("showing_answer".to_string(), "true".to_string());
                state.sub_state = "showing_answer".to_string();
            } else {
                state.set_ui_state("showing_answer".to_string(), "false".to_string());
                state.sub_state = "studying".to_string();
            }
        })
    }

    /// Check if study session is active
    pub fn is_study_session_active(&self) -> bool {
        let state = self.state.read().unwrap();
        state.current_session.is_some() && state.current_screen == Screen::StudySession
    }

    /// Check if currently showing answer
    pub fn is_showing_answer(&self) -> bool {
        self.state
            .read()
            .unwrap()
            .ui_state
            .get("showing_answer")
            .map(|s| s == "true")
            .unwrap_or(false)
    }

    /// Subscribe to state changes
    pub fn subscribe<F>(&self, callback: F) -> Uuid
    where
        F: Fn(&AppState) + Send + Sync + 'static,
    {
        let subscription_id = Uuid::new_v4();
        let subscription = StateSubscription {
            id: subscription_id,
            callback: Arc::new(callback),
        };

        self.subscriptions
            .write()
            .unwrap()
            .insert(subscription_id, subscription);
        subscription_id
    }

    /// Unsubscribe from state changes
    pub fn unsubscribe(&self, subscription_id: Uuid) -> TuiResult<()> {
        self.subscriptions.write().unwrap().remove(&subscription_id);
        Ok(())
    }

    /// Notify all subscribers of state changes
    fn notify_subscribers(&self, state: &AppState) {
        let subscriptions = self.subscriptions.read().unwrap().clone();

        for subscription in subscriptions.values() {
            (subscription.callback)(state);
        }
    }
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StateStore {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            subscriptions: Arc::clone(&self.subscriptions),
        }
    }
}

impl AppState {
    /// Create new default app state
    pub fn new() -> Self {
        Self {
            current_screen: Screen::MainMenu,
            selected_deck_id: None,
            current_session: None,
            user_preferences: UserPreferences::default(),
            navigation_history: Vec::new(),
            loading: false,
            error: None,
            message: None,
            sub_state: "default".to_string(),
            ui_state: HashMap::new(),
            last_action: None,
            action_history: Vec::new(),
            main_menu_selected: 0,
            deck_list_selected: None,
            settings_selected: 0,
            deck_count: 0,
        }
    }

    /// Get current screen
    pub fn current_screen(&self) -> Screen {
        self.current_screen.clone()
    }

    /// Get sub-state
    pub fn sub_state(&self) -> &str {
        &self.sub_state
    }

    /// Check if study session is active
    pub fn is_study_session_active(&self) -> bool {
        self.current_session.is_some() && self.current_screen == Screen::StudySession
    }

    /// Check if currently showing answer
    pub fn is_showing_answer(&self) -> bool {
        self.ui_state
            .get("showing_answer")
            .map(|s| s == "true")
            .unwrap_or(false)
    }

    /// Set UI state helper method
    pub fn set_ui_state(&mut self, key: String, value: String) {
        self.ui_state.insert(key, value);
    }

    /// Add system message
    pub fn add_system_message(&mut self, message: String) {
        self.message = Some(SystemMessage::info("Info", message.as_str()));
    }
}

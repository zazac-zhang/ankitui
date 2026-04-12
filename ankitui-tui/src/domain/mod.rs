//! Domain layer - UI application services and view models

pub mod app_state;
pub mod services;
pub mod viewmodels;

// Re-export key types with explicit imports to avoid conflicts
pub use viewmodels::{
    AppStats, CardViewModel, DeckStats, DeckViewModel, GlobalStats, SettingsSection, SettingsViewModel, StatsViewModel,
    StudySessionStats, StudySessionViewModel,
};

// Use the types from app_state.rs to avoid conflicts
pub use app_state::{
    AppState, CardRating, DeckState, SessionState, SettingsState, StatsState, StudyState, UserPreferences,
};

pub use services::*;

//! Domain layer - UI application services and view models

pub mod viewmodels;
pub mod app_state;
pub mod services;

// Re-export key types with explicit imports to avoid conflicts
pub use viewmodels::{
    DeckViewModel, CardViewModel, StudySessionViewModel, StudySessionStats,
    StatsViewModel, DeckStats, GlobalStats, SettingsViewModel, SettingsSection,
    AppStats
};

// Use the types from app_state.rs to avoid conflicts
pub use app_state::{
    CardRating, SessionState, UserPreferences, AppState, DeckState, StudyState,
    StatsState, SettingsState
};

pub use services::*;
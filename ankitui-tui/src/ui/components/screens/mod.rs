//! Screen components for the TUI application

// Screen modules
pub mod common;
pub mod deck;
pub mod study;
pub mod stats;
pub mod settings;
pub mod menu;

// Re-export main screen types
pub use menu::MenuScreen;
pub use deck::{DeckScreen, DeckCreateScreen, DeckEditScreen, DeckManageScreen};
pub use study::{StudyQuestionScreen, StudyAnswerScreen, StudyRatingScreen, StudyFinishedScreen};
pub use stats::{StatsScreen, GlobalStatsScreen, DeckStatsScreen, ProgressScreen};
pub use settings::{SettingsScreen, StudyPrefsScreen, UiSettingsScreen, DataManageScreen};
pub use common::{LoadingScreen, ErrorScreen, ConfirmScreen, InputScreen};
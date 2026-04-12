//! Screen components for the TUI application

// Screen modules
pub mod common;
pub mod deck;
pub mod help;
pub mod menu;
pub mod search;
pub mod settings;
pub mod stats;
pub mod study;

// Re-export main screen types
pub use common::{ConfirmScreen, ErrorScreen, InputScreen, LoadingScreen};
pub use deck::{DeckCreateScreen, DeckEditScreen, DeckManageScreen, DeckScreen};
pub use help::HelpScreen;
pub use menu::MenuScreen;
pub use search::{SearchScreen, SearchType};
pub use settings::{DataManageScreen, SettingsScreen, StudyPrefsScreen, UiSettingsScreen};
pub use stats::{DeckStatsScreen, GlobalStatsScreen, ProgressScreen, StatsScreen};
pub use study::{StudyAnswerScreen, StudyFinishedScreen, StudyQuestionScreen, StudyRatingScreen};

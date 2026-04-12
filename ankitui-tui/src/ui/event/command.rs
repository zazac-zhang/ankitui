//! Command system for the TUI application - State-aware

use crate::domain::CardRating;
use crate::ui::state::Screen;
use std::collections::HashMap;
use uuid::Uuid;

/// Command types for the application - State-aware
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType {
    // Navigation commands
    NavigateTo(Screen),
    NavigateBack,
    NavigateToMainMenu,
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    NavigatePageUp,
    NavigatePageDown,
    NavigateHome,
    NavigateEnd,

    // Selection commands
    Select,
    Cancel,
    Confirm,
    SelectPreviousDeck,
    SelectNextDeck,

    // System commands
    Quit,
    Resize(u16, u16),
    FocusGained,
    FocusLost,
    Paste(String),

    // Deck commands
    LoadDecks,
    SelectDeck(Uuid),
    StartStudySessionDefault,
    StartStudySession(Uuid, Option<u32>),
    CreateDeck(String, Option<String>),
    CreateDeckPrompt,
    DeleteDeck(Uuid),
    DeleteDeckPrompt,
    UpdateDeck(Uuid, String, Option<String>),
    FindDeck(String),

    // Study session commands
    EndStudySession,
    RateCurrentCard(CardRating),
    ShowAnswer,
    HideAnswer,
    SkipCurrentCard,
    RefreshSession,

    // Card commands
    CreateCard(String, String, Vec<String>),
    CreateCardPrompt,
    UpdateCard(Uuid, Option<String>, Option<String>, Option<Vec<String>>),
    DeleteCard(Uuid),
    SaveCard,
    CancelEdit,
    ToggleCardSide,
    PasteCardContent(String),

    // User preference commands
    LoadUserPreferences,
    UpdateUserPreferences(HashMap<String, String>),
    LoadUserStats,

    // Session control commands
    PauseSession,
    ResumeSession,

    // UI control commands
    ShowMessage(String),
    ClearMessage,
    SetLoading(bool),
    ClearError,
    ShowHelp,
    RefreshScreen,

    // Configuration commands
    UpdateTheme(String),
    UpdateLanguage(String),
    UpdateStudyGoals(u32, u32),
    ConfirmSetting,

    // Search commands
    SearchDecks(String),
    SearchCards(String),
    StartSearch,
    SearchBackspace,
    ImportCards(String),

    // Statistics commands
    LoadStatistics(Uuid),
    RefreshStatistics,
    ScrollStatsUp,
    ScrollStatsDown,

    // Mouse interaction commands
    Click(u16, u16),
    RightClick(u16, u16),
    MouseMove(u16, u16),
    UpdateHover(u16, u16),
    ShowDeckContextMenu(u16, u16),
    ShowCardContextMenu(u16, u16),

    // Scrolling commands
    ScrollUp,
    ScrollDown,

    // Unknown/Unhandled commands
    Unknown,
}

/// Command wrapper with metadata
#[derive(Debug, Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub metadata: CommandMetadata,
}

/// Command metadata for additional context
#[derive(Debug, Clone, Default)]
pub struct CommandMetadata {
    pub screen: Option<crate::ui::state::Screen>,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub deck_id: Option<Uuid>,
    pub card_id: Option<Uuid>,
    pub extra: HashMap<String, String>,
}

impl Command {
    /// Create a new command with default metadata
    pub fn new(command_type: CommandType) -> Self {
        Self {
            command_type,
            metadata: CommandMetadata::default(),
        }
    }

    /// Create a user command
    pub fn user(command_type: CommandType) -> Self {
        Self::new(command_type)
    }

    /// Create a system command
    pub fn system(command_type: CommandType) -> Self {
        let mut command = Self::new(command_type);
        command
            .metadata
            .extra
            .insert("source".to_string(), "system".to_string());
        command
    }

    /// Add metadata to command
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.extra.insert(key, value);
        self
    }

    /// Set screen context for command
    pub fn with_screen(mut self, screen: crate::ui::state::Screen) -> Self {
        self.metadata.screen = Some(screen);
        self
    }

    /// Get command description for logging
    pub fn description(&self) -> String {
        match &self.command_type {
            CommandType::NavigateTo(screen) => format!("Navigate to {:?}", screen),
            CommandType::NavigateBack => "Navigate back".to_string(),
            CommandType::NavigateToMainMenu => "Navigate to main menu".to_string(),
            CommandType::NavigateUp => "Navigate up".to_string(),
            CommandType::NavigateDown => "Navigate down".to_string(),
            CommandType::NavigateLeft => "Navigate left".to_string(),
            CommandType::NavigateRight => "Navigate right".to_string(),
            CommandType::NavigatePageUp => "Navigate page up".to_string(),
            CommandType::NavigatePageDown => "Navigate page down".to_string(),
            CommandType::NavigateHome => "Navigate home".to_string(),
            CommandType::NavigateEnd => "Navigate end".to_string(),
            CommandType::Select => "Select".to_string(),
            CommandType::Cancel => "Cancel".to_string(),
            CommandType::Confirm => "Confirm".to_string(),
            CommandType::Quit => "Quit".to_string(),
            CommandType::LoadDecks => "Load decks".to_string(),
            CommandType::SelectDeck(deck_id) => format!("Select deck: {}", deck_id),
            CommandType::StartStudySession(deck_id, _) => format!("Start study session: {}", deck_id),
            CommandType::CreateDeck(name, _) => format!("Create deck: {}", name),
            CommandType::DeleteDeck(deck_id) => format!("Delete deck: {}", deck_id),
            CommandType::UpdateDeck(deck_id, name, _) => format!("Update deck {}: {}", deck_id, name),
            CommandType::EndStudySession => "End study session".to_string(),
            CommandType::RateCurrentCard(rating) => format!("Rate card: {:?}", rating),
            CommandType::ShowAnswer => "Show answer".to_string(),
            CommandType::HideAnswer => "Hide answer".to_string(),
            CommandType::CreateCard(front, back, tags) => {
                format!("Create card: {} [{}]", front, tags.join(", "))
            }
            CommandType::UpdateCard(card_id, front, back, tags) => {
                let tags_str = tags.clone().map_or_else(|| "no tags".to_string(), |t| t.join(", "));
                format!(
                    "Update card {}: {} -> {} [{}]",
                    card_id,
                    front.as_deref().unwrap_or("no change"),
                    back.as_deref().unwrap_or("no change"),
                    tags_str
                )
            }
            CommandType::DeleteCard(card_id) => format!("Delete card: {}", card_id),
            CommandType::LoadUserPreferences => "Load user preferences".to_string(),
            CommandType::UpdateUserPreferences(_) => "Update user preferences".to_string(),
            CommandType::LoadUserStats => "Load user statistics".to_string(),
            CommandType::PauseSession => "Pause session".to_string(),
            CommandType::ResumeSession => "Resume session".to_string(),
            CommandType::SkipCurrentCard => "Skip current card".to_string(),
            CommandType::ShowMessage(_) => "Show message".to_string(),
            CommandType::ClearMessage => "Clear message".to_string(),
            CommandType::SetLoading(loading) => format!("Set loading: {}", loading),
            CommandType::ClearError => "Clear error".to_string(),
            CommandType::UpdateTheme(theme) => format!("Update theme: {}", theme),
            CommandType::UpdateLanguage(language) => format!("Update language: {}", language),
            CommandType::UpdateStudyGoals(cards, minutes) => {
                format!("Update study goals: {} cards, {} minutes", cards, minutes)
            }
            CommandType::SearchDecks(query) => format!("Search decks: {}", query),
            CommandType::SearchCards(query) => format!("Search cards: {}", query),
            CommandType::LoadStatistics(deck_id) => format!("Load statistics for deck: {}", deck_id),
            CommandType::RefreshStatistics => "Refresh statistics".to_string(),
            CommandType::Unknown => "Unknown command".to_string(),
            _ => format!("Command: {:?}", self.command_type),
        }
    }

    /// Check if command is valid in current context
    pub fn is_valid_for_screen(&self, screen: &crate::ui::state::Screen) -> bool {
        match &self.command_type {
            CommandType::NavigateTo(_)
            | CommandType::NavigateBack
            | CommandType::NavigateToMainMenu
            | CommandType::NavigateUp
            | CommandType::NavigateDown
            | CommandType::NavigateLeft
            | CommandType::NavigateRight
            | CommandType::NavigatePageUp
            | CommandType::NavigatePageDown
            | CommandType::NavigateHome
            | CommandType::NavigateEnd
            | CommandType::Select
            | CommandType::Cancel
            | CommandType::Confirm
            | CommandType::Quit => true,
            CommandType::LoadDecks | CommandType::CreateDeck(_, _) => matches!(
                screen,
                crate::ui::state::Screen::DeckSelection | crate::ui::state::Screen::MainMenu
            ),
            CommandType::SelectDeck(_) | CommandType::DeleteDeck(_) | CommandType::UpdateDeck(_, _, _) => {
                matches!(screen, crate::ui::state::Screen::DeckSelection)
            }
            CommandType::StartStudySession(_, _) => matches!(screen, crate::ui::state::Screen::DeckSelection),
            CommandType::EndStudySession
            | CommandType::RateCurrentCard(_)
            | CommandType::ShowAnswer
            | CommandType::HideAnswer
            | CommandType::PauseSession
            | CommandType::ResumeSession
            | CommandType::SkipCurrentCard => matches!(screen, crate::ui::state::Screen::StudySession),
            CommandType::CreateCard(_, _, _)
            | CommandType::UpdateCard(_, _, _, _)
            | CommandType::DeleteCard(_) => matches!(
                screen,
                crate::ui::state::Screen::CardEditor | crate::ui::state::Screen::DeckManagement
            ),
            CommandType::LoadUserPreferences
            | CommandType::UpdateUserPreferences(_)
            | CommandType::UpdateTheme(_)
            | CommandType::UpdateLanguage(_)
            | CommandType::UpdateStudyGoals(_, _) => matches!(screen, crate::ui::state::Screen::Settings),
            CommandType::LoadUserStats | CommandType::LoadStatistics(_) | CommandType::RefreshStatistics => {
                matches!(screen, crate::ui::state::Screen::Statistics)
            }
            CommandType::ShowMessage(_)
            | CommandType::ClearMessage
            | CommandType::SetLoading(_)
            | CommandType::ClearError => true,
            CommandType::ShowHelp => matches!(screen, crate::ui::state::Screen::Help),
            CommandType::SearchDecks(_) | CommandType::SearchCards(_) | CommandType::StartSearch => {
                matches!(screen, crate::ui::state::Screen::Search)
            }
            _ => false,
        }
    }
}

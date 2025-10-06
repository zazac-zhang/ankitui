//! Application state management for TUI
//!
//! Aligns TUI application state with core session and card states

use ankitui_core::data::{Card, CardState};
use ankitui_core::core::Rating;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Card rating for UI events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardRating {
    Again,
    Hard,
    Good,
    Easy,
}

impl From<CardRating> for Rating {
    fn from(rating: CardRating) -> Self {
        match rating {
            CardRating::Again => Rating::Again,
            CardRating::Hard => Rating::Hard,
            CardRating::Good => Rating::Good,
            CardRating::Easy => Rating::Easy,
        }
    }
}

impl From<Rating> for CardRating {
    fn from(rating: Rating) -> Self {
        match rating {
            Rating::Again => CardRating::Again,
            Rating::Hard => CardRating::Hard,
            Rating::Good => CardRating::Good,
            Rating::Easy => CardRating::Easy,
        }
    }
}

/// Session state for tracking study sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub deck_id: Uuid,
    pub cards_studied: usize,
    pub total_cards: usize,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub remaining_new: usize,
    pub remaining_review: usize,
}

impl SessionState {
    pub fn new(deck_id: Uuid) -> Self {
        Self {
            deck_id,
            cards_studied: 0,
            total_cards: 0,
            start_time: chrono::Utc::now(),
            remaining_new: 0,
            remaining_review: 0,
        }
    }

    pub fn current_position(&self) -> usize {
        self.cards_studied
    }

    pub fn progress_percentage(&self) -> f32 {
        if self.total_cards == 0 {
            0.0
        } else {
            (self.cards_studied as f32 / self.total_cards as f32) * 100.0
        }
    }

    pub fn session_duration_minutes(&self) -> f32 {
        let duration = chrono::Utc::now() - self.start_time;
        duration.num_minutes() as f32
    }

    pub fn is_complete(&self) -> bool {
        self.cards_studied >= self.total_cards
    }
}

/// User preferences configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub display_name: String,
    pub theme: String,
    pub auto_advance: bool,
    pub show_progress: bool,
}

impl UserPreferences {
    pub fn new(name: String) -> Self {
        Self {
            display_name: name,
            theme: "default".to_string(),
            auto_advance: false,
            show_progress: true,
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            display_name: "User".to_string(),
            theme: "default".to_string(),
            auto_advance: false,
            show_progress: true,
        }
    }
}

/// Main application state
#[derive(Debug, Clone)]
pub enum AppState {
    /// Main menu screen
    MainMenu,
    /// Deck selection and management
    DeckSelection(DeckState),
    /// Active study session
    StudySession(StudyState),
    /// Statistics and analytics
    Statistics(StatsState),
    /// Settings and configuration
    Settings(SettingsState),
}

/// Deck management states
#[derive(Debug, Clone)]
pub enum DeckState {
    /// Browsing available decks
    Browsing,
    /// Creating new deck
    Creating,
    /// Editing existing deck
    Editing { deck_id: Uuid },
    /// Managing cards in deck
    ManagingCards { deck_id: Uuid },
    /// Card creation/editing
    EditingCard { deck_id: Uuid, card_id: Option<Uuid> },
}

/// Study session states - aligned with core SessionState
#[derive(Debug, Clone)]
pub enum StudyState {
    /// Session not started
    NotStarted { deck_id: Uuid },
    /// Active card review
    InProgress {
        current_card: Option<Card>,
        show_answer: bool,
        card_start_time: chrono::DateTime<chrono::Utc>,
    },
    /// Session paused
    Paused {
        resume_state: Box<StudyState>,
        pause_time: chrono::DateTime<chrono::Utc>,
    },
    /// Session finished with results
    Finished {
        cards_studied: usize,
        completion_time: chrono::DateTime<chrono::Utc>,
    },
    /// Review rating selection
    Rating {
        card: Card,
        response_time: i64, // milliseconds
    },
    /// Loading next card
    Loading,
}

/// Statistics viewing states
#[derive(Debug, Clone)]
pub enum StatsState {
    /// Global statistics overview
    GlobalOverview,
    /// Specific deck statistics
    DeckStats { deck_id: Uuid },
    /// Learning progress charts
    ProgressCharts,
    /// Achievement system
    Achievements,
}

/// Settings management states
#[derive(Debug, Clone)]
pub enum SettingsState {
    /// Main settings menu
    MainMenu,
    /// Study preferences
    StudyPreferences,
    /// UI customization
    UiCustomization,
    /// Data management
    DataManagement,
}

impl AppState {
    /// Check if session is in a specific state
    pub fn is_session_in_state(&self, expected_state: &StudyState) -> bool {
        match self {
            AppState::StudySession(current_state) => std::mem::discriminant(current_state) == std::mem::discriminant(expected_state),
            _ => false,
        }
    }

    /// Check if application is in active study mode
    pub fn is_studying(&self) -> bool {
        matches!(self, AppState::StudySession(_))
    }

    /// Get current deck ID if available
    pub fn get_current_deck_id(&self) -> Option<Uuid> {
        match self {
            AppState::DeckSelection(state) => match state {
                DeckState::Editing { deck_id }
                | DeckState::ManagingCards { deck_id }
                | DeckState::EditingCard { deck_id, .. } => Some(*deck_id),
                _ => None,
            },

            AppState::StudySession(study_state) => {
                match study_state {
                    StudyState::NotStarted { deck_id } => Some(*deck_id),
                    StudyState::Finished { .. } => None,
                    StudyState::InProgress { current_card, .. }
                        => current_card.as_ref().map(|card| card.content.id),
                    StudyState::Paused { resume_state, .. } => {
                        match resume_state.as_ref() {
                            StudyState::NotStarted { deck_id } => Some(*deck_id),
                            StudyState::InProgress { current_card, .. }
                                => current_card.as_ref().map(|card| card.content.id),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl StudyState {
    /// Get current card if available
    pub fn get_current_card(&self) -> Option<&Card> {
        match self {
            StudyState::InProgress { current_card, .. } => current_card.as_ref(),
            StudyState::Rating { card, .. } => Some(card),
            StudyState::Paused { resume_state, .. } => resume_state.get_current_card(),
            _ => None,
        }
    }

    /// Check if answer should be shown
    pub fn should_show_answer(&self) -> bool {
        match self {
            StudyState::InProgress { show_answer, .. } => *show_answer,
            StudyState::Rating { .. } => true,
            StudyState::Paused { resume_state, .. } => resume_state.should_show_answer(),
            _ => false,
        }
    }

    /// Calculate response time for current card
    pub fn get_response_time_ms(&self) -> Option<i64> {
        match self {
            StudyState::InProgress { card_start_time, .. } => {
                Some((chrono::Utc::now() - *card_start_time).num_milliseconds())
            }
            StudyState::Rating { response_time, .. } => Some(*response_time),
            StudyState::Paused { pause_time, resume_state } => {
                // Account for pause time in response calculation
                let base_time = resume_state.get_response_time_ms()?;
                let pause_duration = (chrono::Utc::now() - *pause_time).num_milliseconds();
                Some(base_time + pause_duration)
            }
            _ => None,
        }
    }
}
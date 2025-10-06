//! UI View models for displaying core data models
//!
//! Provides UI-specific state for TUI components while using core models directly

use ankitui_core::data::models::{Card, Deck, CardState};
use std::collections::HashMap;
use uuid::Uuid;

/// Deck view model for UI display with selection state
#[derive(Debug, Clone)]
pub struct DeckViewModel {
    pub deck: Deck,
    pub selected: bool,
    pub focused: bool,
}

impl DeckViewModel {
    pub fn new(deck: Deck) -> Self {
        Self {
            deck,
            selected: false,
            focused: false,
        }
    }

    pub fn with_selection(deck: Deck, selected: bool) -> Self {
        Self {
            deck,
            selected,
            focused: false,
        }
    }

    pub fn with_focus(deck: Deck, focused: bool) -> Self {
        Self {
            deck,
            selected: false,
            focused,
        }
    }

    pub fn display_name(&self) -> &str {
        &self.deck.name
    }

    pub fn has_due_cards(&self) -> bool {
        // Check if deck has any cards with due state
        // This would need to be calculated from deck data
        self.deck.description.is_some() // placeholder
    }

    pub fn has_new_cards(&self) -> bool {
        // Check if deck has any new cards
        // This would need to be calculated from deck data
        true // placeholder
    }
}

/// Card view model for study sessions
#[derive(Debug, Clone)]
pub struct CardViewModel {
    pub card: Card,
    pub show_answer: bool,
    pub question_shown_at: chrono::DateTime<chrono::Utc>,
}

impl CardViewModel {
    pub fn new(card: Card) -> Self {
        Self {
            card,
            show_answer: false,
            question_shown_at: chrono::Utc::now(),
        }
    }

    pub fn reveal_answer(&mut self) {
        self.show_answer = true;
    }

    pub fn hide_answer(&mut self) {
        self.show_answer = false;
    }

    pub fn question_time_ms(&self) -> i64 {
        let duration = chrono::Utc::now() - self.question_shown_at;
        duration.num_milliseconds()
    }

    pub fn answer_time_ms(&self) -> Option<i64> {
        if self.show_answer {
            Some(self.question_time_ms())
        } else {
            None
        }
    }

    pub fn get_state_display(&self) -> &'static str {
        match self.card.state.state {
            CardState::New => "New",
            CardState::Learning => "Learning",
            CardState::Review => "Review",
            CardState::Relearning => "Relearning",
            CardState::Buried => "Buried",
            CardState::Suspended => "Suspended",
        }
    }

    pub fn get_interval_display(&self) -> String {
        if self.card.state.interval == 0 {
            "New".to_string()
        } else if self.card.state.interval == 1 {
            "1 day".to_string()
        } else {
            format!("{} days", self.card.state.interval)
        }
    }

    pub fn get_ease_display(&self) -> String {
        format!("{:.0}%", self.card.state.ease_factor * 100.0)
    }
}

/// Study session view model
#[derive(Debug, Clone)]
pub struct StudySessionViewModel {
    pub deck_name: String,
    pub current_card: Option<CardViewModel>,
    pub cards_remaining: usize,
    pub session_time: chrono::Duration,
    pub session_stats: StudySessionStats,
}

impl StudySessionViewModel {
    pub fn new(deck_name: String) -> Self {
        Self {
            deck_name,
            current_card: None,
            cards_remaining: 0,
            session_time: chrono::Duration::zero(),
            session_stats: StudySessionStats::new(),
        }
    }

    pub fn with_current_card(deck_name: String, card: CardViewModel) -> Self {
        Self {
            deck_name,
            current_card: Some(card),
            cards_remaining: 0,
            session_time: chrono::Duration::zero(),
            session_stats: StudySessionStats::new(),
        }
    }
}

/// Study session statistics
#[derive(Debug, Clone)]
pub struct StudySessionStats {
    pub cards_studied: usize,
    pub total_cards_studied: usize,
    pub new_cards: usize,
    pub review_cards: usize,
    pub correct_answers: usize,
    pub average_time_seconds: f32,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl StudySessionStats {
    pub fn new() -> Self {
        Self {
            cards_studied: 0,
            total_cards_studied: 0,
            new_cards: 0,
            review_cards: 0,
            correct_answers: 0,
            average_time_seconds: 0.0,
            started_at: chrono::Utc::now(),
            ended_at: None,
        }
    }

    pub fn get_accuracy(&self) -> f32 {
        if self.total_cards_studied == 0 {
            0.0
        } else {
            self.correct_answers as f32 / self.total_cards_studied as f32
        }
    }

    pub fn get_time_display(&self) -> String {
        if self.average_time_seconds < 60.0 {
            format!("{:.1}s", self.average_time_seconds)
        } else {
            let minutes = (self.average_time_seconds / 60.0) as u32;
            let seconds = (self.average_time_seconds % 60.0) as u32;
            format!("{}m {}s", minutes, seconds)
        }
    }
}

/// Statistics view model for dashboard
#[derive(Debug, Clone)]
pub struct StatsViewModel {
    pub deck_stats: HashMap<Uuid, DeckStats>,
    pub global_stats: GlobalStats,
    pub selected_deck: Option<Uuid>,
}

impl StatsViewModel {
    pub fn new() -> Self {
        Self {
            deck_stats: HashMap::new(),
            global_stats: GlobalStats::new(),
            selected_deck: None,
        }
    }
}

/// Individual deck statistics
#[derive(Debug, Clone)]
pub struct DeckStats {
    pub deck_id: Uuid,
    pub deck_name: String,
    pub total_cards: usize,
    pub due_cards: usize,
    pub new_cards: usize,
    pub learned_cards: usize,
    pub retention_rate: f32,
    pub average_ease: f32,
}

/// Global statistics across all decks
#[derive(Debug, Clone)]
pub struct GlobalStats {
    pub total_decks: usize,
    pub total_cards: usize,
    pub cards_due_today: usize,
    pub cards_studied_today: usize,
    pub study_streak_days: i32,
    pub overall_retention: f32,
}

impl GlobalStats {
    pub fn new() -> Self {
        Self {
            total_decks: 0,
            total_cards: 0,
            cards_due_today: 0,
            cards_studied_today: 0,
            study_streak_days: 0,
            overall_retention: 0.0,
        }
    }
}

/// Settings view model
#[derive(Debug, Clone)]
pub struct SettingsViewModel {
    pub current_section: SettingsSection,
    pub changed_values: HashMap<String, String>,
}

impl SettingsViewModel {
    pub fn new() -> Self {
        Self {
            current_section: SettingsSection::StudyPreferences,
            changed_values: HashMap::new(),
        }
    }

    pub fn with_section(section: SettingsSection) -> Self {
        Self {
            current_section: section,
            changed_values: HashMap::new(),
        }
    }

    pub fn has_unsaved_changes(&self) -> bool {
        !self.changed_values.is_empty()
    }

    pub fn mark_dirty(&mut self, key: &str, value: String) {
        self.changed_values.insert(key.to_string(), value);
    }

    pub fn clear_changes(&mut self) {
        self.changed_values.clear();
    }
}

/// Settings sections
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsSection {
    StudyPreferences,
    UiCustomization,
    DataManagement,
}
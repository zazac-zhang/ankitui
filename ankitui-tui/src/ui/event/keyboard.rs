//! Keyboard event handling

use crossterm::event::{KeyCode, KeyEvent as CrosstermKeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

/// Keyboard event wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl KeyEvent {
    pub fn new(key_code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self {
            key_code,
            modifiers,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn char(key: char) -> Self {
        Self::new(KeyCode::Char(key), KeyModifiers::NONE)
    }

    pub fn ctrl_char(key: char) -> Self {
        Self::new(KeyCode::Char(key), KeyModifiers::CONTROL)
    }

    pub fn alt_char(key: char) -> Self {
        Self::new(KeyCode::Char(key), KeyModifiers::ALT)
    }

    pub fn function_key(n: u8) -> Self {
        Self::new(KeyCode::F(n), KeyModifiers::NONE)
    }

    pub fn is_char(&self, c: char) -> bool {
        matches!(self.key_code, KeyCode::Char(k) if k == c)
    }

    pub fn is_ctrl_char(&self, c: char) -> bool {
        matches!(self.key_code, KeyCode::Char(k) if k == c) && self.modifiers.contains(KeyModifiers::CONTROL)
    }

    pub fn is_alt_char(&self, c: char) -> bool {
        matches!(self.key_code, KeyCode::Char(k) if k == c) && self.modifiers.contains(KeyModifiers::ALT)
    }

    pub fn is_function_key(&self, n: u8) -> bool {
        matches!(self.key_code, KeyCode::F(k) if k == n)
    }

    pub fn is_enter(&self) -> bool {
        matches!(self.key_code, KeyCode::Enter)
    }

    pub fn is_escape(&self) -> bool {
        matches!(self.key_code, KeyCode::Esc)
    }

    pub fn is_space(&self) -> bool {
        matches!(self.key_code, KeyCode::Char(' '))
    }

    pub fn is_tab(&self) -> bool {
        matches!(self.key_code, KeyCode::Tab)
    }

    pub fn is_backtab(&self) -> bool {
        matches!(self.key_code, KeyCode::BackTab)
    }

    pub fn is_backspace(&self) -> bool {
        matches!(self.key_code, KeyCode::Backspace)
    }

    pub fn is_delete(&self) -> bool {
        matches!(self.key_code, KeyCode::Delete)
    }

    pub fn is_insert(&self) -> bool {
        matches!(self.key_code, KeyCode::Insert)
    }

    pub fn is_home(&self) -> bool {
        matches!(self.key_code, KeyCode::Home)
    }

    pub fn is_end(&self) -> bool {
        matches!(self.key_code, KeyCode::End)
    }

    pub fn is_page_up(&self) -> bool {
        matches!(self.key_code, KeyCode::PageUp)
    }

    pub fn is_page_down(&self) -> bool {
        matches!(self.key_code, KeyCode::PageDown)
    }

    pub fn is_up(&self) -> bool {
        matches!(self.key_code, KeyCode::Up)
    }

    pub fn is_down(&self) -> bool {
        matches!(self.key_code, KeyCode::Down)
    }

    pub fn is_left(&self) -> bool {
        matches!(self.key_code, KeyCode::Left)
    }

    pub fn is_right(&self) -> bool {
        matches!(self.key_code, KeyCode::Right)
    }

    pub fn has_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    pub fn has_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }

    pub fn has_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    pub fn description(&self) -> String {
        let mut parts = Vec::new();

        if self.has_ctrl() {
            parts.push("Ctrl");
        }
        if self.has_alt() {
            parts.push("Alt");
        }
        if self.has_shift() {
            parts.push("Shift");
        }

        let key_desc = match self.key_code {
            KeyCode::Backspace => "Backspace",
            KeyCode::Enter => "Enter",
            KeyCode::Left => "Left",
            KeyCode::Right => "Right",
            KeyCode::Up => "Up",
            KeyCode::Down => "Down",
            KeyCode::Home => "Home",
            KeyCode::End => "End",
            KeyCode::PageUp => "PageUp",
            KeyCode::PageDown => "PageDown",
            KeyCode::Tab => "Tab",
            KeyCode::BackTab => "BackTab",
            KeyCode::Delete => "Delete",
            KeyCode::Insert => "Insert",
            KeyCode::F(n) => &format!("F{}", n),
            KeyCode::Char(c) => &c.to_string(),
            KeyCode::Null => "Null",
            KeyCode::Esc => "Esc",
            _ => "Unknown",
        };

        if parts.is_empty() {
            key_desc.to_string()
        } else {
            format!("{}+{}", parts.join("+"), key_desc)
        }
    }
}

impl From<CrosstermKeyEvent> for KeyEvent {
    fn from(event: CrosstermKeyEvent) -> Self {
        Self::new(event.code, event.modifiers)
    }
}

/// Keyboard action mapping
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAction {
    // Navigation
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    NavigatePageUp,
    NavigatePageDown,
    NavigateHome,
    NavigateEnd,
    NavigateToTop,
    NavigateToBottom,

    // Selection
    Select,
    Deselect,
    SelectAll,
    DeselectAll,

    // Actions
    Confirm,
    Cancel,
    Back,
    Help,
    Search,
    Filter,

    // Study actions
    RateAgain,
    RateHard,
    RateGood,
    RateEasy,
    ShowAnswer,
    HideAnswer,
    FlipCard,

    // Editing
    Edit,
    Create,
    Delete,
    Save,
    Discard,

    // Application
    Quit,
    Refresh,
    ToggleFullscreen,
    ToggleDebug,

    // Media
    PlayMedia,
    PauseMedia,
    StopMedia,

    // Custom
    Custom(String),
}

impl KeyAction {
    pub fn description(&self) -> String {
        match self {
            KeyAction::NavigateUp => "Navigate up".to_string(),
            KeyAction::NavigateDown => "Navigate down".to_string(),
            KeyAction::NavigateLeft => "Navigate left".to_string(),
            KeyAction::NavigateRight => "Navigate right".to_string(),
            KeyAction::NavigatePageUp => "Page up".to_string(),
            KeyAction::NavigatePageDown => "Page down".to_string(),
            KeyAction::NavigateHome => "Go to home".to_string(),
            KeyAction::NavigateEnd => "Go to end".to_string(),
            KeyAction::NavigateToTop => "Go to top".to_string(),
            KeyAction::NavigateToBottom => "Go to bottom".to_string(),
            KeyAction::Select => "Select".to_string(),
            KeyAction::Deselect => "Deselect".to_string(),
            KeyAction::SelectAll => "Select all".to_string(),
            KeyAction::DeselectAll => "Deselect all".to_string(),
            KeyAction::Confirm => "Confirm".to_string(),
            KeyAction::Cancel => "Cancel".to_string(),
            KeyAction::Back => "Go back".to_string(),
            KeyAction::Help => "Show help".to_string(),
            KeyAction::Search => "Search".to_string(),
            KeyAction::Filter => "Filter".to_string(),
            KeyAction::RateAgain => "Rate card: Again".to_string(),
            KeyAction::RateHard => "Rate card: Hard".to_string(),
            KeyAction::RateGood => "Rate card: Good".to_string(),
            KeyAction::RateEasy => "Rate card: Easy".to_string(),
            KeyAction::ShowAnswer => "Show answer".to_string(),
            KeyAction::HideAnswer => "Hide answer".to_string(),
            KeyAction::FlipCard => "Flip card".to_string(),
            KeyAction::Edit => "Edit".to_string(),
            KeyAction::Create => "Create".to_string(),
            KeyAction::Delete => "Delete".to_string(),
            KeyAction::Save => "Save".to_string(),
            KeyAction::Discard => "Discard".to_string(),
            KeyAction::Quit => "Quit application".to_string(),
            KeyAction::Refresh => "Refresh".to_string(),
            KeyAction::ToggleFullscreen => "Toggle fullscreen".to_string(),
            KeyAction::ToggleDebug => "Toggle debug mode".to_string(),
            KeyAction::PlayMedia => "Play media".to_string(),
            KeyAction::PauseMedia => "Pause media".to_string(),
            KeyAction::StopMedia => "Stop media".to_string(),
            KeyAction::Custom(desc) => desc.clone(),
        }
    }
}

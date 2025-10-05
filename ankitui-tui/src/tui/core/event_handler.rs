//! Event Handling
//!
//! Handles keyboard events and user input for the TUI interface

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Represents user actions in the application
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // Navigation
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    Tab,
    BackTab,

    // Selection
    Select,
    Cancel,

    // Card review
    ShowAnswer,
    RateAgain,
    RateHard,
    RateGood,
    RateEasy,
    SkipCard,
    ResetCard,

    // Deck management
    Create,
    Delete,
    Edit,
    Info,
    SwitchDeck,

    // Application
    Quit,
    Help,
    Refresh,

    // Text input
    Char(char),
    Backspace,

    // Settings editing
    StartEdit,
    Save,
    Discard,

    // Unknown
    Unknown,
}

/// Event handler for keyboard input
pub struct Events {
    rx: mpsc::Receiver<Action>,
    _handle: thread::JoinHandle<()>,
}

impl Events {
    /// Create a new event handler
    pub fn new(tick_rate: Duration) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut last_tick = std::time::Instant::now();

            loop {
                // Poll for events with timeout
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_millis(1));

                if event::poll(timeout).unwrap_or(false) {
                    if let Ok(Event::Key(key)) = event::read() {
                        let action = Self::convert_key_event(key);
                        if tx.send(action).is_err() {
                            break;
                        }
                    }
                }

                // Send tick event for periodic updates
                if last_tick.elapsed() >= tick_rate {
                    last_tick = std::time::Instant::now();
                    // Note: We could add a Tick action here if needed for animations
                }
            }
        });

        Ok(Self {
            rx,
            _handle: handle,
        })
    }

    /// Get the next action from the event queue
    pub fn next(&self) -> Result<Action> {
        match self.rx.recv() {
            Ok(action) => Ok(action),
            Err(_) => Err(anyhow::anyhow!("Event channel closed")),
        }
    }

    /// Convert crossterm KeyEvent to our Action
    fn convert_key_event(key_event: KeyEvent) -> Action {
        match key_event.code {
            // Navigation
            KeyCode::Up => Action::Up,
            KeyCode::Down => Action::Down,
            KeyCode::Left => Action::Left,
            KeyCode::Right => Action::Right,
            KeyCode::PageUp => Action::PageUp,
            KeyCode::PageDown => Action::PageDown,
            KeyCode::Home => Action::Home,
            KeyCode::End => Action::End,

            // Selection
            KeyCode::Enter => Action::Select,
            KeyCode::Esc => Action::Cancel,

            // Tab navigation
            KeyCode::Tab => {
                if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    Action::BackTab
                } else {
                    Action::Tab
                }
            }

            // Card review - number keys for ratings
            KeyCode::Char('1') => Action::RateAgain,
            KeyCode::Char('2') => Action::RateHard,
            KeyCode::Char('3') => Action::RateGood,
            KeyCode::Char('4') => Action::RateEasy,

            // Card review - letter shortcuts
            KeyCode::Char('a') | KeyCode::Char('A') => Action::RateAgain,
            KeyCode::Char('h') | KeyCode::Char('H') => Action::RateHard,
            KeyCode::Char('g') | KeyCode::Char('G') => Action::RateGood,
            KeyCode::Char('x') | KeyCode::Char('X') => Action::RateEasy, // Changed from 'e' to 'x'
            KeyCode::Char(' ') => Action::ShowAnswer,
            KeyCode::Char('s') | KeyCode::Char('S') => Action::SkipCard,
            KeyCode::Char('r') | KeyCode::Char('R') => Action::ResetCard,

            // Deck management - letter shortcuts
            KeyCode::Char('c') | KeyCode::Char('C') => Action::Create,
            KeyCode::Char('d') | KeyCode::Char('D') => Action::Delete,
            KeyCode::Char('i') | KeyCode::Char('I') => Action::Info,
            KeyCode::Char('w') | KeyCode::Char('W') => Action::SwitchDeck, // Switch deck

            // Application
            KeyCode::Char('q') | KeyCode::Char('Q') => Action::Quit,
            KeyCode::F(1) => Action::Help,
            KeyCode::F(5) => Action::Refresh,

            // Settings editing
            KeyCode::Char('e') | KeyCode::Char('E') => Action::StartEdit,
            KeyCode::F(2) => Action::Save,
            KeyCode::F(3) => Action::Discard,

            // Text editing
            KeyCode::Backspace => Action::Backspace,
            KeyCode::Delete => Action::Delete,
            KeyCode::Char(c) => Action::Char(c),

            _ => Action::Unknown,
        }
    }
}

/// Get help text for keyboard shortcuts
pub fn get_help_text() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Navigation", ""),
        ("↑/↓", "Move up/down"),
        ("←/→", "Move left/right"),
        ("Enter/Space", "Select"),
        ("Esc", "Cancel/Back"),
        ("Tab", "Next field"),
        ("Shift+Tab", "Previous field"),
        ("", ""),
        ("Card Review", ""),
        ("Space", "Show answer"),
        ("1/A", "Rate: Again"),
        ("2/H", "Rate: Hard"),
        ("3/G", "Rate: Good"),
        ("4/E", "Rate: Easy"),
        ("S", "Skip card"),
        ("R", "Reset card"),
        ("", ""),
        ("Deck Management", ""),
        ("C", "Create deck"),
        ("D", "Delete deck"),
        ("I", "Deck info"),
        ("W", "Switch deck"),
        ("", ""),
        ("Application", ""),
        ("Q", "Quit"),
        ("F1", "Show help"),
        ("F5", "Refresh"),
    ]
}

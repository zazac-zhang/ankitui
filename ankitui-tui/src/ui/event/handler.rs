//! Event Handler - Context-aware event processing
//!
//! Implements state-dependent event handling where the same event can have
//! different meanings based on current application state.

use crate::ui::state::{AppState, Screen};
use crate::ui::event::{Event, Command, CommandType};
use crate::domain::CardRating;
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent, MouseEvent as CrosstermMouseEvent, MouseEventKind};
use uuid;

/// Event Handler - Context-aware event processing
pub struct EventHandler {
    current_state: AppState,
}

impl EventHandler {
    pub fn new(initial_state: AppState) -> Self {
        Self {
            current_state: initial_state,
        }
    }

    /// Update the current state
    pub fn update_state(&mut self, new_state: AppState) {
        self.current_state = new_state;
    }

    /// Get current state
    pub fn current_state(&self) -> &AppState {
        &self.current_state
    }

    /// Handle event with context awareness
    pub fn handle_event(&self, event: Event) -> Command {
        match event {
            Event::Key(key_event) => self.handle_key_event_contextual(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event_contextual(mouse_event),
            Event::Resize(width, height) => Command::system(CommandType::Resize(width, height)),
            Event::FocusGained => Command::system(CommandType::FocusGained),
            Event::FocusLost => Command::system(CommandType::FocusLost),
            Event::Paste(content) => self.handle_paste_contextual(content),
        }
    }

    /// Context-aware keyboard event handling
    fn handle_key_event_contextual(&self, event: KeyEvent) -> Command {
        let screen = self.current_state.current_screen();

        match (event.code, event.modifiers) {
            // Navigation keys - context dependent
            (KeyCode::Up, KeyModifiers::NONE) => self.handle_navigation_up(screen),
            (KeyCode::Down, KeyModifiers::NONE) => self.handle_navigation_down(screen),
            (KeyCode::Left, KeyModifiers::NONE) => self.handle_navigation_left(screen),
            (KeyCode::Right, KeyModifiers::NONE) => self.handle_navigation_right(screen),

            // Selection keys - context dependent
            (KeyCode::Enter, KeyModifiers::NONE) => self.handle_select_contextual(screen),
            (KeyCode::Char(' '), KeyModifiers::NONE) => self.handle_space_contextual(screen),

            // Study session keys - only active in study mode
            (KeyCode::Char('1'), KeyModifiers::NONE) if screen == Screen::StudySession =>
                Command::user(CommandType::RateCurrentCard(CardRating::Again)),
            (KeyCode::Char('2'), KeyModifiers::NONE) if screen == Screen::StudySession =>
                Command::user(CommandType::RateCurrentCard(CardRating::Hard)),
            (KeyCode::Char('3'), KeyModifiers::NONE) if screen == Screen::StudySession =>
                Command::user(CommandType::RateCurrentCard(CardRating::Good)),
            (KeyCode::Char('4'), KeyModifiers::NONE) if screen == Screen::StudySession =>
                Command::user(CommandType::RateCurrentCard(CardRating::Easy)),
            (KeyCode::Char(' '), KeyModifiers::NONE) if screen == Screen::StudySession =>
                Command::user(CommandType::ShowAnswer),

            // Escape key - context dependent
            (KeyCode::Esc, KeyModifiers::NONE) => self.handle_escape_contextual(screen),

            // Quit keys - global
            (KeyCode::Char('q'), KeyModifiers::CONTROL) |
            (KeyCode::Char('c'), KeyModifiers::CONTROL) =>
                Command::user(CommandType::Quit),

            // Help key - global
            (KeyCode::F(1), KeyModifiers::NONE) |
            (KeyCode::Char('?'), KeyModifiers::NONE) =>
                Command::user(CommandType::ShowHelp),

            // Refresh keys - context dependent
            (KeyCode::F(5), KeyModifiers::NONE) => self.handle_refresh_contextual(screen),

            // Search key - context dependent
            (KeyCode::Char('/'), KeyModifiers::NONE) => self.handle_search_contextual(screen),

            // Create key - context dependent
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => self.handle_create_contextual(screen),

            // Delete key - context dependent
            (KeyCode::Delete, KeyModifiers::NONE) |
            (KeyCode::Backspace, KeyModifiers::CONTROL) => self.handle_delete_contextual(screen),

            _ => Command::user(CommandType::Unknown),
        }
    }

    /// Context-aware mouse event handling
    fn handle_mouse_event_contextual(&self, event: CrosstermMouseEvent) -> Command {
        let screen = self.current_state.current_screen();

        match event.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) =>
                self.handle_left_click_contextual(event.column, event.row, screen),
            MouseEventKind::Down(crossterm::event::MouseButton::Right) =>
                self.handle_right_click_contextual(event.column, event.row, screen),
            MouseEventKind::ScrollUp =>
                self.handle_scroll_up_contextual(event.column, event.row, screen),
            MouseEventKind::ScrollDown =>
                self.handle_scroll_down_contextual(event.column, event.row, screen),
            MouseEventKind::Moved =>
                Command::user(CommandType::MouseMove(event.column, event.row)),
            _ => Command::user(CommandType::Unknown),
        }
    }

    // Context-specific navigation handlers
    fn handle_navigation_up(&self, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::SelectPreviousDeck),
            Screen::MainMenu => Command::user(CommandType::NavigateUp),
            Screen::StudySession => Command::user(CommandType::NavigateUp),
            Screen::Statistics => Command::user(CommandType::NavigateUp),
            _ => Command::user(CommandType::NavigateUp),
        }
    }

    fn handle_navigation_down(&self, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::SelectNextDeck),
            Screen::MainMenu => Command::user(CommandType::NavigateDown),
            Screen::StudySession => Command::user(CommandType::NavigateDown),
            Screen::Statistics => Command::user(CommandType::NavigateDown),
            _ => Command::user(CommandType::NavigateDown),
        }
    }

    fn handle_navigation_left(&self, _screen: Screen) -> Command {
        Command::user(CommandType::NavigateLeft)
    }

    fn handle_navigation_right(&self, _screen: Screen) -> Command {
        Command::user(CommandType::NavigateRight)
    }

    // Context-specific selection handlers
    fn handle_select_contextual(&self, screen: Screen) -> Command {
        match screen {
            Screen::MainMenu => Command::user(CommandType::Confirm),
            Screen::DeckSelection => Command::user(CommandType::StartStudySessionDefault),
            Screen::StudySession => {
                if self.current_state.is_showing_answer() {
                    Command::user(CommandType::RateCurrentCard(CardRating::Good))
                } else {
                    Command::user(CommandType::ShowAnswer)
                }
            }
            Screen::CardEditor => Command::user(CommandType::SaveCard),
            Screen::Settings => Command::user(CommandType::ConfirmSetting),
            _ => Command::user(CommandType::Select),
        }
    }

    fn handle_space_contextual(&self, screen: Screen) -> Command {
        match screen {
            Screen::StudySession => {
                if self.current_state.is_showing_answer() {
                    Command::user(CommandType::RateCurrentCard(CardRating::Good))
                } else {
                    Command::user(CommandType::ShowAnswer)
                }
            }
            Screen::DeckSelection => Command::user(CommandType::SelectDeck(uuid::Uuid::nil())),
            Screen::CardEditor => Command::user(CommandType::ToggleCardSide),
            _ => Command::user(CommandType::Select),
        }
    }

    // Context-specific escape handlers
    fn handle_escape_contextual(&self, screen: Screen) -> Command {
        match screen {
            Screen::StudySession => {
                if self.current_state.is_study_session_active() {
                    Command::user(CommandType::EndStudySession)
                } else {
                    Command::user(CommandType::NavigateToMainMenu)
                }
            }
            Screen::CardEditor => Command::user(CommandType::CancelEdit),
            Screen::Settings => Command::user(CommandType::NavigateToMainMenu),
            Screen::Statistics => Command::user(CommandType::NavigateToMainMenu),
            _ => Command::user(CommandType::NavigateBack),
        }
    }

    // Context-specific refresh handlers
    fn handle_refresh_contextual(&self, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::LoadDecks),
            Screen::Statistics => Command::user(CommandType::RefreshStatistics),
            Screen::StudySession => Command::user(CommandType::RefreshSession),
            _ => Command::user(CommandType::RefreshScreen),
        }
    }

    // Context-specific search handlers
    fn handle_search_contextual(&self, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::SearchDecks(String::new())),
            Screen::StudySession => Command::user(CommandType::SearchCards(String::new())),
            _ => Command::user(CommandType::StartSearch),
        }
    }

    // Context-specific create handlers
    fn handle_create_contextual(&self, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::CreateDeckPrompt),
            Screen::StudySession => Command::user(CommandType::CreateCardPrompt),
            _ => Command::user(CommandType::CreateDeckPrompt),
        }
    }

    // Context-specific delete handlers
    fn handle_delete_contextual(&self, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::DeleteDeckPrompt),
            Screen::CardEditor => Command::user(CommandType::DeleteCard(uuid::Uuid::nil())),
            _ => Command::user(CommandType::DeleteCard(uuid::Uuid::nil())),
        }
    }

    // Mouse event handlers
    fn handle_left_click_contextual(&self, x: u16, y: u16, screen: Screen) -> Command {
        match screen {
            Screen::StudySession => {
                if self.current_state.is_showing_answer() {
                    // Check if click is on a rating button
                    if y >= 10 && y <= 14 { // Rating button area
                        let rating = match x {
                            10..=15 => CardRating::Again,
                            17..=22 => CardRating::Hard,
                            24..=29 => CardRating::Good,
                            31..=36 => CardRating::Easy,
                            _ => return Command::user(CommandType::ShowAnswer),
                        };
                        Command::user(CommandType::RateCurrentCard(rating))
                    } else {
                        Command::user(CommandType::ShowAnswer)
                    }
                } else {
                    Command::user(CommandType::ShowAnswer)
                }
            }
            _ => Command::user(CommandType::Click(x, y)),
        }
    }

    fn handle_right_click_contextual(&self, x: u16, y: u16, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::ShowDeckContextMenu(x, y)),
            Screen::StudySession => Command::user(CommandType::ShowCardContextMenu(x, y)),
            _ => Command::user(CommandType::RightClick(x, y)),
        }
    }

    fn handle_scroll_up_contextual(&self, _x: u16, _y: u16, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::SelectPreviousDeck),
            Screen::CardEditor => Command::user(CommandType::ScrollUp),
            Screen::Statistics => Command::user(CommandType::ScrollStatsUp),
            _ => Command::user(CommandType::ScrollUp),
        }
    }

    fn handle_scroll_down_contextual(&self, _x: u16, _y: u16, screen: Screen) -> Command {
        match screen {
            Screen::DeckSelection => Command::user(CommandType::SelectNextDeck),
            Screen::CardEditor => Command::user(CommandType::ScrollDown),
            Screen::Statistics => Command::user(CommandType::ScrollStatsDown),
            _ => Command::user(CommandType::ScrollDown),
        }
    }

    fn handle_paste_contextual(&self, content: String) -> Command {
        let screen = self.current_state.current_screen();

        match screen {
            Screen::CardEditor => Command::user(CommandType::PasteCardContent(content)),
            Screen::DeckSelection if content.contains('\n') => {
                // Try to import cards from pasted content
                Command::user(CommandType::ImportCards(content))
            }
            _ => Command::system(CommandType::Paste(content)),
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new(AppState::default())
    }
}
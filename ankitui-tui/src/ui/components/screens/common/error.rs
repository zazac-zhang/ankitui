//! Error display screen component

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Error screen for displaying error messages
pub struct ErrorScreen {
    state: ComponentState,
    title: String,
    message: String,
    can_dismiss: bool,
}

impl ErrorScreen {
    pub fn new(title: String, message: String) -> Self {
        Self {
            state: ComponentState::new(),
            title,
            message,
            can_dismiss: true,
        }
    }

    pub fn with_dismiss(mut self, can_dismiss: bool) -> Self {
        self.can_dismiss = can_dismiss;
        self
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn can_dismiss(&self) -> bool {
        self.can_dismiss
    }
}

impl Component for ErrorScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let help_text = if self.can_dismiss {
            "\n\nPress Enter or Escape to continue"
        } else {
            ""
        };

        let text = format!("{}{}\n\nPress Enter to acknowledge", self.message, help_text);

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(self.title.as_str()))
            .style(Style::default().fg(Color::Red));

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        if !self.can_dismiss {
            return Ok(false);
        }

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        // Return true to indicate screen should be dismissed
                        Ok(true)
                    }
                    _ => Ok(false),
                }
            }
            _ => Ok(false),
        }
    }

    fn update(&mut self) -> TuiResult<()> {
        Ok(())
    }

    fn can_focus(&self) -> bool {
        self.can_dismiss
    }

    fn id(&self) -> &str {
        "error_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

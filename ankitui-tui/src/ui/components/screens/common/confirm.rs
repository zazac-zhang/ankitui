//! Confirmation dialog screen component

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame, widgets::{Paragraph, Block, Borders}, style::{Style, Color}};

/// Confirmation dialog screen
pub struct ConfirmScreen {
    state: ComponentState,
    title: String,
    message: String,
    confirmed: bool,
    selection: bool, // true for Yes, false for No
}

impl ConfirmScreen {
    pub fn new(title: String, message: String) -> Self {
        Self {
            state: ComponentState::new(),
            title,
            message,
            confirmed: false,
            selection: false, // default to No
        }
    }

    pub fn is_confirmed(&self) -> bool {
        self.confirmed
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn set_selection(&mut self, selection: bool) {
        if self.selection != selection {
            self.selection = selection;
            self.mark_dirty();
        }
    }
}

impl Component for ConfirmScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let yes_style = if self.selection {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };

        let no_style = if !self.selection {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        };

        let text = format!(
            "{}\n\n[ {} ]  Yes\n[ {} ]  No\n\nEnter: Confirm, Tab: Switch",
            self.message,
            if self.selection { "✓" } else { " " },
            if !self.selection { "✓" } else { " " }
        );

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(self.title.as_str()));

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Enter => {
                        self.confirmed = self.selection;
                        Ok(true) // Return true to indicate dialog completed
                    }
                    KeyCode::Tab | KeyCode::Left | KeyCode::Right => {
                        self.set_selection(!self.selection);
                        Ok(false)
                    }
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        self.set_selection(true);
                        Ok(false)
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.set_selection(false);
                        Ok(false)
                    }
                    KeyCode::Esc => {
                        self.confirmed = false;
                        Ok(true) // Cancel
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
        true
    }

    fn id(&self) -> &str {
        "confirm_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}
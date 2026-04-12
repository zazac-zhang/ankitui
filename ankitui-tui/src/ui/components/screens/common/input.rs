//! Text input dialog screen component

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Text input dialog screen
pub struct InputScreen {
    state: ComponentState,
    title: String,
    prompt: String,
    input: String,
    cursor_position: usize,
    completed: bool,
    cancelled: bool,
}

impl InputScreen {
    pub fn new(title: String, prompt: String) -> Self {
        Self {
            state: ComponentState::new(),
            title,
            prompt,
            input: String::new(),
            cursor_position: 0,
            completed: false,
            cancelled: false,
        }
    }

    pub fn with_default(mut self, default: String) -> Self {
        self.input = default.clone();
        self.cursor_position = default.len();
        self
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.mark_dirty();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
            self.mark_dirty();
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.input.insert(self.cursor_position, ch);
        self.cursor_position += 1;
        self.mark_dirty();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position < self.input.len() {
            self.input.remove(self.cursor_position);
            self.mark_dirty();
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
            self.mark_dirty();
        }
    }
}

impl Component for InputScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let cursor_indicator = if self.cursor_position < self.input.len() {
            format!(
                "{}_{}",
                &self.input[..self.cursor_position],
                &self.input[self.cursor_position..]
            )
        } else {
            format!("{}_", self.input)
        };

        let text = format!(
            "{}\n\n> {}\n\nEnter: Confirm, Esc: Cancel",
            self.prompt, cursor_indicator
        );

        let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(self.title.as_str()));

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Enter => {
                        self.completed = true;
                        Ok(true) // Return true to indicate dialog completed
                    }
                    KeyCode::Esc => {
                        self.cancelled = true;
                        Ok(true) // Return true to indicate dialog cancelled
                    }
                    KeyCode::Left => {
                        self.move_cursor_left();
                        Ok(false)
                    }
                    KeyCode::Right => {
                        self.move_cursor_right();
                        Ok(false)
                    }
                    KeyCode::Backspace => {
                        self.backspace();
                        Ok(false)
                    }
                    KeyCode::Delete => {
                        self.delete_char();
                        Ok(false)
                    }
                    KeyCode::Home => {
                        self.cursor_position = 0;
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::End => {
                        self.cursor_position = self.input.len();
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Char(ch) => {
                        self.insert_char(ch);
                        Ok(false)
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
        "input_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

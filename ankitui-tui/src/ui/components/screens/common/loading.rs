//! Loading screen component

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame, widgets::{Paragraph, Block, Borders}, style::{Style, Color}};

/// Loading screen for async operations
pub struct LoadingScreen {
    state: ComponentState,
    message: String,
    loading_dots: u8,
}

impl LoadingScreen {
    pub fn new(message: String) -> Self {
        Self {
            state: ComponentState::new(),
            message,
            loading_dots: 0,
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
        self.mark_dirty();
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

impl Component for LoadingScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let dots = ".".repeat((self.loading_dots % 4) as usize);
        let text = format!("{}{}", self.message, dots);

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Loading"))
            .style(Style::default().fg(Color::Yellow));

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> {
        // Loading screen doesn't handle input
        Ok(false)
    }

    fn update(&mut self) -> TuiResult<()> {
        // Animate loading dots
        self.loading_dots = self.loading_dots.wrapping_add(1);
        self.mark_dirty();
        Ok(())
    }

    fn can_focus(&self) -> bool {
        false
    }

    fn id(&self) -> &str {
        "loading_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}
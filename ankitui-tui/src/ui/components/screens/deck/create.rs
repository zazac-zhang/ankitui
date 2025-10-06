//! Deck creation screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame, widgets::{Paragraph, Block, Borders}, style::Style};

/// Deck creation screen
pub struct DeckCreateScreen {
    state: ComponentState,
}

impl DeckCreateScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for DeckCreateScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Create New Deck\n\nName: _________________\nDescription: _________________\n\n\nEnter: Create | Esc: Cancel";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Create Deck"))
            .style(Style::default());

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> {
        Ok(false)
    }

    fn update(&mut self) -> TuiResult<()> {
        Ok(())
    }

    fn can_focus(&self) -> bool {
        true
    }

    fn id(&self) -> &str {
        "deck_create_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}
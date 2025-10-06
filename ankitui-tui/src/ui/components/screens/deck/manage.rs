//! Deck management screen (placeholder)

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame, widgets::{Paragraph, Block, Borders}, style::Style};

pub struct DeckManageScreen {
    state: ComponentState,
}

impl DeckManageScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for DeckManageScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Manage Deck Cards\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Manage Cards"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "deck_manage_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}
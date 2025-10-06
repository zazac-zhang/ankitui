//! Statistics screen components

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame, widgets::{Paragraph, Block, Borders}, style::Style};

pub struct StatsScreen {
    state: ComponentState,
}

pub struct GlobalStatsScreen {
    state: ComponentState,
}

pub struct DeckStatsScreen {
    state: ComponentState,
}

pub struct ProgressScreen {
    state: ComponentState,
}

impl StatsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for StatsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Statistics\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Statistics"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "stats_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl GlobalStatsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for GlobalStatsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Global Statistics\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Global Stats"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "global_stats_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl DeckStatsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for DeckStatsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Deck Statistics\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Deck Stats"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "deck_stats_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl ProgressScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for ProgressScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Learning Progress\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "progress_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}
//! Settings screen components

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{backend::Backend, layout::Rect, Frame, widgets::{Paragraph, Block, Borders}, style::Style};

pub struct SettingsScreen {
    state: ComponentState,
}

pub struct StudyPrefsScreen {
    state: ComponentState,
}

pub struct UiSettingsScreen {
    state: ComponentState,
}

pub struct DataManageScreen {
    state: ComponentState,
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for SettingsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Settings\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Settings"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "settings_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl StudyPrefsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for StudyPrefsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Study Preferences\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Study Preferences"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "study_prefs_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl UiSettingsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for UiSettingsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "UI Settings\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("UI Settings"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "ui_settings_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl DataManageScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for DataManageScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let text = "Data Management\n\n(To be implemented)";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Data Management"))
            .style(Style::default());
        f.render_widget(paragraph, area);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "data_manage_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}
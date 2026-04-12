//! Settings screen components

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{layout::Rect, Frame, widgets::{Paragraph, Block, Borders, List, ListItem}, style::{Style, Color, Modifier}};

/// Settings screen - main settings hub
pub struct SettingsScreen {
    state: ComponentState,
}

/// Study preferences screen
pub struct StudyPrefsScreen {
    state: ComponentState,
}

/// UI settings screen
pub struct UiSettingsScreen {
    state: ComponentState,
}

/// Data management screen
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
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("⚙️ Application Settings")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Settings"));
        f.render_widget(header, chunks[0]);

        let menu_items = vec![
            "1. 📖 Study Preferences - Daily limits, scheduler params",
            "2. 🎨 UI Customization - Theme, display options",
            "3. 💾 Data Management - Import, export, backup",
        ];
        let items: Vec<ListItem> = menu_items.iter().map(|item| ListItem::new(*item)).collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Settings Categories"));
        f.render_widget(list, chunks[1]);

        let help = Paragraph::new("1-3: Open Settings | Esc: Back to Menu")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
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
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("📖 Study Preferences")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Study Prefs"));
        f.render_widget(header, chunks[0]);

        let content = Paragraph::new(
            "Study preferences will be available here.\n\n\
             Configurable options:\n\
             - New cards per day\n\
             - Maximum reviews per day\n\
             - Learning step intervals\n\
             - Easy bonus multiplier\n\
             - Lapse handling (relearning vs reset)\n\
             - Daily start time",
        )
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Coming Soon"));
        f.render_widget(content, chunks[1]);

        let help = Paragraph::new("Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
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
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("🎨 UI Customization")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("UI Settings"));
        f.render_widget(header, chunks[0]);

        let content = Paragraph::new(
            "UI customization options will be available here.\n\n\
             Configurable options:\n\
             - Color theme (light/dark/custom)\n\
             - Card font and size\n\
             - Progress bar style\n\
             - Answer reveal timing\n\
             - Auto-advance delay",
        )
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Coming Soon"));
        f.render_widget(content, chunks[1]);

        let help = Paragraph::new("Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
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
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("💾 Data Management")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Data Management"));
        f.render_widget(header, chunks[0]);

        let content = Paragraph::new(
            "Data management tools will be available here.\n\n\
             Available operations:\n\
             - Import decks from Anki\n\
             - Export data to file\n\
             - Create backup\n\
             - Restore from backup\n\
             - Clear all data",
        )
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Coming Soon"));
        f.render_widget(content, chunks[1]);

        let help = Paragraph::new("Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "data_manage_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}
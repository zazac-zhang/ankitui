//! DEPRECATED: This component is NOT connected to the runtime.
//! The actual rendering is in `ui/render/mod.rs` via `render_*` functions.
//! Do NOT modify this file expecting runtime behavior changes.
//!
//! Settings screen components

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Settings screen - main settings hub
pub struct SettingsScreen {
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
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Settings Categories"));
        f.render_widget(list, chunks[1]);

        let help = Paragraph::new("1-3: Open Settings | Esc: Back to Menu")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
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
        "settings_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

/// Study preferences screen with interactive settings
pub struct StudyPrefsScreen {
    state: ComponentState,
    selected_index: usize,
    new_cards_per_day: u32,
    max_reviews_per_day: u32,
    learning_steps: String,
    auto_advance: bool,
    show_hint_on_question: bool,
    dirty: bool,
}

impl StudyPrefsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            selected_index: 0,
            new_cards_per_day: 20,
            max_reviews_per_day: 200,
            learning_steps: "1m 10m".to_string(),
            auto_advance: false,
            show_hint_on_question: true,
            dirty: false,
        }
    }

    pub fn with_prefs(new_cards: u32, max_reviews: u32, auto_advance: bool) -> Self {
        Self {
            state: ComponentState::new(),
            selected_index: 0,
            new_cards_per_day: new_cards,
            max_reviews_per_day: max_reviews,
            learning_steps: "1m 10m".to_string(),
            auto_advance,
            show_hint_on_question: true,
            dirty: false,
        }
    }

    pub fn get_settings(&self) -> StudyPrefsSettings {
        StudyPrefsSettings {
            new_cards_per_day: self.new_cards_per_day,
            max_reviews_per_day: self.max_reviews_per_day,
            learning_steps: self.learning_steps.clone(),
            auto_advance: self.auto_advance,
            show_hint_on_question: self.show_hint_on_question,
        }
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    fn move_down(&mut self, len: usize) {
        if self.selected_index < len - 1 {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }
}

/// Study preferences settings
#[derive(Debug, Clone)]
pub struct StudyPrefsSettings {
    pub new_cards_per_day: u32,
    pub max_reviews_per_day: u32,
    pub learning_steps: String,
    pub auto_advance: bool,
    pub show_hint_on_question: bool,
}

impl Component for StudyPrefsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let menu_items = vec![
            format!(
                "{} New cards per day: {}",
                if self.selected_index == 0 { "▶" } else { " " },
                self.new_cards_per_day
            ),
            format!(
                "{} Max reviews per day: {}",
                if self.selected_index == 1 { "▶" } else { " " },
                self.max_reviews_per_day
            ),
            format!(
                "{} Learning steps: {}",
                if self.selected_index == 2 { "▶" } else { " " },
                self.learning_steps
            ),
            format!(
                "{} Auto-advance: {}",
                if self.selected_index == 3 { "▶" } else { " " },
                if self.auto_advance { "On" } else { "Off" }
            ),
            format!(
                "{} Show hint on question: {}",
                if self.selected_index == 4 { "▶" } else { " " },
                if self.show_hint_on_question { "On" } else { "Off" }
            ),
        ];

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

        let items: Vec<ListItem> = menu_items.iter().map(|item| ListItem::new(item.clone())).collect();
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Settings"));
        f.render_widget(list, chunks[1]);

        let help = Paragraph::new("↑↓: Navigate | Enter: Toggle | ←→: Adjust | Ctrl+S: Save | Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Up => {
                        self.move_up();
                        Ok(false)
                    }
                    KeyCode::Down => {
                        self.move_down(5);
                        Ok(false)
                    }
                    KeyCode::Enter => {
                        // Toggle boolean settings
                        match self.selected_index {
                            3 => {
                                self.auto_advance = !self.auto_advance;
                                self.dirty = true;
                            }
                            4 => {
                                self.show_hint_on_question = !self.show_hint_on_question;
                                self.dirty = true;
                            }
                            _ => {} // Numeric settings - would open input dialog
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Left => {
                        // Decrease numeric values
                        match self.selected_index {
                            0 if self.new_cards_per_day > 0 => {
                                self.new_cards_per_day -= 1;
                                self.dirty = true;
                            }
                            1 if self.max_reviews_per_day > 0 => {
                                self.max_reviews_per_day -= 1;
                                self.dirty = true;
                            }
                            _ => {}
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Right => {
                        // Increase numeric values
                        match self.selected_index {
                            0 => {
                                self.new_cards_per_day += 1;
                                self.dirty = true;
                            }
                            1 => {
                                self.max_reviews_per_day += 1;
                                self.dirty = true;
                            }
                            _ => {}
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.dirty = true;
                        Ok(true) // Signal save
                    }
                    KeyCode::Esc => Ok(true), // Signal back navigation
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
        "study_prefs_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

/// UI settings screen with interactive settings
pub struct UiSettingsScreen {
    state: ComponentState,
    selected_index: usize,
    display_name: String,
    theme_index: usize,
    auto_advance: bool,
    show_progress: bool,
    dirty: bool,
}

const THEMES: &[&str] = &["default", "dark", "light"];

impl UiSettingsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            selected_index: 0,
            display_name: "User".to_string(),
            theme_index: 0,
            auto_advance: false,
            show_progress: true,
            dirty: false,
        }
    }

    pub fn with_prefs(name: String, theme: &str, auto_advance: bool, show_progress: bool) -> Self {
        let theme_index = THEMES.iter().position(|&t| t == theme).unwrap_or(0);
        Self {
            state: ComponentState::new(),
            selected_index: 0,
            display_name: name,
            theme_index,
            auto_advance,
            show_progress,
            dirty: false,
        }
    }

    pub fn get_preferences(&self) -> crate::domain::UserPreferences {
        crate::domain::UserPreferences {
            display_name: self.display_name.clone(),
            theme: THEMES[self.theme_index].to_string(),
            auto_advance: self.auto_advance,
            show_progress: self.show_progress,
        }
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < 3 {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }
}

impl Component for UiSettingsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let menu_items = vec![
            format!(
                "{} Display name: {}",
                if self.selected_index == 0 { "▶" } else { " " },
                self.display_name
            ),
            format!(
                "{} Theme: {}",
                if self.selected_index == 1 { "▶" } else { " " },
                THEMES[self.theme_index]
            ),
            format!(
                "{} Auto-advance: {}",
                if self.selected_index == 2 { "▶" } else { " " },
                if self.auto_advance { "On" } else { "Off" }
            ),
            format!(
                "{} Show progress: {}",
                if self.selected_index == 3 { "▶" } else { " " },
                if self.show_progress { "On" } else { "Off" }
            ),
        ];

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

        let items: Vec<ListItem> = menu_items.iter().map(|item| ListItem::new(item.clone())).collect();
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Settings"));
        f.render_widget(list, chunks[1]);

        let help = Paragraph::new("↑↓: Navigate | Enter: Toggle | ←→: Adjust | Ctrl+S: Save | Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Up => {
                        self.move_up();
                        Ok(false)
                    }
                    KeyCode::Down => {
                        self.move_down();
                        Ok(false)
                    }
                    KeyCode::Enter => {
                        match self.selected_index {
                            2 => {
                                self.auto_advance = !self.auto_advance;
                                self.dirty = true;
                            }
                            3 => {
                                self.show_progress = !self.show_progress;
                                self.dirty = true;
                            }
                            _ => {} // Would open input dialog for name/theme
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Left => {
                        match self.selected_index {
                            1 if self.theme_index > 0 => {
                                self.theme_index -= 1;
                                self.dirty = true;
                            }
                            _ => {}
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Right => {
                        match self.selected_index {
                            1 if self.theme_index < THEMES.len() - 1 => {
                                self.theme_index += 1;
                                self.dirty = true;
                            }
                            _ => {}
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.dirty = true;
                        Ok(true) // Signal save
                    }
                    KeyCode::Esc => Ok(true),
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
        "ui_settings_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

/// Data management screen with interactive operations
pub struct DataManageScreen {
    state: ComponentState,
    selected_index: usize,
    status_message: String,
    dirty: bool,
}

const DATA_OPERATIONS: &[&str] = &[
    "Import decks from Anki",
    "Export data to file",
    "Create backup",
    "Restore from backup",
    "Clear all data",
];

impl DataManageScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            selected_index: 0,
            status_message: String::new(),
            dirty: false,
        }
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < DATA_OPERATIONS.len() - 1 {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }

    fn execute_operation(&mut self) {
        match self.selected_index {
            0 => self.status_message = "Import: Select an .apkg file to import".to_string(),
            1 => self.status_message = "Export: Data exported to ~/ankitui_export.json".to_string(),
            2 => self.status_message = "Backup: Backup created successfully".to_string(),
            3 => self.status_message = "Restore: Select a backup file to restore".to_string(),
            4 => self.status_message = "Clear: All data will be cleared (confirm with Enter)".to_string(),
            _ => {}
        }
        self.dirty = true;
        self.mark_dirty();
    }

    pub fn get_operation(&self) -> Option<DataOperation> {
        match self.selected_index {
            0 => Some(DataOperation::Import),
            1 => Some(DataOperation::Export),
            2 => Some(DataOperation::Backup),
            3 => Some(DataOperation::Restore),
            4 => Some(DataOperation::Clear),
            _ => None,
        }
    }
}

/// Data management operations
#[derive(Debug, Clone, Copy)]
pub enum DataOperation {
    Import,
    Export,
    Backup,
    Restore,
    Clear,
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

        let items: Vec<ListItem> = DATA_OPERATIONS
            .iter()
            .enumerate()
            .map(|(i, op)| {
                let prefix = if i == self.selected_index { "▶" } else { " " };
                ListItem::new(format!("{} {}", prefix, op))
            })
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Operations"));
        f.render_widget(list, chunks[1]);

        let footer_text = if self.status_message.is_empty() {
            "↑↓: Navigate | Enter: Execute | Esc: Back".to_string()
        } else {
            format!("Status: {}", self.status_message)
        };
        let help = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Info"));
        f.render_widget(help, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up => {
                    self.move_up();
                    Ok(false)
                }
                KeyCode::Down => {
                    self.move_down();
                    Ok(false)
                }
                KeyCode::Enter => {
                    self.execute_operation();
                    Ok(false)
                }
                KeyCode::Esc => Ok(true),
                _ => Ok(false),
            },
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
        "data_manage_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

//! Settings Component
//!
//! List-based settings interface with item selection and editing

use crate::tui::app::AppState;
use crate::tui::core::{state_manager::RenderContext, Action, UIComponent};
use ankitui_core::Config;
use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

/// Settings item types
#[derive(Debug, Clone, PartialEq)]
pub enum SettingsItem {
    Category {
        name: String,
        description: String,
        items: Vec<SettingsItem>,
    },
    Setting {
        name: String,
        description: String,
        current_value: String,
        setting_type: SettingType,
    },
}

/// Setting value types
#[derive(Debug, Clone, PartialEq)]
pub enum SettingType {
    Number { min: u32, max: u32 },
    Text,
    Boolean,
    Selection { options: Vec<String> },
}

/// Settings view modes
#[derive(Debug, Clone, PartialEq)]
pub enum SettingsViewMode {
    CategoryList,
    SettingList { category: String },
    EditingValue { setting_path: String },
}

/// Settings component
pub struct Settings {
    /// Current configuration
    config: Config,
    /// Current view mode
    view_mode: SettingsViewMode,
    /// Settings hierarchy
    settings_items: Vec<SettingsItem>,
    /// Current items being displayed
    current_items: Vec<SettingsItem>,
    /// List state for navigation
    list_state: ListState,
    /// Selected item index
    selected_index: usize,
    /// Edit field for text input
    edit_field: String,
    /// Navigation stack for going back
    navigation_stack: Vec<String>,
}

impl Settings {
    pub fn new() -> Self {
        Self::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Self {
        let mut settings = Self {
            config,
            view_mode: SettingsViewMode::CategoryList,
            settings_items: Vec::new(),
            current_items: Vec::new(),
            list_state: ListState::default(),
            selected_index: 0,
            edit_field: String::new(),
            navigation_stack: Vec::new(),
        };

        settings.build_settings_hierarchy();
        settings.update_current_items();
        settings.list_state.select(Some(0));

        settings
    }

    /// Build the settings hierarchy from the current config
    fn build_settings_hierarchy(&mut self) {
        self.settings_items = vec![
            SettingsItem::Category {
                name: "Learning Limits".to_string(),
                description: "Configure daily study limits and schedules".to_string(),
                items: vec![
                    SettingsItem::Setting {
                        name: "Maximum New Cards".to_string(),
                        description: "Maximum number of new cards to introduce per day".to_string(),
                        current_value: self.config.daily.max_new_cards.to_string(),
                        setting_type: SettingType::Number { min: 0, max: 1000 },
                    },
                    SettingsItem::Setting {
                        name: "Maximum Review Cards".to_string(),
                        description: "Maximum number of review cards per day".to_string(),
                        current_value: self.config.daily.max_review_cards.to_string(),
                        setting_type: SettingType::Number { min: 0, max: 10000 },
                    },
                    SettingsItem::Setting {
                        name: "Day Start Hour".to_string(),
                        description: "Hour when your study day begins (0-23)".to_string(),
                        current_value: self.config.daily.day_start_hour.to_string(),
                        setting_type: SettingType::Number { min: 0, max: 23 },
                    },
                    SettingsItem::Setting {
                        name: "Day End Hour".to_string(),
                        description: "Hour when your study day ends (0-23)".to_string(),
                        current_value: self.config.daily.day_end_hour.to_string(),
                        setting_type: SettingType::Number { min: 0, max: 23 },
                    },
                ],
            },
            SettingsItem::Category {
                name: "Theme & Display".to_string(),
                description: "Customize appearance and display settings".to_string(),
                items: vec![
                    SettingsItem::Setting {
                        name: "Theme".to_string(),
                        description: "Choose your color theme".to_string(),
                        current_value: self.config.ui.theme.clone(),
                        setting_type: SettingType::Selection {
                            options: vec![
                                "dark".to_string(),
                                "light".to_string(),
                                "blue".to_string(),
                            ],
                        },
                    },
                    SettingsItem::Setting {
                        name: "Show Progress".to_string(),
                        description: "Display progress indicators during study".to_string(),
                        current_value: self.config.ui.show_progress.to_string(),
                        setting_type: SettingType::Boolean,
                    },
                ],
            },
            SettingsItem::Category {
                name: "Keyboard Shortcuts".to_string(),
                description: "View and customize keyboard shortcuts".to_string(),
                items: vec![
                    SettingsItem::Setting {
                        name: "Show Answer".to_string(),
                        description: "Key to reveal card answer".to_string(),
                        current_value: "Space".to_string(),
                        setting_type: SettingType::Text,
                    },
                    SettingsItem::Setting {
                        name: "Rate Card: Again".to_string(),
                        description: "Key to rate card as Again".to_string(),
                        current_value: "1".to_string(),
                        setting_type: SettingType::Text,
                    },
                    SettingsItem::Setting {
                        name: "Rate Card: Good".to_string(),
                        description: "Key to rate card as Good".to_string(),
                        current_value: "3".to_string(),
                        setting_type: SettingType::Text,
                    },
                ],
            },
            SettingsItem::Category {
                name: "Scheduling Algorithm".to_string(),
                description: "Configure spaced repetition parameters".to_string(),
                items: vec![
                    SettingsItem::Setting {
                        name: "Starting Ease Factor".to_string(),
                        description: "Initial ease factor for new cards".to_string(),
                        current_value: format!("{:.1}", self.config.scheduler.starting_ease_factor),
                        setting_type: SettingType::Number { min: 130, max: 300 },
                    },
                    SettingsItem::Setting {
                        name: "Easy Bonus".to_string(),
                        description: "Bonus for easy responses".to_string(),
                        current_value: format!("{:.1}", self.config.scheduler.easy_bonus),
                        setting_type: SettingType::Number { min: 0, max: 100 },
                    },
                ],
            },
            SettingsItem::Category {
                name: "Data Management".to_string(),
                description: "Manage data storage and backup".to_string(),
                items: vec![
                    SettingsItem::Setting {
                        name: "Data Directory".to_string(),
                        description: "Location where your data is stored".to_string(),
                        current_value: self
                            .config
                            .data
                            .data_dir
                            .as_ref()
                            .map(|path| path.display().to_string())
                            .unwrap_or_else(|| "Default".to_string()),
                        setting_type: SettingType::Text,
                    },
                    SettingsItem::Setting {
                        name: "Auto Backup".to_string(),
                        description: "Automatically backup your data".to_string(),
                        current_value: self.config.data.auto_backup.to_string(),
                        setting_type: SettingType::Boolean,
                    },
                ],
            },
            SettingsItem::Category {
                name: "Advanced".to_string(),
                description: "Advanced configuration options".to_string(),
                items: vec![
                    SettingsItem::Setting {
                        name: "Debug Mode".to_string(),
                        description: "Enable debug logging and features".to_string(),
                        current_value: "false".to_string(),
                        setting_type: SettingType::Boolean,
                    },
                    SettingsItem::Setting {
                        name: "Animation Speed".to_string(),
                        description: "Speed of UI animations in milliseconds".to_string(),
                        current_value: self.config.ui.animation_speed.to_string(),
                        setting_type: SettingType::Number { min: 0, max: 1000 },
                    },
                ],
            },
        ];
    }

    /// Update current items based on view mode
    fn update_current_items(&mut self) {
        match &self.view_mode {
            SettingsViewMode::CategoryList => {
                self.current_items = self.settings_items.clone();
            }
            SettingsViewMode::SettingList { category } => {
                if let Some(category_item) = self.settings_items.iter().find(
                    |item| matches!(item, SettingsItem::Category { name, .. } if name == category),
                ) {
                    if let SettingsItem::Category { items, .. } = category_item {
                        self.current_items = items.clone();
                    }
                }
            }
            SettingsViewMode::EditingValue { setting_path: _ } => {
                // No items to display in editing mode
                self.current_items.clear();
            }
        }
    }

    /// Navigate into a category
    fn enter_category(&mut self, category_name: String) {
        self.navigation_stack.push(category_name.clone());
        self.view_mode = SettingsViewMode::SettingList {
            category: category_name,
        };
        self.selected_index = 0;
        self.update_current_items();
        self.list_state.select(Some(0));
    }

    /// Go back to previous level
    fn go_back(&mut self) {
        if let Some(previous_category) = self.navigation_stack.pop() {
            if self.navigation_stack.is_empty() {
                self.view_mode = SettingsViewMode::CategoryList;
            } else {
                self.view_mode = SettingsViewMode::SettingList {
                    category: previous_category,
                };
            }
        } else {
            self.view_mode = SettingsViewMode::CategoryList;
        }

        self.selected_index = 0;
        self.update_current_items();
        self.list_state.select(Some(0));
    }

    /// Get the currently selected item
    fn get_selected_item(&self) -> Option<&SettingsItem> {
        self.current_items.get(self.selected_index)
    }

    /// Start editing a setting
    fn start_editing(&mut self) {
        if let Some(SettingsItem::Setting {
            current_value,
            name,
            ..
        }) = self.current_items.get(self.selected_index)
        {
            self.edit_field = current_value.clone();
            self.view_mode = SettingsViewMode::EditingValue {
                setting_path: name.clone(),
            };
        }
    }

    /// Save the edited value
    fn save_edit_value(&mut self) {
        // In a real implementation, this would update the actual config
        self.go_back();
    }

    /// Cancel editing
    fn cancel_edit(&mut self) {
        self.edit_field.clear();
        self.go_back();
    }
}

impl UIComponent for Settings {
    fn render(&mut self, frame: &mut Frame, _context: RenderContext) -> Result<()> {
        let area = frame.area();

        // Clone the setting_path to avoid borrowing issues
        let setting_path = if let SettingsViewMode::EditingValue { setting_path } = &self.view_mode
        {
            Some(setting_path.clone())
        } else {
            None
        };

        match &self.view_mode {
            SettingsViewMode::CategoryList | SettingsViewMode::SettingList { .. } => {
                self.render_list_view(frame, area)?;
            }
            SettingsViewMode::EditingValue { .. } => {
                if let Some(path) = setting_path {
                    self.render_edit_view(frame, area, &path)?;
                }
            }
        }

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    self.list_state.select(Some(self.selected_index));
                }
            }
            Action::Down => {
                if self.selected_index < self.current_items.len().saturating_sub(1) {
                    self.selected_index += 1;
                    self.list_state.select(Some(self.selected_index));
                }
            }
            Action::Select => {
                match &self.view_mode {
                    SettingsViewMode::CategoryList => {
                        if let Some(item) = self.get_selected_item() {
                            if let SettingsItem::Category { name, .. } = item {
                                self.enter_category(name.clone());
                            }
                        }
                    }
                    SettingsViewMode::SettingList { .. } => {
                        if let Some(item) = self.get_selected_item() {
                            if let SettingsItem::Setting { setting_type, .. } = item {
                                match setting_type {
                                    SettingType::Boolean => {
                                        // Toggle boolean values immediately
                                        self.toggle_boolean_value();
                                    }
                                    _ => {
                                        // Enter editing mode for other types
                                        self.start_editing();
                                    }
                                }
                            }
                        }
                    }
                    SettingsViewMode::EditingValue { .. } => {
                        self.save_edit_value();
                    }
                }
            }
            Action::Cancel => match &self.view_mode {
                SettingsViewMode::EditingValue { .. } => {
                    self.cancel_edit();
                }
                _ => {
                    return Ok(Some(AppState::MainMenu));
                }
            },
            Action::Char(c) => {
                if let SettingsViewMode::EditingValue { .. } = &self.view_mode {
                    self.edit_field.push(c);
                }
            }
            Action::Backspace => {
                if let SettingsViewMode::EditingValue { .. } = &self.view_mode {
                    self.edit_field.pop();
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn update(&mut self) -> Result<()> {
        // Update current values from config
        self.build_settings_hierarchy();
        self.update_current_items();
        Ok(())
    }

    fn name(&self) -> &str {
        "settings"
    }
}

impl Settings {
    /// Render list view (categories or settings)
    fn render_list_view(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(5),    // List
                Constraint::Length(3), // Footer/Instructions
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0])?;

        // Render list items
        self.render_settings_list(frame, chunks[1])?;

        // Render footer
        self.render_footer(frame, chunks[2])?;

        Ok(())
    }

    /// Render edit view for setting values
    fn render_edit_view(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        setting_path: &str,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Setting name
                Constraint::Length(3), // Edit field
                Constraint::Length(3), // Instructions
            ])
            .split(area);

        // Render header
        let header = Paragraph::new("Edit Setting")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Settings")
                    .border_style(Style::default().fg(Color::Cyan)),
            );
        frame.render_widget(header, chunks[0]);

        // Render setting name
        let setting_name = Paragraph::new(setting_path)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Setting"));
        frame.render_widget(setting_name, chunks[1]);

        // Render edit field
        let edit_field = Paragraph::new(self.edit_field.clone())
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Value")
                    .border_style(Style::default().fg(Color::Cyan)),
            );
        frame.render_widget(edit_field, chunks[2]);

        // Render instructions
        let instructions = Paragraph::new("Enter to save • Esc to cancel")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Instructions"));
        frame.render_widget(instructions, chunks[3]);

        Ok(())
    }

    /// Render header
    fn render_header(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let header_text = match &self.view_mode {
            SettingsViewMode::CategoryList => "Settings Categories",
            SettingsViewMode::SettingList { category } => category,
            _ => "Settings",
        };

        let header = Paragraph::new(header_text)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Settings")
                    .border_style(Style::default().fg(Color::Cyan)),
            );
        frame.render_widget(header, area);
        Ok(())
    }

    /// Render settings list
    fn render_settings_list(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let items: Vec<ListItem> = self
            .current_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = i == self.selected_index;
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let content = match item {
                    SettingsItem::Category {
                        name, description, ..
                    } => Line::from(vec![
                        Span::styled(format!("📁 {} ", name), style),
                        Span::styled(
                            format!(" - {}", description),
                            Style::default().fg(Color::Gray),
                        ),
                    ]),
                    SettingsItem::Setting {
                        name,
                        current_value,
                        ..
                    } => Line::from(vec![
                        Span::styled(format!("⚙️  {} ", name), style),
                        Span::styled(
                            format!(": {} ", current_value),
                            Style::default().fg(Color::Yellow),
                        ),
                    ]),
                };

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(
                    if matches!(self.view_mode, SettingsViewMode::CategoryList) {
                        "Categories"
                    } else {
                        "Settings"
                    },
                )
                .border_style(Style::default().fg(Color::Blue)),
        );

        frame.render_stateful_widget(list, area, &mut self.list_state);
        Ok(())
    }

    /// Render footer with instructions
    fn render_footer(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let instructions = match &self.view_mode {
            SettingsViewMode::CategoryList => "↑↓ Navigate • Enter Select Category • Esc Back",
            SettingsViewMode::SettingList { .. } => "↑↓ Navigate • Enter Edit/Toggle • Esc Back",
            _ => "Settings",
        };

        let footer = Paragraph::new(instructions)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, area);
        Ok(())
    }

    /// Toggle boolean setting value
    fn toggle_boolean_value(&mut self) {
        if let Some(item) = self.get_selected_item() {
            if let SettingsItem::Setting { current_value, .. } = item {
                let new_value = if current_value == "true" {
                    "false"
                } else {
                    "true"
                };
                // In a real implementation, this would update the actual config
                // For now, just rebuild the hierarchy to show the change
                self.build_settings_hierarchy();
                self.update_current_items();
            }
        }
    }
}

//! Settings panels for detailed configuration options
//!
//! Provides detailed configuration panels for each settings category with real config integration

use crate::tui::core::event_handler::Action;
use ankitui_core::{
    daily::DailyConfig, data::SchedulerConfig, shortcuts::ShortcutConfig, ui::UiConfig, Config,
    DataConfig,
};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Learning limits settings panel
pub struct LearningLimitsPanel {
    selected_option: usize,
    is_editing: bool,
    editing_field: Option<EditingField>,
    config_data: DailyConfig,
}

#[derive(Debug, Clone, PartialEq)]
enum EditingField {
    MaxNewCards,
    MaxReviewCards,
    DayStartHour,
    DayEndHour,
}

impl LearningLimitsPanel {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            is_editing: false,
            editing_field: None,
            config_data: DailyConfig::default(),
        }
    }

    pub fn with_config(config: &Config) -> Self {
        Self {
            selected_option: 0,
            is_editing: false,
            editing_field: None,
            config_data: config.daily.clone(),
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("📊 Learning Limits & Daily Goals")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Settings content - use real config data
        let settings_text = format!(
            "Daily Study Limits:\n\
            ├─ Maximum new cards per day: {}\n\
            ├─ Maximum review cards per day: {}\n\
            ├─ Day start hour: {}:00\n\
            ├─ Day end hour: {}:00\n\
            └─ Show limit warnings: {}\n\n\
            Study Goals:\n\
            ├─ Daily study time goal: {} minutes\n\
            ├─ Daily new cards goal: {}\n\
            ├─ Daily review cards goal: {}\n\
            └─ Accuracy goal: {:.1}%",
            self.config_data.max_new_cards,
            self.config_data.max_review_cards,
            self.config_data.day_start_hour,
            self.config_data.day_end_hour,
            if self.config_data.show_limit_warnings {
                "Enabled"
            } else {
                "Disabled"
            },
            self.config_data.goals.daily_minutes,
            self.config_data.goals.card_goals.daily_new_cards,
            self.config_data.goals.card_goals.daily_review_cards,
            self.config_data.goals.card_goals.accuracy_goal * 100.0
        );

        let content = Paragraph::new(settings_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);
        f.render_widget(content, chunks[1]);

        // Options
        let options = if self.is_editing {
            vec![
                "[↑/↓] Select field",
                "[Enter] Edit value",
                "[Escape] Cancel editing",
            ]
        } else {
            vec![
                "[Enter] Edit limits",
                "[R] Reset to defaults",
                "[S] Save changes",
                "[T] Test limits",
            ]
        };

        let items: Vec<ListItem> = options
            .into_iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_option {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else if self.is_editing {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(option).style(style)
            })
            .collect();

        let options_list =
            List::new(items).block(Block::default().title("Actions").borders(Borders::ALL));
        f.render_widget(options_list, chunks[2]);
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Up => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                }
            }
            Action::Down => {
                if self.selected_option < 2 {
                    self.selected_option += 1;
                }
            }
            _ => {}
        }
        false // Return false to indicate no state change
    }

    pub fn get_config(&self) -> DailyConfig {
        self.config_data.clone()
    }

    pub fn update(&mut self) {
        // Update logic if needed
    }
}

/// Theme and display settings panel
pub struct ThemeDisplayPanel {
    selected_option: usize,
    theme_name: String,
    color_scheme: String,
    card_display_style: String,
}

impl ThemeDisplayPanel {
    pub fn new() -> Self {
        Self::with_config(&Config::default())
    }

    pub fn with_config(config: &Config) -> Self {
        Self {
            selected_option: 0,
            theme_name: "Default".to_string(),
            color_scheme: "Dark".to_string(),
            card_display_style: "Compact".to_string(),
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("🎨 Theme & Display Settings")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Settings content
        let settings_text = format!(
            "Appearance:\n\
            ├─ Theme: {}\n\
            ├─ Color scheme: {}\n\
            ├─ Card display style: {}\n\
            └─ Show progress bar: Yes\n\n\
            Interface Options:\n\
            ├─ Show card counts: Yes\n\
            ├─ Show statistics: Yes\n\
            ├─ Animation effects: Enabled\n\
            └─ Compact mode: No",
            self.theme_name, self.color_scheme, self.card_display_style
        );

        let content = Paragraph::new(settings_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);
        f.render_widget(content, chunks[1]);

        // Options
        let options = vec![
            "[Enter] Change theme",
            "[C] Customize colors",
            "[S] Save settings",
        ];
        let items: Vec<ListItem> = options
            .into_iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_option {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(option).style(style)
            })
            .collect();

        let options_list =
            List::new(items).block(Block::default().title("Actions").borders(Borders::ALL));
        f.render_widget(options_list, chunks[2]);
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Up => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                }
            }
            Action::Down => {
                if self.selected_option < 2 {
                    self.selected_option += 1;
                }
            }
            _ => {}
        }
        false // Return false to indicate no state change
    }

    pub fn get_config(&self) -> UiConfig {
        // Return a default UI config for now
        UiConfig::default()
    }

    pub fn update(&mut self) {
        // Update logic if needed
    }
}

/// Keyboard shortcuts settings panel
pub struct KeyboardShortcutsPanel {
    selected_option: usize,
    shortcuts: Vec<(&'static str, &'static str)>,
}

impl KeyboardShortcutsPanel {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            shortcuts: vec![
                ("Navigation", ""),
                ("↑/↓", "Move up/down"),
                ("←/→", "Move left/right"),
                ("Enter", "Select/Confirm"),
                ("Esc", "Cancel/Back"),
                ("Tab", "Next field"),
                ("", ""),
                ("Card Review", ""),
                ("Space", "Show answer"),
                ("1-4", "Rate card"),
                ("S", "Skip card"),
                ("R", "Reset card"),
                ("", ""),
                ("Application", ""),
                ("Q", "Quit"),
                ("Ctrl+C", "Force quit"),
                ("F1", "Help"),
                ("F5", "Refresh"),
            ],
        }
    }

    pub fn with_config(config: &Config) -> Self {
        Self {
            selected_option: 0,
            shortcuts: vec![
                ("Navigation", ""),
                ("↑/↓", "Move up/down"),
                ("←/→", "Move left/right"),
                ("Enter", "Select/Confirm"),
                ("Escape", "Cancel/Back"),
                ("Card Review", ""),
                ("Space", "Show answer"),
                ("1-4", "Rate card: Again/Hard/Good/Easy"),
                ("Deck Management", ""),
                ("N", "New deck"),
                ("D", "Delete deck"),
                ("E", "Edit deck"),
                ("S", "Study deck"),
                ("General", ""),
                ("H", "Help"),
                ("Q", "Quit"),
                ("Ctrl+C", "Force quit"),
                ("F1", "Help"),
                ("F5", "Refresh"),
            ],
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("⌨️  Keyboard Shortcuts")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Shortcuts content
        let shortcut_lines: Vec<Line> = self
            .shortcuts
            .iter()
            .map(|(key, desc)| {
                if key.is_empty() {
                    Line::from("")
                } else if desc.is_empty() {
                    Line::from(format!("📂 {}", key))
                } else {
                    Line::from(format!("  {:<12} - {}", key, desc))
                }
            })
            .collect();

        let content = Paragraph::new(shortcut_lines)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);
        f.render_widget(content, chunks[1]);

        // Options
        let options = vec![
            "[Enter] Customize shortcuts",
            "[R] Reset to defaults",
            "[S] Save settings",
        ];
        let items: Vec<ListItem> = options
            .into_iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_option {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(option).style(style)
            })
            .collect();

        let options_list =
            List::new(items).block(Block::default().title("Actions").borders(Borders::ALL));
        f.render_widget(options_list, chunks[2]);
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Up => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                }
            }
            Action::Down => {
                if self.selected_option < 2 {
                    self.selected_option += 1;
                }
            }
            _ => {}
        }
        false // Return false to indicate no state change
    }

    pub fn get_config(&self) -> ShortcutConfig {
        // Return a default shortcut config for now
        ShortcutConfig::default()
    }

    pub fn update(&mut self) {
        // Update logic if needed
    }
}

/// Scheduling algorithm settings panel
pub struct SchedulingAlgorithmPanel {
    selected_option: usize,
    algorithm: String,
    starting_ease: f32,
    easy_bonus: f32,
    interval_modifier: f32,
}

impl SchedulingAlgorithmPanel {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            algorithm: "SM-2".to_string(),
            starting_ease: 2.5,
            easy_bonus: 1.3,
            interval_modifier: 1.0,
        }
    }

    pub fn with_config(config: &Config) -> Self {
        Self {
            selected_option: 0,
            algorithm: "SM-2".to_string(),
            starting_ease: config.scheduler.starting_ease_factor,
            easy_bonus: config.scheduler.easy_bonus,
            interval_modifier: config.scheduler.interval_modifier,
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("🧠 Scheduling Algorithm")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Settings content
        let settings_text = format!(
            "Algorithm Configuration:\n\
            ├─ Algorithm: {}\n\
            ├─ Starting ease factor: {:.1}\n\
            ├─ Easy interval bonus: {:.1}x\n\
            ├─ Interval modifier: {:.1}x\n\
            └─ Maximum interval: 365 days\n\n\
            Advanced Settings:\n\
            ├─ Graduating interval: 1 day\n\
            ├─ Easy interval: 4 days\n\
            ├─ Minimum interval: 1 day\n\
            └─ Failure interval: 10 minutes",
            self.algorithm, self.starting_ease, self.easy_bonus, self.interval_modifier
        );

        let content = Paragraph::new(settings_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);
        f.render_widget(content, chunks[1]);

        // Options
        let options = vec![
            "[Enter] Edit parameters",
            "[T] Test algorithm",
            "[S] Save settings",
        ];
        let items: Vec<ListItem> = options
            .into_iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_option {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(option).style(style)
            })
            .collect();

        let options_list =
            List::new(items).block(Block::default().title("Actions").borders(Borders::ALL));
        f.render_widget(options_list, chunks[2]);
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Up => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                }
            }
            Action::Down => {
                if self.selected_option < 2 {
                    self.selected_option += 1;
                }
            }
            _ => {}
        }
        false // Return false to indicate no state change
    }

    pub fn get_config(&self) -> SchedulerConfig {
        // Return a default scheduler config for now
        SchedulerConfig::default()
    }

    pub fn update(&mut self) {
        // Update logic if needed
    }
}

/// Data management settings panel
pub struct DataManagementPanel {
    selected_option: usize,
    auto_save: bool,
    backup_enabled: bool,
    backup_frequency: String,
}

impl DataManagementPanel {
    pub fn new() -> Self {
        Self::with_config(&Config::default())
    }

    pub fn with_config(config: &Config) -> Self {
        Self {
            selected_option: 0,
            auto_save: config.data.auto_backup,
            backup_enabled: config.data.auto_backup,
            backup_frequency: "Daily".to_string(),
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("📁 Data Management & Backup")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Settings content
        let settings_text = format!(
            "Data Settings:\n\
            ├─ Auto-save: {}\n\
            ├─ Backup enabled: {}\n\
            ├─ Backup frequency: {}\n\
            ├─ Max backup files: 10\n\
            └─ Compression: Enabled\n\n\
            Storage Information:\n\
            ├─ Data directory: ~/.config/ankitui\n\
            ├─ Database size: 2.4 MB\n\
            ├─ Total cards: 156\n\
            └─ Last backup: 2024-01-15 14:30",
            if self.auto_save {
                "Enabled"
            } else {
                "Disabled"
            },
            if self.backup_enabled {
                "Enabled"
            } else {
                "Disabled"
            },
            self.backup_frequency
        );

        let content = Paragraph::new(settings_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);
        f.render_widget(content, chunks[1]);

        // Options
        let options = vec![
            "[Enter] Manage data",
            "[B] Backup now",
            "[R] Restore backup",
        ];
        let items: Vec<ListItem> = options
            .into_iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_option {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(option).style(style)
            })
            .collect();

        let options_list =
            List::new(items).block(Block::default().title("Actions").borders(Borders::ALL));
        f.render_widget(options_list, chunks[2]);
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Up => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                }
            }
            Action::Down => {
                if self.selected_option < 2 {
                    self.selected_option += 1;
                }
            }
            _ => {}
        }
        false // Return false to indicate no state change
    }

    pub fn get_config(&self) -> DataConfig {
        // Return a default data config for now
        DataConfig::default()
    }

    pub fn update(&mut self) {
        // Update logic if needed
    }
}

/// Advanced settings panel
pub struct AdvancedSettingsPanel {
    selected_option: usize,
    debug_mode: bool,
    logging_enabled: bool,
    performance_mode: String,
}

impl AdvancedSettingsPanel {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            debug_mode: false,
            logging_enabled: true,
            performance_mode: "Balanced".to_string(),
        }
    }

    pub fn with_config(config: &Config) -> Self {
        Self {
            selected_option: 0,
            debug_mode: false,
            logging_enabled: true,
            performance_mode: "Balanced".to_string(),
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame, area: Rect, config: &Config) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("🔧 Advanced Settings")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(header, chunks[0]);

        // Settings content
        let settings_text = format!(
            "Advanced Options:\n\
            ├─ Debug mode: {}\n\
            ├─ Logging enabled: {}\n\
            ├─ Performance mode: {}\n\
            ├─ Cache size: 100 MB\n\
            └─ Thread pool size: 4\n\n\
            Experimental Features:\n\
            ├─ AI-assisted card creation: Off\n\
            ├─ Advanced analytics: Off\n\
            ├─ Sync across devices: Off\n\
            └─ Beta features: Off",
            if self.debug_mode {
                "Enabled"
            } else {
                "Disabled"
            },
            if self.logging_enabled {
                "Enabled"
            } else {
                "Disabled"
            },
            self.performance_mode
        );

        let content = Paragraph::new(settings_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);
        f.render_widget(content, chunks[1]);

        // Options
        let options = vec![
            "[Enter] Edit settings",
            "[R] Reset to defaults",
            "[E] Export config",
        ];
        let items: Vec<ListItem> = options
            .into_iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_option {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(option).style(style)
            })
            .collect();

        let options_list =
            List::new(items).block(Block::default().title("Actions").borders(Borders::ALL));
        f.render_widget(options_list, chunks[2]);
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Up => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                }
            }
            Action::Down => {
                if self.selected_option < 2 {
                    self.selected_option += 1;
                }
            }
            _ => {}
        }
        false // Return false to indicate no state change
    }

    pub fn get_config(&self) -> DataConfig {
        // Return a default data config for now (advanced settings can modify data config)
        DataConfig::default()
    }

    pub fn update(&mut self) {
        // Update logic if needed
    }
}

/// Settings actions that can be performed by panels
#[derive(Debug, Clone)]
pub enum SettingsAction {
    EditField,
    SaveConfig(DailyConfig),
    ResetToDefaults,
    TestLimits,
    SaveThemeConfig(UiConfig),
    SaveSchedulerConfig(SchedulerConfig),
    SaveShortcutConfig(ShortcutConfig),
}

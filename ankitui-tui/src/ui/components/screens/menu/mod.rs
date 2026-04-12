//! Main menu screen component

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Main menu screen
pub struct MenuScreen {
    state: ComponentState,
    selected_index: usize,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            selected_index: 0,
        }
    }

    pub const MENU_ITEMS: &'static [&'static str] = &[
        "📚 Study Cards",
        "🗂️ Manage Decks",
        "📊 Statistics",
        "⚙️ Settings",
        "❌ Quit",
    ];

    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < Self::MENU_ITEMS.len() - 1 {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }
}

impl Component for MenuScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let items: Vec<ListItem> = Self::MENU_ITEMS
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                let style = if i == self.selected_index {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let prefix = if i == self.selected_index { "▶ " } else { "  " };
                let text = format!("{}{}", prefix, item);

                ListItem::new(text).style(style)
            })
            .collect();

        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("AnkiTUI - Main Menu"));

        f.render_widget(list, area);

        // Help text at bottom
        if area.height > 10 {
            let help_area = Rect {
                y: area.y + area.height - 3,
                height: 3,
                ..area
            };

            let help_text = "↑↓: Navigate | Enter: Select | Esc: Quit";
            let help = Paragraph::new(help_text).style(Style::default().fg(Color::Cyan));

            f.render_widget(help, help_area);
        }
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Up => {
                        self.move_selection_up();
                        Ok(false)
                    }
                    KeyCode::Down => {
                        self.move_selection_down();
                        Ok(false)
                    }
                    KeyCode::Enter => {
                        // Signal menu selection
                        Ok(true)
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        // Select quit option
                        self.selected_index = Self::MENU_ITEMS.len() - 1;
                        Ok(true)
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
        "menu_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

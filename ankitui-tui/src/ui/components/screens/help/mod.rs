//! DEPRECATED: This component is NOT connected to the runtime.
//! The actual rendering is in `ui/render/mod.rs` via `render_*` functions.
//! Do NOT modify this file expecting runtime behavior changes.
//!
//! Help screen component

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Help screen with categorized shortcuts
pub struct HelpScreen {
    state: ComponentState,
    selected_category: usize,
}

const HELP_CATEGORIES: &[(&str, &[(&str, &str)])] = &[
    (
        "Global Shortcuts",
        &[
            ("Ctrl+Q / Ctrl+C", "Quit application"),
            ("F1 / ?", "Show this help"),
            ("F5", "Refresh current screen"),
            ("Esc", "Go back / Cancel"),
        ],
    ),
    (
        "Navigation",
        &[
            ("Up / Down", "Navigate items"),
            ("Left / Right", "Navigate tabs / Adjust values"),
            ("Enter", "Confirm / Select / Execute"),
            ("Tab", "Switch search type"),
            ("PageUp/PageDown", "Scroll up/down"),
            ("Home/End", "Jump to top/bottom"),
        ],
    ),
    (
        "Study Session",
        &[
            ("Space", "Show answer / Confirm"),
            ("1", "Again - Review soon"),
            ("2", "Hard - Review later"),
            ("3", "Good - Normal interval"),
            ("4", "Easy - Longer interval"),
            ("B", "Bury card - Skip until next session"),
            ("Ctrl+S", "Suspend card - Hide indefinitely"),
            ("U", "Unbury card - Restore buried card"),
            ("Ctrl+U", "Unsuspend card - Restore suspended card"),
        ],
    ),
    (
        "Deck Management",
        &[
            ("Enter", "Start studying selected deck"),
            ("Ctrl+N", "Create new deck (via menu)"),
            ("Delete", "Delete deck (via menu)"),
            ("F5", "Refresh deck list"),
            ("/", "Search decks"),
            ("Esc", "Return to main menu"),
        ],
    ),
    (
        "Settings",
        &[
            ("Up / Down", "Navigate settings"),
            ("Enter", "Toggle boolean option"),
            ("Left / Right", "Adjust numeric values"),
            ("Esc", "Return to main menu"),
        ],
    ),
];

impl HelpScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            selected_category: 0,
        }
    }

    fn move_up(&mut self) {
        if self.selected_category > 0 {
            self.selected_category -= 1;
            self.mark_dirty();
        }
    }

    fn move_down(&mut self) {
        if self.selected_category < HELP_CATEGORIES.len() - 1 {
            self.selected_category += 1;
            self.mark_dirty();
        }
    }
}

impl Component for HelpScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("❓ Keyboard Shortcuts & Help")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Help"));
        f.render_widget(header, chunks[0]);

        let category_names: Vec<&str> = HELP_CATEGORIES.iter().map(|(name, _)| *name).collect();
        let categories_list: Vec<ListItem> = category_names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let prefix = if i == self.selected_category { "▶" } else { " " };
                ListItem::new(format!("{} {}", prefix, name))
            })
            .collect();
        let cat_list = List::new(categories_list).block(Block::default().borders(Borders::ALL).title("Categories"));
        f.render_widget(cat_list, chunks[1]);

        let (_, shortcuts) = HELP_CATEGORIES[self.selected_category];
        let shortcut_items: Vec<ListItem> = shortcuts
            .iter()
            .map(|(key, desc)| ListItem::new(format!("  {:20} {}", key, desc)))
            .collect();
        let shortcut_list = List::new(shortcut_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(HELP_CATEGORIES[self.selected_category].0),
        );
        f.render_widget(shortcut_list, chunks[2]);

        let help = Paragraph::new("↑↓: Navigate categories | Esc: Close")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[3]);
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
        "help_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

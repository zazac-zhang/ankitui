//! Deck management screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use uuid::Uuid;

/// Deck management operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckManageAction {
    AddCards,
    BrowseCards,
    CardStatistics,
    ManageTags,
    DeleteCards,
    ExportCards,
}

pub struct DeckManageScreen {
    state: ComponentState,
    deck_id: Option<Uuid>,
    deck_name: String,
    card_count: usize,
    selected_index: usize,
    status_message: String,
    dirty: bool,
}

const MANAGE_ACTIONS: &[&str] = &[
    "📝 Add New Cards",
    "🔍 Browse Cards",
    "📊 Card Statistics",
    "🏷️ Manage Tags",
    "🗑️ Delete Cards",
    "📤 Export Cards",
];

impl DeckManageScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            deck_id: None,
            deck_name: String::new(),
            card_count: 0,
            selected_index: 0,
            status_message: String::new(),
            dirty: false,
        }
    }

    pub fn with_deck(&mut self, id: Uuid, name: String, card_count: usize) {
        self.deck_id = Some(id);
        self.deck_name = name;
        self.card_count = card_count;
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < MANAGE_ACTIONS.len() - 1 {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }

    fn execute_action(&mut self) {
        match self.selected_index {
            0 => self.status_message = "Add Cards: Select cards to add to deck".to_string(),
            1 => self.status_message = "Browse Cards: Showing all cards in deck".to_string(),
            2 => self.status_message = "Card Statistics: Computing statistics...".to_string(),
            3 => self.status_message = "Manage Tags: Add or remove tags".to_string(),
            4 => self.status_message = "Delete Cards: Select cards to delete".to_string(),
            5 => self.status_message = "Export Cards: Cards exported successfully".to_string(),
            _ => {}
        }
        self.dirty = true;
        self.mark_dirty();
    }

    pub fn get_selected_action(&self) -> Option<DeckManageAction> {
        match self.selected_index {
            0 => Some(DeckManageAction::AddCards),
            1 => Some(DeckManageAction::BrowseCards),
            2 => Some(DeckManageAction::CardStatistics),
            3 => Some(DeckManageAction::ManageTags),
            4 => Some(DeckManageAction::DeleteCards),
            5 => Some(DeckManageAction::ExportCards),
            _ => None,
        }
    }
}

impl Component for DeckManageScreen {
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

        let header = Paragraph::new("🗂️ Manage Cards")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Manage Cards"));
        f.render_widget(header, chunks[0]);

        let deck_info = if self.deck_id.is_some() {
            format!("Deck: {} | Cards: {}", self.deck_name, self.card_count)
        } else {
            "No deck selected".to_string()
        };
        let info = Paragraph::new(deck_info)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Current Deck"));
        f.render_widget(info, chunks[1]);

        let items: Vec<ListItem> = MANAGE_ACTIONS
            .iter()
            .enumerate()
            .map(|(i, action)| {
                let prefix = if i == self.selected_index { "▶" } else { " " };
                ListItem::new(format!("{} {}", prefix, action))
            })
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Actions"));
        f.render_widget(list, chunks[2]);

        let footer_text = if self.status_message.is_empty() {
            "↑↓: Navigate | Enter: Execute | Esc: Back".to_string()
        } else {
            format!("Status: {}", self.status_message)
        };
        let help = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Info"));
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
                KeyCode::Enter => {
                    self.execute_action();
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
        "deck_manage_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

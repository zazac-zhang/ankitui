//! Deck editing screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use uuid::Uuid;

pub struct DeckEditScreen {
    state: ComponentState,
    deck_id: Option<Uuid>,
    deck_name: String,
    deck_description: String,
    card_count: usize,
    selected_index: usize,
    new_cards_per_day: u32,
    max_reviews_per_day: u32,
    learning_steps: String,
    dirty: bool,
    status_message: String,
}

const EDIT_FIELDS: &[&str] = &[
    "Deck name",
    "Description",
    "New cards per day",
    "Max reviews per day",
    "Learning steps",
];

impl DeckEditScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            deck_id: None,
            deck_name: String::new(),
            deck_description: String::new(),
            card_count: 0,
            selected_index: 0,
            new_cards_per_day: 20,
            max_reviews_per_day: 200,
            learning_steps: "1m 10m".to_string(),
            dirty: false,
            status_message: String::new(),
        }
    }

    pub fn with_deck(&mut self, id: Uuid, name: String, description: Option<String>, card_count: usize) {
        self.deck_id = Some(id);
        self.deck_name = name;
        self.deck_description = description.unwrap_or_default();
        self.card_count = card_count;
    }

    pub fn with_scheduler(&mut self, new_cards: u32, max_reviews: u32, steps: String) {
        self.new_cards_per_day = new_cards;
        self.max_reviews_per_day = max_reviews;
        self.learning_steps = steps;
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < EDIT_FIELDS.len() - 1 {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }

    pub fn get_deck_updates(&self) -> Option<DeckUpdateData> {
        self.deck_id.map(|id| DeckUpdateData {
            deck_id: id,
            name: self.deck_name.clone(),
            description: self.deck_description.clone(),
            new_cards_per_day: self.new_cards_per_day,
            max_reviews_per_day: self.max_reviews_per_day,
            learning_steps: self.learning_steps.clone(),
        })
    }
}

/// Data structure for deck updates
#[derive(Debug, Clone)]
pub struct DeckUpdateData {
    pub deck_id: Uuid,
    pub name: String,
    pub description: String,
    pub new_cards_per_day: u32,
    pub max_reviews_per_day: u32,
    pub learning_steps: String,
}

impl Component for DeckEditScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("✏️ Edit Deck")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Edit Deck"));
        f.render_widget(header, chunks[0]);

        let _deck_info = if let Some(id) = self.deck_id {
            format!("Deck: {} | Cards: {} | ID: {}", self.deck_name, self.card_count, id)
        } else {
            "No deck selected".to_string()
        };

        let items: Vec<ListItem> = EDIT_FIELDS
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let prefix = if i == self.selected_index { "▶" } else { " " };
                let value = match i {
                    0 => self.deck_name.clone(),
                    1 => {
                        if self.deck_description.is_empty() {
                            "(none)".to_string()
                        } else {
                            self.deck_description.clone()
                        }
                    }
                    2 => self.new_cards_per_day.to_string(),
                    3 => self.max_reviews_per_day.to_string(),
                    4 => self.learning_steps.clone(),
                    _ => String::new(),
                };
                ListItem::new(format!("{} {}: {}", prefix, field, value))
            })
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Fields"));
        f.render_widget(list, chunks[1]);

        let footer_text = if self.status_message.is_empty() {
            "↑↓: Navigate | Enter: Edit | ←→: Adjust | Ctrl+S: Save | Esc: Back".to_string()
        } else {
            format!("Status: {}", self.status_message)
        };
        let help = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Info"));
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
                        // For text fields, would open input dialog
                        match self.selected_index {
                            0 => self.status_message = "Editing deck name... (use ←→ to modify)".to_string(),
                            1 => self.status_message = "Editing description... (use ←→ to modify)".to_string(),
                            2 => self.status_message = "Editing new cards per day".to_string(),
                            3 => self.status_message = "Editing max reviews per day".to_string(),
                            4 => self.status_message = "Editing learning steps".to_string(),
                            _ => {}
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Left => {
                        match self.selected_index {
                            2 if self.new_cards_per_day > 0 => {
                                self.new_cards_per_day -= 1;
                                self.dirty = true;
                            }
                            3 if self.max_reviews_per_day > 0 => {
                                self.max_reviews_per_day -= 1;
                                self.dirty = true;
                            }
                            _ => {}
                        }
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Right => {
                        match self.selected_index {
                            2 => {
                                self.new_cards_per_day += 1;
                                self.dirty = true;
                            }
                            3 => {
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
                        self.status_message = "Deck settings saved".to_string();
                        self.mark_dirty();
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
        "deck_edit_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

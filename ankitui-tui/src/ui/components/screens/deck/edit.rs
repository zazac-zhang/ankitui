//! Deck editing screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{layout::Rect, Frame, widgets::{Paragraph, Block, Borders, List, ListItem}, style::{Style, Color, Modifier}};
use uuid::Uuid;

pub struct DeckEditScreen {
    state: ComponentState,
    deck_id: Option<Uuid>,
    deck_name: String,
    deck_description: String,
    card_count: usize,
}

impl DeckEditScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            deck_id: None,
            deck_name: String::new(),
            deck_description: String::new(),
            card_count: 0,
        }
    }

    pub fn with_deck(&mut self, id: Uuid, name: String, description: Option<String>, card_count: usize) {
        self.deck_id = Some(id);
        self.deck_name = name;
        self.deck_description = description.unwrap_or_default();
        self.card_count = card_count;
    }
}

impl Component for DeckEditScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Length(6),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("✏️ Edit Deck")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Edit Deck"));
        f.render_widget(header, chunks[0]);

        let info = if self.deck_id.is_some() {
            vec![
                format!("Name: {}", self.deck_name),
                format!("Description: {}", if self.deck_description.is_empty() { "(none)" } else { &self.deck_description }),
                format!("Cards: {}", self.card_count),
                format!("Deck ID: {}", self.deck_id.unwrap()),
            ]
        } else {
            vec!["No deck selected.".to_string(), "Navigate to a deck first.".to_string()]
        };

        let info_items: Vec<ListItem> = info.iter().map(|line| ListItem::new(line.as_str())).collect();
        let info_list = List::new(info_items)
            .block(Block::default().borders(Borders::ALL).title("Deck Info"));
        f.render_widget(info_list, chunks[1]);

        let content = Paragraph::new(
            "Deck editing features will be available here.\n\n\
             You will be able to:\n\
             - Rename the deck\n\
             - Update description\n\
             - Configure scheduler settings\n\
             - Manage cards in this deck",
        )
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Edit Options"));
        f.render_widget(content, chunks[2]);

        let help = Paragraph::new("Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[3]);
    }

    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "deck_edit_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}
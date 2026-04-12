//! Deck management screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{layout::Rect, Frame, widgets::{Paragraph, Block, Borders, List, ListItem}, style::{Style, Color, Modifier}};
use uuid::Uuid;

pub struct DeckManageScreen {
    state: ComponentState,
    deck_id: Option<Uuid>,
    deck_name: String,
    card_count: usize,
}

impl DeckManageScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            deck_id: None,
            deck_name: String::new(),
            card_count: 0,
        }
    }

    pub fn with_deck(&mut self, id: Uuid, name: String, card_count: usize) {
        self.deck_id = Some(id);
        self.deck_name = name;
        self.card_count = card_count;
    }
}

impl Component for DeckManageScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Length(4),
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

        let options = vec![
            "1. 📝 Add New Cards",
            "2. 🔍 Browse Cards",
            "3. 📊 Card Statistics",
            "4. 🏷️ Manage Tags",
            "5. 🗑️ Delete Cards",
            "6. 📤 Export Cards",
        ];
        let items: Vec<ListItem> = options.iter().map(|item| ListItem::new(*item)).collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Available Actions"));
        f.render_widget(list, chunks[2]);

        let help = Paragraph::new("1-6: Select Action | Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[3]);
    }

    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "deck_manage_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}
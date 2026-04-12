//! Deck browsing and selection screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ankitui_core::data::models::Deck;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Deck browsing screen
pub struct DeckScreen {
    state: ComponentState,
    decks: Vec<Deck>,
    selected_index: usize,
}

impl DeckScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            decks: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn with_decks(decks: Vec<Deck>) -> Self {
        let mut screen = Self::new();
        screen.decks = decks;
        screen
    }

    pub fn set_decks(&mut self, decks: Vec<Deck>) {
        self.decks = decks;
        if self.selected_index >= self.decks.len() && !self.decks.is_empty() {
            self.selected_index = self.decks.len() - 1;
        }
        self.mark_dirty();
    }

    pub fn get_selected_deck(&self) -> Option<&Deck> {
        self.decks.get(self.selected_index)
    }

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
        if self.selected_index < self.decks.len().saturating_sub(1) {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }
}

impl Component for DeckScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let items: Vec<ListItem> = self
            .decks
            .iter()
            .enumerate()
            .map(|(i, deck)| {
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(ratatui::style::Modifier::BOLD)
                } else {
                    Style::default()
                };

                let text = format!(
                    "{}{}\n  Cards: {} | Created: {}",
                    if i == self.selected_index { "▶ " } else { "  " },
                    deck.name,
                    deck.description.as_deref().unwrap_or("No description"),
                    chrono::DateTime::format(&deck.created_at, "%Y-%m-%d").to_string()
                );

                ListItem::new(text).style(style)
            })
            .collect();

        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Select a Deck"));

        f.render_widget(list, area);

        // Help text at bottom if space allows
        if area.height > 8 {
            let help_area = Rect {
                y: area.y + area.height - 3,
                height: 3,
                ..area
            };

            let help_text = "↑↓: Navigate | Enter: Study | C: Create | E: Edit | D: Delete | Esc: Back";
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
                        // Signal deck selection
                        Ok(true)
                    }
                    KeyCode::Esc => {
                        // Signal back to menu
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
        !self.decks.is_empty()
    }

    fn id(&self) -> &str {
        "deck_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

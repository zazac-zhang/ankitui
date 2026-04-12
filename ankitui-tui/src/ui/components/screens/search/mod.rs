//! Search screen component

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Search type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Decks,
    Cards,
}

/// Search screen with interactive filtering
pub struct SearchScreen {
    state: ComponentState,
    search_type: SearchType,
    query: String,
    results: Vec<String>,
    selected_index: usize,
    dirty: bool,
}

impl SearchScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            search_type: SearchType::Decks,
            query: String::new(),
            results: Vec::new(),
            selected_index: 0,
            dirty: false,
        }
    }

    pub fn with_type(search_type: SearchType) -> Self {
        Self {
            state: ComponentState::new(),
            search_type,
            query: String::new(),
            results: Vec::new(),
            selected_index: 0,
            dirty: false,
        }
    }

    /// Add a search result item
    pub fn add_result(&mut self, result: String) {
        self.results.push(result);
        self.dirty = true;
    }

    /// Set search results
    pub fn set_results(&mut self, results: Vec<String>) {
        self.results = results;
        self.selected_index = 0;
        self.dirty = true;
    }

    /// Update search query
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.dirty = true;
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn get_selected_result(&self) -> Option<&str> {
        if self.results.is_empty() {
            None
        } else {
            Some(&self.results[self.selected_index])
        }
    }

    pub fn get_search_type(&self) -> SearchType {
        self.search_type
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < self.results.len().saturating_sub(1) {
            self.selected_index += 1;
            self.mark_dirty();
        }
    }

    fn toggle_search_type(&mut self) {
        self.search_type = match self.search_type {
            SearchType::Decks => SearchType::Cards,
            SearchType::Cards => SearchType::Decks,
        };
        self.results.clear();
        self.selected_index = 0;
        self.mark_dirty();
    }
}

impl Component for SearchScreen {
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

        let search_type_label = match self.search_type {
            SearchType::Decks => "🔍 Search Decks",
            SearchType::Cards => "🔍 Search Cards",
        };
        let header = Paragraph::new(search_type_label)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Search"));
        f.render_widget(header, chunks[0]);

        let query_display = if self.query.is_empty() {
            "Type to search... (Tab: Switch type)".to_string()
        } else {
            format!("Query: \"{}\" (Tab: Switch type)", self.query)
        };
        let query_bar = Paragraph::new(query_display)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Search Query"));
        f.render_widget(query_bar, chunks[1]);

        if self.results.is_empty() {
            let empty = if self.query.is_empty() {
                "Enter a search query above.\n\nPress Tab to switch between Decks and Cards.".to_string()
            } else {
                format!("No results found for \"{}\"", self.query)
            };
            let empty_para = Paragraph::new(empty)
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL).title("Results"));
            f.render_widget(empty_para, chunks[2]);
        } else {
            let items: Vec<ListItem> = self
                .results
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    let prefix = if i == self.selected_index { "▶" } else { " " };
                    ListItem::new(format!("{} {}", prefix, r))
                })
                .collect();
            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Results ({})", self.results.len())),
            );
            f.render_widget(list, chunks[2]);
        }

        let help = Paragraph::new("Tab: Switch type | ↑↓: Navigate | Enter: Select | Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[3]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Tab => {
                        self.toggle_search_type();
                        Ok(false)
                    }
                    KeyCode::Up => {
                        self.move_up();
                        Ok(false)
                    }
                    KeyCode::Down => {
                        self.move_down();
                        Ok(false)
                    }
                    KeyCode::Enter => {
                        // Signal that a result was selected
                        Ok(!self.results.is_empty())
                    }
                    KeyCode::Esc => Ok(true),
                    KeyCode::Char(ch) => {
                        self.query.push(ch);
                        self.dirty = true;
                        self.mark_dirty();
                        Ok(false)
                    }
                    KeyCode::Backspace => {
                        if !self.query.is_empty() {
                            self.query.pop();
                            self.dirty = true;
                            self.mark_dirty();
                        }
                        Ok(false)
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
        "search_screen"
    }
    fn state(&self) -> &ComponentState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

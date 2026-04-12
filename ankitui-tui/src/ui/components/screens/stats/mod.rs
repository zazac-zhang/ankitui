//! Statistics screen components

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{layout::Rect, Frame, widgets::{Paragraph, Block, Borders, List, ListItem, Row, Cell, Table}, style::{Style, Color, Modifier}};
use std::sync::Arc;

/// Stats screen - main statistics hub
pub struct StatsScreen {
    state: ComponentState,
}

/// Global statistics screen with real data
pub struct GlobalStatsScreen {
    state: ComponentState,
    deck_service: Option<Arc<crate::domain::DeckService>>,
    stats_service: Option<Arc<crate::domain::StatisticsService>>,
    cached_text: Option<String>,
}

/// Deck statistics screen with real deck performance data
pub struct DeckStatsScreen {
    state: ComponentState,
    deck_service: Option<Arc<crate::domain::DeckService>>,
    cached_rows: Option<Vec<Vec<String>>>,
}

impl StatsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
        }
    }
}

impl Component for StatsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("📊 Learning Statistics")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Statistics"));
        f.render_widget(header, chunks[0]);

        let menu_items = vec![
            "1. 🌐 Global Statistics - Overall learning metrics",
            "2. 📚 Deck Statistics - Per-deck performance breakdown",
            "3. 📈 Learning Progress - Retention and study trends",
        ];
        let items: Vec<ListItem> = menu_items.iter().map(|item| ListItem::new(*item)).collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Available Views"));
        f.render_widget(list, chunks[1]);

        let help = Paragraph::new("1-3: Select View | Esc: Back to Menu")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
    }
    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> { Ok(false) }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "stats_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl GlobalStatsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            deck_service: None,
            stats_service: None,
            cached_text: None,
        }
    }

    pub fn with_deck_service(&mut self, deck_service: Arc<crate::domain::DeckService>) {
        self.deck_service = Some(deck_service);
    }

    pub fn with_stats_service(&mut self, stats_service: Arc<crate::domain::StatisticsService>) {
        self.stats_service = Some(stats_service);
    }

    pub fn mark_dirty(&mut self) {
        self.cached_text = None;
        self.state.mark_dirty();
    }
}

impl Component for GlobalStatsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("🌐 Global Statistics")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Global Stats"));
        f.render_widget(header, chunks[0]);

        if let Some(text) = &self.cached_text {
            let content = Paragraph::new(text.as_str())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("Overview"));
            f.render_widget(content, chunks[1]);
        } else {
            let loading = Paragraph::new("Loading statistics...")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Loading"));
            f.render_widget(loading, chunks[1]);
        }

        let help = Paragraph::new("Esc: Back | R: Refresh")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if matches!(key.code, KeyCode::Char('r') | KeyCode::Char('R')) {
                    self.mark_dirty();
                    return Ok(true);
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "global_stats_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

impl DeckStatsScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            deck_service: None,
            cached_rows: None,
        }
    }

    pub fn with_deck_service(&mut self, deck_service: Arc<crate::domain::DeckService>) {
        self.deck_service = Some(deck_service);
    }

    pub fn mark_dirty(&mut self) {
        self.cached_rows = None;
        self.state.mark_dirty();
    }
}

impl Component for DeckStatsScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new("📚 Deck Statistics")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Deck Stats"));
        f.render_widget(header, chunks[0]);

        if let Some(rows) = &self.cached_rows {
            if !rows.is_empty() {
                let header_cells = ["Deck", "Total", "Due", "New"]
                    .iter()
                    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
                let table_rows = rows.iter().map(|row| {
                    let cells = row.iter().map(|c| Cell::from(c.clone()));
                    Row::new(cells)
                });
                let table = Table::new(table_rows, [
                    ratatui::layout::Constraint::Percentage(40),
                    ratatui::layout::Constraint::Percentage(20),
                    ratatui::layout::Constraint::Percentage(20),
                    ratatui::layout::Constraint::Percentage(20),
                ])
                .header(Row::new(header_cells))
                .block(Block::default().borders(Borders::ALL).title("Deck Performance"));
                f.render_widget(table, chunks[1]);
            } else {
                let empty = Paragraph::new("No decks available.")
                    .style(Style::default())
                    .block(Block::default().borders(Borders::ALL).title("No Data"));
                f.render_widget(empty, chunks[1]);
            }
        } else {
            let loading = Paragraph::new("Loading deck statistics...")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Loading"));
            f.render_widget(loading, chunks[1]);
        }

        let help = Paragraph::new("Esc: Back | R: Refresh")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if matches!(key.code, KeyCode::Char('r') | KeyCode::Char('R')) {
                    self.mark_dirty();
                    return Ok(true);
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }
    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "deck_stats_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}

/// Learning progress screen with interactive stats
pub struct ProgressScreen {
    state: ComponentState,
    selected_tab: usize,
    retention_rate: f32,
    cards_matured: usize,
    study_streak: i32,
    forecast_due_today: usize,
    forecast_due_week: usize,
    cards_studied_today: usize,
    total_cards: usize,
    learned_cards: usize,
}

const PROGRESS_TABS: &[&str] = &["Overview", "Retention", "Forecast"];

impl ProgressScreen {
    pub fn new() -> Self {
        Self {
            state: ComponentState::new(),
            selected_tab: 0,
            retention_rate: 0.0,
            cards_matured: 0,
            study_streak: 0,
            forecast_due_today: 0,
            forecast_due_week: 0,
            cards_studied_today: 0,
            total_cards: 0,
            learned_cards: 0,
        }
    }

    pub fn with_stats(
        retention_rate: f32,
        cards_matured: usize,
        study_streak: i32,
        forecast_due_today: usize,
        forecast_due_week: usize,
        cards_studied_today: usize,
        total_cards: usize,
        learned_cards: usize,
    ) -> Self {
        Self {
            state: ComponentState::new(),
            selected_tab: 0,
            retention_rate,
            cards_matured,
            study_streak,
            forecast_due_today,
            forecast_due_week,
            cards_studied_today,
            total_cards,
            learned_cards,
        }
    }

    fn move_tab(&mut self, direction: i32) {
        if direction > 0 {
            self.selected_tab = (self.selected_tab + 1) % PROGRESS_TABS.len();
        } else if self.selected_tab > 0 {
            self.selected_tab -= 1;
        }
        self.mark_dirty();
    }
}

impl Component for ProgressScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let tab_bar = format!("  {}  |  {}  |  {}  ",
            if self.selected_tab == 0 { "[Overview]" } else { " Overview " },
            if self.selected_tab == 1 { "[Retention]" } else { " Retention " },
            if self.selected_tab == 2 { "[Forecast]" } else { " Forecast " },
        );
        let header = Paragraph::new(tab_bar)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("📈 Learning Progress"));
        f.render_widget(header, chunks[0]);

        let content = match self.selected_tab {
            0 => {
                // Overview tab
                let pct = if self.total_cards > 0 {
                    (self.learned_cards as f32 / self.total_cards as f32 * 100.0).min(100.0)
                } else { 0.0 };
                format!(
                    "Cards Studied Today: {}\n\
                     Total Cards: {}\n\
                     Learned Cards: {} ({:.0}%)\n\
                     Study Streak: {} days\n\n\
                     ████████████████████  {:.0}% complete",
                    self.cards_studied_today,
                    self.total_cards,
                    self.learned_cards,
                    pct,
                    self.study_streak,
                    pct,
                )
            }
            1 => {
                // Retention tab
                let good_pct = self.retention_rate * 100.0;
                let poor_pct = 100.0 - good_pct;
                format!(
                    "Overall Retention Rate: {:.1}%\n\n\
                     Good retention: {:.1}%\n\
                     Poor retention: {:.1}%\n\n\
                     Cards matured: {}",
                    self.retention_rate * 100.0,
                    good_pct,
                    poor_pct,
                    self.cards_matured,
                )
            }
            2 => {
                // Forecast tab
                format!(
                    "Due Today: {}\n\
                     Due This Week: {}\n\n\
                     {} cards remaining this week\n\
                     {} new cards to review",
                    self.forecast_due_today,
                    self.forecast_due_week,
                    self.forecast_due_week.saturating_sub(self.forecast_due_today),
                    self.forecast_due_week,
                )
            }
            _ => String::new(),
        };

        let content_para = Paragraph::new(content)
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title(PROGRESS_TABS[self.selected_tab]));
        f.render_widget(content_para, chunks[1]);

        let help = Paragraph::new("←→: Switch tab | R: Refresh | Esc: Back")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(help, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Left => {
                        self.move_tab(-1);
                        Ok(false)
                    }
                    KeyCode::Right => {
                        self.move_tab(1);
                        Ok(false)
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        self.mark_dirty();
                        Ok(true) // Signal refresh
                    }
                    KeyCode::Esc => Ok(true),
                    _ => Ok(false),
                }
            }
            _ => Ok(false),
        }
    }

    fn update(&mut self) -> TuiResult<()> { Ok(()) }
    fn can_focus(&self) -> bool { true }
    fn id(&self) -> &str { "progress_screen" }
    fn state(&self) -> &ComponentState { &self.state }
    fn state_mut(&mut self) -> &mut ComponentState { &mut self.state }
}
//! Statistics Component
//!
//! Modern statistics visualization component

use crate::tui::app::AppState;
use crate::tui::core::event_handler::Action;
use crate::tui::core::{state_manager::RenderContext, UIComponent};
use ankitui_core::{Card, Deck};
use ankitui_core::{DeckStatistics, StatsEngine};
use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Chart, Dataset, Gauge, Paragraph, Row, Table, Wrap},
    Frame,
};

/// Statistics view mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatisticsView {
    Overview,
    Progress,
    Performance,
    Calendar,
}

/// Statistics component
pub struct Stats {
    /// Current view mode
    current_view: StatisticsView,
    /// Available decks
    decks: Vec<Deck>,
    /// Selected deck index
    selected_deck: usize,
    /// Stats engine
    stats_engine: Option<StatsEngine>,
    /// Cached statistics data
    cached_stats: Option<DeckStatistics>,
    /// Animation progress
    animation_progress: f32,
    /// Last update time
    last_update: std::time::Instant,
    /// Whether statistics need to be refreshed
    needs_refresh: bool,
}

impl Stats {
    /// Create a new statistics component
    pub fn new() -> Self {
        Self {
            current_view: StatisticsView::Overview,
            decks: Vec::new(),
            selected_deck: 0,
            stats_engine: None,
            cached_stats: None,
            animation_progress: 0.0,
            last_update: std::time::Instant::now(),
            needs_refresh: true,
        }
    }

    /// Set stats engine
    pub fn set_stats_engine(&mut self, engine: StatsEngine) {
        self.stats_engine = Some(engine);
    }

    /// Set available decks
    pub fn set_decks(&mut self, decks: Vec<Deck>) {
        self.decks = decks;
        self.selected_deck = 0;
        self.needs_refresh = true;
    }

    /// Check if statistics need to be refreshed
    pub fn needs_refresh(&self) -> bool {
        self.needs_refresh
    }

    /// Mark statistics as refreshed
    pub fn mark_refreshed(&mut self) {
        self.needs_refresh = false;
    }

    /// Get selected deck index
    pub fn selected_deck(&self) -> usize {
        self.selected_deck
    }

    /// Get current view name
    fn get_view_name(&self) -> &'static str {
        match self.current_view {
            StatisticsView::Overview => "Overview",
            StatisticsView::Progress => "Progress",
            StatisticsView::Performance => "Performance",
            StatisticsView::Calendar => "Calendar",
        }
    }

    /// Update statistics data
    pub async fn update_statistics(&mut self, cards: &[Card]) -> Result<()> {
        if let Some(ref mut engine) = self.stats_engine {
            if let Some(deck) = self.decks.get(self.selected_deck) {
                // Calculate real statistics using the engine and actual cards
                self.cached_stats =
                    Some(engine.calculate_deck_statistics(deck, cards, None).await?);
            }
        }
        Ok(())
    }

    /// Switch view
    fn switch_view(&mut self, view: StatisticsView) {
        self.current_view = view;
        self.animation_progress = 0.0;
        self.last_update = std::time::Instant::now();
    }

    /// Update animation
    fn update_animation(&mut self) {
        let elapsed = self.last_update.elapsed().as_secs_f32();
        if elapsed < 0.3 {
            self.animation_progress = (elapsed / 0.3).min(1.0);
        } else {
            self.animation_progress = 1.0;
        }
    }

    /// Generate sample data for charts
    fn generate_sample_progress_data(&self) -> Vec<(f64, f64)> {
        // Generate last 7 days of progress data
        vec![
            (1.0, 20.0),
            (2.0, 35.0),
            (3.0, 28.0),
            (4.0, 42.0),
            (5.0, 38.0),
            (6.0, 55.0),
            (7.0, 48.0),
        ]
    }

    /// Generate performance data
    fn generate_performance_data(&self) -> Vec<(f64, f64)> {
        // Performance scores over time
        vec![
            (1.0, 65.0),
            (2.0, 72.0),
            (3.0, 68.0),
            (4.0, 78.0),
            (5.0, 82.0),
            (6.0, 75.0),
            (7.0, 88.0),
        ]
    }

    /// Handle user action (public wrapper)
    pub fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        <Self as UIComponent>::handle_action(self, action)
    }

    /// Render component (public wrapper)
    pub fn render(&mut self, frame: &mut ratatui::Frame, context: RenderContext) -> Result<()> {
        <Self as UIComponent>::render(self, frame, context)
    }
}

impl UIComponent for Stats {
    fn render(&mut self, frame: &mut ratatui::Frame, context: RenderContext) -> Result<()> {
        // Update animations
        self.update_animation();

        let area = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // View selector
                Constraint::Min(10),   // Main content
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0])?;

        // Render view selector
        self.render_view_selector(frame, chunks[1])?;

        // Render main content based on current view
        self.render_main_content(frame, chunks[2])?;

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Left => {
                let views = [
                    StatisticsView::Overview,
                    StatisticsView::Progress,
                    StatisticsView::Performance,
                    StatisticsView::Calendar,
                ];
                let current_index = views
                    .iter()
                    .position(|&v| v == self.current_view)
                    .unwrap_or(0);
                let new_index = if current_index == 0 {
                    views.len() - 1
                } else {
                    current_index - 1
                };
                self.switch_view(views[new_index]);
            }
            Action::Right => {
                let views = [
                    StatisticsView::Overview,
                    StatisticsView::Progress,
                    StatisticsView::Performance,
                    StatisticsView::Calendar,
                ];
                let current_index = views
                    .iter()
                    .position(|&v| v == self.current_view)
                    .unwrap_or(0);
                let new_index = (current_index + 1) % views.len();
                self.switch_view(views[new_index]);
            }
            Action::Up => {
                if self.selected_deck > 0 {
                    self.selected_deck -= 1;
                    // Mark statistics as needing refresh
                    self.needs_refresh = true;
                    self.cached_stats = None;
                    self.last_update = std::time::Instant::now();
                }
            }
            Action::Down => {
                if self.selected_deck < self.decks.len().saturating_sub(1) {
                    self.selected_deck += 1;
                    // Mark statistics as needing refresh
                    self.needs_refresh = true;
                    self.cached_stats = None;
                    self.last_update = std::time::Instant::now();
                }
            }
            Action::Cancel => {
                return Ok(Some(AppState::MainMenu));
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self) -> Result<()> {
        // Update statistics data
        // In real implementation, this would be async
        // self.update_statistics().await?;
        Ok(())
    }

    fn name(&self) -> &str {
        "statistics"
    }
}

impl Stats {
    /// Render header
    fn render_header(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let deck_name = self
            .decks
            .get(self.selected_deck)
            .map(|d| d.name.as_str())
            .unwrap_or("All Decks");

        let header_text = format!("Statistics for {}", deck_name);

        let header = Paragraph::new(header_text)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Statistics")
                    .border_style(Style::default().fg(Color::Cyan)),
            );

        frame.render_widget(header, area);
        Ok(())
    }

    /// Render view selector
    fn render_view_selector(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let views = ["Overview", "Progress", "Performance", "Calendar"];
        let current_index = match self.current_view {
            StatisticsView::Overview => 0,
            StatisticsView::Progress => 1,
            StatisticsView::Performance => 2,
            StatisticsView::Calendar => 3,
        };

        let items: Vec<Span> = views
            .iter()
            .enumerate()
            .map(|(i, &view)| {
                let style = if i == current_index {
                    Style::default()
                        .fg(Color::Cyan)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };
                Span::styled(format!(" {} ", view), style)
            })
            .collect();

        let selector = Paragraph::new(Line::from(items)).block(
            Block::default()
                .borders(Borders::ALL)
                .title("View")
                .border_style(Style::default().fg(Color::Blue)),
        );

        frame.render_widget(selector, area);
        Ok(())
    }

    /// Render main content based on current view
    fn render_main_content(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        match self.current_view {
            StatisticsView::Overview => self.render_overview(frame, area)?,
            StatisticsView::Progress => self.render_progress_chart(frame, area)?,
            StatisticsView::Performance => self.render_performance_chart(frame, area)?,
            StatisticsView::Calendar => self.render_calendar_view(frame, area)?,
        }
        Ok(())
    }

    /// Render overview
    fn render_overview(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left side - Key metrics
        self.render_key_metrics(frame, chunks[0])?;

        // Right side - Progress bars
        self.render_progress_bars(frame, chunks[1])?;

        Ok(())
    }

    /// Render key metrics table
    fn render_key_metrics(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let (total_cards, new_cards, due_today, retention_rate, study_streak) =
            if let Some(ref stats) = self.cached_stats {
                (
                    stats.total_cards.to_string(),
                    stats.new_cards.to_string(),
                    stats.due_today.to_string(),
                    format!("{:.1}%", stats.retention_rate * 100.0),
                    format!("{} days", stats.study_streak_days),
                )
            } else {
                (
                    "Loading...".to_string(),
                    "Loading...".to_string(),
                    "Loading...".to_string(),
                    "Loading...".to_string(),
                    "Loading...".to_string(),
                )
            };

        let rows = vec![
            Row::new(vec!["Total Cards", total_cards.as_str()]),
            Row::new(vec!["New Cards", new_cards.as_str()]),
            Row::new(vec!["Due Today", due_today.as_str()]),
            Row::new(vec!["Retention Rate", retention_rate.as_str()]),
            Row::new(vec!["Study Streak", study_streak.as_str()]),
        ];

        let table = Table::new(
            rows,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Key Metrics")
                .border_style(Style::default().fg(Color::Green)),
        );

        frame.render_widget(table, area);
        Ok(())
    }

    /// Render progress bars
    fn render_progress_bars(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        if let Some(ref stats) = self.cached_stats {
            // Learning progress - percentage of cards that are not new
            let learning_progress = if stats.total_cards > 0 {
                ((stats.total_cards - stats.new_cards) as f64 / stats.total_cards as f64 * 100.0)
                    as u16
            } else {
                0
            };
            let learning_label = format!("{}% Complete", learning_progress);
            let learning_gauge = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Learning Progress"),
                )
                .gauge_style(Style::default().fg(Color::Green))
                .percent(learning_progress)
                .label(learning_label);

            frame.render_widget(learning_gauge, chunks[0]);

            // Daily goal - cards reviewed today vs due today
            let daily_progress = if stats.due_today > 0 {
                (stats.cards_reviewed_today as f64 / stats.due_today as f64 * 100.0).min(100.0)
                    as u16
            } else {
                100 // No cards due today, goal is complete
            };
            let daily_label = format!("{}/{} cards", stats.cards_reviewed_today, stats.due_today);
            let daily_gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title("Daily Goal"))
                .gauge_style(Style::default().fg(Color::Blue))
                .percent(daily_progress)
                .label(daily_label);

            frame.render_widget(daily_gauge, chunks[1]);

            // Weekly consistency - based on study streak
            let weekly_progress = (stats.study_streak_days.min(7) as f64 / 7.0 * 100.0) as u16;
            let weekly_label = format!("{}/7 days", stats.study_streak_days.min(7));
            let weekly_gauge = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Weekly Consistency"),
                )
                .gauge_style(Style::default().fg(Color::Yellow))
                .percent(weekly_progress)
                .label(weekly_label);

            frame.render_widget(weekly_gauge, chunks[2]);

            // Monthly streak - days studied this month
            let monthly_progress =
                (stats.study_streak_days as f64 / 30.0 * 100.0).min(100.0) as u16;
            let monthly_label = format!("{} days", stats.study_streak_days);
            let monthly_gauge = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Monthly Streak"),
                )
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent(monthly_progress)
                .label(monthly_label);

            frame.render_widget(monthly_gauge, chunks[3]);
        } else {
            // Show loading gauges
            for (i, chunk) in chunks.iter().enumerate() {
                let titles = [
                    "Learning Progress",
                    "Daily Goal",
                    "Weekly Consistency",
                    "Monthly Streak",
                ];
                let gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title(titles[i]))
                    .gauge_style(Style::default().fg(Color::Gray))
                    .percent(0)
                    .label("Loading...");
                frame.render_widget(gauge, *chunk);
            }
        }

        Ok(())
    }

    /// Render progress chart
    fn render_progress_chart(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let data = self.generate_sample_progress_data();
        let dataset = Dataset::default()
            .name("Cards Studied")
            .marker(ratatui::symbols::Marker::Dot)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(Color::Green))
            .data(&data);

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("7-Day Progress")
                    .border_style(Style::default().fg(Color::Green)),
            )
            .x_axis(
                ratatui::widgets::Axis::default()
                    .title("Days")
                    .style(Style::default().fg(Color::Gray)),
            )
            .y_axis(
                ratatui::widgets::Axis::default()
                    .title("Cards")
                    .style(Style::default().fg(Color::Gray)),
            );

        frame.render_widget(chart, area);
        Ok(())
    }

    /// Render performance chart
    fn render_performance_chart(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let data = self.generate_performance_data();
        let dataset = Dataset::default()
            .name("Performance Score")
            .marker(ratatui::symbols::Marker::Braille)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&data);

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Performance Trend")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .x_axis(
                ratatui::widgets::Axis::default()
                    .title("Days")
                    .style(Style::default().fg(Color::Gray)),
            )
            .y_axis(
                ratatui::widgets::Axis::default()
                    .title("Score (%)")
                    .style(Style::default().fg(Color::Gray)),
            );

        frame.render_widget(chart, area);
        Ok(())
    }

    /// Render calendar view (placeholder)
    fn render_calendar_view(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let calendar_text = "📅 Study Calendar\n\n\
            ✅ Active study days: 22/30\n\
            🔥 Current streak: 12 days\n\
            📊 Average daily cards: 28\n\
            ⭐ Best day: 65 cards\n\n\
            (Calendar visualization coming soon)";

        let calendar = Paragraph::new(calendar_text)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Study Calendar")
                    .border_style(Style::default().fg(Color::Yellow)),
            );

        frame.render_widget(calendar, area);
        Ok(())
    }
}

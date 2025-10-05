//! Learning Component
//!
//! Modern card learning interface with clean architecture

use crate::tui::app::AppState;
use crate::tui::core::event_handler::Action;
use crate::tui::core::{state_manager::RenderContext, UIComponent};
use ankitui_core::core::scheduler;
use ankitui_core::Card;
use ankitui_core::SessionProgress;
use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

/// Learning component state
#[derive(Debug, Clone)]
pub struct LearningState {
    /// Current card being reviewed
    current_card: Option<Card>,
    /// Whether answer is shown
    answer_shown: bool,
    /// Session progress
    session_progress: Option<SessionProgress>,
    /// Selected rating button (0-4 for Again, Hard, Good, Easy)
    selected_rating: usize,
    /// Animation progress for card flip
    flip_animation: f32,
    /// Last update time
    last_update: std::time::Instant,
}

/// Modern learning component
pub struct Study {
    /// Component state
    state: LearningState,
}

impl Study {
    /// Create a new learning component
    pub fn new() -> Self {
        Self {
            state: LearningState {
                current_card: None,
                answer_shown: false,
                session_progress: None,
                selected_rating: 2, // Default to "Good"
                flip_animation: 0.0,
                last_update: std::time::Instant::now(),
            },
        }
    }

    /// Set current card for learning
    pub fn set_current_card(&mut self, card: Option<Card>) {
        self.state.current_card = card;
        self.state.answer_shown = false;
        self.state.flip_animation = 0.0;
        self.state.last_update = std::time::Instant::now();
    }

    /// Set session progress
    pub fn set_session_progress(&mut self, progress: Option<SessionProgress>) {
        self.state.session_progress = progress;
    }

    /// Show answer
    pub fn show_answer(&mut self) {
        self.state.answer_shown = true;
        self.state.flip_animation = 0.0;
        self.state.last_update = std::time::Instant::now();
    }

    /// Hide answer
    pub fn hide_answer(&mut self) {
        self.state.answer_shown = false;
        self.state.flip_animation = 0.0;
        self.state.last_update = std::time::Instant::now();
    }

    /// Get current card
    pub fn current_card(&self) -> Option<&Card> {
        self.state.current_card.as_ref()
    }

    /// Get whether answer is shown
    pub fn answer_shown(&self) -> bool {
        self.state.answer_shown
    }

    /// Get selected rating
    pub fn selected_rating(&self) -> usize {
        self.state.selected_rating
    }

    /// Request next card (handled by app)
    pub fn request_next_card(&mut self) {
        self.state.current_card = None;
        self.state.answer_shown = false;
        self.state.selected_rating = 2; // Reset to default
        self.state.flip_animation = 0.0;
        self.state.last_update = std::time::Instant::now();
    }

    /// Prepare for rating submission
    pub fn prepare_rating(&self) -> scheduler::Rating {
        match self.state.selected_rating {
            0 => scheduler::Rating::Again,
            1 => scheduler::Rating::Hard,
            2 => scheduler::Rating::Good,
            3 => scheduler::Rating::Easy,
            _ => scheduler::Rating::Good,
        }
    }

    /// Update animations
    fn update_animations(&mut self) {
        let elapsed = self.state.last_update.elapsed().as_secs_f32();

        if !self.state.answer_shown && elapsed < 0.3 {
            // Flip animation when showing answer
            self.state.flip_animation = (elapsed / 0.3).min(1.0);
        } else if self.state.answer_shown && elapsed < 0.3 {
            self.state.flip_animation = (elapsed / 0.3).min(1.0);
        } else {
            self.state.flip_animation = 1.0;
        }
    }

    /// Get rating button text
    fn get_rating_text(&self, index: usize) -> &'static str {
        match index {
            0 => "Again (1)",
            1 => "Hard (2)",
            2 => "Good (3)",
            3 => "Easy (4)",
            _ => "Good (3)",
        }
    }

    /// Get rating button style
    fn get_rating_style(&self, index: usize) -> Style {
        let base_style = match index {
            0 => Style::default().fg(Color::Red),
            1 => Style::default().fg(Color::Yellow),
            2 => Style::default().fg(Color::Green),
            3 => Style::default().fg(Color::Cyan),
            _ => Style::default().fg(Color::Green),
        };

        if index == self.state.selected_rating {
            base_style.add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            base_style
        }
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

impl UIComponent for Study {
    fn render(&mut self, frame: &mut ratatui::Frame, context: RenderContext) -> Result<()> {
        // Update animations
        self.update_animations();

        let area = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Card content
                Constraint::Length(3), // Progress
                Constraint::Length(5), // Rating buttons
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0])?;

        // Render card
        self.render_card(frame, chunks[1])?;

        // Render progress
        self.render_progress(frame, chunks[2])?;

        // Render rating buttons
        self.render_rating_buttons(frame, chunks[3])?;

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::ShowAnswer => {
                if !self.state.answer_shown {
                    self.show_answer();
                }
            }
            Action::RateAgain => {
                if self.state.answer_shown {
                    self.state.selected_rating = 0;
                }
            }
            Action::RateHard => {
                if self.state.answer_shown {
                    self.state.selected_rating = 1;
                }
            }
            Action::RateGood => {
                if self.state.answer_shown {
                    self.state.selected_rating = 2;
                }
            }
            Action::RateEasy => {
                if self.state.answer_shown {
                    self.state.selected_rating = 3;
                }
            }
            Action::Select => {
                if self.state.answer_shown {
                    // Prepare for next card - actual rating submission handled by app
                    self.request_next_card();
                } else {
                    self.show_answer();
                }
            }
            Action::Left => {
                if self.state.answer_shown {
                    if self.state.selected_rating > 0 {
                        self.state.selected_rating -= 1;
                    }
                }
            }
            Action::Right => {
                if self.state.answer_shown {
                    if self.state.selected_rating < 3 {
                        self.state.selected_rating += 1;
                    }
                }
            }
            Action::Cancel => {
                return Ok(Some(AppState::MainMenu));
            }
            Action::SwitchDeck => {
                return Ok(Some(AppState::DeckSelection));
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self) -> Result<()> {
        // Component state updates are handled by the app
        Ok(())
    }

    fn name(&self) -> &str {
        "learning"
    }
}

impl Study {
    /// Render header
    fn render_header(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let header_text = if self.state.answer_shown {
            "Rate this card:"
        } else {
            "Study this card:"
        };

        let header = Paragraph::new(header_text)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Learning Session")
                    .border_style(Style::default().fg(Color::Cyan)),
            );

        frame.render_widget(header, area);
        Ok(())
    }

    /// Render card content
    fn render_card(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(ref card) = self.state.current_card {
            let content = if self.state.answer_shown {
                format!(
                    "Front: {}\n\nBack: {}",
                    card.content.front, card.content.back
                )
            } else {
                format!("Front: {}", card.content.front)
            };

            let paragraph = Paragraph::new(content).wrap(Wrap { trim: true }).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Card")
                    .border_style(if self.state.answer_shown {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Blue)
                    }),
            );

            frame.render_widget(paragraph, area);
        } else {
            let paragraph = Paragraph::new("No cards available")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL).title("Card"));

            frame.render_widget(paragraph, area);
        }
        Ok(())
    }

    /// Render progress bar
    fn render_progress(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(ref progress) = self.state.session_progress {
            let progress_text = format!(
                "Progress: {}/{} cards remaining (New: {}, Learning: {}, Review: {})",
                progress
                    .total_remaining
                    .saturating_sub(progress.review_remaining),
                progress.total_remaining,
                progress.new_remaining,
                progress.learning_remaining,
                progress.review_remaining
            );

            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Green))
                .percent(if progress.total_remaining > 0 {
                    ((progress.total_remaining - progress.review_remaining) as f32
                        / progress.total_remaining as f32
                        * 100.0) as u16
                } else {
                    0
                })
                .label(progress_text);

            frame.render_widget(gauge, area);
        } else {
            let paragraph = Paragraph::new("Loading progress...")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));

            frame.render_widget(paragraph, area);
        }
        Ok(())
    }

    /// Render rating buttons
    fn render_rating_buttons(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        let rating_buttons = ["Again", "Hard", "Good", "Easy"];
        let shortcuts = ["(1)", "(2)", "(3)", "(4)"];

        for (i, ((button, shortcut), chunk)) in rating_buttons
            .iter()
            .zip(shortcuts.iter())
            .zip(button_layout.iter())
            .enumerate()
        {
            let text = if self.state.answer_shown {
                format!("{} {}", button, shortcut)
            } else {
                "Show Answer".to_string()
            };

            let style = if self.state.answer_shown {
                self.get_rating_style(i)
            } else {
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            };

            let paragraph = Paragraph::new(text)
                .style(style)
                .block(Block::default().borders(Borders::ALL).border_style(style));

            frame.render_widget(paragraph, *chunk);
        }

        Ok(())
    }
}

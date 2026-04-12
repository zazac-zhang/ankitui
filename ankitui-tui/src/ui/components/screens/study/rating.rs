//! Study rating selection screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ankitui_core::core::Rating;
use ankitui_core::data::Card;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Study screen for rating card performance
pub struct StudyRatingScreen {
    state: ComponentState,
    card: Card,
    response_time_ms: i64,
    selected_rating: Option<Rating>,
    card_number: usize,
    total_cards: usize,
}

impl StudyRatingScreen {
    pub fn new(card: Card, response_time_ms: i64, card_number: usize, total_cards: usize) -> Self {
        Self {
            state: ComponentState::new(),
            card,
            response_time_ms,
            selected_rating: None,
            card_number,
            total_cards,
        }
    }

    pub fn get_card(&self) -> &Card {
        &self.card
    }

    pub fn get_selected_rating(&self) -> Option<Rating> {
        self.selected_rating
    }

    pub fn get_response_time_display(&self) -> String {
        let seconds = self.response_time_ms / 1000;
        if seconds < 60 {
            format!("{}s", seconds)
        } else {
            let minutes = seconds / 60;
            let remaining_seconds = seconds % 60;
            format!("{}m {}s", minutes, remaining_seconds)
        }
    }

    fn get_rating_info(&self, rating: Rating) -> (&'static str, &'static str, Color) {
        match rating {
            Rating::Again => ("Again", "Show card again soon (reset progress)", Color::Red),
            Rating::Hard => ("Hard", "Review with shorter interval", Color::Yellow),
            Rating::Good => ("Good", "Normal review interval", Color::Green),
            Rating::Easy => ("Easy", "Extend review interval significantly", Color::Cyan),
        }
    }
}

impl Component for StudyRatingScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let ratings = [Rating::Again, Rating::Hard, Rating::Good, Rating::Easy];

        let mut rating_text = String::new();

        for (i, rating) in ratings.iter().enumerate() {
            let (label, description, color) = self.get_rating_info(*rating);
            let key = match i {
                0 => "1",
                1 => "2",
                2 => "3",
                3 => "4",
                _ => "?",
            };

            let selected = self.selected_rating == Some(*rating);
            let prefix = if selected { "● " } else { "○ " };

            rating_text.push_str(&format!("{}[{}] {} - {}\n", prefix, key, label, description));
        }

        rating_text.push_str(&format!(
            "\nResponse time: {} | Card {}/{}",
            self.get_response_time_display(),
            self.card_number,
            self.total_cards
        ));

        let paragraph = Paragraph::new(rating_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("How well did you know this card?"),
        );

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                let rating = match key.code {
                    KeyCode::Char('1') => Some(Rating::Again),
                    KeyCode::Char('2') => Some(Rating::Hard),
                    KeyCode::Char('3') => Some(Rating::Good),
                    KeyCode::Char('4') => Some(Rating::Easy),
                    KeyCode::Enter => {
                        if let Some(selected) = self.selected_rating {
                            return Ok(true); // Confirm selection
                        }
                        None
                    }
                    KeyCode::Esc => None, // Cancel
                    _ => None,
                };

                if let Some(r) = rating {
                    self.selected_rating = Some(r);
                    self.mark_dirty();

                    // Auto-confirm on selection or require Enter?
                    // For now, auto-confirm
                    Ok(true)
                } else {
                    Ok(false)
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
        "study_rating_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

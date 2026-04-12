//! Study question display screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ankitui_core::data::models::Card;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// Study screen showing card question
pub struct StudyQuestionScreen {
    state: ComponentState,
    card: Card,
    question_shown_at: chrono::DateTime<chrono::Utc>,
    show_hint: bool,
    card_number: usize,
    total_cards: usize,
}

impl StudyQuestionScreen {
    pub fn new(card: Card, card_number: usize, total_cards: usize) -> Self {
        Self {
            state: ComponentState::new(),
            card,
            question_shown_at: chrono::Utc::now(),
            show_hint: false,
            card_number,
            total_cards,
        }
    }

    pub fn get_card(&self) -> &Card {
        &self.card
    }

    pub fn get_response_time_ms(&self) -> i64 {
        (chrono::Utc::now() - self.question_shown_at).num_milliseconds()
    }

    pub fn toggle_hint(&mut self) {
        self.show_hint = !self.show_hint;
        self.mark_dirty();
    }

    pub fn get_elapsed_time_display(&self) -> String {
        let elapsed = (chrono::Utc::now() - self.question_shown_at).num_seconds();
        let minutes = elapsed / 60;
        let seconds = elapsed % 60;
        if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

impl Component for StudyQuestionScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        // Card info header
        let header_text = format!(
            "Card {}/{} | State: {} | Interval: {} | Ease: {:.0}%",
            self.card_number,
            self.total_cards,
            match self.card.state.state {
                ankitui_core::data::models::CardState::New => "New",
                ankitui_core::data::models::CardState::Learning => "Learning",
                ankitui_core::data::models::CardState::Review => "Review",
                ankitui_core::data::models::CardState::Relearning => "Relearning",
                ankitui_core::data::models::CardState::Buried => "Buried",
                ankitui_core::data::models::CardState::Suspended => "Suspended",
            },
            if self.card.state.interval == 0 {
                "New".to_string()
            } else if self.card.state.interval == 1 {
                "1 day".to_string()
            } else {
                format!("{} days", self.card.state.interval)
            },
            self.card.state.ease_factor * 100.0
        );

        let header = Paragraph::new(header_text).style(Style::default().fg(Color::Cyan));

        // Question content
        let question_text = format!(
            "{}\n\n{}",
            if self.show_hint && !self.card.content.tags.is_empty() {
                format!("Tags: {}\n\n", self.card.content.tags.join(", "))
            } else {
                String::new()
            },
            self.card.content.front
        );

        let question = Paragraph::new(question_text)
            .block(Block::default().borders(Borders::ALL).title("Question"))
            .style(Style::default().add_modifier(Modifier::BOLD));

        // Instructions
        let instructions = Paragraph::new("Space: Show Answer | H: Toggle Hint | Esc: Pause Session")
            .style(Style::default().fg(Color::Yellow));

        // Layout
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3), // Header
                ratatui::layout::Constraint::Min(0),    // Question (flexible)
                ratatui::layout::Constraint::Length(3), // Instructions
            ])
            .split(area);

        f.render_widget(header, chunks[0]);
        f.render_widget(question, chunks[1]);
        f.render_widget(instructions, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char(' ') => {
                        // Show answer - signal to transition to answer screen
                        Ok(true)
                    }
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        self.toggle_hint();
                        Ok(false)
                    }
                    KeyCode::Esc => {
                        // Signal pause request
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
        true
    }

    fn id(&self) -> &str {
        "study_question_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

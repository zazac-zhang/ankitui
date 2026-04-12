//! Study answer display screen

use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ankitui_core::data::models::Card;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Study screen showing card answer
pub struct StudyAnswerScreen {
    state: ComponentState,
    card: Card,
    answer_shown_at: chrono::DateTime<chrono::Utc>,
}

impl StudyAnswerScreen {
    pub fn new(card: Card) -> Self {
        Self {
            state: ComponentState::new(),
            card,
            answer_shown_at: chrono::Utc::now(),
        }
    }

    pub fn get_card(&self) -> &Card {
        &self.card
    }
}

impl Component for StudyAnswerScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(10), // Question
                ratatui::layout::Constraint::Min(0),     // Answer
                ratatui::layout::Constraint::Length(3),  // Instructions
            ])
            .split(area);

        // Question
        let question = Paragraph::new(self.card.content.front.as_str())
            .block(Block::default().borders(Borders::ALL).title("Question"))
            .style(Style::default().fg(Color::DarkGray));

        // Answer
        let answer = Paragraph::new(self.card.content.back.as_str())
            .block(Block::default().borders(Borders::ALL).title("Answer"))
            .style(Style::default().add_modifier(Modifier::BOLD));

        // Instructions
        let instructions =
            Paragraph::new("Space: Rate Card | Esc: Back to Question").style(Style::default().fg(Color::Yellow));

        f.render_widget(question, chunks[0]);
        f.render_widget(answer, chunks[1]);
        f.render_widget(instructions, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char(' ') => Ok(true), // Go to rating
                    KeyCode::Esc => Ok(true),       // Back to question
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
        "study_answer_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

//! Study session finished screen

use crate::domain::StudySessionStats;
use crate::ui::components::base::{Component, ComponentState};
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Study session completion screen
pub struct StudyFinishedScreen {
    state: ComponentState,
    session_stats: StudySessionStats,
}

impl StudyFinishedScreen {
    pub fn new(session_stats: StudySessionStats) -> Self {
        Self {
            state: ComponentState::new(),
            session_stats,
        }
    }

    pub fn get_stats(&self) -> &StudySessionStats {
        &self.session_stats
    }

    fn format_duration(&self, duration: chrono::Duration) -> String {
        let total_seconds = duration.num_seconds();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

impl Component for StudyFinishedScreen {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let duration = if let Some(end_time) = self.session_stats.ended_at {
            end_time - self.session_stats.started_at
        } else {
            chrono::Utc::now() - self.session_stats.started_at
        };

        let accuracy = if self.session_stats.total_cards_studied > 0 {
            (self.session_stats.correct_answers as f32 / self.session_stats.total_cards_studied as f32) * 100.0
        } else {
            0.0
        };

        let stats_text = format!(
            "📊 Session Complete!\n\n\
             Cards Studied: {}\n\
             New Cards: {}\n\
             Review Cards: {}\n\
             Correct Answers: {}\n\
             Accuracy: {:.1}%\n\
             Study Time: {}\n\n\
             Press Enter to continue",
            self.session_stats.cards_studied,
            self.session_stats.new_cards,
            self.session_stats.review_cards,
            self.session_stats.correct_answers,
            accuracy,
            self.format_duration(duration)
        );

        let paragraph = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title("Study Session Results"))
            .style(Style::default().fg(Color::Green));

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Enter => Ok(true), // Continue
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
        "study_finished_screen"
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

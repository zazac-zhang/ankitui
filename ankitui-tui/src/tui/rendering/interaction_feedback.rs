//! Interactive Feedback System
//!
//! Provides enhanced user interaction feedback including haptic-like responses,
//! visual notifications, and contextual help

use ratatui::{
    backend::Backend,
    layout::{Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::theme::Theme;
use super::renderer::{Renderer, VisualEffect};

/// Types of user interactions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InteractionType {
    Navigation,
    Selection,
    Confirmation,
    Cancellation,
    Input,
    Error,
    Success,
    Warning,
    Info,
    Help,
    Rating,
    Skip,
    Reset,
}

/// Feedback intensity levels
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackIntensity {
    Subtle,
    Normal,
    Strong,
    Critical,
}

/// Contextual feedback message
#[derive(Debug, Clone)]
pub struct FeedbackMessage {
    pub id: String,
    pub message: String,
    pub interaction_type: InteractionType,
    pub intensity: FeedbackIntensity,
    pub created_at: Instant,
    pub duration: Duration,
    pub auto_dismiss: bool,
}

impl FeedbackMessage {
    pub fn new(
        id: String,
        message: String,
        interaction_type: InteractionType,
        intensity: FeedbackIntensity,
    ) -> Self {
        let duration = match intensity {
            FeedbackIntensity::Subtle => Duration::from_millis(1000),
            FeedbackIntensity::Normal => Duration::from_millis(2000),
            FeedbackIntensity::Strong => Duration::from_millis(3000),
            FeedbackIntensity::Critical => Duration::from_millis(5000),
        };

        Self {
            id,
            message,
            interaction_type,
            intensity,
            created_at: Instant::now(),
            duration,
            auto_dismiss: true,
        }
    }

    pub fn persistent(mut self) -> Self {
        self.auto_dismiss = false;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn is_expired(&self) -> bool {
        self.auto_dismiss && self.created_at.elapsed() > self.duration
    }
}

/// Interactive feedback system
pub struct InteractionFeedback {
    messages: Vec<FeedbackMessage>,
    shortcuts: HashMap<String, String>,
    contextual_help: HashMap<InteractionType, Vec<String>>,
    theme: Theme,
    renderer: Renderer,
    last_interaction: Option<(InteractionType, Instant)>,
    interaction_count: HashMap<InteractionType, usize>,
}

impl InteractionFeedback {
    /// Create new feedback system
    pub fn new(theme: Theme) -> Self {
        let mut feedback = Self {
            messages: Vec::new(),
            shortcuts: HashMap::new(),
            contextual_help: HashMap::new(),
            theme: theme.clone(),
            renderer: Renderer::with_theme(theme),
            last_interaction: None,
            interaction_count: HashMap::new(),
        };

        feedback.setup_default_shortcuts();
        feedback.setup_contextual_help();
        feedback
    }

    /// Setup default keyboard shortcuts
    fn setup_default_shortcuts(&mut self) {
        self.shortcuts.insert("↑↓".to_string(), "Navigate".to_string());
        self.shortcuts.insert("Enter".to_string(), "Select/Confirm".to_string());
        self.shortcuts.insert("Space".to_string(), "Show Answer/Action".to_string());
        self.shortcuts.insert("Esc".to_string(), "Cancel/Back".to_string());
        self.shortcuts.insert("1-4".to_string(), "Rate Cards".to_string());
        self.shortcuts.insert("Q".to_string(), "Quit".to_string());
        self.shortcuts.insert("F1".to_string(), "Help".to_string());
        self.shortcuts.insert("Tab".to_string(), "Next Field".to_string());
        self.shortcuts.insert("Ctrl+T".to_string(), "Switch Theme".to_string());
    }

    /// Setup contextual help messages
    fn setup_contextual_help(&mut self) {
        self.contextual_help.insert(
            InteractionType::Navigation,
            vec![
                "Use ↑↓ to navigate through items".to_string(),
                "Use Enter to select the highlighted item".to_string(),
            ],
        );

        self.contextual_help.insert(
            InteractionType::Rating,
            vec![
                "1/A: Again - Card will appear soon".to_string(),
                "2/H: Hard - Card will appear in a few days".to_string(),
                "3/G: Good - Card will appear in about a week".to_string(),
                "4/X: Easy - Card will appear in 2+ weeks".to_string(),
            ],
        );

        self.contextual_help.insert(
            InteractionType::Error,
            vec![
                "Check your input and try again".to_string(),
                "Press F1 for help if needed".to_string(),
            ],
        );

        self.contextual_help.insert(
            InteractionType::Success,
            vec![
                "Great job! Keep up the good work".to_string(),
                "Your progress is being saved".to_string(),
            ],
        );
    }

    /// Add feedback message
    pub fn add_feedback(&mut self, message: String, interaction_type: InteractionType) {
        let intensity = match interaction_type {
            InteractionType::Error => FeedbackIntensity::Strong,
            InteractionType::Success => FeedbackIntensity::Normal,
            InteractionType::Confirmation => FeedbackIntensity::Subtle,
            InteractionType::Cancellation => FeedbackIntensity::Subtle,
            InteractionType::Rating => FeedbackIntensity::Normal,
            _ => FeedbackIntensity::Normal,
        };

        let feedback_id = format!("{}_{:?}", interaction_type.clone() as u8, Instant::now().elapsed().as_millis());
        let feedback_message = FeedbackMessage::new(feedback_id, message, interaction_type.clone(), intensity);

        self.messages.push(feedback_message);
        self.last_interaction = Some((interaction_type.clone(), Instant::now()));

        // Update interaction count
        *self.interaction_count.entry(interaction_type.clone()).or_insert(0) += 1;

        // Add visual effect to renderer
        let visual_effect = match interaction_type.clone() {
            InteractionType::Success => VisualEffect::Success,
            InteractionType::Error => VisualEffect::Error,
            InteractionType::Warning => VisualEffect::Warning,
            InteractionType::Selection => VisualEffect::Highlight,
            _ => VisualEffect::Info,
        };

        self.renderer.add_effect(visual_effect);
    }

    /// Add persistent feedback (doesn't auto-dismiss)
    pub fn add_persistent_feedback(&mut self, message: String, interaction_type: InteractionType) {
        let feedback_id = format!("persistent_{:?}", Instant::now().elapsed().as_millis());
        let feedback_message = FeedbackMessage::new(feedback_id, message, interaction_type, FeedbackIntensity::Normal)
            .persistent();

        self.messages.push(feedback_message);
    }

    /// Dismiss specific feedback message
    pub fn dismiss_feedback(&mut self, id: &str) {
        self.messages.retain(|m| m.id != id);
    }

    /// Dismiss all feedback messages
    pub fn clear_feedback(&mut self) {
        self.messages.clear();
    }

    /// Clean up expired messages
    pub fn cleanup_expired(&mut self) {
        self.messages.retain(|m| !m.is_expired());
    }

    /// Update feedback system
    pub fn update(&mut self) {
        self.cleanup_expired();
        self.renderer.update();
    }

    /// Get the most recent feedback message
    pub fn get_latest_feedback(&self) -> Option<&FeedbackMessage> {
        self.messages.last()
    }

    /// Get feedback messages by type
    pub fn get_feedback_by_type(&self, interaction_type: InteractionType) -> Vec<&FeedbackMessage> {
        self.messages
            .iter()
            .filter(|m| m.interaction_type == interaction_type)
            .collect()
    }

    /// Render feedback bar
    pub fn render_feedback_bar<B: Backend>(&self, f: &mut Frame, area: Rect) {
        if let Some(feedback) = self.get_latest_feedback() {
            let (icon, style) = match feedback.interaction_type {
                InteractionType::Success => ("✅", self.theme.styles.status_success),
                InteractionType::Error => ("❌", self.theme.styles.status_error),
                InteractionType::Warning => ("⚠️", self.theme.styles.status_warning),
                InteractionType::Info => ("ℹ️", self.theme.styles.status_info),
                InteractionType::Confirmation => ("✓", self.theme.styles.status_success),
                InteractionType::Cancellation => ("✗", self.theme.styles.status_error),
                InteractionType::Rating => ("⭐", self.theme.styles.status_info),
                InteractionType::Navigation => ("🧭", self.theme.styles.status_info),
                InteractionType::Selection => ("🎯", self.theme.styles.status_info),
                InteractionType::Input => ("✏️", self.theme.styles.status_info),
                InteractionType::Help => ("💡", self.theme.styles.status_info),
                InteractionType::Skip => ("⏭️", self.theme.styles.status_warning),
                InteractionType::Reset => ("🔄", self.theme.styles.status_info),
            };

            let feedback_widget = Paragraph::new(Line::from(vec![
                Span::styled(format!(" {} ", icon), style),
                Span::styled(&feedback.message, style),
                Span::styled(
                    format!(" [{}]", format_duration(feedback.duration - feedback.created_at.elapsed())),
                    self.theme.styles.caption,
                ),
            ]))
            .style(self.theme.styles.body)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match feedback.intensity {
                        FeedbackIntensity::Critical => self.theme.styles.border_highlight,
                        FeedbackIntensity::Strong => self.theme.styles.border_focused,
                        FeedbackIntensity::Normal => self.theme.styles.border_normal,
                        FeedbackIntensity::Subtle => self.theme.styles.border_normal,
                    })
            );

            f.render_widget(feedback_widget, area);
        } else {
            // Render shortcuts help when no feedback
            self.render_shortcuts_help::<B>(f, area);
        }
    }

    /// Render shortcuts help
    fn render_shortcuts_help<B: Backend>(&self, f: &mut Frame, area: Rect) {
        let shortcuts_text = self.shortcuts
            .iter()
            .take(5) // Show top 5 shortcuts
            .map(|(key, desc)| format!("{}: {}", key, desc))
            .collect::<Vec<_>>()
            .join("  •  ");

        let help_widget = Paragraph::new(Line::from(Span::styled(
            shortcuts_text,
            self.theme.styles.caption,
        )))
        .style(self.theme.styles.body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.styles.border_normal)
                .title(" Quick Help ")
        );

        f.render_widget(help_widget, area);
    }

    /// Render contextual help panel
    pub fn render_contextual_help<B: Backend>(
        &self,
        f: &mut Frame,
        area: Rect,
        current_interaction: InteractionType,
    ) {
        if let Some(help_lines) = self.contextual_help.get(&current_interaction) {
            let help_content: Vec<Line> = help_lines
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    Line::from(vec![
                        Span::styled(format!("• ",), self.theme.styles.status_info),
                        Span::styled(line, self.theme.styles.body),
                    ])
                })
                .collect();

            let help_widget = Paragraph::new(help_content)
                .style(self.theme.styles.body)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(self.theme.styles.border_focused)
                        .title_style(self.theme.styles.subtitle)
                        .title(" Contextual Help ")
                )
                .wrap(Wrap { trim: true });

            f.render_widget(Clear, area);
            f.render_widget(help_widget, area);
        }
    }

    /// Get interaction statistics
    pub fn get_interaction_stats(&self) -> HashMap<InteractionType, usize> {
        self.interaction_count.clone()
    }

    /// Check if user is inactive (no interactions for a while)
    pub fn is_user_inactive(&self, threshold: Duration) -> bool {
        if let Some((_, last_time)) = self.last_interaction {
            last_time.elapsed() > threshold
        } else {
            true
        }
    }

    /// Get user activity level
    pub fn get_activity_level(&self) -> f32 {
        let total_interactions: usize = self.interaction_count.values().sum();
        if total_interactions == 0 {
            return 0.0;
        }

        // Calculate activity based on recent interactions
        let recent_interactions = self.messages
            .iter()
            .filter(|m| m.created_at.elapsed() < Duration::from_secs(60))
            .count();

        (recent_interactions as f32 / total_interactions.max(1) as f32) * 100.0
    }

    /// Update theme
    pub fn update_theme(&mut self, theme: Theme) {
        self.theme = theme.clone();
        self.renderer = Renderer::with_theme(theme);
    }
}

impl Default for InteractionFeedback {
    fn default() -> Self {
        Self::new(Theme::dark())
    }
}

/// Format duration for display
fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    if seconds >= 60 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}

/// Macro for adding feedback quickly
#[macro_export]
macro_rules! feedback {
    ($feedback_system:expr, $message:expr, $interaction_type:expr) => {
        $feedback_system.add_feedback($message.to_string(), $interaction_type);
    };
    ($feedback_system:expr, $message:expr, $interaction_type:expr, persistent) => {
        $feedback_system.add_persistent_feedback($message.to_string(), $interaction_type);
    };
}
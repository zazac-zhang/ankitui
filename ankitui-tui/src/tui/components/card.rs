//! Card Review Component
//!
//! Handles rendering and interaction for card review sessions with modern styling

use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::AppState;
use crate::tui::core::event_handler::Action;
use crate::tui::core::{state_manager::RenderContext, UIComponent};
use crate::tui::rendering::theme::Theme;
use ankitui_core::{AnswerValidation, CardTemplateEngine, SessionProgress};
use ankitui_core::{Card, CardSide, CardType, ExtendedCardContent, MediaRef, MediaType};
use anyhow::Result;

/// Card review UI component with modern styling
pub struct CardComponent {
    show_answer: bool,
    template_engine: CardTemplateEngine,
    user_input: String,
    selected_option: Option<usize>, // For multiple choice
    card_type: Option<CardType>,
    input_mode: bool, // Whether user is currently typing an answer
    theme: Theme,
    animation_progress: f32,                   // For smooth transitions
    rating_animation: f32,                     // For rating feedback animation
    current_card: Option<Card>,                // Current card being reviewed
    session_progress: Option<SessionProgress>, // Session progress information
}

impl CardComponent {
    pub fn new() -> Self {
        Self::with_theme(Theme::dark())
    }

    pub fn with_theme(theme: Theme) -> Self {
        Self {
            show_answer: false,
            template_engine: CardTemplateEngine::new(),
            user_input: String::new(),
            selected_option: None,
            card_type: None,
            input_mode: false,
            theme,
            animation_progress: 0.0,
            rating_animation: 0.0,
            current_card: None,
            session_progress: None,
        }
    }

    pub fn update_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Update animation progress for smooth transitions
    pub fn update_animations(&mut self, delta: f32) {
        self.animation_progress = (self.animation_progress + delta).min(1.0);
        if self.rating_animation > 0.0 {
            self.rating_animation = (self.rating_animation - delta).max(0.0);
        }
    }

    /// Trigger rating feedback animation
    pub fn trigger_rating_animation(&mut self) {
        self.rating_animation = 1.0;
    }

    /// Get current card (for renderer compatibility)
    pub fn current_card(&self) -> Option<&Card> {
        self.current_card.as_ref()
    }

    /// Set current card
    pub fn set_current_card(&mut self, card: Option<Card>) {
        self.current_card = card;
        self.reset_for_new_card();
    }

    /// Set session progress
    pub fn set_session_progress(&mut self, progress: Option<SessionProgress>) {
        self.session_progress = progress;
    }

    /// Render the card review interface with modern styling
    pub fn render<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut Frame,
        area: Rect,
        current_card: Option<&Card>,
        session_progress: Option<&SessionProgress>,
        error_message: Option<&String>,
        success_message: Option<&String>,
    ) {
        // Clear area first
        f.render_widget(Clear, area);

        // Modern layout with better spacing
        let outer_margin = Margin {
            horizontal: 2,
            vertical: 1,
        };
        let inner_area = area.inner(outer_margin);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Enhanced progress bar
                Constraint::Min(12),   // Card content with more space
                Constraint::Length(4), // Enhanced controls
                Constraint::Length(3), // Messages
            ])
            .split(inner_area);

        // Render enhanced progress bar
        self.render_progress_bar::<B>(f, chunks[0], session_progress);

        // Render modernized card content
        self.render_card_content::<B>(f, chunks[1], current_card);

        // Render enhanced controls with rating feedback
        self.render_controls::<B>(f, chunks[2], current_card);

        // Render status messages
        self.render_messages::<B>(f, chunks[3], error_message, success_message);
    }

    /// Render enhanced progress bar with modern styling
    fn render_progress_bar<B: ratatui::backend::Backend>(
        &self,
        f: &mut Frame,
        area: Rect,
        session_progress: Option<&SessionProgress>,
    ) {
        if let Some(progress) = session_progress {
            let total_cards = progress.new_remaining
                + progress.learning_remaining
                + progress.review_remaining
                + progress.relearning_remaining
                + progress.cards_studied_today as usize;

            let completion_percentage = if total_cards > 0 {
                (progress.cards_studied_today as f64 / total_cards as f64) * 100.0
            } else {
                0.0
            };

            // Choose color based on progress
            let progress_color = if completion_percentage >= 80.0 {
                self.theme.colors.progress_good
            } else if completion_percentage >= 50.0 {
                self.theme.colors.progress_warning
            } else {
                self.theme.colors.progress_danger
            };

            let progress_text = format!(
                "🎯 Progress: {}/{} cards  📈 {:.1}%  ⏱️ {} remaining",
                progress.cards_studied_today,
                total_cards,
                completion_percentage,
                progress.total_remaining
            );

            let progress_gauge = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(self.theme.styles.border_normal)
                        .title_style(self.theme.styles.subtitle)
                        .title(" Session Progress "),
                )
                .gauge_style(
                    Style::default()
                        .fg(progress_color)
                        .bg(self.theme.colors.surface_variant)
                        .add_modifier(Modifier::BOLD),
                )
                .percent(completion_percentage as u16)
                .label(progress_text);

            f.render_widget(progress_gauge, area);
        } else {
            // No active session
            let no_session = Paragraph::new("📝 No active session")
                .style(self.theme.styles.caption)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(self.theme.styles.border_normal)
                        .title(" Session Status "),
                );
            f.render_widget(no_session, area);
        }
    }

    /// Render modernized card content
    fn render_card_content<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut Frame,
        area: Rect,
        current_card: Option<&Card>,
    ) {
        if let Some(card) = current_card {
            // Enhanced card layout with better spacing
            let card_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
                .margin(1)
                .split(area);

            // Render front side with modern styling
            self.render_card_front::<B>(f, card_chunks[0], card);

            // Render back side or hint
            if self.show_answer {
                self.render_card_back::<B>(f, card_chunks[1], card);
            } else {
                self.render_card_hint::<B>(f, card_chunks[1], card);
            }
        } else {
            // No cards available
            let no_cards_content = vec![
                Line::from(Span::styled("📚", self.theme.styles.title)),
                Line::from(""),
                Line::from("No cards available for review"),
                Line::from("Try selecting a different deck or creating new cards"),
            ];

            let no_cards = Paragraph::new(no_cards_content)
                .style(self.theme.styles.caption)
                .alignment(ratatui::layout::Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(self.theme.styles.border_normal)
                        .title(" Card Review "),
                );
            f.render_widget(no_cards, area);
        }
    }

    /// Render card front side with modern styling
    fn render_card_front<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut Frame,
        area: Rect,
        card: &Card,
    ) {
        let theme = self.theme.clone();
        let front_lines = self.render_enhanced_card_content(card, CardSide::Front);

        let front_paragraph = Paragraph::new(front_lines)
            .style(theme.styles.body)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.styles.border_normal)
                    .title_style(theme.styles.subtitle)
                    .title(" 📝 Question "),
            )
            .wrap(Wrap { trim: true })
            .scroll((0, 0)); // Add scrolling support

        f.render_widget(front_paragraph, area);
    }

    /// Render card back side with modern styling
    fn render_card_back<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut Frame,
        area: Rect,
        card: &Card,
    ) {
        let theme = self.theme.clone();
        let back_lines = self.render_enhanced_card_content(card, CardSide::Back);

        let back_paragraph = Paragraph::new(back_lines)
            .style(theme.styles.body)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.styles.border_highlight)
                    .title_style(theme.styles.subtitle)
                    .title(" 💡 Answer "),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(back_paragraph, area);
    }

    /// Render card hint with modern styling
    fn render_card_hint<B: ratatui::backend::Backend>(
        &self,
        f: &mut Frame,
        area: Rect,
        card: &Card,
    ) {
        let hint_text = if self.requires_input() {
            match self.card_type {
                Some(CardType::Input | CardType::Cloze) => {
                    format!("✏️  Type your answer and press ENTER")
                }
                Some(CardType::MultipleChoice) => {
                    format!("🔢 Press 1-9 to select an option")
                }
                _ => format!("👁️  Press SPACE to show answer"),
            }
        } else {
            format!("👁️  Press SPACE to show answer")
        };

        // Add media hint if applicable
        let hint_text = if card.content.media.is_some() {
            format!("{}  •  🎵 M: Open media file", hint_text)
        } else {
            hint_text
        };

        let hint_content = vec![
            Line::from(Span::styled("💭", self.theme.styles.subtitle)),
            Line::from(""),
            Line::from(Span::styled(hint_text, self.theme.styles.caption)),
        ];

        let hint_paragraph = Paragraph::new(hint_content)
            .style(self.theme.styles.caption)
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.styles.border_normal)
                    .title(" Hint "),
            );

        f.render_widget(hint_paragraph, area);
    }

    /// Render enhanced controls with rating feedback
    fn render_controls<B: ratatui::backend::Backend>(
        &self,
        f: &mut Frame,
        area: Rect,
        current_card: Option<&Card>,
    ) {
        let controls_text = if self.show_answer && current_card.is_some() {
            self.build_rating_controls()
        } else {
            self.build_question_controls(current_card)
        };

        // Add animation effect if rating was just selected
        let control_style = if self.rating_animation > 0.5 {
            self.theme.styles.button_active
        } else {
            self.theme.styles.body
        };

        let controls_paragraph =
            Paragraph::new(Line::from(Span::styled(controls_text, control_style)))
                .style(self.theme.styles.body)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(self.theme.styles.border_normal)
                        .title_style(self.theme.styles.subtitle)
                        .title(" ⌨️  Controls "),
                )
                .wrap(Wrap { trim: true });

        f.render_widget(controls_paragraph, area);
    }

    /// Build controls text for rating stage
    fn build_rating_controls(&self) -> String {
        let mut controls = vec!["1️⃣ Again  2️⃣ Hard  3️⃣ Good  4️⃣ Easy".to_string()];

        if self.requires_input() {
            controls.push("↩️  Submit Answer".to_string());
        }

        controls.extend_from_slice(&["⏭️  Skip  🔄 Reset  🚪 Back".to_string()]);

        controls.join("  •  ")
    }

    /// Build controls text for question stage
    fn build_question_controls(&self, current_card: Option<&Card>) -> String {
        let mut controls = vec![];

        if self.requires_input() {
            match self.card_type {
                Some(CardType::Input | CardType::Cloze) => {
                    controls.push("✏️  Type Answer  ↩️  Submit".to_string());
                }
                Some(CardType::MultipleChoice) => {
                    controls.push("🔢  1-9 Select Option".to_string());
                }
                _ => {}
            }
        } else {
            controls.push("👁️  SPACE: Show Answer".to_string());
        }

        controls.extend_from_slice(&["⏭️  Skip  🚪 Exit".to_string()]);

        if let Some(card) = current_card {
            if card.content.media.is_some() {
                controls.push("🎵 Media".to_string());
            }
        }

        controls.join("  •  ")
    }

    /// Render status messages with modern styling
    fn render_messages<B: ratatui::backend::Backend>(
        &self,
        f: &mut Frame,
        area: Rect,
        error_message: Option<&String>,
        success_message: Option<&String>,
    ) {
        let (content, style, icon) = if let Some(error) = error_message {
            (
                format!("❌ {}", error),
                self.theme.styles.status_error,
                "⚠️",
            )
        } else if let Some(success) = success_message {
            (
                format!("✅ {}", success),
                self.theme.styles.status_success,
                "🎉",
            )
        } else {
            (
                "Ready for review • Choose your rating wisely".to_string(),
                self.theme.styles.caption,
                "💡",
            )
        };

        let message_widget = Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ", icon), style),
            Span::styled(content, style),
        ]))
        .style(self.theme.styles.body)
        .block(Block::default().borders(Borders::ALL).border_style(
            match (error_message, success_message) {
                (Some(_), _) => self.theme.styles.border_highlight,
                (None, Some(_)) => self.theme.styles.border_highlight,
                _ => self.theme.styles.border_normal,
            },
        ))
        .wrap(Wrap { trim: true });

        f.render_widget(message_widget, area);
    }

    /// Toggle answer visibility
    pub fn toggle_answer(&mut self) {
        self.show_answer = !self.show_answer;
    }

    /// Show answer
    pub fn show_answer(&mut self) {
        self.show_answer = true;
    }

    /// Hide answer
    pub fn hide_answer(&mut self) {
        self.show_answer = false;
    }

    /// Check if answer is currently shown
    pub fn is_answer_shown(&self) -> bool {
        self.show_answer
    }

    /// Reset for new card
    pub fn reset_for_new_card(&mut self) {
        self.show_answer = false;
        self.user_input.clear();
        self.selected_option = None;
        self.input_mode = false;
        self.card_type = None;
    }

    /// Set card type for the current card
    pub fn set_card_type(&mut self, card_type: CardType) {
        self.card_type = Some(card_type);
    }

    /// Check if current card type requires input
    pub fn requires_input(&self) -> bool {
        matches!(
            self.card_type,
            Some(CardType::Input | CardType::Cloze | CardType::MultipleChoice)
        )
    }

    /// Handle user input for card types that need it
    pub fn handle_input(&mut self, key: char) -> bool {
        if !self.requires_input() || self.show_answer {
            return false;
        }

        match self.card_type {
            Some(CardType::Input) | Some(CardType::Cloze) => {
                match key {
                    '\n' | '\r' => {
                        // Submit answer
                        self.show_answer = true;
                        true
                    }
                    '\x08' | '\x7f' => {
                        // Backspace
                        if !self.user_input.is_empty() {
                            self.user_input.pop();
                        }
                        true
                    }
                    _ if key.is_ascii() && !key.is_control() => {
                        self.user_input.push(key);
                        true
                    }
                    _ => false,
                }
            }
            Some(CardType::MultipleChoice) => {
                match key {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        // Safe to unwrap since we verified the character is a digit
                        let option_num =
                            key.to_digit(10).expect("digit conversion failed") as usize;
                        if option_num > 0 {
                            self.selected_option = Some(option_num - 1);
                            self.show_answer = true;
                        }
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// Get the user's answer
    pub fn get_user_answer(&self) -> Option<String> {
        match self.card_type {
            Some(CardType::Input) | Some(CardType::Cloze) => {
                if !self.user_input.is_empty() {
                    Some(self.user_input.clone())
                } else {
                    None
                }
            }
            Some(CardType::MultipleChoice) => self.selected_option.map(|i| (i + 1).to_string()),
            _ => None,
        }
    }

    /// Validate user answer (for input/cloze/multiple choice cards)
    pub fn validate_answer(&self, extended_content: &ExtendedCardContent) -> AnswerValidation {
        if let Some(user_answer) = self.get_user_answer() {
            self.template_engine
                .validate_answer(extended_content, &user_answer)
        } else {
            AnswerValidation::Error("No answer provided".to_string())
        }
    }

    /// Render enhanced card content with card type support
    fn render_enhanced_card_content(&mut self, card: &Card, side: CardSide) -> Vec<Line> {
        // Try to convert to extended content, fall back to basic rendering
        if let Ok(extended_content) = self.convert_to_extended_content(card) {
            self.card_type = Some(extended_content.card_type);

            if let Ok(rendered) = self.template_engine.render_card(&extended_content, side) {
                let mut lines = Vec::new();

                // Add card type indicator
                lines.push(Line::from(vec![Span::styled(
                    format!(
                        "{} ({:?})",
                        if side == CardSide::Front {
                            "Question"
                        } else {
                            "Answer"
                        },
                        extended_content.card_type
                    ),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(""));

                // Add content
                for line in rendered.content.lines() {
                    lines.push(Line::from(line.to_string()));
                }

                // Add input field for applicable card types
                if side == CardSide::Front && self.requires_input() && !self.show_answer {
                    lines.push(Line::from(""));
                    match extended_content.card_type {
                        CardType::Input | CardType::Cloze => {
                            lines.push(Line::from(vec![
                                Span::styled("Your answer: ", Style::default().fg(Color::Yellow)),
                                Span::styled(&self.user_input, Style::default().fg(Color::White)),
                                Span::styled("_", Style::default().fg(Color::DarkGray)), // Cursor
                            ]));
                        }
                        CardType::MultipleChoice => {
                            if let Some(options) = rendered.multiple_choice_options {
                                lines.push(Line::from("Select an option:"));
                                for (i, option) in options.iter().enumerate() {
                                    let is_selected = self.selected_option == Some(i);
                                    lines.push(Line::from(vec![
                                        Span::styled(
                                            format!("{}. ", i + 1),
                                            Style::default().fg(Color::Cyan),
                                        ),
                                        Span::styled(
                                            option.clone(),
                                            if is_selected {
                                                Style::default()
                                                    .fg(Color::Yellow)
                                                    .add_modifier(Modifier::BOLD)
                                            } else {
                                                Style::default().fg(Color::White)
                                            },
                                        ),
                                    ]));
                                }
                            }
                        }
                        _ => {}
                    }
                }

                // Show validation result if answer is shown
                if side == CardSide::Back && self.requires_input() {
                    if let Some(user_answer) = self.get_user_answer() {
                        let validation = self.validate_answer(&extended_content);
                        lines.push(Line::from(""));
                        match validation {
                            AnswerValidation::Correct => {
                                lines.push(Line::from(vec![
                                    Span::styled("✓ ", Style::default().fg(Color::Green)),
                                    Span::styled(
                                        format!("Your answer ({}) is correct!", user_answer),
                                        Style::default().fg(Color::Green),
                                    ),
                                ]));
                            }
                            AnswerValidation::Incorrect(correct_answer) => {
                                lines.push(Line::from(vec![
                                    Span::styled("✗ ", Style::default().fg(Color::Red)),
                                    Span::styled(
                                        format!("Your answer ({}) is incorrect.", user_answer),
                                        Style::default().fg(Color::Red),
                                    ),
                                ]));
                                lines.push(Line::from(vec![
                                    Span::styled(
                                        "Correct answer: ",
                                        Style::default().fg(Color::Yellow),
                                    ),
                                    Span::styled(
                                        correct_answer.clone(),
                                        Style::default().fg(Color::White),
                                    ),
                                ]));
                            }
                            AnswerValidation::Error(msg) => {
                                lines.push(Line::from(vec![
                                    Span::styled("⚠ ", Style::default().fg(Color::Red)),
                                    Span::styled(
                                        format!("Error validating answer: {}", msg),
                                        Style::default().fg(Color::Red),
                                    ),
                                ]));
                            }
                            AnswerValidation::NotApplicable => {}
                        }
                    }
                }

                return lines;
            }
        }

        // Fallback to basic rendering
        self.card_type = Some(CardType::Basic);
        if side == CardSide::Front {
            self.render_basic_front_content(card)
        } else {
            self.render_basic_back_content(card)
        }
    }

    /// Convert basic card to extended content (for backward compatibility)
    fn convert_to_extended_content(
        &self,
        card: &Card,
    ) -> Result<ExtendedCardContent, Box<dyn std::error::Error>> {
        // For backward compatibility, treat basic cards as Basic type
        let mut custom = std::collections::HashMap::new();
        custom.insert(
            "front".to_string(),
            toml::Value::String(card.content.front.clone()),
        );
        custom.insert(
            "back".to_string(),
            toml::Value::String(card.content.back.clone()),
        );

        Ok(ExtendedCardContent {
            id: card.content.id,
            card_type: CardType::Basic, // Default to basic for backward compatibility
            tags: card.content.tags.clone(),
            media: card.content.media.clone().into_iter().collect(),
            custom,
            created_at: card.content.created_at,
            modified_at: card.content.modified_at,
        })
    }

    /// Render basic front content (fallback)
    fn render_basic_front_content(&self, card: &Card) -> Vec<Line> {
        let mut lines = vec![
            Line::from(Span::styled(
                "Front:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        for line in card.content.front.lines() {
            lines.push(Line::from(line.to_string()));
        }

        if let Some(media_ref) = &card.content.media {
            lines.push(Line::from(""));
            lines.extend(self.render_media_info(media_ref));
        }

        lines
    }

    /// Render basic back content (fallback)
    fn render_basic_back_content(&self, card: &Card) -> Vec<Line> {
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Back:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        for line in card.content.back.lines() {
            lines.push(Line::from(line.to_string()));
        }

        lines
    }

    /// Render media information for a card
    fn render_media_info(&self, media_ref: &MediaRef) -> Vec<Line> {
        let media_icon = match media_ref.media_type {
            MediaType::Audio => "🎵",
            MediaType::Image => "🖼️ ",
            MediaType::Video => "🎬",
        };

        let media_type_text = match media_ref.media_type {
            MediaType::Audio => "Audio",
            MediaType::Image => "Image",
            MediaType::Video => "Video",
        };

        let mut lines = vec![
            Line::from(vec![
                Span::styled(
                    format!("{} Media: ", media_icon),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(media_type_text, Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(" ({})", media_ref.path),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
            Line::from(""),
        ];

        // Add media-specific hints
        let hint = match media_ref.media_type {
            MediaType::Audio => {
                "📢 Audio file available - Press 'M' to play (TUI rendering not supported)"
            }
            MediaType::Image => "🖼️  Image file available - Press 'M' to view external viewer",
            MediaType::Video => "🎬 Video file available - Press 'M' to play external viewer",
        };

        lines.push(Line::from(Span::styled(
            hint,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )));

        lines
    }

    /// Enhanced front content rendering with media support
    fn render_front_content_with_media(&self, card: &Card) -> Vec<Line> {
        let mut lines = vec![
            Line::from(Span::styled(
                "Front:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        // Add front text
        for line in card.content.front.lines() {
            lines.push(Line::from(line.to_string()));
        }

        // Add media info if present
        if let Some(media_ref) = &card.content.media {
            lines.push(Line::from(""));
            lines.extend(self.render_media_info(media_ref));
        }

        lines
    }

    /// Enhanced back content rendering with media support
    fn render_back_content_with_media(&self, card: &Card) -> Vec<Line> {
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Back:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        // Add back text
        for line in card.content.back.lines() {
            lines.push(Line::from(line.to_string()));
        }

        // Note: Media is typically shown on front side, but could also be on back
        // For now, we only show media on front to avoid duplication

        lines
    }

    /// Open media file with external application
    pub fn open_media_file(&self, card: &Card) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(media_ref) = &card.content.media {
            // Construct the full path to the media file
            // Note: This assumes media files are stored in a "media" subdirectory
            // In a real implementation, you'd get this from the MediaManager
            let media_path = std::path::Path::new("media").join(&media_ref.path);

            if !media_path.exists() {
                return Err(format!("Media file not found: {}", media_path.display()).into());
            }

            // Open with system default application
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/C", "start", "", &media_path.to_string_lossy()])
                    .spawn()?;
            }

            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .arg(&media_path)
                    .spawn()?;
            }

            #[cfg(target_os = "linux")]
            {
                // Try xdg-open first (most desktop environments)
                if std::process::Command::new("xdg-open")
                    .arg(&media_path)
                    .spawn()
                    .is_err()
                {
                    // Fallback to common commands
                    match media_ref.media_type {
                        MediaType::Image => {
                            std::process::Command::new("xdg-open")
                                .arg(&media_path)
                                .spawn()?;
                        }
                        MediaType::Audio => {
                            std::process::Command::new("xdg-open")
                                .arg(&media_path)
                                .spawn()?;
                        }
                        MediaType::Video => {
                            std::process::Command::new("xdg-open")
                                .arg(&media_path)
                                .spawn()?;
                        }
                    }
                }
            }

            Ok(())
        } else {
            Err("No media file associated with this card".into())
        }
    }

    /// Check if card has media
    pub fn card_has_media(&self, card: &Card) -> bool {
        card.content.media.is_some()
    }

    /// Render component with UIComponent interface (public wrapper)
    pub fn render_ui(&mut self, frame: &mut ratatui::Frame, context: RenderContext) -> Result<()> {
        <Self as UIComponent>::render(self, frame, context)
    }
}

impl UIComponent for CardComponent {
    fn render(&mut self, frame: &mut ratatui::Frame, _context: RenderContext) -> Result<()> {
        // Use the full frame area
        let area = frame.area();

        // Use the current card stored in the component
        let current_card = self.current_card.clone();
        let session_progress = self.session_progress.clone();

        // Call the existing render method with the actual card data
        self.render::<ratatui::backend::CrosstermBackend<std::io::Stdout>>(
            frame,
            area,
            current_card.as_ref(),     // Use actual current_card as reference
            session_progress.as_ref(), // Use actual session_progress
            None,                      // error_message - TODO: Get from app state
            None,                      // success_message - TODO: Get from app state
        );
        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Cancel => {
                // Return to main menu
                return Ok(Some(AppState::MainMenu));
            }
            Action::Select => {
                // Show answer if not shown
                if !self.show_answer {
                    self.show_answer();
                }
            }
            Action::ShowAnswer => {
                self.show_answer();
            }
            Action::RateAgain => {
                // Will be handled by the main app
                return Ok(None);
            }
            Action::RateHard => {
                // Will be handled by the main app
                return Ok(None);
            }
            Action::RateGood => {
                // Will be handled by the main app
                return Ok(None);
            }
            Action::RateEasy => {
                // Will be handled by the main app
                return Ok(None);
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self) -> Result<()> {
        // Update animations
        if self.animation_progress < 1.0 {
            self.animation_progress = (self.animation_progress + 0.1).min(1.0);
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "card_review"
    }
}

impl Default for CardComponent {
    fn default() -> Self {
        Self::new()
    }
}

//! Dialog Components
//!
//! Common dialog components for confirmations, inputs, etc.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// Confirm exit dialog component
pub struct ConfirmExitDialog {
    selected_option: bool, // false = No, true = Yes
}

impl ConfirmExitDialog {
    pub fn new() -> Self {
        Self {
            selected_option: false, // Default to "No" for safety
        }
    }

    /// Render confirm exit dialog
    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        // Calculate dialog area (centered, smaller than full screen)
        let dialog_width = 40.min(area.width - 4);
        let dialog_height = 8.min(area.height - 4);
        let x = (area.width - dialog_width) / 2;
        let y = (area.height - dialog_height) / 2;
        let dialog_area = Rect::new(x, y, dialog_width, dialog_height);

        // Clear the area and draw border
        f.render_widget(Clear, dialog_area);

        // Dialog layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title and message
                Constraint::Length(3), // Options
            ])
            .margin(1)
            .split(dialog_area);

        // Title and message
        let title = Paragraph::new("Are you sure you want to quit?")
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Options - Use simple paragraphs instead of list for better control
        let options_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(8), // No button
                Constraint::Length(4), // Space
                Constraint::Length(8), // Yes button
            ])
            .margin(1)
            .split(chunks[1]);

        // No button
        let no_style = if !self.selected_option {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
        };
        let no_button = Paragraph::new("  No  ")
            .style(no_style)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(no_button, options_chunks[0]);

        // Yes button
        let yes_style = if self.selected_option {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
        };
        let yes_button = Paragraph::new("  Yes  ")
            .style(yes_style)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(yes_button, options_chunks[2]);
    }

    /// Toggle between Yes and No
    pub fn toggle_selection(&mut self) {
        self.selected_option = !self.selected_option;
    }

    /// Check if Yes is selected
    pub fn is_yes_selected(&self) -> bool {
        self.selected_option
    }

    /// Select Yes
    pub fn select_yes(&mut self) {
        self.selected_option = true;
    }

    /// Select No
    pub fn select_no(&mut self) {
        self.selected_option = false;
    }
}

impl Default for ConfirmExitDialog {
    fn default() -> Self {
        Self::new()
    }
}

/// Message dialog component for displaying information
pub struct MessageDialog {
    title: String,
    message: String,
    message_type: MessageType,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

impl MessageDialog {
    pub fn new(title: String, message: String, message_type: MessageType) -> Self {
        Self {
            title,
            message,
            message_type,
        }
    }

    /// Render message dialog
    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Calculate dialog area
        let dialog_width = 60.min(area.width - 4);
        let dialog_height = 10.min(area.height - 4);
        let x = (area.width - dialog_width) / 2;
        let y = (area.height - dialog_height) / 2;
        let dialog_area = Rect::new(x, y, dialog_width, dialog_height);

        // Clear the area
        f.render_widget(Clear, dialog_area);

        // Set color based on message type
        let color = match self.message_type {
            MessageType::Info => Color::Blue,
            MessageType::Success => Color::Green,
            MessageType::Warning => Color::Yellow,
            MessageType::Error => Color::Red,
        };

        let title_style = Style::default().fg(color).add_modifier(Modifier::BOLD);

        let paragraph = Paragraph::new(self.message.clone())
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.title.clone()),
            );

        f.render_widget(paragraph, dialog_area);
    }
}

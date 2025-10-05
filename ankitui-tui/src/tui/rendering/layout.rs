//! Layout Management

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout management utilities for consistent UI components

/// Common layout presets
pub struct LayoutPresets;

impl LayoutPresets {
    /// Create a centered dialog layout
    pub fn centered_dialog(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
        let width = (area.width * width_percent) / 100;
        let height = (area.height * height_percent) / 100;

        let x = area.x + (area.width - width) / 2;
        let y = area.y + (area.height - height) / 2;

        Rect::new(x, y, width, height)
    }

    /// Create a main layout with status bar
    pub fn main_with_status(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Main content
                Constraint::Length(1), // Status bar
            ])
            .split(area)
            .to_vec()
    }

    /// Create a main layout with header and status bar
    pub fn main_with_header_and_status(area: Rect, header_height: u16) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(header_height), // Header
                Constraint::Min(0),                // Main content
                Constraint::Length(1),             // Status bar
            ])
            .split(area)
            .to_vec()
    }

    /// Create a three-column layout
    pub fn three_column(area: Rect, left_width: u16, right_width: u16) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(left_width),        // Left panel
                Constraint::Min(0),                    // Center content
                Constraint::Length(right_width),       // Right panel
            ])
            .split(area)
            .to_vec()
    }

    /// Create a two-column layout
    pub fn two_column(area: Rect, left_width: u16) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(left_width), // Left panel
                Constraint::Min(0),             // Right content
            ])
            .split(area)
            .to_vec()
    }

    /// Create a split layout for card review (question/answer)
    pub fn card_review(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Card type/status
                Constraint::Min(0),    // Card content
                Constraint::Length(3), // Rating buttons
            ])
            .split(area)
            .to_vec()
    }

    /// Create a settings layout with sidebar
    pub fn settings_layout(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(20), // Settings sidebar
                Constraint::Min(0),     // Settings content
            ])
            .split(area)
            .to_vec()
    }

    /// Create a dashboard layout
    pub fn dashboard(area: Rect) -> Vec<Rect> {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Header/stats
                Constraint::Min(0),     // Main content
                Constraint::Length(1),  // Status bar
            ])
            .split(area);

        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left stats
                Constraint::Percentage(50), // Right stats
            ])
            .split(main_chunks[0]);

        vec![header_chunks[0], header_chunks[1], main_chunks[1], main_chunks[2]]
    }

    /// Create a modal layout overlay
    pub fn modal_overlay(area: Rect) -> Vec<Rect> {
        // Create semi-transparent background
        let background = area;

        // Create modal dialog
        let modal = Self::centered_dialog(area, 60, 60);

        vec![background, modal]
    }

    /// Create a help dialog layout
    pub fn help_dialog(area: Rect) -> Vec<Rect> {
        let modal = Self::centered_dialog(area, 80, 80);

        let modal_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Help title
                Constraint::Min(0),    // Help content
                Constraint::Length(1), // Close prompt
            ])
            .split(modal);

        vec![modal_chunks[0], modal_chunks[1], modal_chunks[2]]
    }

    /// Create a progress bar layout
    pub fn progress_with_info(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Progress info
                Constraint::Length(1), // Progress bar
                Constraint::Min(0),    // Details/status
            ])
            .split(area)
            .to_vec()
    }

    /// Create a search results layout
    pub fn search_results(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search input
                Constraint::Min(0),    // Results list
                Constraint::Length(1), // Results count/status
            ])
            .split(area)
            .to_vec()
    }

    /// Create a statistics dashboard layout
    pub fn stats_dashboard(area: Rect) -> Vec<Rect> {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),  // Overview cards
                Constraint::Min(0),     // Charts area
                Constraint::Length(1),  // Status bar
            ])
            .split(area);

        let overview_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25), // Stats 1
                Constraint::Percentage(25), // Stats 2
                Constraint::Percentage(25), // Stats 3
                Constraint::Percentage(25), // Stats 4
            ])
            .split(main_chunks[0]);

        vec![overview_chunks[0], overview_chunks[1], overview_chunks[2], overview_chunks[3], main_chunks[1], main_chunks[2]]
    }
}

/// Layout builder for complex layouts
pub struct LayoutBuilder {
    direction: Direction,
    constraints: Vec<Constraint>,
}

impl LayoutBuilder {
    /// Create new layout builder
    pub fn new() -> Self {
        Self {
            direction: Direction::Vertical,
            constraints: Vec::new(),
        }
    }

    /// Set direction
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Add length constraint
    pub fn length(mut self, length: u16) -> Self {
        self.constraints.push(Constraint::Length(length));
        self
    }

    /// Add min constraint
    pub fn min(mut self, min: u16) -> Self {
        self.constraints.push(Constraint::Min(min));
        self
    }

    /// Add max constraint
    pub fn max(mut self, max: u16) -> Self {
        self.constraints.push(Constraint::Max(max));
        self
    }

    /// Add percentage constraint
    pub fn percentage(mut self, percentage: u16) -> Self {
        self.constraints.push(Constraint::Percentage(percentage));
        self
    }

    /// Add ratio constraint
    pub fn ratio(mut self, numerator: u32, denominator: u32) -> Self {
        self.constraints.push(Constraint::Ratio(numerator, denominator));
        self
    }

    /// Build layout
    pub fn build(self, area: Rect) -> Vec<Rect> {
        if self.constraints.is_empty() {
            vec![area]
        } else {
            Layout::default()
                .direction(self.direction)
                .constraints(self.constraints)
                .split(area)
                .to_vec()
        }
    }
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
}
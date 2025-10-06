//! Layout management for the TUI application

use ratatui::layout::Rect;

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

/// Layout constraints
#[derive(Debug, Clone)]
pub struct LayoutConstraints {
    pub min_width: u16,
    pub min_height: u16,
    pub max_width: Option<u16>,
    pub max_height: Option<u16>,
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_width: 0,
            min_height: 0,
            max_width: None,
            max_height: None,
        }
    }
}
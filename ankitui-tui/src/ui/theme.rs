//! Theme system for the TUI application
//!
//! Provides color schemes and styling presets for different visual themes

use ratatui::style::{Color, Style, Modifier};

/// Theme alias for backward compatibility
pub type Theme = ThemeType;

/// Available themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Default,
    Dark,
    Light,
}

impl ThemeType {
    /// Parse theme from string
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dark" => ThemeType::Dark,
            "light" => ThemeType::Light,
            _ => ThemeType::Default,
        }
    }

    /// Get the name of this theme
    pub fn name(&self) -> &'static str {
        match self {
            ThemeType::Default => "default",
            ThemeType::Dark => "dark",
            ThemeType::Light => "light",
        }
    }
}

/// Color scheme for a theme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub header: Style,
    pub selected: Style,
    pub normal: Style,
    pub warning: Style,
    pub success: Style,
    pub error: Style,
    pub info: Style,
    pub border: Style,
}

impl ColorScheme {
    /// Get the default theme color scheme
    pub fn default_theme() -> Self {
        Self {
            header: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            selected: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            normal: Style::default(),
            warning: Style::default().fg(Color::Yellow),
            success: Style::default().fg(Color::Green),
            error: Style::default().fg(Color::Red),
            info: Style::default().fg(Color::Gray),
            border: Style::default(),
        }
    }

    /// Get the dark theme color scheme
    pub fn dark_theme() -> Self {
        Self {
            header: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            selected: Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD),
            normal: Style::default().fg(Color::LightGray),
            warning: Style::default().fg(Color::Yellow),
            success: Style::default().fg(Color::Green),
            error: Style::default().fg(Color::Red),
            info: Style::default().fg(Color::DarkGray),
            border: Style::default().fg(Color::DarkGray),
        }
    }

    /// Get the light theme color scheme
    pub fn light_theme() -> Self {
        Self {
            header: Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            selected: Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            normal: Style::default().fg(Color::Black),
            warning: Style::default().fg(Color::DarkYellow),
            success: Style::default().fg(Color::DarkGreen),
            error: Style::default().fg(Color::DarkRed),
            info: Style::default().fg(Color::Gray),
            border: Style::default().fg(Color::Gray),
        }
    }

    /// Get color scheme for the given theme type
    pub fn for_theme(theme: ThemeType) -> Self {
        match theme {
            ThemeType::Default => Self::default_theme(),
            ThemeType::Dark => Self::dark_theme(),
            ThemeType::Light => Self::light_theme(),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::default_theme()
    }
}

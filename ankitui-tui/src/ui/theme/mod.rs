//! Theme system for the TUI application

use ratatui::style::{Color, Style};

/// Theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub styles: StylePalette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            colors: ColorPalette::default(),
            styles: StylePalette::default(),
        }
    }
}

/// Color palette
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub foreground: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Gray,
            background: Color::Black,
            foreground: Color::White,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
        }
    }
}

/// Style palette
#[derive(Debug, Clone)]
pub struct StylePalette {
    pub normal: Style,
    pub focused: Style,
    pub selected: Style,
    pub disabled: Style,
    pub header: Style,
    pub footer: Style,
}

impl Default for StylePalette {
    fn default() -> Self {
        Self {
            normal: Style::default(),
            focused: Style::default().add_modifier(ratatui::style::Modifier::REVERSED),
            selected: Style::default().add_modifier(ratatui::style::Modifier::REVERSED),
            disabled: Style::default().fg(Color::DarkGray),
            header: Style::default().add_modifier(ratatui::style::Modifier::BOLD),
            footer: Style::default(),
        }
    }
}
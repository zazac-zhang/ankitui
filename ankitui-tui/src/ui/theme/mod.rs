//! Theme system for the TUI application

use ratatui::style::{Color, Style, Modifier};

/// Available theme types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Default,
    Dark,
    Light,
}

impl ThemeType {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dark" => ThemeType::Dark,
            "light" => ThemeType::Light,
            _ => ThemeType::Default,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            ThemeType::Default => "default",
            ThemeType::Dark => "dark",
            ThemeType::Light => "light",
        }
    }
}

/// Color scheme for rendering
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
    pub fn dark_theme() -> Self {
        Self {
            header: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            selected: Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD),
            normal: Style::default().fg(Color::DarkGray),
            warning: Style::default().fg(Color::Yellow),
            success: Style::default().fg(Color::Green),
            error: Style::default().fg(Color::Red),
            info: Style::default().fg(Color::DarkGray),
            border: Style::default().fg(Color::DarkGray),
        }
    }
    pub fn light_theme() -> Self {
        Self {
            header: Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            selected: Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            normal: Style::default().fg(Color::Black),
            warning: Style::default().fg(Color::Yellow),
            success: Style::default().fg(Color::Green),
            error: Style::default().fg(Color::Red),
            info: Style::default().fg(Color::Gray),
            border: Style::default().fg(Color::Gray),
        }
    }
    pub fn for_theme(theme: ThemeType) -> Self {
        match theme {
            ThemeType::Default => Self::default_theme(),
            ThemeType::Dark => Self::dark_theme(),
            ThemeType::Light => Self::light_theme(),
        }
    }
}

/// Theme configuration (alias for backward compatibility)
pub type Theme = ThemeConfig;

/// Theme configuration struct
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ColorPalette,
    pub styles: StylePalette,
}

impl Default for ThemeConfig {
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
            focused: Style::default().add_modifier(Modifier::REVERSED),
            selected: Style::default().add_modifier(Modifier::REVERSED),
            disabled: Style::default().fg(Color::DarkGray),
            header: Style::default().add_modifier(Modifier::BOLD),
            footer: Style::default(),
        }
    }
}

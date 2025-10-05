//! Modern Theme System
//!
//! Clean theme management with customizable styles

use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Color palette for themes
#[derive(Debug, Clone)]
pub struct ColorPalette {
    /// Primary color for highlights and accents
    pub primary: Color,
    /// Secondary color for secondary elements
    pub secondary: Color,
    /// Background color
    pub background: Color,
    /// Text color
    pub text: Color,
    /// Border color
    pub border: Color,
    /// Success color (green variations)
    pub success: Color,
    /// Warning color (yellow variations)
    pub warning: Color,
    /// Error color (red variations)
    pub error: Color,
    /// Info color (blue variations)
    pub info: Color,
    /// Progress good color (green)
    pub progress_good: Color,
    /// Progress warning color (yellow)
    pub progress_warning: Color,
    /// Progress danger color (red)
    pub progress_danger: Color,
    /// Surface color
    pub surface: Color,
    /// Surface variant color
    pub surface_variant: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            background: Color::Black,
            text: Color::White,
            border: Color::Gray,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            progress_good: Color::Green,
            progress_warning: Color::Yellow,
            progress_danger: Color::Red,
            surface: Color::DarkGray,
            surface_variant: Color::Blue,
        }
    }
}

/// Style palette for different UI elements
#[derive(Debug, Clone)]
pub struct StylePalette {
    /// Default text style
    pub default: Style,
    /// Primary accent style
    pub primary: Style,
    /// Secondary accent style
    pub secondary: Style,
    /// Success style
    pub success: Style,
    /// Warning style
    pub warning: Style,
    /// Error style
    pub error: Style,
    /// Info style
    pub info: Style,
    /// Disabled style
    pub disabled: Style,
    /// Focused style
    pub focused: Style,
    /// Selected style
    pub selected: Style,
    /// Highlight style
    pub highlight: Style,

    // Additional styles for interaction feedback system
    /// Body text style
    pub body: Style,
    /// Caption/subtitle style
    pub caption: Style,
    /// Title style
    pub title: Style,
    /// Subtitle style
    pub subtitle: Style,
    /// Status success style
    pub status_success: Style,
    /// Status warning style
    pub status_warning: Style,
    /// Status error style
    pub status_error: Style,
    /// Status info style
    pub status_info: Style,
    /// Border normal style
    pub border_normal: Style,
    /// Border focused style
    pub border_focused: Style,
    /// Border highlight style
    pub border_highlight: Style,
    /// Button active style
    pub button_active: Style,
    /// Button primary style
    pub button_primary: Style,
}

impl StylePalette {
    /// Create style palette from color palette
    pub fn from_colors(colors: &ColorPalette) -> Self {
        Self {
            default: Style::default().fg(colors.text).bg(colors.background),
            primary: Style::default()
                .fg(colors.primary)
                .add_modifier(Modifier::BOLD),
            secondary: Style::default()
                .fg(colors.secondary)
                .add_modifier(Modifier::ITALIC),
            success: Style::default()
                .fg(colors.success)
                .add_modifier(Modifier::BOLD),
            warning: Style::default()
                .fg(colors.warning)
                .add_modifier(Modifier::BOLD),
            error: Style::default()
                .fg(colors.error)
                .add_modifier(Modifier::BOLD),
            info: Style::default()
                .fg(colors.info)
                .add_modifier(Modifier::BOLD),
            disabled: Style::default()
                .fg(colors.border)
                .add_modifier(Modifier::DIM),
            focused: Style::default()
                .fg(colors.primary)
                .bg(colors.secondary)
                .add_modifier(Modifier::REVERSED),
            selected: Style::default()
                .fg(colors.background)
                .bg(colors.primary)
                .add_modifier(Modifier::BOLD),
            highlight: Style::default()
                .fg(colors.background)
                .bg(colors.secondary)
                .add_modifier(Modifier::BOLD),

            // Additional styles for interaction feedback system
            body: Style::default().fg(colors.text).bg(colors.background),
            caption: Style::default()
                .fg(colors.secondary)
                .add_modifier(Modifier::DIM),
            title: Style::default()
                .fg(colors.primary)
                .add_modifier(Modifier::BOLD),
            subtitle: Style::default()
                .fg(colors.secondary)
                .add_modifier(Modifier::ITALIC),
            status_success: Style::default()
                .fg(colors.success)
                .add_modifier(Modifier::BOLD),
            status_warning: Style::default()
                .fg(colors.warning)
                .add_modifier(Modifier::BOLD),
            status_error: Style::default()
                .fg(colors.error)
                .add_modifier(Modifier::BOLD),
            status_info: Style::default()
                .fg(colors.info)
                .add_modifier(Modifier::BOLD),
            border_normal: Style::default()
                .fg(colors.border),
            border_focused: Style::default()
                .fg(colors.primary)
                .add_modifier(Modifier::BOLD),
            border_highlight: Style::default()
                .fg(colors.secondary)
                .add_modifier(Modifier::BOLD),
            button_active: Style::default()
                .fg(colors.background)
                .bg(colors.primary)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
            button_primary: Style::default()
                .fg(colors.background)
                .bg(colors.primary)
                .add_modifier(Modifier::BOLD),
        }
    }
}

/// Complete theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Color palette
    pub colors: ColorPalette,
    /// Style palette
    pub styles: StylePalette,
    /// Custom styles for specific components
    pub custom_styles: HashMap<String, Style>,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: &str, colors: ColorPalette) -> Self {
        let styles = StylePalette::from_colors(&colors);
        Self {
            name: name.to_string(),
            colors,
            styles,
            custom_styles: HashMap::new(),
        }
    }

    /// Get default style
    pub fn default_style(&self) -> Style {
        self.styles.default
    }

    /// Get primary accent style
    pub fn primary_style(&self) -> Style {
        self.styles.primary
    }

    /// Get secondary accent style
    pub fn secondary_style(&self) -> Style {
        self.styles.secondary
    }

    /// Get success style
    pub fn success_style(&self) -> Style {
        self.styles.success
    }

    /// Get warning style
    pub fn warning_style(&self) -> Style {
        self.styles.warning
    }

    /// Get error style
    pub fn error_style(&self) -> Style {
        self.styles.error
    }

    /// Get info style
    pub fn info_style(&self) -> Style {
        self.styles.info
    }

    /// Get focused style
    pub fn focused_style(&self) -> Style {
        self.styles.focused
    }

    /// Get selected style
    pub fn selected_style(&self) -> Style {
        self.styles.selected
    }

    /// Get highlight style
    pub fn highlight_style(&self) -> Style {
        self.styles.highlight
    }

    /// Get disabled style
    pub fn disabled_style(&self) -> Style {
        self.styles.disabled
    }

    /// Get primary color
    pub fn primary_color(&self) -> Color {
        self.colors.primary
    }

    /// Get secondary color
    pub fn secondary_color(&self) -> Color {
        self.colors.secondary
    }

    /// Get background color
    pub fn background_color(&self) -> Color {
        self.colors.background
    }

    /// Get text color
    pub fn text_color(&self) -> Color {
        self.colors.text
    }

    /// Get border color
    pub fn border_color(&self) -> Color {
        self.colors.border
    }

    /// Get custom style by name
    pub fn custom_style(&self, name: &str) -> Option<&Style> {
        self.custom_styles.get(name)
    }

    /// Add custom style
    pub fn add_custom_style(&mut self, name: String, style: Style) {
        self.custom_styles.insert(name, style);
    }

    /// Check if this is a dark theme
    pub fn is_dark(&self) -> bool {
        matches!(self.colors.background, Color::Black | Color::DarkGray)
    }

    /// Check if this is a light theme
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }
}

/// Predefined themes
impl Theme {
    /// Dark theme (default)
    pub fn dark() -> Self {
        let colors = ColorPalette {
            primary: Color::Cyan,
            secondary: Color::Blue,
            background: Color::Black,
            text: Color::White,
            border: Color::Gray,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            progress_good: Color::Green,
            progress_warning: Color::Yellow,
            progress_danger: Color::Red,
            surface: Color::DarkGray,
            surface_variant: Color::Blue,
        };
        Self::new("Dark", colors)
    }

    /// Light theme
    pub fn light() -> Self {
        let colors = ColorPalette {
            primary: Color::Blue,
            secondary: Color::Cyan,
            background: Color::White,
            text: Color::Black,
            border: Color::Gray,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            progress_good: Color::Green,
            progress_warning: Color::Yellow,
            progress_danger: Color::Red,
            surface: Color::Gray,
            surface_variant: Color::Cyan,
        };
        Self::new("Light", colors)
    }

    /// Ocean theme
    pub fn ocean() -> Self {
        let colors = ColorPalette {
            primary: Color::Cyan,
            secondary: Color::LightBlue,
            background: Color::Blue,
            text: Color::White,
            border: Color::Blue,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::LightCyan,
            progress_good: Color::Green,
            progress_warning: Color::Yellow,
            progress_danger: Color::Red,
            surface: Color::Blue,
            surface_variant: Color::LightBlue,
        };
        Self::new("Ocean", colors)
    }

    /// Forest theme
    pub fn forest() -> Self {
        let colors = ColorPalette {
            primary: Color::Green,
            secondary: Color::LightGreen,
            background: Color::Green,
            text: Color::White,
            border: Color::Green,
            success: Color::LightGreen,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            progress_good: Color::LightGreen,
            progress_warning: Color::Yellow,
            progress_danger: Color::Red,
            surface: Color::Green,
            surface_variant: Color::LightGreen,
        };
        Self::new("Forest", colors)
    }

    /// Sunset theme
    pub fn sunset() -> Self {
        let colors = ColorPalette {
            primary: Color::Magenta,
            secondary: Color::LightRed,
            background: Color::Magenta,
            text: Color::White,
            border: Color::Red,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            progress_good: Color::Green,
            progress_warning: Color::Yellow,
            progress_danger: Color::Red,
            surface: Color::Magenta,
            surface_variant: Color::LightRed,
        };
        Self::new("Sunset", colors)
    }

    /// Monochrome theme
    pub fn monochrome() -> Self {
        let colors = ColorPalette {
            primary: Color::White,
            secondary: Color::Gray,
            background: Color::Black,
            text: Color::White,
            border: Color::Gray,
            success: Color::Gray,
            warning: Color::Gray,
            error: Color::Gray,
            info: Color::White,
            progress_good: Color::Gray,
            progress_warning: Color::Gray,
            progress_danger: Color::Gray,
            surface: Color::DarkGray,
            surface_variant: Color::Gray,
        };
        Self::new("Monochrome", colors)
    }
}

/// Theme manager for handling multiple themes
pub struct ThemeManager {
    /// Current theme
    current_theme: Theme,
    /// Available themes
    available_themes: HashMap<String, Theme>,
    /// Theme history for undo/redo
    theme_history: Vec<Theme>,
    max_history_size: usize,
}

impl ThemeManager {
    /// Create new theme manager with default theme
    pub fn new() -> Self {
        let current_theme = Theme::dark();
        Self::with_theme(current_theme)
    }

    /// Create theme manager with specific theme
    pub fn with_theme(theme: Theme) -> Self {
        let mut available_themes = HashMap::new();

        // Add all predefined themes
        available_themes.insert("dark".to_string(), Theme::dark());
        available_themes.insert("light".to_string(), Theme::light());
        available_themes.insert("ocean".to_string(), Theme::ocean());
        available_themes.insert("forest".to_string(), Theme::forest());
        available_themes.insert("sunset".to_string(), Theme::sunset());
        available_themes.insert("monochrome".to_string(), Theme::monochrome());

        Self {
            current_theme: theme.clone(),
            available_themes,
            theme_history: vec![theme],
            max_history_size: 10,
        }
    }

    /// Get current theme
    pub fn current_theme(&self) -> &Theme {
        &self.current_theme
    }

    /// Switch to a different theme by name
    pub fn switch_theme(&mut self, theme_name: &str) -> Result<(), String> {
        if let Some(theme) = self.available_themes.get(theme_name) {
            self.set_theme(theme.clone());
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name))
        }
    }

    /// Set current theme
    pub fn set_theme(&mut self, theme: Theme) {
        // Add current theme to history
        self.theme_history.push(self.current_theme.clone());
        if self.theme_history.len() > self.max_history_size {
            self.theme_history.remove(0);
        }

        self.current_theme = theme;
    }

    /// Add custom theme
    pub fn add_theme(&mut self, name: String, theme: Theme) {
        self.available_themes.insert(name, theme);
    }

    /// Get all available theme names
    pub fn available_themes(&self) -> Vec<String> {
        self.available_themes.keys().cloned().collect()
    }

    /// Check if theme exists
    pub fn has_theme(&self, name: &str) -> bool {
        self.available_themes.contains_key(name)
    }

    /// Undo last theme change
    pub fn undo_theme_change(&mut self) -> bool {
        if self.theme_history.len() > 1 {
            self.theme_history.pop(); // Remove current theme
            if let Some(previous_theme) = self.theme_history.last() {
                self.current_theme = previous_theme.clone();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get theme history
    pub fn theme_history(&self) -> &[Theme] {
        &self.theme_history
    }

    /// Create custom theme from colors
    pub fn create_custom_theme(&self, name: &str, colors: ColorPalette) -> Theme {
        Theme::new(name, colors)
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
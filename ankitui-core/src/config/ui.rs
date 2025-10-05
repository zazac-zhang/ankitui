//! UI Configuration Module
//!
//! Contains user interface and display settings

use serde::{Deserialize, Serialize};

/// UI configuration settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiConfig {
    /// Terminal color theme
    pub theme: String,

    /// Enable mouse support
    pub mouse_support: bool,

    /// Show progress indicators
    pub show_progress: bool,

    /// Show card counter during review
    pub show_card_counter: bool,

    /// Animation speed in milliseconds
    pub animation_speed: u64,

    /// Terminal refresh rate in Hz
    pub refresh_rate: u16,

    /// Color scheme
    pub color_scheme: ColorScheme,

    /// Font settings (for compatible terminals)
    pub font_settings: FontSettings,

    /// Layout settings
    pub layout: LayoutConfig,

    /// Display options
    pub display: DisplayOptions,

    /// Animation settings
    pub animations: AnimationConfig,

    /// Theme settings
    pub themes: ThemeSettings,

    /// Accessibility settings
    pub accessibility: AccessibilitySettings,
}

/// Color scheme configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorScheme {
    /// Primary color
    pub primary: String,

    /// Secondary color
    pub secondary: String,

    /// Success color
    pub success: String,

    /// Warning color
    pub warning: String,

    /// Error color
    pub error: String,

    /// Background color
    pub background: String,

    /// Text color
    pub text: String,

    /// Border color
    pub border: String,

    /// Highlight color
    pub highlight: String,

    /// Muted color
    pub muted: String,
}

/// Font settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontSettings {
    /// Font family
    pub family: String,

    /// Font size
    pub size: u8,

    /// Bold font
    pub bold: bool,

    /// Italic font
    pub italic: bool,

    /// Underline
    pub underline: bool,

    /// Line height
    pub line_height: f32,

    /// Letter spacing
    pub letter_spacing: f32,
}

/// Layout configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutConfig {
    /// Main layout style
    pub main_layout: String,

    /// Side panel width
    pub side_panel_width: u16,

    /// Header height
    pub header_height: u16,

    /// Footer height
    pub footer_height: u16,

    /// Border style
    pub border_style: String,

    /// Padding size
    pub padding: u16,

    /// Margin size
    pub margin: u16,

    /// Responsive design
    pub responsive: bool,

    /// Breakpoints for responsive layout
    pub breakpoints: Breakpoints,
}

/// Responsive breakpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Breakpoints {
    /// Small screen width
    pub small: u16,

    /// Medium screen width
    pub medium: u16,

    /// Large screen width
    pub large: u16,

    /// Extra large screen width
    pub extra_large: u16,
}

/// Display options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayOptions {
    /// Show toolbar
    pub show_toolbar: bool,

    /// Show status bar
    pub show_status_bar: bool,

    /// Show help text
    pub show_help: bool,

    /// Show tooltips
    pub show_tooltips: bool,

    /// Show shortcuts bar
    pub show_shortcuts_bar: bool,

    /// Show progress bar
    pub show_progress_bar: bool,

    /// Show statistics
    pub show_statistics: bool,

    /// Show clock
    pub show_clock: bool,

    /// Show battery indicator
    pub show_battery: bool,

    /// Show network status
    pub show_network_status: bool,
}

/// Animation configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// Enable animations
    pub enabled: bool,

    /// Animation speed multiplier
    pub speed_multiplier: f32,

    /// Enable fade animations
    pub fade_animations: bool,

    /// Enable slide animations
    pub slide_animations: bool,

    /// Enable bounce animations
    pub bounce_animations: bool,

    /// Animation duration in milliseconds
    pub default_duration: u64,

    /// Smooth scrolling
    pub smooth_scrolling: bool,

    /// Scrolling speed
    pub scroll_speed: u8,
}

/// Theme settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeSettings {
    /// Current theme name
    pub current_theme: String,

    /// Available themes
    pub available_themes: Vec<String>,

    /// Custom theme path
    pub custom_theme_path: Option<String>,

    /// Auto-switch themes
    pub auto_switch: bool,

    /// Light theme name
    pub light_theme: String,

    /// Dark theme name
    pub dark_theme: String,

    /// Theme switching schedule
    pub theme_schedule: ThemeSchedule,
}

/// Theme switching schedule
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeSchedule {
    /// Enable scheduled theme switching
    pub enabled: bool,

    /// Light theme start time (hour in 24h format)
    pub light_start: u8,

    /// Dark theme start time (hour in 24h format)
    pub dark_start: u8,
}

/// Accessibility settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// High contrast mode
    pub high_contrast: bool,

    /// Large text mode
    pub large_text: bool,

    /// Reduced motion
    pub reduced_motion: bool,

    /// Screen reader support
    pub screen_reader: bool,

    /// Keyboard navigation
    pub keyboard_navigation: bool,

    /// Focus indicators
    pub focus_indicators: bool,

    /// Font size multiplier
    pub font_size_multiplier: f32,

    /// Color blind mode
    pub color_blind_mode: String,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            mouse_support: false,
            show_progress: true,
            show_card_counter: true,
            animation_speed: 50,
            refresh_rate: 60,
            color_scheme: ColorScheme::default(),
            font_settings: FontSettings::default(),
            layout: LayoutConfig::default(),
            display: DisplayOptions::default(),
            animations: AnimationConfig::default(),
            themes: ThemeSettings::default(),
            accessibility: AccessibilitySettings::default(),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            primary: "blue".to_string(),
            secondary: "gray".to_string(),
            success: "green".to_string(),
            warning: "yellow".to_string(),
            error: "red".to_string(),
            background: "black".to_string(),
            text: "white".to_string(),
            border: "gray".to_string(),
            highlight: "cyan".to_string(),
            muted: "darkgray".to_string(),
        }
    }
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            family: "monospace".to_string(),
            size: 12,
            bold: false,
            italic: false,
            underline: false,
            line_height: 1.2,
            letter_spacing: 0.0,
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            main_layout: "horizontal".to_string(),
            side_panel_width: 30,
            header_height: 3,
            footer_height: 2,
            border_style: "single".to_string(),
            padding: 1,
            margin: 0,
            responsive: false,
            breakpoints: Breakpoints::default(),
        }
    }
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            small: 40,
            medium: 80,
            large: 120,
            extra_large: 160,
        }
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_toolbar: true,
            show_status_bar: true,
            show_help: true,
            show_tooltips: true,
            show_shortcuts_bar: true,
            show_progress_bar: true,
            show_statistics: true,
            show_clock: false,
            show_battery: false,
            show_network_status: false,
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            speed_multiplier: 1.0,
            fade_animations: true,
            slide_animations: true,
            bounce_animations: false,
            default_duration: 200,
            smooth_scrolling: true,
            scroll_speed: 5,
        }
    }
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            current_theme: "default".to_string(),
            available_themes: vec![
                "default".to_string(),
                "dark".to_string(),
                "light".to_string(),
            ],
            custom_theme_path: None,
            auto_switch: false,
            light_theme: "light".to_string(),
            dark_theme: "dark".to_string(),
            theme_schedule: ThemeSchedule::default(),
        }
    }
}

impl Default for ThemeSchedule {
    fn default() -> Self {
        Self {
            enabled: false,
            light_start: 8,
            dark_start: 20,
        }
    }
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            high_contrast: false,
            large_text: false,
            reduced_motion: false,
            screen_reader: false,
            keyboard_navigation: true,
            focus_indicators: true,
            font_size_multiplier: 1.0,
            color_blind_mode: "none".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_config_default() {
        let config = UiConfig::default();
        assert_eq!(config.theme, "default");
        assert_eq!(config.mouse_support, false);
        assert_eq!(config.show_progress, true);
        assert_eq!(config.refresh_rate, 60);
    }

    #[test]
    fn test_color_scheme() {
        let scheme = ColorScheme {
            primary: "blue".to_string(),
            secondary: "gray".to_string(),
            success: "green".to_string(),
            warning: "yellow".to_string(),
            error: "red".to_string(),
            background: "black".to_string(),
            text: "white".to_string(),
            border: "gray".to_string(),
            highlight: "cyan".to_string(),
            muted: "darkgray".to_string(),
        };

        assert_eq!(scheme.primary, "blue");
        assert_eq!(scheme.error, "red");
        assert_eq!(scheme.success, "green");
    }

    #[test]
    fn test_layout_config() {
        let layout = LayoutConfig {
            responsive: true,
            side_panel_width: 40,
            main_layout: "vertical".to_string(),
            ..Default::default()
        };

        assert!(layout.responsive);
        assert_eq!(layout.side_panel_width, 40);
        assert_eq!(layout.main_layout, "vertical");
    }

    #[test]
    fn test_accessibility_settings() {
        let mut accessibility = AccessibilitySettings::default();
        accessibility.high_contrast = true;
        accessibility.large_text = true;
        accessibility.font_size_multiplier = 1.5;

        assert!(accessibility.high_contrast);
        assert!(accessibility.large_text);
        assert_eq!(accessibility.font_size_multiplier, 1.5);
    }

    #[test]
    fn test_animation_config() {
        let animations = AnimationConfig {
            enabled: false,
            speed_multiplier: 0.5,
            ..Default::default()
        };

        assert!(!animations.enabled);
        assert_eq!(animations.speed_multiplier, 0.5);
    }
}

//! TUI Configuration Manager
//!
//! Centralized configuration management for the TUI layer

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// TUI Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TUIConfig {
    /// Theme settings
    pub theme: ThemeConfig,
    /// Display settings
    pub display: DisplayConfig,
    /// Keyboard shortcuts
    pub shortcuts: ShortcutConfig,
    /// Performance settings
    pub performance: PerformanceConfig,
    /// Accessibility settings
    pub accessibility: AccessibilityConfig,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Current theme name
    pub current_theme: String,
    /// Custom color overrides
    pub custom_colors: HashMap<String, String>,
    /// Animation speed (0.0 - 1.0)
    pub animation_speed: f32,
    /// Enable visual effects
    pub enable_effects: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            current_theme: "dark".to_string(),
            custom_colors: HashMap::new(),
            animation_speed: 0.5,
            enable_effects: true,
        }
    }
}

/// Display configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Terminal size constraints
    pub min_width: u16,
    pub min_height: u16,
    /// Border style
    pub border_style: String,
    /// Show card counter
    pub show_card_counter: bool,
    /// Show progress bar
    pub show_progress_bar: bool,
    /// Show study streak
    pub show_study_streak: bool,
    /// Refresh rate in milliseconds
    pub refresh_rate_ms: u64,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            min_width: 80,
            min_height: 24,
            border_style: "simple".to_string(),
            show_card_counter: true,
            show_progress_bar: true,
            show_study_streak: true,
            refresh_rate_ms: 250,
        }
    }
}

/// Keyboard shortcuts configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    /// Card review shortcuts
    pub card_review: CardReviewShortcuts,
    /// Navigation shortcuts
    pub navigation: NavigationShortcuts,
    /// Global shortcuts
    pub global: GlobalShortcuts,
}

/// Card review shortcuts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardReviewShortcuts {
    pub show_answer: String,
    pub rate_again: String,
    pub rate_hard: String,
    pub rate_good: String,
    pub rate_easy: String,
    pub skip_card: String,
}

impl Default for CardReviewShortcuts {
    fn default() -> Self {
        Self {
            show_answer: "space".to_string(),
            rate_again: "1".to_string(),
            rate_hard: "2".to_string(),
            rate_good: "3".to_string(),
            rate_easy: "4".to_string(),
            skip_card: "s".to_string(),
        }
    }
}

/// Navigation shortcuts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationShortcuts {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub select: String,
    pub back: String,
    pub quit: String,
    pub help: String,
}

impl Default for NavigationShortcuts {
    fn default() -> Self {
        Self {
            up: "up".to_string(),
            down: "down".to_string(),
            left: "left".to_string(),
            right: "right".to_string(),
            select: "enter".to_string(),
            back: "esc".to_string(),
            quit: "q".to_string(),
            help: "?".to_string(),
        }
    }
}

/// Global shortcuts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShortcuts {
    pub toggle_theme: String,
    pub toggle_debug: String,
    pub refresh: String,
    pub save: String,
}

impl Default for GlobalShortcuts {
    fn default() -> Self {
        Self {
            toggle_theme: "t".to_string(),
            toggle_debug: "d".to_string(),
            refresh: "r".to_string(),
            save: "ctrl+s".to_string(),
        }
    }
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            card_review: CardReviewShortcuts::default(),
            navigation: NavigationShortcuts::default(),
            global: GlobalShortcuts::default(),
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Target FPS
    pub target_fps: u32,
    /// Maximum render cache size
    pub max_cache_size: usize,
    /// Enable async rendering
    pub enable_async_rendering: bool,
    /// Preload components
    pub preload_components: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            target_fps: 30,
            max_cache_size: 1000,
            enable_async_rendering: false,
            preload_components: true,
        }
    }
}

/// Accessibility configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    /// High contrast mode
    pub high_contrast: bool,
    /// Large text mode
    pub large_text: bool,
    /// Reduced motion
    pub reduced_motion: bool,
    /// Screen reader support
    pub screen_reader_support: bool,
    /// Keyboard navigation only
    pub keyboard_only: bool,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            high_contrast: false,
            large_text: false,
            reduced_motion: false,
            screen_reader_support: false,
            keyboard_only: false,
        }
    }
}

impl Default for TUIConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            display: DisplayConfig::default(),
            shortcuts: ShortcutConfig::default(),
            performance: PerformanceConfig::default(),
            accessibility: AccessibilityConfig::default(),
        }
    }
}

/// TUI Configuration Manager
pub struct TUIConfigManager {
    /// Current configuration
    config: TUIConfig,
    /// Configuration file path
    config_path: PathBuf,
    /// Whether configuration has been modified
    modified: bool,
}

impl TUIConfigManager {
    /// Create new configuration manager
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_config(&config_path)?;

        Ok(Self {
            config,
            config_path,
            modified: false,
        })
    }

    /// Get configuration file path
    fn get_config_path() -> Result<PathBuf> {
        let mut path =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        path.push("ankitui");
        path.push("tui_config.toml");

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(path)
    }

    /// Load configuration from file
    fn load_config(path: &PathBuf) -> Result<TUIConfig> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: TUIConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config file
            let default_config = TUIConfig::default();
            let content = toml::to_string_pretty(&default_config)?;
            std::fs::write(path, content)?;
            Ok(default_config)
        }
    }

    /// Save configuration to file
    pub fn save(&mut self) -> Result<()> {
        if self.modified {
            let content = toml::to_string_pretty(&self.config)?;
            std::fs::write(&self.config_path, content)?;
            self.modified = false;
        }
        Ok(())
    }

    /// Get current configuration
    pub fn config(&self) -> &TUIConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut TUIConfig),
    {
        updater(&mut self.config);
        self.modified = true;
        Ok(())
    }

    /// Set theme
    pub fn set_theme(&mut self, theme_name: String) -> Result<()> {
        self.config.theme.current_theme = theme_name;
        self.modified = true;
        Ok(())
    }

    /// Set animation speed
    pub fn set_animation_speed(&mut self, speed: f32) -> Result<()> {
        self.config.theme.animation_speed = speed.clamp(0.0, 1.0);
        self.modified = true;
        Ok(())
    }

    /// Toggle visual effects
    pub fn toggle_effects(&mut self) -> Result<()> {
        self.config.theme.enable_effects = !self.config.theme.enable_effects;
        self.modified = true;
        Ok(())
    }

    /// Set display setting
    pub fn set_display_setting(&mut self, key: &str, value: toml::Value) -> Result<()> {
        match key {
            "min_width" => {
                if let Some(width) = value.as_integer() {
                    self.config.display.min_width = width as u16;
                    self.modified = true;
                }
            }
            "min_height" => {
                if let Some(height) = value.as_integer() {
                    self.config.display.min_height = height as u16;
                    self.modified = true;
                }
            }
            "show_card_counter" => {
                if let Some(show) = value.as_bool() {
                    self.config.display.show_card_counter = show;
                    self.modified = true;
                }
            }
            "show_progress_bar" => {
                if let Some(show) = value.as_bool() {
                    self.config.display.show_progress_bar = show;
                    self.modified = true;
                }
            }
            "refresh_rate_ms" => {
                if let Some(rate) = value.as_integer() {
                    self.config.display.refresh_rate_ms = rate as u64;
                    self.modified = true;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Update keyboard shortcut
    pub fn update_shortcut(&mut self, category: &str, action: &str, key: String) -> Result<()> {
        match category {
            "card_review" => match action {
                "show_answer" => {
                    self.config.shortcuts.card_review.show_answer = key;
                    self.modified = true;
                }
                "rate_again" => {
                    self.config.shortcuts.card_review.rate_again = key;
                    self.modified = true;
                }
                "rate_hard" => {
                    self.config.shortcuts.card_review.rate_hard = key;
                    self.modified = true;
                }
                "rate_good" => {
                    self.config.shortcuts.card_review.rate_good = key;
                    self.modified = true;
                }
                "rate_easy" => {
                    self.config.shortcuts.card_review.rate_easy = key;
                    self.modified = true;
                }
                "skip_card" => {
                    self.config.shortcuts.card_review.skip_card = key;
                    self.modified = true;
                }
                _ => {}
            },
            "navigation" => match action {
                "up" => {
                    self.config.shortcuts.navigation.up = key;
                    self.modified = true;
                }
                "down" => {
                    self.config.shortcuts.navigation.down = key;
                    self.modified = true;
                }
                "left" => {
                    self.config.shortcuts.navigation.left = key;
                    self.modified = true;
                }
                "right" => {
                    self.config.shortcuts.navigation.right = key;
                    self.modified = true;
                }
                "select" => {
                    self.config.shortcuts.navigation.select = key;
                    self.modified = true;
                }
                "back" => {
                    self.config.shortcuts.navigation.back = key;
                    self.modified = true;
                }
                "quit" => {
                    self.config.shortcuts.navigation.quit = key;
                    self.modified = true;
                }
                "help" => {
                    self.config.shortcuts.navigation.help = key;
                    self.modified = true;
                }
                _ => {}
            },
            "global" => match action {
                "toggle_theme" => {
                    self.config.shortcuts.global.toggle_theme = key;
                    self.modified = true;
                }
                "toggle_debug" => {
                    self.config.shortcuts.global.toggle_debug = key;
                    self.modified = true;
                }
                "refresh" => {
                    self.config.shortcuts.global.refresh = key;
                    self.modified = true;
                }
                "save" => {
                    self.config.shortcuts.global.save = key;
                    self.modified = true;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    /// Check if configuration has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Reset to default configuration
    pub fn reset_to_defaults(&mut self) -> Result<()> {
        self.config = TUIConfig::default();
        self.modified = true;
        Ok(())
    }

    /// Export configuration to string
    pub fn export_config(&self) -> Result<String> {
        Ok(toml::to_string_pretty(&self.config)?)
    }

    /// Import configuration from string
    pub fn import_config(&mut self, config_str: &str) -> Result<()> {
        let imported_config: TUIConfig = toml::from_str(config_str)?;
        self.config = imported_config;
        self.modified = true;
        Ok(())
    }
}

impl Drop for TUIConfigManager {
    fn drop(&mut self) {
        // Auto-save on drop if modified
        if self.modified {
            let _ = self.save();
        }
    }
}

impl Default for TUIConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create TUI config manager")
    }
}

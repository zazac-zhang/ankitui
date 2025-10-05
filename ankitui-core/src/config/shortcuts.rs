//! Shortcuts Configuration Module
//!
//! Essential keyboard shortcuts for the TUI flashcard application

use serde::{Deserialize, Serialize};

/// Keyboard shortcuts configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortcutConfig {
    /// Show answer during review
    pub show_answer: String,

    /// Rate card as "Again" (0)
    pub rate_again: String,

    /// Rate card as "Hard" (1)
    pub rate_hard: String,

    /// Rate card as "Good" (2)
    pub rate_good: String,

    /// Rate card as "Easy" (3)
    pub rate_easy: String,

    /// Pause/Resume session
    pub toggle_pause: String,

    /// Exit current session
    pub exit_session: String,

    /// Show help
    pub show_help: String,

    /// Show statistics
    pub show_stats: String,

    /// Undo last action
    pub undo: String,

    /// Redo last action
    pub redo: String,

    /// Search
    pub search: String,

    /// Edit card
    pub edit_card: String,

    /// Delete card
    pub delete_card: String,

    /// Navigation up
    pub up: String,

    /// Navigation down
    pub down: String,

    /// Navigation left
    pub left: String,

    /// Navigation right
    pub right: String,

    /// Select/confirm
    pub select: String,

    /// Go back
    pub back: String,

    /// Quit application
    pub quit: String,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            show_answer: " ".to_string(),  // Space
            rate_again: "0".to_string(),   // 0
            rate_hard: "1".to_string(),    // 1
            rate_good: "2".to_string(),    // 2
            rate_easy: "3".to_string(),    // 3
            toggle_pause: "p".to_string(), // p
            exit_session: "q".to_string(), // q
            show_help: "?".to_string(),    // ?
            show_stats: "s".to_string(),   // s
            undo: "u".to_string(),         // u
            redo: "U".to_string(),         // Shift+U
            search: "/".to_string(),       // /
            edit_card: "e".to_string(),    // e
            delete_card: "d".to_string(),  // d
            up: "up".to_string(),          // Up arrow
            down: "down".to_string(),      // Down arrow
            left: "left".to_string(),      // Left arrow
            right: "right".to_string(),    // Right arrow
            select: "enter".to_string(),   // Enter
            back: "escape".to_string(),    // Escape
            quit: "ctrl+c".to_string(),    // Ctrl+C
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_config_default() {
        let config = ShortcutConfig::default();
        assert_eq!(config.show_answer, " ");
        assert_eq!(config.rate_again, "0");
        assert_eq!(config.toggle_pause, "p");
        assert_eq!(config.exit_session, "q");
        assert_eq!(config.show_help, "?");
        assert_eq!(config.show_stats, "s");
    }

    #[test]
    fn test_navigation_shortcuts() {
        let config = ShortcutConfig::default();
        assert_eq!(config.up, "up");
        assert_eq!(config.down, "down");
        assert_eq!(config.left, "left");
        assert_eq!(config.right, "right");
        assert_eq!(config.select, "enter");
        assert_eq!(config.back, "escape");
    }

    #[test]
    fn test_rating_shortcuts() {
        let config = ShortcutConfig::default();
        assert_eq!(config.rate_again, "0");
        assert_eq!(config.rate_hard, "1");
        assert_eq!(config.rate_good, "2");
        assert_eq!(config.rate_easy, "3");
    }
}

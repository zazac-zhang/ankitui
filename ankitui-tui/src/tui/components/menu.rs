//! Main Menu Component
//!
//! Modern main menu implementation with clean architecture

use crate::tui::app::AppState;
use crate::tui::core::event_handler::Action;
use crate::tui::core::{state_manager::RenderContext, UIComponent};
use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListState, ListItem, Paragraph},
    Frame,
};

const MENU_ITEMS: &[&str] = &[
    "📚 Start Review",
    "🗂️ Deck Management",
    "📊 View Statistics",
    "⚙️ Settings",
    "❓ Help",
    "🚪 Quit",
];

/// Modern main menu component
pub struct Menu {
    /// Current selection
    selected_item: usize,
    /// List state for rendering
    list_state: ListState,
    /// Menu items (as strings for easier manipulation)
    menu_items: Vec<String>,
    /// Animation progress for smooth transitions
    animation_progress: f32,
    /// Last update time for animations
    last_update: std::time::Instant,
}

impl Menu {
    /// Create a new main menu component
    pub fn new() -> Self {
        let menu_items = MENU_ITEMS.iter().map(|s| s.to_string()).collect();
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            selected_item: 0,
            list_state,
            menu_items,
            animation_progress: 0.0,
            last_update: std::time::Instant::now(),
        }
    }

    /// Get selected main menu item - legacy compatibility method
    /// Matches Components::get_selected_main_menu_item behavior
    pub fn get_selected_main_menu_item(&self) -> usize {
        self.selected_item
    }

    /// Navigate main menu - legacy compatibility method
    /// Matches Components::navigate_main_menu behavior
    pub fn navigate_main_menu(&mut self, action: Action) {
        match action {
            Action::Up => {
                if self.selected_item > 0 {
                    self.selected_item -= 1;
                    self.list_state.select(Some(self.selected_item));
                    self.trigger_animation();
                }
            }
            Action::Down => {
                if self.selected_item < 5 {
                    // Legacy: hard-coded limit
                    self.selected_item += 1;
                    self.list_state.select(Some(self.selected_item));
                    self.trigger_animation();
                }
            }
            _ => {}
        }
    }

    /// Get currently selected menu item
    pub fn selected_item(&self) -> usize {
        self.selected_item
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
            self.list_state.select(Some(self.selected_item));
            self.trigger_animation();
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected_item < self.menu_items.len() - 1 {
            self.selected_item += 1;
            self.list_state.select(Some(self.selected_item));
            self.trigger_animation();
        }
    }

    /// Get the state that should be entered when current item is selected
    /// Migrated from legacy Components::handle_main_menu_action logic
    pub fn get_target_state(&self) -> AppState {
        match self.selected_item {
            0 => AppState::DeckSelection, // Start Review -> go to deck selection first
            1 => AppState::DeckManagement, // Deck Management
            2 => AppState::Statistics,    // View Statistics
            3 => AppState::Settings,      // Settings
            4 => AppState::Help,          // Help
            5 => AppState::ConfirmExit,   // Quit -> go to confirm exit
            _ => AppState::MainMenu,      // Should not happen
        }
    }

    /// Trigger selection animation
    fn trigger_animation(&mut self) {
        self.animation_progress = 0.0;
        self.last_update = std::time::Instant::now();
    }

    /// Update animation progress
    fn update_animation(&mut self) {
        let elapsed = self.last_update.elapsed().as_secs_f32();
        if elapsed < 0.3 {
            // 300ms animation
            self.animation_progress = (elapsed / 0.3).min(1.0);
        } else {
            self.animation_progress = 1.0;
        }
    }

    /// Get styled menu items
    fn get_menu_items(&self) -> Vec<ListItem> {
        self.menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_item {
                    Style::default()
                        .fg(Color::Cyan)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                } else {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::empty())
                };

                // Add animation effect to selected item
                let content = if i == self.selected_item && self.animation_progress < 1.0 {
                    let indent = "  ".repeat((self.animation_progress * 3.0) as usize);
                    format!("{}{}", indent, item)
                } else {
                    format!("  {}", item)
                };

                ListItem::new(Line::from(Span::styled(content, style)))
            })
            .collect()
    }
    /// Handle user action (public wrapper)
    pub fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        <Self as UIComponent>::handle_action(self, action)
    }

    /// Render component (public wrapper)
    pub fn render(&mut self, frame: &mut ratatui::Frame, context: RenderContext) -> Result<()> {
        <Self as UIComponent>::render(self, frame, context)
    }
}

impl UIComponent for Menu {
    fn render(&mut self, frame: &mut ratatui::Frame, context: RenderContext) -> Result<()> {
        // Update animations
        self.update_animation();

        // Use the full frame area like the legacy renderer
        let rect = frame.area();

        // Create layout similar to legacy Components::render_main_menu
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(7),    // Menu items
                Constraint::Length(3), // Messages/hints
            ])
            .split(rect);

        // Render title - legacy style
        let title = Paragraph::new("AnkiTUI - Terminal Spaced Repetition Learning")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(title, chunks[0]);

        // Render menu items with legacy styling
        let list_items: Vec<ListItem> = self
            .menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_item {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(item.as_str()).style(style)
            })
            .collect();

        let list = List::new(list_items)
            .block(Block::default().title("Main Menu").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);

        // Render hints - legacy style
        let hint = Paragraph::new("↑/↓: Navigate  Enter: Select  Q: Quit  F1: Help")
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(hint, chunks[2]);

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Up => {
                self.move_up();
                Ok(None)
            }
            Action::Down => {
                self.move_down();
                Ok(None)
            }
            Action::Select => Ok(Some(self.get_target_state())),
            Action::Cancel => Ok(Some(AppState::ConfirmExit)),
            _ => Ok(None),
        }
    }

    fn update(&mut self) -> Result<()> {
        // Update any dynamic content if needed
        self.update_animation();
        Ok(())
    }

    fn name(&self) -> &str {
        "main_menu"
    }
}

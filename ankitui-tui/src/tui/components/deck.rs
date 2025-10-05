//! Deck Selection Component
//!
//! Modern deck management interface

use crate::tui::app::AppState;
use crate::tui::core::event_handler::Action;
use crate::tui::core::{state_manager::RenderContext, UIComponent};
use ankitui_core::{Card, Deck, DeckManager};
use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use uuid::Uuid;

/// Deck selection component
pub struct DeckSelector {
    /// Available decks
    decks: Vec<Deck>,
    /// Selected deck index
    selected_deck: usize,
    /// List state for rendering
    list_state: ListState,
    /// Current mode
    mode: DeckSelectionMode,
    /// Search query
    search_query: String,
    /// Sort order
    sort_order: DeckSortOrder,
    /// Edit field for deck name/description editing
    edit_field: String,
    /// Edit field position (0 = name, 1 = description)
    edit_position: usize,
    /// Confirmation dialog state
    show_confirm_dialog: bool,
    /// Confirmation message
    confirm_message: String,
    /// Cards in the selected deck (for card management)
    cards: Vec<Card>,
    /// Selected card index (for card management)
    selected_card: usize,
    /// List state for card management
    card_list_state: ListState,
    /// Card management action selection
    selected_card_action: usize,
    /// Management action selection (for deck management layout)
    selected_management_action: usize,
    /// Focus area for deck management (0 = deck list, 1 = actions panel)
    focus_area: usize,
}

/// Selection mode
#[derive(Debug, Clone, PartialEq)]
pub enum DeckSelectionMode {
    Browse,
    Select,
    Manage,
    ManageCards,
    Edit,
    Create,
    Delete,
    Import,
    Export,
    Statistics,
}

/// Sort order for decks
#[derive(Debug, Clone, PartialEq)]
pub enum DeckSortOrder {
    Name,
    Created,
    Modified,
    CardCount,
}

impl DeckSelector {
    /// Create a new deck selection component
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut card_list_state = ListState::default();
        card_list_state.select(Some(0));

        Self {
            decks: Vec::new(),
            selected_deck: 0,
            list_state,
            mode: DeckSelectionMode::Manage,
            search_query: String::new(),
            sort_order: DeckSortOrder::Name,
            edit_field: String::new(),
            edit_position: 0,
            show_confirm_dialog: false,
            confirm_message: String::new(),
            cards: Vec::new(),
            selected_card: 0,
            card_list_state,
            selected_card_action: 0,
            selected_management_action: 0,
            focus_area: 0, // Start with deck list focused
        }
    }

    /// Set deck manager and load real decks
    pub async fn set_deck_manager(&mut self, manager: &DeckManager) -> Result<()> {
        // Load real decks from the deck manager
        match manager.get_all_decks().await {
            Ok(deck_data) => {
                // Extract just the deck info (not cards) for display
                self.decks = deck_data.into_iter().map(|(deck, _cards)| deck).collect();
                self.selected_deck = 0;
                if !self.decks.is_empty() {
                    self.list_state.select(Some(0));
                } else {
                    self.list_state.select(None);
                }
            }
            Err(e) => {
                // If loading fails, set empty deck list
                self.decks = Vec::new();
                self.selected_deck = 0;
                self.list_state.select(None);
                return Err(anyhow::anyhow!("Failed to load decks: {}", e));
            }
        }
        Ok(())
    }

    /// Load cards for the selected deck
    pub async fn load_cards_for_deck(&mut self, manager: &DeckManager) -> Result<()> {
        if let Some(deck) = self.selected_deck() {
            match manager.get_cards(&deck.uuid).await {
                Ok(cards) => {
                    self.cards = cards;
                    self.selected_card = 0;
                    if !self.cards.is_empty() {
                        self.card_list_state.select(Some(0));
                    } else {
                        self.card_list_state.select(None);
                    }
                }
                Err(e) => {
                    // If loading fails, set empty card list
                    self.cards = Vec::new();
                    self.selected_card = 0;
                    self.card_list_state.select(None);
                    return Err(anyhow::anyhow!("Failed to load cards: {}", e));
                }
            }
        }
        Ok(())
    }

    /// Get currently selected deck
    pub fn selected_deck(&self) -> Option<&Deck> {
        self.decks.get(self.selected_deck)
    }

    /// Get selected deck (alias for components compatibility)
    pub fn get_selected_deck(&self) -> Option<&Deck> {
        self.selected_deck()
    }

    /// Update decks list
    pub fn update_decks(&mut self, decks: Vec<Deck>) {
        self.decks = decks;
        self.selected_deck = 0;
        if !self.decks.is_empty() {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_deck > 0 {
            self.selected_deck -= 1;
            self.list_state.select(Some(self.selected_deck));
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected_deck < self.decks.len().saturating_sub(1) {
            self.selected_deck += 1;
            self.list_state.select(Some(self.selected_deck));
        }
    }

    /// Move card selection up
    pub fn move_card_up(&mut self) {
        if self.selected_card > 0 {
            self.selected_card -= 1;
            self.card_list_state.select(Some(self.selected_card));
        }
    }

    /// Move card selection down
    pub fn move_card_down(&mut self) {
        if self.selected_card < self.cards.len().saturating_sub(1) {
            self.selected_card += 1;
            self.card_list_state.select(Some(self.selected_card));
        }
    }

    /// Move card action selection up
    pub fn move_card_action_up(&mut self) {
        if self.selected_card_action > 0 {
            self.selected_card_action -= 1;
        }
    }

    /// Move card action selection down
    pub fn move_card_action_down(&mut self) {
        const CARD_ACTIONS: &[&str] = &[
            "📝 Edit Card",
            "🗑️  Delete Card",
            "📊 View Card Stats",
            "📋 Copy Card",
            "⏸️ Suspend Card",
            "▶️ Unsuspend Card",
        ];
        if self.selected_card_action < CARD_ACTIONS.len().saturating_sub(1) {
            self.selected_card_action += 1;
        }
    }

    /// Sort decks
    fn sort_decks(&mut self) {
        match self.sort_order {
            DeckSortOrder::Name => {
                self.decks.sort_by(|a, b| a.name.cmp(&b.name));
            }
            DeckSortOrder::Created => {
                self.decks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            }
            DeckSortOrder::Modified => {
                self.decks.sort_by(|a, b| a.modified_at.cmp(&b.modified_at));
            }
            DeckSortOrder::CardCount => {
                // Note: Sort by name since Deck doesn't have card_count field
                self.decks.sort_by(|a, b| a.name.cmp(&b.name));
            }
        }
    }

    /// Filter decks by search query
    fn filter_decks(&self) -> Vec<&Deck> {
        if self.search_query.is_empty() {
            self.decks.iter().collect()
        } else {
            self.decks
                .iter()
                .filter(|deck| {
                    deck.name
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
                        || deck
                            .description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&self.search_query.to_lowercase()))
                            .unwrap_or(false)
                })
                .collect()
        }
    }

    /// Get deck info text
    fn get_deck_info_text<'a>(&self, deck: &'a Deck) -> Vec<Line<'a>> {
        let mut lines = vec![Line::from(vec![
            Span::styled("Cards: ", Style::default().fg(Color::Gray)),
            Span::styled("📚", Style::default().fg(Color::Cyan)), // Use emoji since no card_count
        ])];

        if let Some(description) = &deck.description {
            lines.push(Line::from(vec![
                Span::styled("Description: ", Style::default().fg(Color::Gray)),
                Span::styled(description, Style::default().fg(Color::White)),
            ]));
        }

        lines.push(Line::from(vec![
            Span::styled("Modified: ", Style::default().fg(Color::Gray)),
            Span::styled(
                deck.modified_at.format("%Y-%m-%d %H:%M").to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]));

        lines
    }
    /// Handle user action (public wrapper)
    pub fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        <Self as UIComponent>::handle_action(self, action)
    }

    /// Render component (public wrapper)
    pub fn render(&mut self, frame: &mut ratatui::Frame, context: RenderContext) -> Result<()> {
        <Self as UIComponent>::render(self, frame, context)
    }

    /// Get selected index (for renderer compatibility)
    pub fn selected_index(&self) -> usize {
        self.selected_deck
    }

    /// Get decks (for renderer compatibility)
    pub fn decks(&self) -> &[Deck] {
        &self.decks
    }

    /// Get current editing mode
    pub fn current_mode(&self) -> DeckSelectionMode {
        self.mode.clone()
    }

    /// Get current edit field content
    pub fn edit_field(&self) -> &str {
        &self.edit_field
    }

    /// Set edit field content (for input handling)
    pub fn set_edit_field(&mut self, content: String) {
        self.edit_field = content;
    }

    /// Get edit position (0 = name, 1 = description)
    pub fn edit_position(&self) -> usize {
        self.edit_position
    }

    /// Get confirmation dialog state
    pub fn show_confirm_dialog(&self) -> bool {
        self.show_confirm_dialog
    }

    /// Get confirmation message
    pub fn confirm_message(&self) -> &str {
        &self.confirm_message
    }

    /// Get pending operations for the main app to process
    pub fn get_pending_operation(&self) -> Option<DeckOperation> {
        match self.mode {
            DeckSelectionMode::Edit => {
                if self.edit_position == 0 && !self.edit_field.is_empty() {
                    if let Some(deck) = self.selected_deck() {
                        return Some(DeckOperation::Update {
                            deck_uuid: deck.uuid,
                            name: Some(self.edit_field.clone()),
                            description: None,
                        });
                    }
                }
            }
            DeckSelectionMode::Create => {
                if self.edit_position == 0 && !self.edit_field.is_empty() {
                    return Some(DeckOperation::Create {
                        name: self.edit_field.clone(),
                        description: None,
                    });
                }
            }
            DeckSelectionMode::Delete => {
                if let Some(deck) = self.selected_deck() {
                    return Some(DeckOperation::Delete {
                        deck_uuid: deck.uuid,
                    });
                }
            }
            DeckSelectionMode::Import => {
                if !self.edit_field.is_empty() {
                    return Some(DeckOperation::Import {
                        file_path: self.edit_field.clone(),
                    });
                }
            }
            DeckSelectionMode::Export => {
                if let Some(deck) = self.selected_deck() {
                    return Some(DeckOperation::Export {
                        deck_uuid: deck.uuid,
                        include_state: true, // Default to including state
                        file_path: format!("{}.toml", deck.name),
                    });
                }
            }
            DeckSelectionMode::Statistics => {
                if let Some(deck) = self.selected_deck() {
                    return Some(DeckOperation::ViewStatistics {
                        deck_uuid: deck.uuid,
                    });
                }
            }
            _ => {}
        }
        None
    }

    /// Set the deck selector mode
    pub fn set_mode(&mut self, mode: DeckSelectionMode) {
        self.mode = mode;
        // Reset edit state when switching modes
        self.edit_field.clear();
        self.edit_position = 0;
        self.show_confirm_dialog = false;
        self.confirm_message.clear();
    }

    /// Clear pending operation (called after main app processes it)
    pub fn clear_pending_operation(&mut self) {
        self.mode = DeckSelectionMode::Manage;
        self.edit_field.clear();
        self.edit_position = 0;
        self.show_confirm_dialog = false;
        self.confirm_message.clear();
        self.selected_card_action = 0;
    }

    /// Get cards (for component registry)
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Render management layout with sidebar and main content
    fn render_management_layout(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Create horizontal layout: sidebar (30%) + main content (70%)
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // Deck sidebar
                Constraint::Percentage(70), // Main content area
            ])
            .split(area);

        // Left sidebar - Deck list
        self.render_deck_sidebar(frame, horizontal_chunks[0])?;

        // Right main content - Selected deck details and actions
        self.render_main_content(frame, horizontal_chunks[1])?;

        Ok(())
    }

    /// Render deck sidebar
    fn render_deck_sidebar(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Deck list
                Constraint::Length(3), // Controls
            ])
            .split(area);

        // Render header with focus indicator
        let header_text = if self.focus_area == 0 {
            "📚 Decks [●]"
        } else {
            "📚 Decks [○]"
        };
        let header_style = if self.focus_area == 0 {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        let header = Paragraph::new(header_text).style(header_style).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if self.focus_area == 0 {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::DarkGray)
                }),
        );
        frame.render_widget(header, chunks[0]);

        // Render deck list
        self.render_compact_deck_list(frame, chunks[1])?;

        // Render sidebar controls with focus indicator
        let controls_text = "↑↓Select Enter:Review Tab:Actions Esc:Menu";
        let controls_style = if self.focus_area == 0 {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Gray)
        };
        let controls = Paragraph::new(controls_text).style(controls_style).block(
            Block::default()
                .borders(Borders::ALL)
                .title(if self.focus_area == 0 {
                    " Controls "
                } else {
                    " Controls "
                })
                .border_style(if self.focus_area == 0 {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::DarkGray)
                }),
        );
        frame.render_widget(controls, chunks[2]);

        Ok(())
    }

    /// Render compact deck list for sidebar
    fn render_compact_deck_list(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let filtered_decks = self.filter_decks();

        let items: Vec<ListItem> = filtered_decks
            .iter()
            .enumerate()
            .map(|(i, deck)| {
                let is_selected = if let Some(filtered_index) =
                    filtered_decks.iter().position(|d| d.uuid == deck.uuid)
                {
                    filtered_index == self.selected_deck
                } else {
                    false
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let content = format!("📁 {}", deck.name);
                ListItem::new(Line::from(Span::styled(content, style)))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        );

        // Create a mutable copy of list_state for rendering
        let mut list_state = self.list_state.clone();
        frame.render_stateful_widget(list, area, &mut list_state);
        Ok(())
    }

    /// Render main content area
    fn render_main_content(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Deck details header
                Constraint::Length(6), // Deck info
                Constraint::Min(8),    // Management actions
                Constraint::Length(3), // Instructions
            ])
            .split(area);

        // Render deck details header
        if let Some(deck) = self.selected_deck() {
            let header_text = if self.focus_area == 1 {
                format!("🗂️ {} [●]", deck.name)
            } else {
                format!("🗂️ {} [○]", deck.name)
            };
            let header_style = if self.focus_area == 1 {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let header = Paragraph::new(header_text).style(header_style).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if self.focus_area == 1 {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            );
            frame.render_widget(header, chunks[0]);

            // Render deck information
            let info_text = if let Some(desc) = &deck.description {
                format!("Description: {}", desc)
            } else {
                "No description available".to_string()
            };
            let info = Paragraph::new(info_text)
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Information")
                        .border_style(Style::default().fg(Color::Blue)),
                );
            frame.render_widget(info, chunks[1]);

            // Render management actions
            self.render_management_actions(frame, chunks[2])?;

            // Render instructions based on focus area
            let instructions = if self.focus_area == 1 {
                "↑↓ Select Action | Enter:Execute | Tab:Decks | Esc:Menu"
            } else {
                "Tab to focus actions panel"
            };
            let instr_style = if self.focus_area == 1 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Gray)
            };
            let instr = Paragraph::new(instructions).style(instr_style).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(if self.focus_area == 1 {
                        " Actions "
                    } else {
                        " Actions "
                    })
                    .border_style(if self.focus_area == 1 {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            );
            frame.render_widget(instr, chunks[3]);
        } else {
            // No deck selected
            let no_deck = Paragraph::new("No deck selected")
                .style(Style::default().fg(Color::Gray))
                .alignment(ratatui::layout::Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Deck Management ")
                        .border_style(Style::default().fg(Color::Gray)),
                );
            frame.render_widget(no_deck, area);
        }

        Ok(())
    }

    /// Render management actions
    fn render_management_actions(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let actions = vec![
            "📝 Edit Deck Properties",
            "🗑️  Delete Deck",
            "📤 Export Deck",
            "📥 Import Cards",
            "📊 View Statistics",
            "🃏 Manage Cards",
        ];

        let items: Vec<ListItem> = actions
            .into_iter()
            .enumerate()
            .map(|(i, action)| {
                let style = if i == self.selected_management_action {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(Span::styled(action, style)))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Actions ")
                .border_style(Style::default().fg(Color::Green)),
        );

        frame.render_widget(list, area);
        Ok(())
    }
}

/// Deck operations to be processed by the main application
#[derive(Debug, Clone)]
pub enum DeckOperation {
    Create {
        name: String,
        description: Option<String>,
    },
    Update {
        deck_uuid: Uuid,
        name: Option<String>,
        description: Option<String>,
    },
    Delete {
        deck_uuid: Uuid,
    },
    Import {
        file_path: String,
    },
    Export {
        deck_uuid: Uuid,
        include_state: bool,
        file_path: String,
    },
    ViewStatistics {
        deck_uuid: Uuid,
    },
}

impl UIComponent for DeckSelector {
    fn render(&mut self, frame: &mut ratatui::Frame, _context: RenderContext) -> Result<()> {
        let area = frame.area();

        // Handle confirmation dialog overlay
        if self.show_confirm_dialog {
            self.render_confirmation_dialog(frame, area)?;
            return Ok(());
        }

        // Handle different modes with different layouts
        match self.mode {
            DeckSelectionMode::Edit | DeckSelectionMode::Create => {
                self.render_edit_mode(frame, area)?;
            }
            DeckSelectionMode::Import | DeckSelectionMode::Export => {
                self.render_import_export_mode(frame, area)?;
            }
            DeckSelectionMode::Statistics => {
                self.render_statistics_mode(frame, area)?;
            }
            DeckSelectionMode::ManageCards => {
                self.render_card_management_layout(frame, area)?;
            }
            _ => {
                // New left-right layout for Deck Management
                if self.mode == DeckSelectionMode::Manage {
                    // Management layout: sidebar + main content
                    self.render_management_layout(frame, area)?;
                } else {
                    // Legacy layout for other modes
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3), // Header
                            Constraint::Min(5),    // Deck list
                            Constraint::Length(3), // Info panel
                            Constraint::Length(3), // Controls (moved to bottom)
                        ])
                        .split(area);

                    // Render header
                    self.render_header(frame, chunks[0])?;

                    // Render deck list
                    self.render_deck_list(frame, chunks[1])?;

                    // Render info panel
                    if let Some(deck) = self.selected_deck() {
                        self.render_deck_info(frame, chunks[2], deck)?;
                    }

                    // Render controls (now at bottom)
                    self.render_controls(frame, chunks[3])?;
                }
            }
        }

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Up => {
                match self.mode {
                    DeckSelectionMode::Manage => {
                        if self.focus_area == 0 {
                            // Focus on deck list - move deck selection up
                            self.move_up();
                        } else {
                            // Focus on actions panel - move action selection up
                            if self.selected_management_action > 0 {
                                self.selected_management_action -= 1;
                            }
                        }
                    }
                    DeckSelectionMode::ManageCards => self.move_card_up(),
                    _ => {}
                }
            }
            Action::Down => {
                match self.mode {
                    DeckSelectionMode::Manage => {
                        if self.focus_area == 0 {
                            // Focus on deck list - move deck selection down
                            self.move_down();
                        } else {
                            // Focus on actions panel - move action selection down
                            const MANAGEMENT_ACTIONS: &[&str] = &[
                                "📝 Edit Deck Properties",
                                "🗑️  Delete Deck",
                                "📤 Export Deck",
                                "📥 Import Cards",
                                "📊 View Statistics",
                                "🃏 Manage Cards",
                            ];
                            if self.selected_management_action
                                < MANAGEMENT_ACTIONS.len().saturating_sub(1)
                            {
                                self.selected_management_action += 1;
                            }
                        }
                    }
                    DeckSelectionMode::ManageCards => self.move_card_down(),
                    _ => {}
                }
            }
            Action::Left => {
                match self.mode {
                    DeckSelectionMode::ManageCards => self.move_card_action_up(),
                    DeckSelectionMode::Manage => {
                        // In deck management, Left/Right moves between focus areas
                        if self.focus_area > 0 {
                            self.focus_area -= 1;
                        }
                    }
                    _ => {}
                }
            }
            Action::Right => {
                match self.mode {
                    DeckSelectionMode::ManageCards => self.move_card_action_down(),
                    DeckSelectionMode::Manage => {
                        // In deck management, Left/Right moves between focus areas
                        if self.focus_area < 1 {
                            self.focus_area += 1;
                        }
                    }
                    _ => {}
                }
            }
            Action::Tab => {
                match self.mode {
                    DeckSelectionMode::Manage => {
                        // Toggle focus between deck list and actions
                        self.focus_area = if self.focus_area == 0 { 1 } else { 0 };
                    }
                    _ => {}
                }
            }
            Action::Select => {
                if self.show_confirm_dialog {
                    // Handle confirmation dialog - Yes to confirm, No to cancel
                    self.mode = DeckSelectionMode::Manage;
                    self.show_confirm_dialog = false;
                    self.confirm_message.clear();
                } else if let Some(_deck) = self.selected_deck() {
                    match self.mode {
                        DeckSelectionMode::Select => {
                            // Don't transition state here - let main app handle session creation
                            return Ok(None);
                        }
                        DeckSelectionMode::Manage => {
                            if self.focus_area == 0 {
                                // Focus on deck list - start review session
                                return Ok(None); // Let main app handle deck selection for review
                            } else {
                                // Focus on actions panel - execute selected action
                                const MANAGEMENT_ACTIONS: &[&str] = &[
                                    "📝 Edit Deck Properties",
                                    "🗑️  Delete Deck",
                                    "📤 Export Deck",
                                    "📥 Import Cards",
                                    "📊 View Statistics",
                                    "🃏 Manage Cards",
                                ];

                                match self.selected_management_action {
                                    0 => {
                                        // Edit Deck Properties
                                        let selected_deck = self.selected_deck().cloned();
                                        self.mode = DeckSelectionMode::Edit;
                                        if let Some(deck) = selected_deck {
                                            self.edit_field = deck.name.clone();
                                            self.edit_position = 0;
                                        }
                                    }
                                    1 => {
                                        // Delete Deck - show confirmation
                                        if let Some(deck) = self.selected_deck().cloned() {
                                            self.show_confirm_dialog = true;
                                            self.confirm_message =
                                                format!("Delete deck '{}'?", deck.name);
                                        }
                                    }
                                    2 => {
                                        // Export Deck
                                        let selected_deck = self.selected_deck().cloned();
                                        self.mode = DeckSelectionMode::Export;
                                        if let Some(deck) = selected_deck {
                                            self.edit_field = format!("{}.toml", deck.name);
                                        }
                                    }
                                    3 => {
                                        // Import Cards
                                        self.mode = DeckSelectionMode::Import;
                                        self.edit_field.clear();
                                    }
                                    4 => {
                                        // View Statistics
                                        let selected_deck = self.selected_deck().cloned();
                                        self.mode = DeckSelectionMode::Statistics;
                                        if let Some(deck) = selected_deck {
                                            self.edit_field = deck.uuid.to_string();
                                        }
                                    }
                                    5 => {
                                        // Manage Cards - enter card management mode
                                        self.mode = DeckSelectionMode::ManageCards;
                                        self.selected_card = 0;
                                        self.selected_card_action = 0;
                                        self.focus_area = 0; // Reset focus to card list
                                        if !self.cards.is_empty() {
                                            self.card_list_state.select(Some(0));
                                        } else {
                                            self.card_list_state.select(None);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        DeckSelectionMode::Browse => {
                            // Just browsing, could show more details
                        }
                        DeckSelectionMode::Edit => {
                            // In edit mode, move to next field or confirm
                            self.edit_position = (self.edit_position + 1) % 2; // Toggle between name (0) and description (1)
                            if self.edit_position == 0 {
                                // Back to name, means user wants to confirm
                                self.mode = DeckSelectionMode::Manage;
                            }
                        }
                        DeckSelectionMode::Create => {
                            // In create mode, move to next field or confirm
                            self.edit_position = (self.edit_position + 1) % 2; // Toggle between name (0) and description (1)
                            if self.edit_position == 0 {
                                // Back to name, means user wants to confirm creation
                                self.mode = DeckSelectionMode::Manage;
                            }
                        }
                        DeckSelectionMode::Delete => {
                            // Delete is a confirmation action
                            self.mode = DeckSelectionMode::Manage;
                        }
                        DeckSelectionMode::Import => {
                            // Confirm import with file path
                            if !self.edit_field.is_empty() {
                                self.mode = DeckSelectionMode::Manage;
                            }
                        }
                        DeckSelectionMode::Export => {
                            // Confirm export with file path
                            if !self.edit_field.is_empty() {
                                self.mode = DeckSelectionMode::Manage;
                            }
                        }
                        DeckSelectionMode::Statistics => {
                            // Statistics is view-only, return to manage on Enter
                            self.mode = DeckSelectionMode::Manage;
                        }
                        DeckSelectionMode::ManageCards => {
                            // Handle card action selection
                            if self.selected_card_action == 0 {
                                // Edit Card - would transition to edit mode
                                self.mode = DeckSelectionMode::Manage;
                            } else if self.selected_card_action == 1 {
                                // Delete Card - would show confirmation
                                self.mode = DeckSelectionMode::Manage;
                            }
                            // Return to manage mode for now
                            self.mode = DeckSelectionMode::Manage;
                        }
                    }
                }
            }
            Action::Cancel => {
                if self.show_confirm_dialog {
                    // Cancel from confirmation dialog
                    self.show_confirm_dialog = false;
                    self.confirm_message.clear();
                    self.mode = DeckSelectionMode::Manage;
                } else {
                    match self.mode {
                        DeckSelectionMode::Edit
                        | DeckSelectionMode::Create
                        | DeckSelectionMode::Delete
                        | DeckSelectionMode::Import
                        | DeckSelectionMode::Export
                        | DeckSelectionMode::Statistics
                        | DeckSelectionMode::ManageCards => {
                            // Cancel editing/creating/deleting/importing/exporting and return to manage mode
                            self.mode = DeckSelectionMode::Manage;
                            self.edit_field.clear();
                            self.edit_position = 0;
                            self.show_confirm_dialog = false;
                        }
                        _ => {
                            return Ok(Some(AppState::MainMenu));
                        }
                    }
                }
            }
            Action::Create => {
                // Enter create mode
                self.mode = DeckSelectionMode::Create;
                self.edit_field = String::new();
                self.edit_position = 0; // Start with name editing
            }
            Action::Delete => {
                // Show delete confirmation
                if self.selected_deck().is_some() && self.mode == DeckSelectionMode::Manage {
                    self.mode = DeckSelectionMode::Delete;
                    self.show_confirm_dialog = true;
                    if let Some(deck) = self.selected_deck() {
                        self.confirm_message = format!("Delete deck '{}'?", deck.name);
                    }
                }
            }
            Action::Edit => {
                // Enter edit mode for selected deck
                if let Some(deck) = self.selected_deck().cloned() {
                    self.mode = DeckSelectionMode::Edit;
                    self.edit_field = deck.name;
                    self.edit_position = 0; // Start with name editing
                }
            }
            Action::Char(c) => {
                // Handle character input for editing and shortcuts
                match self.mode {
                    DeckSelectionMode::Edit | DeckSelectionMode::Create => {
                        if self.edit_position == 0 {
                            // Editing name
                            self.edit_field.push(c);
                        }
                        // Description editing not implemented yet
                    }
                    DeckSelectionMode::Import | DeckSelectionMode::Export => {
                        // Editing file path
                        self.edit_field.push(c);
                    }
                    DeckSelectionMode::Manage => {
                        match c {
                            'c' | 'C' => {
                                // Create new deck
                                self.mode = DeckSelectionMode::Create;
                                self.edit_field.clear();
                                self.edit_position = 0;
                            }
                            'd' | 'D' => {
                                // Delete selected deck
                                if let Some(deck) = self.selected_deck() {
                                    self.confirm_message = format!("Delete deck '{}'?", deck.name);
                                    self.show_confirm_dialog = true;
                                }
                            }
                            'i' | 'I' => {
                                // Import deck
                                self.mode = DeckSelectionMode::Import;
                                self.edit_field.clear();
                                self.edit_position = 0;
                            }
                            'e' | 'E' => {
                                // Export deck
                                if let Some(deck) = self.selected_deck().cloned() {
                                    self.edit_field = deck.uuid.to_string();
                                    self.mode = DeckSelectionMode::Export;
                                }
                            }
                            's' | 'S' => {
                                // View statistics
                                if let Some(deck) = self.selected_deck().cloned() {
                                    self.edit_field = deck.uuid.to_string();
                                    self.mode = DeckSelectionMode::Statistics;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Action::Backspace => {
                // Handle backspace for editing
                match self.mode {
                    DeckSelectionMode::Edit | DeckSelectionMode::Create => {
                        if self.edit_position == 0 {
                            self.edit_field.pop();
                        }
                    }
                    DeckSelectionMode::Import | DeckSelectionMode::Export => {
                        self.edit_field.pop();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self) -> Result<()> {
        // Sort decks
        self.sort_decks();
        Ok(())
    }

    fn name(&self) -> &str {
        "deck_selection"
    }
}

impl DeckSelector {
    /// Render header
    fn render_header(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let header_text = match self.mode {
            DeckSelectionMode::Select => "Select a deck to study",
            DeckSelectionMode::Manage => "Manage your decks",
            DeckSelectionMode::Browse => "Browse your collection",
            DeckSelectionMode::Edit => "Edit deck properties",
            DeckSelectionMode::Create => "Create new deck",
            DeckSelectionMode::Delete => "Delete deck confirmation",
            DeckSelectionMode::Import => "Import deck from file",
            DeckSelectionMode::Export => "Export deck to file",
            DeckSelectionMode::Statistics => "Deck statistics and information",
            DeckSelectionMode::ManageCards => "Manage cards in deck",
        };

        let header = Paragraph::new(header_text)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Deck Selection")
                    .border_style(Style::default().fg(Color::Cyan)),
            );

        frame.render_widget(header, area);
        Ok(())
    }

    /// Render controls
    fn render_controls(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let controls_text = match self.mode {
            DeckSelectionMode::Manage => format!(
                "↑↓ Navigate | Enter Edit | C Create | D Delete | I Import | E Export | S Statistics | Esc Back | Sort: {:?}",
                self.sort_order
            ),
            DeckSelectionMode::Edit => {
                if self.edit_position == 0 {
                    "Editing Name | Enter Next | Esc Cancel".to_string()
                } else {
                    "Editing Description | Enter Confirm | Esc Cancel".to_string()
                }
            }
            DeckSelectionMode::Create => {
                if self.edit_position == 0 {
                    "Enter Deck Name | Enter Next | Esc Cancel".to_string()
                } else {
                    "Enter Description (Optional) | Enter Confirm | Esc Cancel".to_string()
                }
            }
            DeckSelectionMode::Delete => {
                "Delete Confirmation | Enter Confirm | Esc Cancel".to_string()
            }
            DeckSelectionMode::Import => {
                "Enter File Path | Enter Confirm | Esc Cancel".to_string()
            }
            DeckSelectionMode::Export => {
                "Enter Export Path | Enter Confirm | Esc Cancel".to_string()
            }
            DeckSelectionMode::Statistics => {
                "Viewing Statistics | Enter Refresh | Esc Back".to_string()
            }
            _ => "↑↓ Navigate | Enter Select | Esc Back".to_string(),
        };

        let controls = Paragraph::new(controls_text)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(controls, area);
        Ok(())
    }

    /// Render deck list
    fn render_deck_list(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let filtered_decks = self.filter_decks();

        let items: Vec<ListItem> = filtered_decks
            .iter()
            .enumerate()
            .map(|(_i, deck)| {
                let is_selected = if let Some(filtered_index) =
                    filtered_decks.iter().position(|d| d.uuid == deck.uuid)
                {
                    filtered_index == self.selected_deck
                } else {
                    false
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let content = format!("{} 📚", deck.name); // Use emoji since no card_count
                ListItem::new(Line::from(Span::styled(content, style)))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Decks")
                .border_style(Style::default().fg(Color::Blue)),
        );

        frame.render_stateful_widget(list, area, &mut self.list_state);
        Ok(())
    }

    /// Render deck info
    fn render_deck_info(&self, frame: &mut Frame, area: Rect, deck: &Deck) -> Result<()> {
        let info_lines = self.get_deck_info_text(deck);

        let info = Paragraph::new(info_lines).wrap(Wrap { trim: true }).block(
            Block::default()
                .borders(Borders::ALL)
                .title(deck.name.clone())
                .border_style(Style::default().fg(Color::Green)),
        );

        frame.render_widget(info, area);
        Ok(())
    }

    /// Render edit/create mode
    fn render_edit_mode(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Name field
                Constraint::Length(3), // Description field
                Constraint::Min(5),    // Instructions
                Constraint::Length(3), // Controls (moved to bottom)
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0])?;

        // Render name field
        let name_field = Paragraph::new(self.edit_field.clone())
            .style(if self.edit_position == 0 {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Deck Name")
                    .border_style(if self.edit_position == 0 {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default().fg(Color::Gray)
                    }),
            );
        frame.render_widget(name_field, chunks[1]);

        // Render description field
        let description = if self.mode == DeckSelectionMode::Edit {
            if let Some(deck) = self.selected_deck() {
                deck.description.as_deref().unwrap_or("")
            } else {
                ""
            }
        } else {
            ""
        };

        let description_field = Paragraph::new(description)
            .style(if self.edit_position == 1 {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Description (Optional)")
                    .border_style(if self.edit_position == 1 {
                        Style::default().fg(Color::Cyan)
                    } else {
                        Style::default().fg(Color::Gray)
                    }),
            );
        frame.render_widget(description_field, chunks[2]);

        // Render instructions
        let instructions = match self.mode {
            DeckSelectionMode::Edit => {
                "Editing deck properties. Use Enter to move between fields or confirm."
            }
            DeckSelectionMode::Create => "Creating new deck. Enter name and optional description.",
            _ => "",
        };

        let instructions_widget = Paragraph::new(instructions)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Instructions")
                    .border_style(Style::default().fg(Color::Yellow)),
            );
        frame.render_widget(instructions_widget, chunks[3]);

        // Render controls (now at bottom)
        self.render_controls(frame, chunks[4])?;

        Ok(())
    }

    /// Render confirmation dialog
    fn render_confirmation_dialog(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Calculate dialog dimensions
        let dialog_width = 50.min(area.width - 4);
        let dialog_height = 8.min(area.height - 4);
        let dialog_area = Rect {
            x: area.x + (area.width - dialog_width) / 2,
            y: area.y + (area.height - dialog_height) / 2,
            width: dialog_width,
            height: dialog_height,
        };

        // Create background overlay
        let background = Block::default().style(Style::default().bg(Color::Black));
        frame.render_widget(background, area);

        // Render dialog border
        let dialog_border = Block::default()
            .borders(Borders::ALL)
            .title("Confirmation")
            .border_style(Style::default().fg(Color::Red));
        frame.render_widget(dialog_border, dialog_area);

        // Create inner area for content
        let inner_area = Rect {
            x: dialog_area.x + 1,
            y: dialog_area.y + 1,
            width: dialog_area.width - 2,
            height: dialog_area.height - 2,
        };

        // Render confirmation message
        let msg_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Message
                Constraint::Length(1), // Buttons
            ])
            .split(inner_area);

        let message = Paragraph::new(self.confirm_message.clone())
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });
        frame.render_widget(message, msg_chunks[0]);

        // Render buttons
        let button_area = msg_chunks[1];
        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(10), // Yes
                Constraint::Length(4),  // Space
                Constraint::Length(10), // No
            ])
            .split(button_area);

        let yes_button = Paragraph::new("  Yes  ")
            .style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));

        let no_button = Paragraph::new("  No  ")
            .style(
                Style::default()
                    .fg(Color::Red)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(yes_button, button_chunks[0]);
        frame.render_widget(no_button, button_chunks[2]);

        Ok(())
    }

    /// Render import/export mode
    fn render_import_export_mode(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // File path field
                Constraint::Min(5),    // Instructions
                Constraint::Length(3), // Controls (moved to bottom)
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0])?;

        // Render file path field
        let field_title = if self.mode == DeckSelectionMode::Import {
            "Import File Path"
        } else {
            "Export File Path"
        };

        let path_field = Paragraph::new(self.edit_field.clone())
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(field_title)
                    .border_style(Style::default().fg(Color::Cyan)),
            );

        frame.render_widget(path_field, chunks[1]);

        // Render instructions
        let instructions = if self.mode == DeckSelectionMode::Import {
            "Enter the path to the TOML file to import.\n\
             Example: /path/to/deck.toml\n\
             Supported format: TOML"
        } else {
            "Enter the export path for the deck.\n\
             Example: /path/to/export.toml\n\
             Format: TOML with optional state data"
        };

        let instructions_paragraph = Paragraph::new(instructions)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Instructions")
                    .border_style(Style::default().fg(Color::Yellow)),
            );

        frame.render_widget(instructions_paragraph, chunks[2]);

        // Render controls (now at bottom)
        self.render_controls(frame, chunks[3])?;

        Ok(())
    }

    /// Render statistics mode
    fn render_statistics_mode(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Statistics content
                Constraint::Length(3), // Controls (moved to bottom)
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0])?;

        // Render statistics content
        let stats_text = format!(
            "Deck Statistics\n\n\
             📊 Total Cards: Loading...\n\
             📚 New Cards: Loading...\n\
             📖 Learning Cards: Loading...\n\
             🔄 Review Cards: Loading...\n\
             ⏰ Due Today: Loading...\n\
             📈 Retention Rate: Loading...\n\
             🔥 Study Streak: Loading...\n\n\
             (Statistics calculation requires integration with StatsEngine)\n\
             Deck UUID: {}",
            self.edit_field
        );

        let stats_paragraph = Paragraph::new(stats_text)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Statistics Overview")
                    .border_style(Style::default().fg(Color::Green)),
            );

        frame.render_widget(stats_paragraph, chunks[1]);

        // Render controls (now at bottom)
        self.render_controls(frame, chunks[2])?;

        Ok(())
    }

    /// Render card management layout
    fn render_card_management_layout(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(15),   // Main content (card list + actions)
                Constraint::Length(3), // Instructions
                Constraint::Length(3), // Controls
            ])
            .split(area);

        // Render header
        let deck_name = self
            .selected_deck()
            .map(|d| d.name.as_str())
            .unwrap_or("Unknown Deck");
        let header = Paragraph::new(format!("🃏 Managing Cards - {}", deck_name))
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Card Management")
                    .border_style(Style::default().fg(Color::Cyan)),
            );
        frame.render_widget(header, chunks[0]);

        // Render main content area
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // Card list
                Constraint::Percentage(40), // Card actions and details
            ])
            .split(chunks[1]);

        // Render card list
        self.render_card_list(frame, main_chunks[0])?;

        // Render card actions and details
        self.render_card_actions_panel(frame, main_chunks[1])?;

        // Render instructions
        let instructions = "↑↓ Navigate Cards | ←→ Actions | Enter Execute | Esc Back";
        let instr = Paragraph::new(instructions)
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            );
        frame.render_widget(instr, chunks[2]);

        // Render controls
        let controls = "N:New A:All Cards F:Find S:Sort by:Date";
        let controls_widget = Paragraph::new(controls)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(controls_widget, chunks[3]);

        Ok(())
    }

    /// Render card list
    fn render_card_list(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // List header
                Constraint::Min(10),   // Card list
            ])
            .split(area);

        // Render list header
        let header = Paragraph::new("📋 Card List")
            .style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            );
        frame.render_widget(header, chunks[0]);

        // Render card list items
        let items: Vec<ListItem> = self
            .cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                let is_selected = i == self.selected_card;
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                // Get card preview (front content truncated)
                let front_preview = if card.content.front.len() > 30 {
                    format!("{}...", &card.content.front[..30])
                } else {
                    card.content.front.clone()
                };

                let content = format!("{} • {}", front_preview, card.state.due.format("%m-%d"));
                ListItem::new(Line::from(Span::styled(content, style)))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} cards ", self.cards.len()))
                .border_style(Style::default().fg(Color::Green)),
        );

        // Create a mutable copy of card_list_state for rendering
        let mut card_list_state = self.card_list_state.clone();
        frame.render_stateful_widget(list, chunks[1], &mut card_list_state);
        Ok(())
    }

    /// Render card actions panel
    fn render_card_actions_panel(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Card details
                Constraint::Min(8),    // Actions
            ])
            .split(area);

        // Render selected card details
        self.render_selected_card_details(frame, chunks[0])?;

        // Render card actions
        self.render_card_actions(frame, chunks[1])?;

        Ok(())
    }

    /// Render selected card details
    fn render_selected_card_details(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(card) = self.cards.get(self.selected_card) {
            let front_preview = if card.content.front.len() > 50 {
                format!("{}...", &card.content.front[..50])
            } else {
                card.content.front.clone()
            };

            let back_preview = if card.content.back.len() > 50 {
                format!("{}...", &card.content.back[..50])
            } else {
                card.content.back.clone()
            };

            let details = format!(
                "🎯 Selected Card #{}\n\
                 Front: {}\n\
                 Back: {}\n\
                 Due: {}\n\
                 State: {:?}",
                self.selected_card + 1,
                front_preview,
                back_preview,
                card.state.due.format("%Y-%m-%d %H:%M"),
                card.state.state
            );

            let details_widget = Paragraph::new(details)
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Card Details")
                        .border_style(Style::default().fg(Color::Blue)),
                );
            frame.render_widget(details_widget, area);
        } else {
            let no_card = Paragraph::new("No cards available")
                .style(Style::default().fg(Color::Gray))
                .alignment(ratatui::layout::Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Card Details")
                        .border_style(Style::default().fg(Color::Gray)),
                );
            frame.render_widget(no_card, area);
        }

        Ok(())
    }

    /// Render card actions
    fn render_card_actions(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        const CARD_ACTIONS: &[&str] = &[
            "📝 Edit Card",
            "🗑️  Delete Card",
            "📊 View Card Stats",
            "📋 Copy Card",
            "⏸️ Suspend Card",
            "▶️ Unsuspend Card",
        ];

        let items: Vec<ListItem> = CARD_ACTIONS
            .iter()
            .enumerate()
            .map(|(i, action)| {
                let style = if i == self.selected_card_action {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(Span::styled(*action, style)))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Actions")
                .border_style(Style::default().fg(Color::Yellow)),
        );

        frame.render_widget(list, area);
        Ok(())
    }
}

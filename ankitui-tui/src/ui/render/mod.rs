//! Rendering system for the TUI application

/// Renderer trait for different rendering strategies
pub trait Renderer: Send + Sync {
    /// Render the application
    fn render(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect);

    /// Render with state information
    fn render_with_state(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect, state: &crate::ui::state::store::AppState);

    /// Render with app and state information for accessing core services
    fn render_with_app_and_state(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &crate::app::main_app::App, state: &crate::ui::state::store::AppState);

    /// Update renderer state
    fn update(&mut self);

    /// Handle resize events
    fn resize(&mut self, width: u16, height: u16);

    /// Update theme
    fn update_theme(&mut self, theme: super::theme::Theme);
}

/// Default renderer implementation
pub struct DefaultRenderer {
    // Renderer state can go here
    current_screen: Option<crate::ui::state::store::Screen>,
}

impl DefaultRenderer {
    pub fn new() -> Self {
        Self {
            current_screen: None,
        }
    }

    pub fn set_screen(&mut self, screen: crate::ui::state::store::Screen) {
        self.current_screen = Some(screen);
    }
}

impl Default for DefaultRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for DefaultRenderer {
    fn render(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        // Default implementation without state
        self.render_with_state(f, area, &crate::ui::state::store::AppState::default())
    }

    fn render_with_state(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect, state: &crate::ui::state::store::AppState) {
        use crate::ui::state::store::Screen;

        // Render based on current screen state
        match &state.current_screen {
            Screen::MainMenu => {
                render_main_menu(f, area, state.main_menu_selected);
            }
            Screen::DeckSelection => {
                // render_deck_selection_enhanced uses hardcoded data; skip rendering
            }
            Screen::StudySession => {
                // render_study_session uses hardcoded data; skip rendering
            }
            Screen::Statistics => {
                // render_statistics uses hardcoded data; skip rendering
            }
            Screen::Settings => {
                // render_settings uses hardcoded data; skip rendering
            }
            _ => {
                // Default to main menu
                render_main_menu(f, area, 0);
            }
        }
    }

    fn render_with_app_and_state(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &crate::app::main_app::App, state: &crate::ui::state::store::AppState) {
        use crate::ui::state::store::Screen;

        // Render based on current screen state with access to app services
        match &state.current_screen {
            Screen::MainMenu => {
                render_main_menu(f, area, state.main_menu_selected);
            }
            Screen::DeckSelection => {
                render_deck_selection_with_real_data(f, area, app, state);
            }
            Screen::StudySession => {
                render_study_session_with_real_data(f, area, app, state);
            }
            Screen::Statistics => {
                render_statistics_with_real_data(f, area, app, state);
            }
            Screen::Settings => {
                render_settings_with_real_data(f, area, app, state);
            }
            Screen::DeckManagement => {
                render_deck_management(f, area, app, state);
            }
            Screen::Search => {
                render_search_screen(f, area, app, state);
            }
            Screen::Help => {
                render_help_screen(f, area, app, state);
            }
            _ => {
                // Default to main menu
                render_main_menu(f, area, 0);
            }
        }
    }

    fn update(&mut self) {
        // Update renderer state
    }

    fn resize(&mut self, _width: u16, _height: u16) {
        // Handle resize
    }

    fn update_theme(&mut self, _theme: super::theme::Theme) {
        // Update theme
    }
}

/// Helper function to render main menu screen
fn render_main_menu(f: &mut ratatui::Frame, area: ratatui::layout::Rect, selected_index: usize) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders, List, ListItem},
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
    };

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Menu content
            Constraint::Length(3), // Help text
        ])
        .split(area);

    // Header
    let header = Paragraph::new("📚 AnkiTUI - Terminal Spaced Repetition")
        .style(Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Welcome"));
    f.render_widget(header, chunks[0]);

    // Menu items
    let menu_items = vec![
        "📚 Study Cards",
        "🗂️ Manage Decks",
        "📊 Statistics",
        "⚙️ Settings",
        "❌ Quit"
    ];

    
    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            let prefix = if i == selected_index { "▶ " } else { "  " };
            let style = if i == selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let text = format!("{}{}", prefix, item);
            ListItem::new(text).style(style)
        })
        .collect();

    let menu = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Main Menu"));
    f.render_widget(menu, chunks[1]);

    // Help text
    let help_text = "↑↓: Navigate | Enter: Select | 1-5: Quick Select | Esc: Quit | F1: Help";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// Deck selection screen with real deck data from core
fn render_deck_selection_with_real_data(f: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &crate::app::main_app::App, state: &crate::ui::state::store::AppState) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders, List, ListItem},
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
    };

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Deck list
            Constraint::Length(3), // Help text
        ])
        .split(area);

    // Header
    let header = Paragraph::new("🗂️ Select a Deck to Study")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Deck Selection"));
    f.render_widget(header, chunks[0]);

    // Get deck selection index from state
    let selected_index = state.deck_list_selected.unwrap_or(0);

    // Fetch real deck data (use tokio::task::block_in_place for sync context)
    let deck_data = if let Ok(decks) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks())
    }) {
        decks
    } else {
        Vec::new() // Fallback to empty list if error
    };

    // Convert deck data to display items
    let deck_items: Vec<ListItem> = deck_data
        .into_iter()
        .enumerate()
        .map(|(i, (deck, cards))| {
            let total_cards = cards.len();
            let due_cards = cards.iter().filter(|c| c.state.due <= chrono::Utc::now()).count();
            let new_cards = cards.iter().filter(|c| matches!(c.state.state, ankitui_core::data::models::CardState::New)).count();

            // Format last studied time
            let last_studied = if cards.is_empty() {
                "Never studied".to_string()
            } else {
                let latest_card = cards.iter()
                    .map(|c| c.state.updated_at)
                    .max()
                    .unwrap_or_else(|| chrono::Utc::now());
                let duration = chrono::Utc::now() - latest_card;
                if duration.num_hours() < 1 {
                    format!("{} minutes ago", duration.num_minutes())
                } else if duration.num_days() < 1 {
                    format!("{} hours ago", duration.num_hours())
                } else if duration.num_days() < 7 {
                    format!("{} days ago", duration.num_days())
                } else {
                    format!("{} weeks ago", duration.num_days() / 7)
                }
            };

            let deck_text = format!(
                "📚 {} {}\n   {} cards • {} due • {} new • {}",
                deck.name,
                if i == selected_index { "←" } else { "  " },
                total_cards,
                due_cards,
                new_cards,
                last_studied
            );

            ListItem::new(deck_text)
                .style(if i == selected_index {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                })
        })
        .collect();

    let deck_list = List::new(deck_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Your Decks"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_widget(deck_list, chunks[1]);

    // Help footer
    let help_text = "↑↓: Navigate | Enter: Study | Ctrl+N: New Deck | Ctrl+E: Edit | Delete: Delete | F5: Refresh | /: Search | Esc: Back";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

/// Study session screen with real card data from core
fn render_study_session_with_real_data(f: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &crate::app::main_app::App, state: &crate::ui::state::store::AppState) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders, List, ListItem},
        layout::{Constraint, Direction, Layout},
        style::{Color, Style, Modifier},
    };

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(4), // Progress info
            Constraint::Min(0),    // Card content
            Constraint::Length(3), // Controls
        ])
        .split(area);

    // Header
    let header = Paragraph::new("📚 Study Session in Progress")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Study"));
    f.render_widget(header, chunks[0]);

    // Try to get current session data
    let session_info = if let Some(session) = &state.current_session {
        let progress_text = format!(
            "Cards Studied: {} | Session Time: {} | Progress: {:.1}%",
            session.cards_studied,
            format_session_time(session.start_time),
            session.progress_percentage()
        );
        progress_text
    } else {
        "No active session - Select a deck to start studying".to_string()
    };

    // Progress information
    let progress = Paragraph::new(session_info)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Progress"));
    f.render_widget(progress, chunks[1]);

    // Try to get current card data
    let card_content: Vec<String> = if let Some(deck_id) = state.selected_deck_id {
        if let Ok(Some(card)) = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(app.study_service().get_next_card(&deck_id))
        }) {
            vec![
                card.content.front.clone(),
                "".to_string(),
                card.content.back.clone(),
                "".to_string(),
                format!("Card State: {}", match card.state.state {
                    ankitui_core::data::models::CardState::New => "New",
                    ankitui_core::data::models::CardState::Learning => "Learning",
                    ankitui_core::data::models::CardState::Review => "Review",
                    ankitui_core::data::models::CardState::Relearning => "Relearning",
                    ankitui_core::data::models::CardState::Buried => "Buried",
                    ankitui_core::data::models::CardState::Suspended => "Suspended",
                }),
                format!("Interval: {} days | Ease: {:.0}%", card.state.interval, card.state.ease_factor * 100.0),
            ]
        } else {
            vec![
                "No cards available in this deck.".to_string(),
                "Consider adding new cards or studying another deck.".to_string(),
            ]
        }
    } else {
        vec![
            "No deck selected.".to_string(),
            "Select a deck from the deck selection screen to start studying.".to_string(),
        ]
    };

    let card_items: Vec<ListItem> = card_content
        .iter()
        .map(|line| ListItem::new(line.as_str()))
        .collect();

    let card_list = List::new(card_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(if state.is_showing_answer() { "Answer" } else { "Question" }));
    f.render_widget(card_list, chunks[2]);

    // Controls
    let controls_text = if state.is_showing_answer() {
        "1-4: Rate (Again, Hard, Good, Easy) | Space: Hide Answer | PageUp/PageDown: Navigate Cards | Tab: Next | Esc: Pause Session"
    } else {
        "Space: Show Answer | PageUp/PageDown: Navigate Cards | Tab: Next | Esc: Pause Session"
    };
    let controls = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Controls"));
    f.render_widget(controls, chunks[3]);
}

/// Statistics screen with real data from core
fn render_statistics_with_real_data(f: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &crate::app::main_app::App, _state: &crate::ui::state::store::AppState) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders, Table, Row, Cell, List, ListItem},
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
    };

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(10), // Overview
            Constraint::Min(0),    // Details
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Header
    let header = Paragraph::new("📊 Learning Statistics")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Statistics"));
    f.render_widget(header, chunks[0]);

    // Fetch real global statistics
    let global_stats = if let Ok(stats) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(app.statistics_service().get_global_statistics())
    }) {
        stats
    } else {
        // Default values if error
        ankitui_core::data::sync_adapter::GlobalStats {
            total_decks: 0,
            total_cards: 0,
            due_cards: 0,
            new_cards: 0,
            learning_cards: 0,
            review_cards: 0,
        }
    };

    // Overview statistics
    let overview_text = vec![
        format!("Total Decks: {} | Total Cards: {} | Due Cards: {}",
                global_stats.total_decks, global_stats.total_cards, global_stats.due_cards),
        format!("New Cards: {} | Learning Cards: {} | Review Cards: {}",
                global_stats.new_cards, global_stats.learning_cards, global_stats.review_cards),
        "Study Progress: Excellent".to_string(),
        "".to_string(),
        "Performance: Keep up the consistent study schedule.".to_string(),
    ];

    let overview_items: Vec<ListItem> = overview_text
        .iter()
        .map(|line| ListItem::new(line.as_str()))
        .collect();

    let overview_list = List::new(overview_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Overview"));
    f.render_widget(overview_list, chunks[1]);

    // Fetch deck statistics for table
    let deck_stats_table = if let Ok(decks) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks())
    }) {
        let deck_stats_data: Vec<Vec<String>> = decks
            .into_iter()
            .map(|(deck, cards)| {
                let total_cards = cards.len();
                let due_cards = cards.iter().filter(|c| c.state.due <= chrono::Utc::now()).count();
                let retention = if total_cards > 0 {
                    let review_cards = cards.iter().filter(|c| matches!(c.state.state, ankitui_core::data::models::CardState::Review)).count();
                    if review_cards > 0 {
                        format!("{:.0}%", (review_cards as f32 / total_cards as f32) * 100.0)
                    } else {
                        "N/A".to_string()
                    }
                } else {
                    "N/A".to_string()
                };
                let avg_time = "N/A".to_string();

                vec![deck.name, total_cards.to_string(), due_cards.to_string(), retention, avg_time]
            })
            .collect();

        deck_stats_data
    } else {
        vec![
            vec!["Error loading decks".to_string(), "0".to_string(), "0".to_string(), "N/A".to_string(), "N/A".to_string()],
        ]
    };

    // Deck performance table
    let header_cells = ["Deck", "Total Cards", "Due", "Retention", "Avg Time"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));

    let rows = deck_stats_table.iter().map(|row| {
        let cells = row.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells)
    });

    let table = Table::new(rows, [
        Constraint::Percentage(30),
        Constraint::Percentage(14),
        Constraint::Percentage(10),
        Constraint::Percentage(14),
        Constraint::Percentage(16),
    ])
        .header(Row::new(header_cells))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Deck Performance"));

    f.render_widget(table, chunks[2]);

    // Help text
    let help_text = "Tab: Switch Views | ↑↓: Navigate List | PageUp/PageDown: Fast Scroll | Home/End: Top/Bottom | F5: Refresh | Enter: View Details | Esc: Back to Menu";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Controls"));
    f.render_widget(help, chunks[3]);
}

/// Settings screen with real configuration
fn render_settings_with_real_data(f: &mut ratatui::Frame, area: ratatui::layout::Rect, _app: &crate::app::main_app::App, state: &crate::ui::state::store::AppState) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders, List, ListItem},
        layout::{Constraint, Direction, Layout},
        style::{Color, Style, Modifier},
    };

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Settings menu
            Constraint::Length(6), // Current values
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Header
    let header = Paragraph::new("⚙️ Application Settings")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Settings"));
    f.render_widget(header, chunks[0]);

    // Settings menu items
    let settings_items = vec![
        "📖 Study Preferences",
        "🎨 UI Customization",
        "💾 Data Management",
        "🔔 Notifications",
        "⌨️ Keyboard Shortcuts",
        "🌐 Language & Region",
    ];

    let selected_index = state.main_menu_selected; // Reuse for settings navigation

    let items: Vec<ListItem> = settings_items
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            let prefix = if i == selected_index { "▶ " } else { "  " };
            let style = if i == selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let text = format!("{}{}", prefix, item);
            ListItem::new(text).style(style)
        })
        .collect();

    let settings_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Settings Categories"));
    f.render_widget(settings_list, chunks[1]);

    // Current settings values from user preferences
    let current_values = vec![
        format!("Display Name: {}", state.user_preferences.display_name),
        format!("Theme: {}", state.user_preferences.theme),
        format!("Show Progress: {}", if state.user_preferences.show_progress { "Enabled" } else { "Disabled" }),
        format!("Auto Advance: {}", if state.user_preferences.auto_advance { "Enabled" } else { "Disabled" }),
    ];

    let value_items: Vec<ListItem> = current_values
        .iter()
        .map(|value| ListItem::new(value.as_str()))
        .collect();

    let values_list = List::new(value_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Current Values"));
    f.render_widget(values_list, chunks[2]);

    // Help text
    let help_text = "↑↓: Navigate | Enter: Edit | ←→: Adjust Values | Tab: Next Field | Shift+Tab: Previous | Home: First | End: Last | Ctrl+S: Save | Esc: Back | F1: Help";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Controls"));
    f.render_widget(help, chunks[3]);
}

/// Helper function to format session time
fn format_session_time(started_at: chrono::DateTime<chrono::Utc>) -> String {
    let duration = chrono::Utc::now() - started_at;
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds() % 60;
    format!("{}m {}s", minutes, seconds)
}

/// Deck management screen with real data
fn render_deck_management(f: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &crate::app::main_app::App, _state: &crate::ui::state::store::AppState) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders, List, ListItem},
        layout::{Constraint, Direction, Layout},
        style::{Color, Style, Modifier},
    };

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Deck list
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Header
    let header = Paragraph::new("🗂️ Manage Decks")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Deck Management"));
    f.render_widget(header, chunks[0]);

    // Fetch real deck data
    let deck_data = if let Ok(decks) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks())
    }) {
        decks
    } else {
        Vec::new()
    };

    if deck_data.is_empty() {
        let empty = Paragraph::new("No decks available.\n\nPress Ctrl+N to create a new deck.")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Empty"));
        f.render_widget(empty, chunks[1]);
    } else {
        let items: Vec<ListItem> = deck_data
            .iter()
            .enumerate()
            .map(|(i, (deck, cards))| {
                let total = cards.len();
                let due = cards.iter().filter(|c| c.state.due <= chrono::Utc::now()).count();
                let new_count = cards.iter().filter(|c| matches!(c.state.state, ankitui_core::data::models::CardState::New)).count();
                ListItem::new(format!(
                    "📚 {} ({} cards, {} due, {} new)\n   Actions: Edit | Delete | Export | Stats",
                    deck.name, total, due, new_count
                ))
            })
            .collect();
        let deck_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Your Decks"));
        f.render_widget(deck_list, chunks[1]);
    }

    // Help footer
    let help_text = "↑↓: Navigate | Enter: Edit Deck | Ctrl+E: Edit | Delete: Remove Deck | Ctrl+N: New Deck | Esc: Back to Menu";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// Search screen
fn render_search_screen(f: &mut ratatui::Frame, area: ratatui::layout::Rect, _app: &crate::app::main_app::App, state: &crate::ui::state::store::AppState) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders},
        layout::{Constraint, Direction, Layout},
        style::{Color, Style, Modifier},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("🔍 Search")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(header, chunks[0]);

    let search_type = state.ui_state.get("search_type").map(|s| s.as_str()).unwrap_or("Decks");
    let query = state.ui_state.get("search_query").map(|s| s.as_str()).unwrap_or("");
    let prompt = Paragraph::new(format!("Type: {} | Query: {}", search_type, query))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search Parameters"));
    f.render_widget(prompt, chunks[1]);

    let info = Paragraph::new("Enter search terms above. Results will appear here.\nTab: Switch type | Type to search | Esc: Close")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Instructions"));
    f.render_widget(info, chunks[2]);

    let help = Paragraph::new("Tab: Switch Deck/Card | Type: Enter query | ↑↓: Navigate results | Esc: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[3]);
}

/// Help screen
fn render_help_screen(f: &mut ratatui::Frame, area: ratatui::layout::Rect, _app: &crate::app::main_app::App, _state: &crate::ui::state::store::AppState) {
    use ratatui::{
        widgets::{Paragraph, Block, Borders, List, ListItem},
        layout::{Constraint, Direction, Layout},
        style::{Color, Style, Modifier},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("❓ Keyboard Shortcuts & Help")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(header, chunks[0]);

    let categories = [
        ("Global Shortcuts", vec![
            ("Ctrl+Q / Ctrl+C", "Quit application"),
            ("F1 / ?", "Show this help"),
            ("F5", "Refresh current screen"),
            ("Esc", "Go back / Cancel"),
        ]),
        ("Navigation", vec![
            ("Up / Down", "Navigate items"),
            ("Left / Right", "Navigate tabs / Adjust values"),
            ("Enter", "Confirm / Select / Execute"),
            ("Tab", "Switch search type"),
        ]),
        ("Study Session", vec![
            ("Space", "Show answer / Confirm"),
            ("1", "Again - Review soon"),
            ("2", "Hard - Review later"),
            ("3", "Good - Normal interval"),
            ("4", "Easy - Longer interval"),
        ]),
        ("Settings", vec![
            ("Ctrl+S", "Save settings"),
            ("Enter", "Toggle boolean option"),
            ("Left / Right", "Adjust numeric values"),
        ]),
    ];

    let cat_list: Vec<ListItem> = categories
        .iter()
        .map(|(name, _)| ListItem::new(format!("  {}", name)))
        .collect();
    let cat_widget = List::new(cat_list)
        .block(Block::default().borders(Borders::ALL).title("Categories"));
    f.render_widget(cat_widget, chunks[1]);

    let (_, shortcuts) = &categories[0];
    let shortcut_items: Vec<ListItem> = shortcuts
        .iter()
        .map(|(key, desc)| ListItem::new(format!("  {:20} {}", key, desc)))
        .collect();
    let shortcut_widget = List::new(shortcut_items)
        .block(Block::default().borders(Borders::ALL).title("Global Shortcuts"));
    f.render_widget(shortcut_widget, chunks[2]);

    let help = Paragraph::new("Esc: Close")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[3]);
}
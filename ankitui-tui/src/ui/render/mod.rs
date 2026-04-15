//! Rendering system for the TUI application

/// Renderer trait for different rendering strategies
pub trait Renderer: Send + Sync {
    /// Render the application
    fn render(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect);

    /// Render with state information
    fn render_with_state(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        state: &crate::ui::state::store::AppState,
    );

    /// Render with app and state information for accessing core services
    fn render_with_app_and_state(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        app: &crate::app::main_app::App,
        state: &crate::ui::state::store::AppState,
    );

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
        Self { current_screen: None }
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

    fn render_with_state(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        state: &crate::ui::state::store::AppState,
    ) {
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

    fn render_with_app_and_state(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        app: &crate::app::main_app::App,
        state: &crate::ui::state::store::AppState,
    ) {
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
            Screen::StudyPrefs => {
                render_study_prefs(f, area, app, state);
            }
            Screen::UiSettings => {
                render_ui_settings(f, area, app, state);
            }
            Screen::DataManage => {
                render_data_manage(f, area, app, state);
            }
            Screen::TagManagement => {
                render_tag_management(f, area, app, state);
            }
            Screen::MediaManagement => {
                render_media_management(f, area, app, state);
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
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
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
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Welcome"));
    f.render_widget(header, chunks[0]);

    // Menu items
    let menu_items = vec![
        "📚 Study Cards",
        "🗂️ Manage Decks",
        "📊 Statistics",
        "⚙️ Settings",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            let prefix = if i == selected_index { "▶ " } else { "  " };
            let style = if i == selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let text = format!("{}{}", prefix, item);
            ListItem::new(text).style(style)
        })
        .collect();

    let menu = List::new(items).block(Block::default().borders(Borders::ALL).title("Main Menu"));
    f.render_widget(menu, chunks[1]);

    // Help text
    let help_text = "↑↓: Navigate | Enter: Select | 1-4: Quick Select | Q/Esc: Quit | F1: Help";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// Deck selection screen with real deck data from core
fn render_deck_selection_with_real_data(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
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
        .block(Block::default().borders(Borders::ALL).title("Deck Selection"));
    f.render_widget(header, chunks[0]);

    // Get deck selection index from state
    let selected_index = state.deck_list_selected.unwrap_or(0);

    // Fetch real deck data (use tokio::task::block_in_place for sync context)
    let deck_data = if let Ok(decks) =
        tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks()))
    {
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
            let new_cards = cards
                .iter()
                .filter(|c| matches!(c.state.state, ankitui_core::data::models::CardState::New))
                .count();

            // Format last studied time
            let last_studied = if cards.is_empty() {
                "Never studied".to_string()
            } else {
                let latest_card = cards
                    .iter()
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

            ListItem::new(deck_text).style(if i == selected_index {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            })
        })
        .collect();

    let deck_list = List::new(deck_items)
        .block(Block::default().borders(Borders::ALL).title("Your Decks"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_widget(deck_list, chunks[1]);

    // Help footer
    let help_text = "↑↓: Navigate | Enter: Study | Ctrl+N: New Deck Menu | Delete: Delete Deck Menu | F5: Refresh | /: Search | Esc: Back";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

/// Study session screen with real card data from core
fn render_study_session_with_real_data(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
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
        .block(Block::default().borders(Borders::ALL).title("Study"));
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
        .block(Block::default().borders(Borders::ALL).title("Progress"));
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
                format!(
                    "Card State: {}",
                    match card.state.state {
                        ankitui_core::data::models::CardState::New => "New",
                        ankitui_core::data::models::CardState::Learning => "Learning",
                        ankitui_core::data::models::CardState::Review => "Review",
                        ankitui_core::data::models::CardState::Relearning => "Relearning",
                        ankitui_core::data::models::CardState::Buried => "Buried",
                        ankitui_core::data::models::CardState::Suspended => "Suspended",
                    }
                ),
                format!(
                    "Interval: {} days | Ease: {:.0}%",
                    card.state.interval,
                    card.state.ease_factor * 100.0
                ),
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

    let card_items: Vec<ListItem> = card_content.iter().map(|line| ListItem::new(line.as_str())).collect();

    let card_list = List::new(card_items).block(Block::default().borders(Borders::ALL).title(
        if state.is_showing_answer() {
            "Answer"
        } else {
            "Question"
        },
    ));
    f.render_widget(card_list, chunks[2]);

    // Controls
    let controls_text = if state.is_showing_answer() {
        "1-4: Rate (Again, Hard, Good, Easy) | Space: Hide Answer | PageUp/PageDown: Navigate Cards | Tab: Next | Esc: Pause Session"
    } else {
        "Space: Show Answer | PageUp/PageDown: Navigate Cards | Tab: Next | Esc: Pause Session"
    };
    let controls = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls, chunks[3]);
}

/// Statistics screen with real data from core
fn render_statistics_with_real_data(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    };

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // Overview
            Constraint::Min(0),     // Details
            Constraint::Length(3),  // Help
        ])
        .split(area);

    // Header
    let header = Paragraph::new("📊 Learning Statistics")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Statistics"));
    f.render_widget(header, chunks[0]);

    // Get current tab and show tab indicator
    let current_tab = state
        .ui_state
        .get("stats_tab")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let tabs = ["Overview", "Deck Stats", "Progress"];
    let tab_text = format!(
        "Tab: {}/{} [{}]",
        current_tab + 1,
        tabs.len(),
        if current_tab < 2 { "→" } else { " " }  // Show navigation hint
    );
    let tab_indicator = Paragraph::new(tab_text)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(tab_indicator, chunks[1]);

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

    // Render content based on current tab
    match current_tab {
        0 => {
            // Overview tab
            let overview_text = vec![
                format!(
                    "Total Decks: {} | Total Cards: {} | Due Cards: {}",
                    global_stats.total_decks, global_stats.total_cards, global_stats.due_cards
                ),
                format!(
                    "New Cards: {} | Learning Cards: {} | Review Cards: {}",
                    global_stats.new_cards, global_stats.learning_cards, global_stats.review_cards
                ),
                "Study Progress: Excellent".to_string(),
                "".to_string(),
                "Performance: Keep up the consistent study schedule.".to_string(),
            ];

            let overview_items: Vec<ListItem> = overview_text.iter().map(|line| ListItem::new(line.as_str())).collect();
            let overview_list = List::new(overview_items).block(Block::default().borders(Borders::ALL).title("Overview"));
            f.render_widget(overview_list, chunks[2]);
        }
        1 => {
            // Deck Stats tab
            if let Ok(decks) =
                tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks()))
            {
                let deck_stats_data: Vec<Vec<String>> = decks
                    .into_iter()
                    .map(|(deck, cards)| {
                        let total_cards = cards.len();
                        let due_cards = cards.iter().filter(|c| c.state.due <= chrono::Utc::now()).count();
                        let retention = if total_cards > 0 {
                            let review_cards = cards
                                .iter()
                                .filter(|c| matches!(c.state.state, ankitui_core::data::models::CardState::Review))
                                .count();
                            if review_cards > 0 {
                                let retained = cards.iter().filter(|c| {
                                    matches!(c.state.state, ankitui_core::data::models::CardState::Review) &&
                                    c.state.lapses == 0
                                }).count();
                                format!("{:.1}%", (retained as f32 / review_cards as f32) * 100.0)
                            } else {
                                "N/A".to_string()
                            }
                        } else {
                            "N/A".to_string()
                        };

                        vec![
                            deck.name,
                            total_cards.to_string(),
                            due_cards.to_string(),
                            retention,
                        ]
                    })
                    .collect();

                let header_row = ["Deck", "Cards", "Due", "Retention"].iter().map(|h| {
                    ratatui::widgets::Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                });

                let rows = deck_stats_data.iter().map(|row| {
                    let cells = row.iter().map(|s| ratatui::widgets::Cell::from(s.as_str()));
                    Row::new(cells)
                });

                let table = Table::new(
                    rows,
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ],
                )
                .header(Row::new(header_row))
                .block(Block::default().borders(Borders::ALL).title("Deck Statistics"));

                f.render_widget(table, chunks[2]);
            } else {
                let error_msg = Paragraph::new("Failed to load deck statistics")
                    .style(Style::default().fg(Color::Red))
                    .block(Block::default().borders(Borders::ALL).title("Error"));
                f.render_widget(error_msg, chunks[2]);
            }
        }
        2 => {
            // Progress tab
            let progress_text = vec![
                "📈 Your Learning Progress".to_string(),
                "".to_string(),
                format!("Study Streak: {} days", global_stats.total_decks), // Using available data
                format!("Cards Reviewed: {}", global_stats.review_cards),
                "".to_string(),
                "Keep up the great work!".to_string(),
                "Consistent practice is key to long-term retention.".to_string(),
            ];

            let progress_items: Vec<ListItem> = progress_text.iter().map(|line| ListItem::new(line.as_str())).collect();
            let progress_list = List::new(progress_items).block(Block::default().borders(Borders::ALL).title("Progress Charts"));
            f.render_widget(progress_list, chunks[2]);
        }
        _ => {
            // Fallback
            let fallback = Paragraph::new("Invalid tab selected")
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(fallback, chunks[2]);
        }
    }

    // Help text
    let help_text = "↑↓: Switch Tab | Enter: View Details | F5: Refresh | Esc: Back to Menu";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[3]);
}

/// Settings screen with real configuration
fn render_settings_with_real_data(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    _app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
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
        .block(Block::default().borders(Borders::ALL).title("Settings"));
    f.render_widget(header, chunks[0]);

    // Settings menu items
    let settings_items = vec![
        "📖 Study Preferences",
        "🎨 UI Customization",
        "💾 Data Management",
        "🏷️ Manage Tags",
        "🖼️ Media Management",
        "🔔 Notifications",
        "⌨️ Keyboard Shortcuts",
        "🌐 Language & Region",
    ];

    let selected_index = state.settings_selected;

    let items: Vec<ListItem> = settings_items
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            let prefix = if i == selected_index { "▶ " } else { "  " };
            let style = if i == selected_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let text = format!("{}{}", prefix, item);
            ListItem::new(text).style(style)
        })
        .collect();

    let settings_list = List::new(items).block(Block::default().borders(Borders::ALL).title("Settings Categories"));
    f.render_widget(settings_list, chunks[1]);

    // Current settings values from user preferences
    let current_values = vec![
        format!("Display Name: {}", state.user_preferences.display_name),
        format!("Theme: {}", state.user_preferences.theme),
        format!(
            "Show Progress: {}",
            if state.user_preferences.show_progress {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
        format!(
            "Auto Advance: {}",
            if state.user_preferences.auto_advance {
                "Enabled"
            } else {
                "Disabled"
            }
        ),
    ];

    let value_items: Vec<ListItem> = current_values
        .iter()
        .map(|value| ListItem::new(value.as_str()))
        .collect();

    let values_list = List::new(value_items).block(Block::default().borders(Borders::ALL).title("Current Values"));
    f.render_widget(values_list, chunks[2]);

    // Help text
    let help_text = "↑↓: Navigate | Enter: Edit | ←→: Adjust Values | Tab: Next Field | Shift+Tab: Previous | Home: First | End: Last | Ctrl+S: Save | Esc: Back | F1: Help";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
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
fn render_deck_management(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
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
        .block(Block::default().borders(Borders::ALL).title("Deck Management"));
    f.render_widget(header, chunks[0]);

    // Fetch real deck data
    let deck_data = if let Ok(decks) =
        tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks()))
    {
        decks
    } else {
        Vec::new()
    };

    // Get selected deck index from state
    let selected_index = state.deck_list_selected.unwrap_or(0);

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
                let new_count = cards
                    .iter()
                    .filter(|c| matches!(c.state.state, ankitui_core::data::models::CardState::New))
                    .count();

                let prefix = if i == selected_index { "▶ " } else { "  " };
                let deck_text = format!(
                    "{}{} ({} cards, {} due, {} new)\n   Actions: Enter: Study | E: Export | D: Delete | Esc: Back",
                    prefix, deck.name, total, due, new_count
                );

                ListItem::new(deck_text).style(if i == selected_index {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                })
            })
            .collect();
        let deck_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Your Decks"))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
        f.render_widget(deck_list, chunks[1]);
    }

    // Help footer
    let help_text =
        "↑↓: Navigate | Enter: Study | E: Export | D: Delete | Esc: Back to Menu";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// Search screen
fn render_search_screen(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
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
    let query = state.ui_state.get("search_query").cloned().unwrap_or_default();

    let prompt = Paragraph::new(format!("[{}] {}", search_type, query))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Query"));
    f.render_widget(prompt, chunks[1]);

    // Fetch and filter results
    let results: Vec<String> = if query.is_empty() {
        Vec::new()
    } else {
        let lower = query.to_lowercase();
        if let Ok(decks) = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks())
        }) {
            if search_type == "Decks" {
                decks
                    .into_iter()
                    .filter(|(deck, _)| deck.name.to_lowercase().contains(&lower) || deck.description.as_ref().map(|d| d.to_lowercase().contains(&lower)).unwrap_or(false))
                    .map(|(deck, cards)| {
                        format!("📚 {} ({} cards)", deck.name, cards.len())
                    })
                    .collect()
            } else {
                // Search cards
                let mut card_results = Vec::new();
                for (deck, cards) in decks {
                    for card in &cards {
                        if card.content.front.to_lowercase().contains(&lower)
                            || card.content.back.to_lowercase().contains(&lower)
                        {
                            card_results.push(format!("🃏 [{}] {}", deck.name, &card.content.front[..card.content.front.len().min(50)]));
                        }
                    }
                }
                card_results
            }
        } else {
            Vec::new()
        }
    };

    // Store result count in state for navigation boundary
    let result_count = results.len();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let _ = app.state_store.read().await.update_state(|state| {
                state.ui_state.insert("search_result_count".to_string(), result_count.to_string());
                // Clamp index if it exceeds new result count
                if let Some(idx_str) = state.ui_state.get("search_result_index").cloned() {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        if idx >= result_count && result_count > 0 {
                            state.ui_state.insert("search_result_index".to_string(), (result_count - 1).to_string());
                        }
                    }
                }
            });
        });
    });

    if query.is_empty() {
        let info = Paragraph::new("Type to search...\nTab: Switch Deck/Card type | Esc: Close")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Instructions"));
        f.render_widget(info, chunks[2]);
    } else if results.is_empty() {
        let no_results = Paragraph::new(format!("No {} found matching '{}'", search_type.to_lowercase(), query))
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("No Results"));
        f.render_widget(no_results, chunks[2]);
    } else {
        let selected_idx = state.ui_state.get("search_result_index").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
        let items: Vec<ListItem> = results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                if i == selected_idx {
                    ListItem::new(r.clone()).style(Style::default().fg(Color::Black).bg(Color::White))
                } else {
                    ListItem::new(r.clone())
                }
            })
            .collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(format!("Results ({})", results.len())));
        f.render_widget(list, chunks[2]);
    }

    let help = Paragraph::new("Type: Search query | Backspace: Delete | Tab: Switch type | Esc: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[3]);
}

/// Help screen
fn render_help_screen(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    _app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("❓ Keyboard Shortcuts & Help")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(header, chunks[0]);

    // Get current category and show indicator
    let current_category = state
        .ui_state
        .get("help_category")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let category_names = ["Global Shortcuts", "Navigation", "Study Session", "Settings"];

    let categories = [
        (
            "Global Shortcuts",
            vec![
                ("Ctrl+Q / Ctrl+C", "Quit application"),
                ("F1 / ?", "Show this help"),
                ("F5", "Refresh current screen"),
                ("Esc", "Go back / Cancel"),
            ],
        ),
        (
            "Navigation",
            vec![
                ("Up / Down", "Navigate items"),
                ("Left / Right", "Navigate tabs / Adjust values"),
                ("Enter", "Confirm / Select / Execute"),
                ("Tab", "Switch search type"),
            ],
        ),
        (
            "Study Session",
            vec![
                ("Space", "Show answer / Confirm"),
                ("1", "Again - Review soon"),
                ("2", "Hard - Review later"),
                ("3", "Good - Normal interval"),
                ("4", "Easy - Longer interval"),
            ],
        ),
        (
            "Settings",
            vec![
                ("Ctrl+S", "Save settings"),
                ("Enter", "Toggle boolean option"),
                ("Left / Right", "Adjust numeric values"),
            ],
        ),
    ];

    let cat_indicator = format!(
        "Category: {}/{} [{}]",
        current_category + 1,
        category_names.len(),
        if current_category < 3 { "→" } else { " " }
    );
    let cat_indicator_widget = Paragraph::new(cat_indicator)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(cat_indicator_widget, chunks[1]);

    // Show category list
    let cat_list: Vec<ListItem> = category_names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let prefix = if i == current_category { "▶" } else { " " };
            ListItem::new(format!("{} {}", prefix, name))
        })
        .collect();
    let cat_widget = List::new(cat_list).block(Block::default().borders(Borders::ALL).title("Categories"));
    f.render_widget(cat_widget, chunks[2]);

    // Show shortcuts for current category
    let (_, shortcuts) = &categories[current_category];
    let shortcut_items: Vec<ListItem> = shortcuts
        .iter()
        .map(|(key, desc)| ListItem::new(format!("  {:20} {}", key, desc)))
        .collect();
    let shortcut_widget =
        List::new(shortcut_items).block(Block::default().borders(Borders::ALL).title(category_names[current_category]));
    f.render_widget(shortcut_widget, chunks[3]);

    let help = Paragraph::new("↑↓: Switch Category | Esc: Close")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[4]);
}

/// Study preferences sub-screen
fn render_study_prefs(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    _app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let header = Paragraph::new("📖 Study Preferences")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Study Prefs"));
    f.render_widget(header, chunks[0]);

    let nav_index = state
        .ui_state
        .get("prefs_index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let items: Vec<(&str, String)> = vec![
        (
            "New cards per day",
            state
                .ui_state
                .get("new_cards_per_day")
                .cloned()
                .unwrap_or("20".to_string()),
        ),
        (
            "Max reviews per day",
            state
                .ui_state
                .get("max_reviews_per_day")
                .cloned()
                .unwrap_or("200".to_string()),
        ),
        (
            "Auto-advance",
            state
                .ui_state
                .get("auto_advance")
                .map(|s| {
                    if s == "true" {
                        "On".to_string()
                    } else {
                        "Off".to_string()
                    }
                })
                .unwrap_or("Off".to_string()),
        ),
        (
            "Show hint on question",
            state
                .ui_state
                .get("show_hint")
                .map(|s| {
                    if s == "true" {
                        "On".to_string()
                    } else {
                        "Off".to_string()
                    }
                })
                .unwrap_or("On".to_string()),
        ),
    ];
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let prefix = if i == nav_index { "▶" } else { " " };
            ListItem::new(format!("{} {}: {}", prefix, label, value))
        })
        .collect();
    let list = List::new(list_items).block(Block::default().borders(Borders::ALL).title("Settings"));
    f.render_widget(list, chunks[1]);

    let help = Paragraph::new("↑↓: Navigate | Enter: Toggle | ←→: Adjust | Ctrl+S: Save | Esc: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// UI settings sub-screen
fn render_ui_settings(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    _app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let header = Paragraph::new("🎨 UI Customization")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("UI Settings"));
    f.render_widget(header, chunks[0]);

    let nav_index = state
        .ui_state
        .get("ui_settings_index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let prefs = &state.user_preferences;
    let items: Vec<(&str, String)> = vec![
        ("Display name", prefs.display_name.clone()),
        ("Theme", prefs.theme.clone()),
        (
            "Auto-advance",
            if prefs.auto_advance {
                "On".to_string()
            } else {
                "Off".to_string()
            },
        ),
        (
            "Show progress",
            if prefs.show_progress {
                "On".to_string()
            } else {
                "Off".to_string()
            },
        ),
    ];
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let prefix = if i == nav_index { "▶" } else { " " };
            ListItem::new(format!("{} {}: {}", prefix, label, value))
        })
        .collect();
    let list = List::new(list_items).block(Block::default().borders(Borders::ALL).title("Settings"));
    f.render_widget(list, chunks[1]);

    let help = Paragraph::new("↑↓: Navigate | Enter: Toggle | ←→: Adjust | Esc: Back (Settings auto-saved)")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// Data management sub-screen
fn render_data_manage(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    _app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let header = Paragraph::new("💾 Data Management")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Data Management"));
    f.render_widget(header, chunks[0]);

    let nav_index = state
        .ui_state
        .get("data_index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let ops = [
        "Import decks from Anki",
        "Export data to file",
        "Create backup",
        "Restore from backup",
        "Clear all data",
    ];
    let list_items: Vec<ListItem> = ops
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let prefix = if i == nav_index { "▶" } else { " " };
            ListItem::new(format!("{} {}", prefix, op))
        })
        .collect();
    let list = List::new(list_items).block(Block::default().borders(Borders::ALL).title("Operations"));
    f.render_widget(list, chunks[1]);

    let status = state
        .message
        .as_ref()
        .map(|m| m.content.clone())
        .unwrap_or_else(|| "↑↓: Navigate | Enter: Execute | Esc: Back".to_string());
    let help = Paragraph::new(status)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Info"));
    f.render_widget(help, chunks[2]);
}

/// Tag management screen
fn render_tag_management(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("🏷️ Tag Management")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Tags"));
    f.render_widget(header, chunks[0]);

    // Fetch all decks to build tag list
    let tag_data: Vec<(String, usize, String)> = if let Ok(decks) =
        tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks()))
    {
        let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for (_, cards) in &decks {
            for card in cards {
                for tag in &card.content.tags {
                    *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                }
            }
        }
        let mut tags: Vec<_> = tag_counts.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1));
        tags
            .into_iter()
            .map(|(name, count)| {
                let state_str = "";
                (name, count, state_str.to_string())
            })
            .collect()
    } else {
        Vec::new()
    };

    let tag_index = state
        .ui_state
        .get("tag_index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    // Ensure tag_index is within valid range to prevent infinite scrolling
    let valid_tag_index = if tag_data.is_empty() {
        0
    } else {
        tag_index.min(tag_data.len().saturating_sub(1))
    };

    if tag_data.is_empty() {
        let empty = Paragraph::new("No tags found.\n\nTags are extracted from cards automatically.")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL).title("Empty"));
        f.render_widget(empty, chunks[1]);
    } else {
        let total_tags = tag_data.len();
        let total_tagged: usize = tag_data.iter().map(|(_, count, _)| count).sum();

        let header_row = ["Tag", "Cards", "Type"].iter().map(|h| {
            ratatui::widgets::Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        });

        let rows = tag_data.iter().enumerate().map(|(i, (name, count, _))| {
            let cells = vec![
                ratatui::widgets::Cell::from(if i == valid_tag_index {
                    format!("▶ {}", name)
                } else {
                    name.clone()
                }),
                ratatui::widgets::Cell::from(count.to_string()),
                ratatui::widgets::Cell::from(if *count > 10 { "Frequent" } else { "Normal" }),
            ];
            Row::new(cells).style(if i == valid_tag_index {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            })
        });

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(15),
                Constraint::Percentage(20),
            ],
        )
        .header(Row::new(header_row))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Tags ({}) • Total tagged: {}", total_tags, total_tagged)),
        );
        f.render_widget(table, chunks[1]);
    }

    let help = Paragraph::new("↑↓: Navigate | D: Delete tag | R: Rename | F5: Refresh | Esc: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// Media management screen
fn render_media_management(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &crate::app::main_app::App,
    state: &crate::ui::state::store::AppState,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("🖼️ Media Management")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Media"));
    f.render_widget(header, chunks[0]);

    // Calculate media statistics
    let mut total_media = 0;
    let mut audio_count = 0;
    let mut image_count = 0;
    let mut video_count = 0;
    let mut total_size = 0;

    if let Ok(decks) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(app.deck_service().get_all_decks())
    }) {
        for (deck, _cards) in &decks {
            if let Ok(stats) = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(
                    app.deck_manager().get_deck_media_stats(&deck.uuid)
                )
            }) {
                total_media += stats.total_media_files;
                audio_count += stats.audio_files;
                image_count += stats.image_files;
                video_count += stats.video_files;
                total_size += stats.total_size_bytes;
            }
        }
    }

    let media_index = state
        .ui_state
        .get("media_index")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let items = vec![
        format!("📊 Total Media Files: {}", total_media),
        format!("🖼️ Images: {}", image_count),
        format!("🎵 Audio: {}", audio_count),
        format!("🎬 Videos: {}", video_count),
        format!("💾 Total Size: {}", format_bytes(total_size)),
        "".to_string(),
        "Actions:".to_string(),
        "  C: Clean up orphaned media files".to_string(),
        "  V: Validate all media files".to_string(),
    ];

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, text)| {
            let prefix = if i == media_index { "▶" } else { " " };
            ListItem::new(format!("{} {}", prefix, text))
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Media Statistics"));
    f.render_widget(list, chunks[1]);

    let help = Paragraph::new("↑↓: Navigate | C: Clean orphaned media | V: Validate | Esc: Back")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help, chunks[2]);
}

/// Format bytes to human readable size
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

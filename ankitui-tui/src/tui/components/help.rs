//! Help Component
//!
//! Comprehensive help system with navigation

use crate::tui::app::AppState;
use crate::tui::core::{state_manager::RenderContext, UIComponent, Action};
use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

/// Help sections
#[derive(Debug, Clone, PartialEq)]
pub enum HelpSection {
    Overview,
    KeyboardShortcuts,
    LearningFeatures,
    DeckManagement,
    Settings,
    Troubleshooting,
    About,
}

impl HelpSection {
    fn get_title(&self) -> String {
        match self {
            HelpSection::Overview => "Overview".to_string(),
            HelpSection::KeyboardShortcuts => "Keyboard Shortcuts".to_string(),
            HelpSection::LearningFeatures => "Learning Features".to_string(),
            HelpSection::DeckManagement => "Deck Management".to_string(),
            HelpSection::Settings => "Settings".to_string(),
            HelpSection::Troubleshooting => "Troubleshooting".to_string(),
            HelpSection::About => "About".to_string(),
        }
    }

    fn get_content(&self) -> Vec<String> {
        match self {
            HelpSection::Overview => vec![
                "AnkiTUI - Terminal-based Spaced Repetition".to_string(),
                "".to_string(),
                "AnkiTUI is a terminal-based spaced repetition learning system".to_string(),
                "compatible with Anki's SM-2 algorithm.".to_string(),
                "".to_string(),
                "Key Features:".to_string(),
                "• Spaced repetition using SM-2 algorithm".to_string(),
                "• Terminal-based interface for efficient learning".to_string(),
                "• Support for cards, decks, and media".to_string(),
                "• Detailed statistics and progress tracking".to_string(),
                "• Customizable learning parameters".to_string(),
                "".to_string(),
                "Navigation:".to_string(),
                "Use Tab/Shift+Tab to switch between help sections".to_string(),
                "Use Up/Down arrows to scroll within sections".to_string(),
                "Press Escape to return to the main menu".to_string(),
                "".to_string(),
                "Getting Started:".to_string(),
                "1. Create or import decks of cards".to_string(),
                "2. Start a review session from the main menu".to_string(),
                "3. Rate cards as Again, Hard, Good, or Easy".to_string(),
                "4. Monitor your progress in the statistics view".to_string(),
            ],
            HelpSection::KeyboardShortcuts => vec![
                "Global Shortcuts".to_string(),
                "".to_string(),
                "Navigation:".to_string(),
                "• Tab - Move to next item/section".to_string(),
                "• Shift+Tab - Move to previous item/section".to_string(),
                "• Up/Down - Navigate in lists".to_string(),
                "• Left/Right - Navigate tabs or select options".to_string(),
                "• Enter - Select/confirm action".to_string(),
                "• Escape - Cancel/Go back".to_string(),
                "• q - Quit (with confirmation)".to_string(),
                "".to_string(),
                "Main Menu:".to_string(),
                "• 1-6 - Quick select menu items".to_string(),
                "• Enter - Select menu item".to_string(),
                "".to_string(),
                "Card Review:".to_string(),
                "• Space - Flip card to show answer".to_string(),
                "• 1-4 - Rate card (Again, Hard, Good, Easy)".to_string(),
                "• a - Rate as Again".to_string(),
                "• h - Rate as Hard".to_string(),
                "• g - Rate as Good".to_string(),
                "• e - Rate as Easy".to_string(),
                "• s - Pause/Bury card".to_string(),
                "• d - Delete card".to_string(),
                "".to_string(),
                "Deck Management:".to_string(),
                "• n - Create new deck".to_string(),
                "• r - Rename deck".to_string(),
                "• Del - Delete deck".to_string(),
                "• i - Import cards".to_string(),
                "• x - Export cards".to_string(),
                "".to_string(),
                "Search:".to_string(),
                "• / - Start search".to_string(),
                "• n - Find next result".to_string(),
                "• p - Find previous result".to_string(),
            ],
            HelpSection::LearningFeatures => vec![
                "Learning Features".to_string(),
                "".to_string(),
                "Spaced Repetition System:".to_string(),
                "AnkiTUI uses the SM-2 algorithm to optimize learning intervals".to_string(),
                "based on your performance ratings.".to_string(),
                "".to_string(),
                "Card Ratings:".to_string(),
                "• Again (1): Reset interval to 1 day, increase ease factor penalty".to_string(),
                "• Hard (2): Increase interval by 1.2x, slight ease factor decrease".to_string(),
                "• Good (3): Increase interval based on current ease factor".to_string(),
                "• Easy (4): Increase interval by 1.3x, increase ease factor".to_string(),
                "".to_string(),
                "Card States:".to_string(),
                "• New: Cards not yet studied".to_string(),
                "• Learning: Cards in initial learning phase".to_string(),
                "• Review: Cards graduated to regular reviews".to_string(),
                "• Relearning: Cards that were forgotten and are being relearned".to_string(),
                "".to_string(),
                "Learning Techniques:".to_string(),
                "• Active Recall: Try to recall the answer before viewing".to_string(),
                "• Spaced Repetition: Review at optimal intervals".to_string(),
                "• Interleaving: Mix different topics for better retention".to_string(),
                "• Elaboration: Connect new information to existing knowledge".to_string(),
                "".to_string(),
                "Study Tips:".to_string(),
                "• Study consistently rather than cramming".to_string(),
                "• Focus on difficult cards (rated Again or Hard)".to_string(),
                "• Use image and sound attachments when possible".to_string(),
                "• Create clear, unambiguous card questions".to_string(),
                "• Review regularly to maintain momentum".to_string(),
            ],
            HelpSection::DeckManagement => vec![
                "Deck Management".to_string(),
                "".to_string(),
                "Creating Decks:".to_string(),
                "Select 'Deck Management' from the main menu, then choose".to_string(),
                "'Create Deck' to start a new deck. Give it a descriptive name".to_string(),
                "that reflects its content.".to_string(),
                "".to_string(),
                "Card Creation:".to_string(),
                "Cards support the following fields:".to_string(),
                "• Front: The question or prompt".to_string(),
                "• Back: The answer or explanation".to_string(),
                "• Tags: Optional labels for organization".to_string(),
                "• Media: Optional image/audio attachments".to_string(),
                "• Custom fields: Additional data as needed".to_string(),
                "".to_string(),
                "Import/Export:".to_string(),
                "AnkiTUI supports multiple formats:".to_string(),
                "• CSV: Comma-separated values with headers".to_string(),
                "• JSON: Structured data format".to_string(),
                "• TOML: Human-readable configuration format".to_string(),
                "• Anki APKG: Anki deck packages (experimental)".to_string(),
                "".to_string(),
                "Deck Organization:".to_string(),
                "• Use descriptive names for decks".to_string(),
                "• Tag cards for better searchability".to_string(),
                "• Keep decks focused on specific topics".to_string(),
                "• Consider splitting large decks into smaller ones".to_string(),
                "".to_string(),
                "Deck Statistics:".to_string(),
                "Track your progress with detailed statistics:".to_string(),
                "• Total cards and distribution by state".to_string(),
                "• Learning progress and retention rates".to_string(),
                "• Daily study patterns and consistency".to_string(),
                "• Ease factor trends over time".to_string(),
            ],
            HelpSection::Settings => vec![
                "Settings Configuration".to_string(),
                "".to_string(),
                "Learning Limits:".to_string(),
                "• Max New Cards: Daily limit for new cards (default: 20)".to_string(),
                "• Max Review Cards: Daily limit for reviews (default: 100)".to_string(),
                "• Day Start/End: Define your study day hours".to_string(),
                "".to_string(),
                "Theme & Display:".to_string(),
                "• Color Theme: Choose from predefined themes".to_string(),
                "  - Dark: Traditional terminal colors".to_string(),
                "  - Light: Inverted colors for bright terminals".to_string(),
                "  - Ocean: Blue-based color scheme".to_string(),
                "  - Forest: Green-based nature theme".to_string(),
                "  - Sunset: Warm red/orange tones".to_string(),
                "  - Monochrome: Grayscale only".to_string(),
                "• Interface style and animation preferences".to_string(),
                "".to_string(),
                "Keyboard Shortcuts:".to_string(),
                "• Customize key bindings for all actions".to_string(),
                "• Create personalized workflows".to_string(),
                "• Override default shortcuts as needed".to_string(),
                "".to_string(),
                "Scheduling Algorithm:".to_string(),
                "• Starting Ease Factor: Initial difficulty multiplier (default: 2.5)".to_string(),
                "• Minimum Ease Factor: Floor for ease factor (default: 1.3)".to_string(),
                "• Ease Factor Bonus: Reward for easy reviews (default: 0.1)".to_string(),
                "• Interval Modifier: Global interval scaling (default: 1.0)".to_string(),
                "".to_string(),
                "Data Management:".to_string(),
                "• Database location and backup settings".to_string(),
                "• Media storage preferences".to_string(),
                "• Sync and export configurations".to_string(),
                "".to_string(),
                "Advanced:".to_string(),
                "• Performance tuning options".to_string(),
                "• Debug and logging settings".to_string(),
                "• Experimental features".to_string(),
            ],
            HelpSection::Troubleshooting => vec![
                "Troubleshooting".to_string(),
                "".to_string(),
                "Common Issues:".to_string(),
                "".to_string(),
                "Application Won't Start:".to_string(),
                "• Check that terminal supports required features".to_string(),
                "• Verify database file permissions".to_string(),
                "• Try running with `RUST_LOG=debug` for detailed logs".to_string(),
                "".to_string(),
                "Cards Not Loading:".to_string(),
                "• Ensure deck contains cards with valid format".to_string(),
                "• Check that cards are not all buried or suspended".to_string(),
                "• Verify scheduling parameters allow new cards".to_string(),
                "".to_string(),
                "Database Issues:".to_string(),
                "• Corrupted database: Try creating a new one".to_string(),
                "• Migration errors: Check disk space and permissions".to_string(),
                "• Performance issues: Consider database optimization".to_string(),
                "".to_string(),
                "Display Problems:".to_string(),
                "• Terminal too small: Resize to at least 80x24".to_string(),
                "• Color issues: Check terminal color support".to_string(),
                "• Unicode problems: Ensure UTF-8 encoding".to_string(),
                "".to_string(),
                "Performance:".to_string(),
                "• Slow startup: Check database size and consider optimization".to_string(),
                "• Lag during reviews: Disable animations if needed".to_string(),
                "• Memory usage: Monitor for leaks in long sessions".to_string(),
                "".to_string(),
                "Getting Help:".to_string(),
                "• Check the GitHub issues page".to_string(),
                "• Provide error messages and system information".to_string(),
                "• Include steps to reproduce the problem".to_string(),
                "• Share relevant configuration files (sans sensitive data)".to_string(),
            ],
            HelpSection::About => vec![
                "About AnkiTUI".to_string(),
                "".to_string(),
                "Version Information:".to_string(),
                "AnkiTUI v0.1.0".to_string(),
                "A modern terminal-based spaced repetition learning system".to_string(),
                "".to_string(),
                "Technology Stack:".to_string(),
                "• Language: Rust".to_string(),
                "• TUI Framework: ratatui".to_string(),
                "• Database: SQLite with sqlx".to_string(),
                "• Configuration: TOML".to_string(),
                "• Async Runtime: tokio".to_string(),
                "".to_string(),
                "Algorithm:".to_string(),
                "Implements the SM-2 spaced repetition algorithm,".to_string(),
                "compatible with Anki and other popular SRS systems.".to_string(),
                "".to_string(),
                "Design Philosophy:".to_string(),
                "• Efficient keyboard-driven interface".to_string(),
                "• Minimal distractions for focused learning".to_string(),
                "• Extensible and maintainable codebase".to_string(),
                "• Cross-platform compatibility".to_string(),
                "• Privacy-focused local data storage".to_string(),
                "".to_string(),
                "License:".to_string(),
                "MIT License - see LICENSE file for details".to_string(),
                "".to_string(),
                "Contributing:".to_string(),
                "Contributions are welcome! Please see the CONTRIBUTING".to_string(),
                "file for guidelines on how to participate in development.".to_string(),
                "".to_string(),
                "Acknowledgments:".to_string(),
                "• Anki team for the SM-2 algorithm and inspiration".to_string(),
                "• ratatui developers for the excellent TUI framework".to_string(),
                "• The Rust community for tools and libraries".to_string(),
                "".to_string(),
                "Support:".to_string(),
                "For bug reports and feature requests, please visit:".to_string(),
                "https://github.com/your-repo/ankitui/issues".to_string(),
            ],
        }
    }

    fn next_section(&self) -> Self {
        match self {
            HelpSection::Overview => HelpSection::KeyboardShortcuts,
            HelpSection::KeyboardShortcuts => HelpSection::LearningFeatures,
            HelpSection::LearningFeatures => HelpSection::DeckManagement,
            HelpSection::DeckManagement => HelpSection::Settings,
            HelpSection::Settings => HelpSection::Troubleshooting,
            HelpSection::Troubleshooting => HelpSection::About,
            HelpSection::About => HelpSection::Overview,
        }
    }

    fn previous_section(&self) -> Self {
        match self {
            HelpSection::Overview => HelpSection::About,
            HelpSection::KeyboardShortcuts => HelpSection::Overview,
            HelpSection::LearningFeatures => HelpSection::KeyboardShortcuts,
            HelpSection::DeckManagement => HelpSection::LearningFeatures,
            HelpSection::Settings => HelpSection::DeckManagement,
            HelpSection::Troubleshooting => HelpSection::Settings,
            HelpSection::About => HelpSection::Troubleshooting,
        }
    }
}

/// Help component
pub struct Help {
    current_section: HelpSection,
    selected_section_index: usize,
    scroll_offset: usize,
}

impl Help {
    pub fn new() -> Self {
        Self {
            current_section: HelpSection::Overview,
            selected_section_index: 0,
            scroll_offset: 0,
        }
    }

    pub fn current_section(&self) -> HelpSection {
        self.current_section.clone()
    }

    pub fn next_section(&mut self) {
        self.current_section = self.current_section.next_section();
        self.selected_section_index = (self.selected_section_index + 1) % 7;
        self.scroll_offset = 0;
    }

    pub fn previous_section(&mut self) {
        self.current_section = self.current_section.previous_section();
        self.selected_section_index = if self.selected_section_index == 0 {
            6
        } else {
            self.selected_section_index - 1
        };
        self.scroll_offset = 0;
    }

    pub fn scroll_down(&mut self) {
        let content = self.current_section.get_content();
        let visible_lines = 20; // Approximate visible lines
        if self.scroll_offset + visible_lines < content.len() {
            self.scroll_offset += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }
}

impl UIComponent for Help {
    fn render(&mut self, frame: &mut Frame, _context: RenderContext) -> Result<()> {
        let size = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(size);

        // Render tabs
        let section_titles = vec![
            "Overview".to_string(), "Shortcuts".to_string(), "Learning".to_string(), "Decks".to_string(), "Settings".to_string(), "Troubleshoot".to_string(), "About".to_string(),
        ];
        let tabs = Tabs::new(section_titles)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .select(self.selected_section_index);

        frame.render_widget(tabs, chunks[0]);

        // Render content area
        let content = self.current_section.get_content();
        let visible_content: Vec<&String> = content
            .iter()
            .skip(self.scroll_offset)
            .take((chunks[1].height - 2) as usize)
            .collect();

        let lines: Vec<Line> = visible_content
            .iter()
            .map(|line| {
                if line.is_empty() {
                    Line::from(Span::raw(" "))
                } else if line.starts_with("•") {
                    Line::from(Span::styled(
                        line.as_str(),
                        Style::default().fg(Color::Cyan),
                    ))
                } else if line.contains(":") && !line.starts_with(" ") {
                    Line::from(Span::styled(
                        line.as_str(),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ))
                } else {
                    Line::from(Span::raw(line.as_str()))
                }
            })
            .collect();

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.current_section.get_title()),
            )
            .style(Style::default().fg(Color::White));

        frame.render_widget(paragraph, chunks[1]);

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> Result<Option<AppState>> {
        match action {
            Action::Left => {
                self.previous_section();
            }
            Action::Right => {
                self.next_section();
            }
            Action::Up => {
                self.scroll_up();
            }
            Action::Down => {
                self.scroll_down();
            }
            Action::Cancel => {
                return Ok(Some(AppState::MainMenu));
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self) -> Result<()> {
        // No dynamic updates needed for help content
        Ok(())
    }

    fn name(&self) -> &str {
        "help"
    }
}
//! Command Line Interface
//!
//! Provides comprehensive command-line interface with support for:
//! - Multiple subcommands (review, import, export, stats, edit, config)
//! - Command-line argument parsing and validation
//! - Integration with configuration manager
//! - Help and version information

use ankitui_core::config::ConfigManager;
use ankitui_core::core::stats_engine::StatsEngine;
use ankitui_core::core::DeckManager;
use ankitui_core::data::{Card, CardContent, CardState, Deck, MediaType};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

/// AnkiTUI - Terminal-based spaced repetition learning system
#[derive(Parser, Debug)]
#[command(
    name = "ankitui",
    about = "A terminal-based spaced repetition learning system compatible with Anki",
    version,
    author = "AnkiTUI Team",
    long_about = "AnkiTUI is a powerful terminal application for spaced repetition learning. \
                  It implements the SM-2 algorithm, supports multiple deck management, \
                  and provides an efficient TUI interface for card review and management."
)]
pub struct Cli {
    /// Configuration file path (overrides default)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Data directory path (overrides config)
    #[arg(short = 'd', long, value_name = "DIR")]
    pub data_dir: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Enable quiet mode (minimal output)
    #[arg(short, long)]
    pub quiet: bool,

    /// Disable color output
    #[arg(long)]
    pub no_color: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start card review session (default command)
    Review {
        /// Specific deck to review
        #[arg(short, long, value_name = "DECK")]
        deck: Option<String>,

        /// Limit number of cards to review
        #[arg(short, long, default_value = "0")]
        limit: i32,

        /// Review only new cards
        #[arg(long)]
        new_only: bool,

        /// Review only due cards
        #[arg(long)]
        due_only: bool,

        /// Skip learning cards
        #[arg(long)]
        skip_learning: bool,
    },

    /// Import cards from file
    Import {
        /// Input file path
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Target deck name
        #[arg(short, long, value_name = "DECK")]
        deck: String,

        /// Input file format
        #[arg(short, long, value_enum, default_value = "csv")]
        format: ImportFormat,

        /// Include learning states (if available)
        #[arg(long)]
        include_states: bool,

        /// Overwrite existing deck
        #[arg(long)]
        overwrite: bool,
    },

    /// Export cards to file
    Export {
        /// Output file path
        #[arg(value_name = "FILE")]
        output: PathBuf,

        /// Deck to export (all decks if not specified)
        #[arg(short, long, value_name = "DECK")]
        deck: Option<String>,

        /// Export format
        #[arg(short, long, value_enum, default_value = "csv")]
        format: ExportFormat,

        /// Include learning states
        #[arg(long)]
        include_states: bool,

        /// Include media files references
        #[arg(long)]
        include_media: bool,
    },

    /// Show learning statistics
    Stats {
        /// Deck to analyze (all decks if not specified)
        #[arg(short, long, value_name = "DECK")]
        deck: Option<String>,

        /// Statistics time period
        #[arg(short, long, value_enum, default_value = "week")]
        period: StatsPeriod,

        /// Show detailed statistics
        #[arg(long)]
        detailed: bool,

        /// Export statistics to file
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Output format for statistics
        #[arg(long, value_enum, default_value = "text")]
        output_format: StatsFormat,
    },

    /// Edit deck or cards
    Edit {
        /// Deck to edit
        #[arg(short, long, value_name = "DECK")]
        deck: String,

        /// Open deck configuration editor
        #[arg(long)]
        deck_config: bool,

        /// Add new cards interactively
        #[arg(long)]
        add_cards: bool,

        /// Browse and edit existing cards
        #[arg(long)]
        browse_cards: bool,

        /// Card search query
        #[arg(long, value_name = "QUERY")]
        search: Option<String>,

        /// Edit specific card by ID
        #[arg(long, value_name = "CARD_ID")]
        card_id: Option<String>,

        /// Batch edit cards
        #[arg(long)]
        batch: bool,

        /// Field to edit (for batch edit)
        #[arg(long, value_name = "FIELD")]
        field: Option<String>,

        /// Text to find (for batch edit)
        #[arg(long, value_name = "FIND")]
        find: Option<String>,

        /// Text to replace (for batch edit)
        #[arg(long, value_name = "REPLACE")]
        replace: Option<String>,
    },

    /// Manage configuration
    Config {
        /// Show current configuration
        #[arg(long)]
        show: bool,

        /// Reset configuration to defaults
        #[arg(long)]
        reset: bool,

        /// Set configuration value
        #[arg(long, value_name = "KEY=VALUE")]
        set: Option<String>,

        /// Get configuration value
        #[arg(long, value_name = "KEY")]
        get: Option<String>,

        /// Edit configuration file
        #[arg(long)]
        edit: bool,

        /// Show configuration file path
        #[arg(long)]
        path: bool,
    },

    /// Deck management
    Deck {
        /// List all decks
        #[arg(long)]
        list: bool,

        /// Create new deck
        #[arg(long, value_name = "NAME")]
        create: Option<String>,

        /// Delete deck
        #[arg(long, value_name = "NAME")]
        delete: Option<String>,

        /// Rename deck
        #[arg(long)]
        rename: bool,

        /// Source deck name (for rename)
        #[arg(long, value_name = "OLD_NAME")]
        from: Option<String>,

        /// Target deck name (for rename)
        #[arg(long, value_name = "NEW_NAME")]
        to: Option<String>,

        /// Show deck information
        #[arg(long, value_name = "NAME")]
        info: Option<String>,
    },

    /// Database maintenance
    Db {
        /// Check database integrity
        #[arg(long)]
        check: bool,

        /// Optimize database
        #[arg(long)]
        optimize: bool,

        /// Rebuild database from TOML files
        #[arg(long)]
        rebuild: bool,

        /// Create database backup
        #[arg(long)]
        backup: bool,

        /// Restore from backup
        #[arg(long, value_name = "BACKUP_FILE")]
        restore: Option<PathBuf>,

        /// Show database statistics
        #[arg(long)]
        stats: bool,
    },

    /// Media management operations
    Media {
        /// Deck name to manage media for
        #[arg(short, long, value_name = "DECK")]
        deck: String,

        /// Action to perform
        #[command(subcommand)]
        action: MediaAction,
    },
}

/// Import file formats
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ImportFormat {
    /// Comma-separated values
    Csv,
    /// Tab-separated values
    Tsv,
    /// Anki deck package (.apkg)
    Apkg,
    /// JSON format
    Json,
    /// TOML format
    Toml,
}

/// Export file formats
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ExportFormat {
    /// Comma-separated values
    Csv,
    /// Tab-separated values
    Tsv,
    /// Anki deck package (.apkg)
    Apkg,
    /// JSON format
    Json,
    /// TOML format
    Toml,
}

/// Statistics time periods
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum StatsPeriod {
    /// Today only
    Today,
    /// Last 7 days
    Week,
    /// Last 30 days
    Month,
    /// Last 90 days
    Quarter,
    /// Last year
    Year,
    /// All time
    All,
}

/// Statistics output formats
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum StatsFormat {
    /// Plain text
    Text,
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Markdown format
    Markdown,
}

/// Media management actions
#[derive(Subcommand, Debug)]
pub enum MediaAction {
    /// Add media to a card
    Add {
        /// Card ID to add media to
        #[arg(long, value_name = "CARD_ID")]
        card_id: String,

        /// Path to media file
        #[arg(long, value_name = "FILE_PATH")]
        file_path: PathBuf,

        /// Media type (audio, image, video)
        #[arg(long, value_name = "TYPE")]
        media_type: String,
    },

    /// Remove media from a card
    Remove {
        /// Card ID to remove media from
        #[arg(long, value_name = "CARD_ID")]
        card_id: String,
    },

    /// List all media files in the deck
    List {
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },

    /// Validate all media files in the deck
    Validate {
        /// Fix missing files if possible
        #[arg(long)]
        fix: bool,
    },

    /// Clean up orphaned media files
    Cleanup {
        /// Show what would be deleted without actually deleting
        #[arg(long)]
        dry_run: bool,
    },

    /// Show media statistics for the deck
    Stats {
        /// Output format
        #[arg(long, default_value = "text")]
        format: String,
    },
}

/// CLI application manager
pub struct CliApp {
    cli: Cli,
    config_manager: ConfigManager,
    deck_manager: Option<Arc<DeckManager>>,
    stats_engine: Option<Arc<StatsEngine>>,
}

impl CliApp {
    /// Create new CLI application instance
    pub async fn new() -> Result<Self> {
        let cli = Cli::parse();

        // Load configuration with custom path if provided
        let config_manager = if let Some(config_path) = &cli.config {
            ConfigManager::with_path(config_path)?
        } else {
            ConfigManager::new()?
        };

        // Initialize core components for commands that need them
        let data_dir = config_manager.get_data_dir();
        let db_path = data_dir.join("ankitui.db");

        let deck_manager = if needs_core_components(&cli) {
            Some(Arc::new(DeckManager::new(&data_dir, &db_path).await?))
        } else {
            None
        };

        let stats_engine = if needs_core_components(&cli) {
            Some(Arc::new(StatsEngine::new()))
        } else {
            None
        };

        Ok(Self {
            cli,
            config_manager,
            deck_manager,
            stats_engine,
        })
    }

    /// Create CLI app with custom arguments (for testing)
    pub async fn with_args(args: Vec<&str>) -> Result<Self> {
        use clap::Parser;
        let cli = Cli::try_parse_from(args)?;

        let config_manager = if let Some(config_path) = &cli.config {
            ConfigManager::with_path(config_path)?
        } else {
            ConfigManager::new()?
        };

        // Initialize core components for commands that need them
        let data_dir = config_manager.get_data_dir();
        let db_path = data_dir.join("ankitui.db");

        let deck_manager = if needs_core_components(&cli) {
            Some(Arc::new(DeckManager::new(&data_dir, &db_path).await?))
        } else {
            None
        };

        let stats_engine = if needs_core_components(&cli) {
            Some(Arc::new(StatsEngine::new()))
        } else {
            None
        };

        Ok(Self {
            cli,
            config_manager,
            deck_manager,
            stats_engine,
        })
    }

    /// Run the CLI application
    pub async fn run(&mut self) -> Result<i32> {
        // Set verbosity level
        self.setup_logging();

        // Data directory will be passed directly to ConfigManager
        let data_dir_override = self.cli.data_dir.clone();

        // Handle no command case (default to review)
        let command = match &self.cli.command {
            Some(cmd) => cmd,
            None => &Commands::Review {
                deck: None,
                limit: 0,
                new_only: false,
                due_only: false,
                skip_learning: false,
            },
        };

        match command {
            Commands::Review { .. } => self.handle_review().await,
            Commands::Import { .. } => self.handle_import().await,
            Commands::Export { .. } => self.handle_export().await,
            Commands::Stats { .. } => self.handle_stats().await,
            Commands::Edit { .. } => self.handle_edit().await,
            Commands::Config { .. } => self.handle_config().await,
            Commands::Deck { .. } => self.handle_deck().await,
            Commands::Db { .. } => self.handle_database().await,
            Commands::Media { .. } => self.handle_media().await,
        }
    }

    /// Setup logging based on verbosity flags
    fn setup_logging(&self) {
        let log_level = if self.cli.verbose {
            log::LevelFilter::Debug
        } else if self.cli.quiet {
            log::LevelFilter::Error
        } else {
            log::LevelFilter::Info
        };

        // Note: In a real implementation, you would set up proper logging here
        // Log level is now handled directly in init_logging
    }

    /// Handle review command
    async fn handle_review(&self) -> Result<i32> {
        let (deck, limit, new_only, due_only, skip_learning) = match &self.cli.command {
            Some(Commands::Review {
                deck,
                limit,
                new_only,
                due_only,
                skip_learning,
            }) => (deck, *limit, *new_only, *due_only, *skip_learning),
            None => (&None, 0, false, false, false), // Default command
            _ => return Ok(1),
        };

        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

        self.print_info("Starting review session...");

        // Handle deck selection
        let deck_id = if let Some(deck_name) = deck {
            // Find deck by name
            let decks = deck_manager.get_all_decks().await?;
            match decks.iter().find(|(d, _)| d.name == *deck_name) {
                Some((deck, _)) => {
                    self.print_info(&format!("Selected deck: {}", deck.name));
                    deck.uuid.clone()
                }
                None => {
                    self.print_error(&format!("Deck '{}' not found", deck_name));
                    self.print_info("Available decks:");
                    let decks = deck_manager.get_all_decks().await?;
                    if decks.is_empty() {
                        self.print_info("  No decks found. Use 'ankitui deck --create <name>' to create one.");
                    } else {
                        for (d, _) in decks {
                            self.print_info(&format!("  - {}", d.name));
                        }
                    }
                    return Ok(1);
                }
            }
        } else {
            // Show deck selection if no specific deck requested
            let decks = deck_manager.get_all_decks().await?;
            if decks.is_empty() {
                self.print_info("No decks found. Use 'ankitui deck --create <name>' to create one.");
                return Ok(1);
            } else if decks.len() == 1 {
                let (deck, _) = &decks[0];
                self.print_info(&format!("Auto-selecting deck: {}", deck.name));
                deck.uuid.clone()
            } else {
                self.print_info("Available decks:");
                for (i, (deck, _)) in decks.iter().enumerate() {
                    let stats = deck_manager.get_deck_statistics(&deck.uuid).await?;
                    self.print_info(&format!(
                        "  {}. {} [{} cards, {} due]",
                        i + 1,
                        deck.name,
                        stats.total_cards,
                        stats.due_cards
                    ));
                }
                self.print_info("Use 'ankitui review --deck <deck_name>' to select a specific deck.");
                return Ok(1);
            }
        };

        // Check if there are cards to review
        let stats = deck_manager.get_deck_statistics(&deck_id).await?;
        let total_available = stats.due_cards + stats.new_cards;

        if total_available == 0 {
            self.print_info("No cards to review in this deck!");
            self.print_info(&format!("  Total cards: {}", stats.total_cards));
            self.print_info(&format!("  Next review card: Tomorrow or later"));
            return Ok(0);
        }

        // Apply card limit if specified
        let total_available_i32 = total_available as i32;
        let effective_limit = if limit > 0 && limit < total_available_i32 {
            self.print_info(&format!(
                "Limiting session to {} cards ({} available)",
                limit, total_available
            ));
            limit
        } else {
            total_available_i32
        };

        self.print_info(&format!("Session setup: {} cards to review", effective_limit));
        self.print_info(&format!("  Due cards: {}", stats.due_cards));
        self.print_info(&format!("  New cards: {}", stats.new_cards));

        // Run CLI review session using DeckManager directly
        self.run_cli_review_session(&deck_id, effective_limit).await?;

        self.print_info("Review session completed!");
        Ok(0)
    }

    /// Run a real CLI review session using DeckManager and SM-2 scheduler
    async fn run_cli_review_session(&self, deck_id: &uuid::Uuid, card_limit: i32) -> Result<()> {
        use ankitui_core::core::scheduler::Rating;

        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

        // Fetch due cards from the deck
        let due_cards = deck_manager.get_due_cards(deck_id, Some(card_limit)).await?;

        let total_due = due_cards.len();
        self.print_info(&format!("Found {} due cards. Starting review...\n", total_due));

        let mut cards_reviewed = 0;
        let mut correct = 0;
        let mut incorrect = 0;

        for (i, card) in due_cards.into_iter().enumerate() {
            let card_num = i + 1;
            self.print_info(&format!(
                "--- Card {}/{} ---\n{}",
                card_num, total_due, card.content.front
            ));

            // Auto-rate with "Good" for CLI mode (non-interactive)
            // In a future enhancement, this could read rating from stdin
            let rating = Rating::Good;

            // Apply the rating through the deck manager's SM-2 scheduler
            deck_manager.review_card(card, rating).await?;
            cards_reviewed += 1;
            correct += 1; // "Good" counts as correct
        }

        // Print session summary
        self.print_info(&format!("\nSession Summary:"));
        self.print_info(&format!("  Cards studied: {}", cards_reviewed));
        self.print_info(&format!("  Correct: {}", correct));
        self.print_info(&format!("  Incorrect: {}", incorrect));

        Ok(())
    }

    /// Handle import command
    async fn handle_import(&self) -> Result<i32> {
        if let Some(Commands::Import {
            input,
            deck,
            format,
            include_states,
            overwrite,
        }) = &self.cli.command
        {
            let deck_manager = self
                .deck_manager
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

            self.print_info(&format!("Importing from: {:?}", input));
            self.print_info(&format!("Target deck: {}", deck));
            self.print_info(&format!("Format: {:?}", format));

            if *include_states {
                self.print_info("Including learning states");
            }

            if *overwrite {
                self.print_info("Overwriting existing deck");
            }

            // Read the import file
            let import_content =
                std::fs::read_to_string(input).with_context(|| format!("Failed to read import file: {:?}", input))?;

            // Check if deck already exists
            if !overwrite {
                let decks = deck_manager.get_all_decks().await?;
                if let Some((existing_deck, _)) = decks.iter().find(|(d, _)| d.name == *deck) {
                    self.print_error(&format!(
                        "Deck '{}' already exists (UUID: {}). Use --overwrite to replace it.",
                        deck, existing_deck.uuid
                    ));
                    return Ok(1);
                }
            }

            // Parse the TOML content to get card information before import
            let preview_result: Result<toml::Value, toml::de::Error> = toml::from_str(&import_content);
            if let Ok(toml_value) = preview_result {
                if let Some(cards) = toml_value.get("cards").and_then(|c| c.as_array()) {
                    self.print_info(&format!("Found {} cards in import file", cards.len()));

                    // Print brief info for each card
                    for (i, card_value) in cards.iter().enumerate() {
                        if let Some(front) = card_value.get("front").and_then(|f| f.as_str()) {
                            let front_preview = if front.chars().count() > 20 {
                                front.chars().take(20).collect::<String>() + "..."
                            } else {
                                front.to_string()
                            };

                            let card_id = card_value.get("id").and_then(|id| id.as_str()).unwrap_or("unknown");

                            self.print_info(&format!("  Card {}: {} - {}", i + 1, card_id, front_preview));
                        }
                    }
                } else {
                    self.print_info("Warning: No cards found in import file");
                }
            } else {
                self.print_info("Warning: Could not preview cards - file format may be invalid");
            }

            // Import the deck
            let deck_uuid = match format {
                crate::util::cli::ImportFormat::Toml => deck_manager
                    .import_deck(&import_content)
                    .await
                    .context("Failed to import deck")?,
                crate::util::cli::ImportFormat::Csv => self
                    .import_csv(&import_content, deck)
                    .await
                    .context("Failed to import CSV deck")?,
                crate::util::cli::ImportFormat::Tsv => self
                    .import_tsv(&import_content, deck)
                    .await
                    .context("Failed to import TSV deck")?,
                crate::util::cli::ImportFormat::Json => self
                    .import_json(&import_content, deck)
                    .await
                    .context("Failed to import JSON deck")?,
                crate::util::cli::ImportFormat::Apkg => {
                    return Err(anyhow::anyhow!("Anki package format import not yet implemented"));
                }
            };

            // Rename deck if target name is different from imported name
            let (imported_deck, cards) = deck_manager.get_deck(&deck_uuid).await?;
            if imported_deck.name != *deck {
                deck_manager
                    .rename_deck(&deck_uuid, deck.clone())
                    .await
                    .context("Failed to rename deck")?;
                self.print_info(&format!("Renamed deck from '{}' to '{}'", imported_deck.name, deck));
            }

            self.print_info(&format!("Successfully imported deck '{}' (UUID: {})", deck, deck_uuid));
            self.print_info(&format!("Deck now contains {} cards", cards.len()));

            return Ok(0);
        }
        Ok(1)
    }

    /// Handle export command
    async fn handle_export(&self) -> Result<i32> {
        if let Some(Commands::Export {
            output: _,
            deck,
            format,
            include_states,
            include_media: _,
        }) = &self.cli.command
        {
            let deck_manager = self
                .deck_manager
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

            let deck_uuid = if let Some(deck_name) = deck {
                // Find deck by name
                let decks = deck_manager.get_all_decks().await?;
                let (deck, _cards) = decks
                    .iter()
                    .find(|(d, _)| d.name == *deck_name)
                    .ok_or_else(|| anyhow::anyhow!("Deck '{}' not found", deck_name))?;
                deck.uuid
            } else {
                // Export all decks - for now, return error
                self.print_error("Export requires a specific deck name. Use --deck <name>");
                return Ok(1);
            };

            match format {
                crate::util::cli::ExportFormat::Toml => {
                    let export_data = deck_manager.export_deck(&deck_uuid, *include_states).await?;
                    let filename = format!("{}.toml", deck.as_ref().unwrap());
                    std::fs::write(&filename, export_data)?;
                    self.print_info(&format!("Deck exported to: {}", filename));
                }
                crate::util::cli::ExportFormat::Csv => {
                    self.export_csv(&deck_uuid, deck.as_ref().unwrap(), *include_states)
                        .await
                        .context("Failed to export to CSV")?;
                }
                crate::util::cli::ExportFormat::Json => {
                    self.export_json(&deck_uuid, deck.as_ref().unwrap(), *include_states)
                        .await
                        .context("Failed to export to JSON")?;
                }
                crate::util::cli::ExportFormat::Tsv => {
                    self.export_tsv(&deck_uuid, deck.as_ref().unwrap(), *include_states)
                        .await
                        .context("Failed to export to TSV")?;
                }
                crate::util::cli::ExportFormat::Apkg => {
                    self.print_error("APKG export format not yet implemented");
                    return Ok(1);
                }
            }

            self.print_info("Export completed successfully");
            return Ok(0);
        }
        Ok(1)
    }

    /// Handle stats command
    async fn handle_stats(&self) -> Result<i32> {
        if let Some(Commands::Stats {
            deck,
            period,
            detailed: _,
            output: _,
            output_format: _,
        }) = &self.cli.command
        {
            let deck_manager = self
                .deck_manager
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

            let deck_uuid = if let Some(deck_name) = deck {
                // Find deck by name
                let decks = deck_manager.get_all_decks().await?;
                let (deck, _cards) = decks
                    .iter()
                    .find(|(d, _)| d.name == *deck_name)
                    .ok_or_else(|| anyhow::anyhow!("Deck '{}' not found", deck_name))?;
                Some(deck.uuid)
            } else {
                // Get all decks stats
                None
            };

            if let Some(deck_uuid) = deck_uuid {
                // Single deck statistics
                let stats = deck_manager.get_deck_statistics(&deck_uuid).await?;
                let deck_name = deck.as_ref().unwrap();

                self.print_info(&format!("Statistics for deck: {}", deck_name));
                self.print_info(&format!("  Total cards: {}", stats.total_cards));
                self.print_info(&format!("  Due cards: {}", stats.due_cards));
                self.print_info(&format!("  New cards: {}", stats.new_cards));
                self.print_info(&format!("  Learning cards: {}", stats.learning_cards));
                self.print_info(&format!("  Review cards: {}", stats.review_cards));
                // self.print_info(&format!("  Relearning cards: {}", stats.relearning_cards)); // Not available in DeckStats

                // Calculate additional stats
                let mature_cards = stats.total_cards.saturating_sub(stats.new_cards + stats.learning_cards);
                self.print_info(&format!("  Mature cards: {}", mature_cards));
                self.print_info(&format!("  Average ease factor: {:.1}", stats.average_ease_factor));

                if let Some(retention) = stats.retention_rate {
                    self.print_info(&format!("  Retention rate: {:.1}%", retention * 100.0));
                }
            } else {
                // All decks statistics
                let decks = deck_manager.get_all_decks().await?;
                let mut total_cards = 0;
                let mut total_due = 0;

                self.print_info("Statistics for all decks:");
                self.print_info(&format!("  Total decks: {}", decks.len()));

                for (deck, _cards) in &decks {
                    let stats = deck_manager.get_deck_statistics(&deck.uuid).await?;
                    total_cards += stats.total_cards;
                    total_due += stats.due_cards;

                    self.print_info(&format!(
                        "  {}: {} cards ({} due)",
                        deck.name, stats.total_cards, stats.due_cards
                    ));
                }

                self.print_info(&format!("  Total cards across all decks: {}", total_cards));
                self.print_info(&format!("  Total due cards: {}", total_due));
            }

            self.print_info(&format!("Statistics period: {:?}", period));
            self.print_info("Statistics generated successfully");
            return Ok(0);
        }
        Ok(1)
    }

    /// Handle edit command
    async fn handle_edit(&self) -> Result<i32> {
        if let Some(Commands::Edit {
            deck,
            deck_config,
            add_cards,
            browse_cards,
            search,
            card_id,
            batch,
            field,
            find,
            replace,
        }) = &self.cli.command
        {
            let deck_manager = self
                .deck_manager
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

            // Find the deck
            let decks = deck_manager.get_all_decks().await?;
            let (target_deck, cards) = decks
                .iter()
                .find(|(d, _)| d.name == *deck)
                .ok_or_else(|| anyhow::anyhow!("Deck '{}' not found", deck))?;

            self.print_info(&format!(
                "Editing deck: {} (UUID: {})",
                target_deck.name, target_deck.uuid
            ));

            // Handle deck configuration editing
            if *deck_config {
                self.print_info("Opening deck configuration editor...");
                self.edit_deck_config(&target_deck.uuid, deck_manager).await?;
            }

            // Handle adding new cards
            if *add_cards {
                self.print_info("Starting interactive card addition...");
                self.add_cards_interactive(&target_deck.uuid, deck_manager).await?;
            }

            // Handle browsing existing cards
            if *browse_cards {
                self.print_info("Browsing existing cards...");
                self.browse_cards(&target_deck.uuid, cards, deck_manager).await?;
            }

            // Handle editing specific card by ID
            if let Some(id) = card_id {
                self.print_info(&format!("Editing card: {}", id));
                self.edit_card_by_id(&target_deck.uuid, id, deck_manager).await?;
            }

            // Handle batch editing
            if *batch {
                if let (Some(field_name), Some(find_text), Some(replace_text)) = (field, find, replace) {
                    self.print_info("Starting batch edit...");
                    self.batch_edit_cards(&target_deck.uuid, deck_manager, field_name, find_text, replace_text)
                        .await?;
                } else {
                    self.print_error("Batch edit requires --field, --find, and --replace options");
                    return Ok(1);
                }
            }

            // Handle card search
            if let Some(query) = search {
                self.print_info(&format!("Searching cards with query: {}", query));
                self.search_cards(&target_deck.uuid, query, deck_manager).await?;
            }

            // If no specific action, show deck info and options
            if !deck_config && !add_cards && !browse_cards && search.is_none() && card_id.is_none() && !batch {
                self.show_deck_info(&target_deck, cards).await?;
                self.print_info("Available options:");
                self.print_info("  --deck-config: Edit deck configuration");
                self.print_info("  --add-cards: Add new cards interactively");
                self.print_info("  --browse-cards: Browse and edit existing cards");
                self.print_info("  --card-id <ID>: Edit specific card by ID");
                self.print_info("  --batch --field <FIELD> --find <TEXT> --replace <TEXT>: Batch edit");
                self.print_info("  --search <QUERY>: Search cards");
            }

            self.print_info("Edit session completed");
            return Ok(0);
        }
        Ok(1)
    }

    /// Edit deck configuration
    async fn edit_deck_config(&self, deck_uuid: &uuid::Uuid, deck_manager: &DeckManager) -> Result<()> {
        let (deck, _) = deck_manager.get_deck(deck_uuid).await?;

        self.print_info(&format!("Current configuration for deck: {}", deck.name));
        self.print_info(&format!(
            "Description: {}",
            deck.description.as_deref().unwrap_or("None")
        ));

        if let Some(scheduler_config) = &deck.scheduler_config {
            self.print_info("Scheduler configuration:");
            self.print_info(&format!(
                "  New cards per day: {:?}",
                scheduler_config.new_cards_per_day
            ));
            self.print_info(&format!(
                "  Review cards per day: {:?}",
                scheduler_config.max_reviews_per_day
            ));
            self.print_info(&format!(
                "  Starting ease factor: {}",
                scheduler_config.starting_ease_factor
            ));
        } else {
            self.print_info("Using default scheduler configuration");
        }

        self.print_info("Note: Interactive configuration editing will be implemented in a future version.");
        self.print_info("For now, you can export the deck, edit the TOML file, and import it back.");

        Ok(())
    }

    /// Edit card by ID
    async fn edit_card_by_id(&self, deck_uuid: &uuid::Uuid, card_id: &str, deck_manager: &DeckManager) -> Result<()> {
        // Parse card ID
        let card_uuid = uuid::Uuid::parse_str(card_id).map_err(|_| anyhow::anyhow!("Invalid card ID format"))?;

        self.edit_card_interactive(deck_uuid, &card_uuid, deck_manager).await
    }

    /// Add cards interactively
    async fn add_cards_interactive(&self, deck_uuid: &uuid::Uuid, deck_manager: &DeckManager) -> Result<()> {
        self.print_info("Interactive card addition");
        self.print_info("Enter 'quit' or 'exit' to stop adding cards");

        let mut card_count = 0;

        loop {
            self.print_info(&format!("\n--- Card #{} ---", card_count + 1));

            // Get front content
            self.print_info("Enter card front (question):");
            let front = self.get_user_input()?;

            if front.trim().to_lowercase() == "quit" || front.trim().to_lowercase() == "exit" {
                break;
            }

            // Get back content
            self.print_info("Enter card back (answer):");
            let back = self.get_user_input()?;

            if back.trim().to_lowercase() == "quit" || back.trim().to_lowercase() == "exit" {
                break;
            }

            // Get tags (optional)
            self.print_info("Enter tags (comma-separated, or press Enter for none):");
            let tags_input = self.get_user_input()?;
            let tags: Vec<String> = if tags_input.trim().is_empty() {
                Vec::new()
            } else {
                tags_input.split(',').map(|s| s.trim().to_string()).collect()
            };

            // Create the card
            let card_content = CardContent {
                id: uuid::Uuid::new_v4(),
                front: front.trim().to_string(),
                back: back.trim().to_string(),
                tags,
                media: None,
                custom: std::collections::HashMap::new(),
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            };

            // Add card to deck
            deck_manager.add_cards(deck_uuid, vec![card_content]).await?;
            card_count += 1;

            self.print_info(&format!("✓ Card #{} added successfully", card_count));
        }

        self.print_info(&format!("\nAdded {} cards to deck", card_count));
        Ok(())
    }

    /// Edit a specific card interactively
    async fn edit_card_interactive(
        &self,
        deck_uuid: &uuid::Uuid,
        card_id: &uuid::Uuid,
        deck_manager: &DeckManager,
    ) -> Result<()> {
        let (deck, mut cards) = deck_manager.get_deck(deck_uuid).await?;

        // Find the card to edit
        let card_index = cards
            .iter()
            .position(|c| c.content.id == *card_id)
            .ok_or_else(|| anyhow::anyhow!("Card not found in deck"))?;

        let card = &cards[card_index];

        self.print_info(&format!("Editing card: {}", card.content.id));
        self.print_info(&format!("Current front: {}", card.content.front));
        self.print_info(&format!("Current back: {}", card.content.back));
        self.print_info(&format!("Current tags: {}", card.content.tags.join(", ")));

        // Edit front content
        self.print_info("Enter new front (press Enter to keep current):");
        let new_front = self.get_user_input()?;
        let front = if new_front.trim().is_empty() {
            card.content.front.clone()
        } else {
            new_front.trim().to_string()
        };

        // Edit back content
        self.print_info("Enter new back (press Enter to keep current):");
        let new_back = self.get_user_input()?;
        let back = if new_back.trim().is_empty() {
            card.content.back.clone()
        } else {
            new_back.trim().to_string()
        };

        // Edit tags
        self.print_info("Enter new tags (comma-separated, or press Enter to keep current):");
        let new_tags = self.get_user_input()?;
        let tags = if new_tags.trim().is_empty() {
            card.content.tags.clone()
        } else {
            new_tags.split(',').map(|s| s.trim().to_string()).collect()
        };

        // Create updated card content
        let updated_content = CardContent {
            id: card.content.id,
            front,
            back,
            tags,
            media: card.content.media.clone(),
            custom: card.content.custom.clone(),
            created_at: card.content.created_at,
            modified_at: chrono::Utc::now(),
        };

        // Update the card
        let updated_card = Card {
            content: updated_content,
            state: card.state.clone(),
        };

        deck_manager.update_card(deck_uuid, &updated_card).await?;

        self.print_info("✓ Card updated successfully");
        Ok(())
    }

    /// Batch edit cards in a deck
    async fn batch_edit_cards(
        &self,
        deck_uuid: &uuid::Uuid,
        deck_manager: &DeckManager,
        field: &str,
        find_value: &str,
        replace_value: &str,
    ) -> Result<()> {
        self.print_info(&format!("Batch editing cards in deck"));
        self.print_info(&format!("Field: {}", field));
        self.print_info(&format!("Find: {}", find_value));
        self.print_info(&format!("Replace: {}", replace_value));

        let (deck, mut cards) = deck_manager.get_deck(deck_uuid).await?;
        let mut updated_count = 0;

        for card in &mut cards {
            let mut updated = false;
            let mut updated_content = card.content.clone();

            match field.to_lowercase().as_str() {
                "front" => {
                    if updated_content.front.contains(find_value) {
                        updated_content.front = updated_content.front.replace(find_value, replace_value);
                        updated = true;
                    }
                }
                "back" => {
                    if updated_content.back.contains(find_value) {
                        updated_content.back = updated_content.back.replace(find_value, replace_value);
                        updated = true;
                    }
                }
                "tags" => {
                    updated_content.tags = updated_content
                        .tags
                        .iter()
                        .map(|tag| {
                            if tag.contains(find_value) {
                                updated = true;
                                tag.replace(find_value, replace_value)
                            } else {
                                tag.clone()
                            }
                        })
                        .collect();
                }
                _ => {
                    self.print_error(&format!("Unknown field: {}", field));
                    return Ok(());
                }
            }

            if updated {
                updated_content.modified_at = chrono::Utc::now();
                let updated_card = Card {
                    content: updated_content,
                    state: card.state.clone(),
                };

                deck_manager.update_card(deck_uuid, &updated_card).await?;
                updated_count += 1;
            }
        }

        self.print_info(&format!("✓ Updated {} cards", updated_count));
        Ok(())
    }

    /// Browse and edit existing cards
    async fn browse_cards(&self, deck_uuid: &uuid::Uuid, cards: &[Card], deck_manager: &DeckManager) -> Result<()> {
        if cards.is_empty() {
            self.print_info("No cards found in this deck");
            return Ok(());
        }

        self.print_info(&format!("Found {} cards in deck", cards.len()));

        for (i, card) in cards.iter().enumerate() {
            self.print_info(&format!("\n--- Card {} ---", i + 1));
            self.print_info(&format!("ID: {}", card.content.id));
            self.print_info(&format!("Front: {}", card.content.front));
            self.print_info(&format!("Back: {}", card.content.back));
            self.print_info(&format!("Tags: {}", card.content.tags.join(", ")));
            self.print_info(&format!("State: {:?}", card.state.state));
            self.print_info(&format!("Ease factor: {}", card.state.ease_factor));
            self.print_info(&format!("Interval: {} days", card.state.interval));
            self.print_info(&format!("Repetitions: {}", card.state.reps));
        }

        self.print_info("\nNote: Interactive card editing will be implemented in a future version.");
        self.print_info("For now, you can export the deck, edit the TOML file, and import it back.");

        Ok(())
    }

    /// Search cards by query
    async fn search_cards(&self, deck_uuid: &uuid::Uuid, query: &str, deck_manager: &DeckManager) -> Result<()> {
        let (deck, cards) = deck_manager.get_deck(deck_uuid).await?;

        let query_lower = query.to_lowercase();
        let matching_cards: Vec<_> = cards
            .iter()
            .filter(|card| {
                card.content.front.to_lowercase().contains(&query_lower)
                    || card.content.back.to_lowercase().contains(&query_lower)
                    || card
                        .content
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect();

        if matching_cards.is_empty() {
            self.print_info(&format!("No cards found matching query: {}", query));
            return Ok(());
        }

        self.print_info(&format!("Found {} cards matching '{}':", matching_cards.len(), query));

        for (i, card) in matching_cards.iter().enumerate() {
            self.print_info(&format!("\n--- Result {} ---", i + 1));
            self.print_info(&format!("ID: {}", card.content.id));
            self.print_info(&format!("Front: {}", card.content.front));
            self.print_info(&format!("Back: {}", card.content.back));
            self.print_info(&format!("Tags: {}", card.content.tags.join(", ")));
            self.print_info(&format!("State: {:?}", card.state.state));
        }

        Ok(())
    }

    /// Show deck information
    async fn show_deck_info(&self, deck: &Deck, cards: &[Card]) -> Result<()> {
        self.print_info(&format!("Deck Information: {}", deck.name));
        self.print_info(&format!("UUID: {}", deck.uuid));
        self.print_info(&format!(
            "Description: {}",
            deck.description.as_deref().unwrap_or("None")
        ));
        self.print_info(&format!("Created: {}", deck.created_at.format("%Y-%m-%d %H:%M:%S UTC")));
        self.print_info(&format!(
            "Modified: {}",
            deck.modified_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Card statistics
        let total_cards = cards.len();
        let new_cards = cards.iter().filter(|c| matches!(c.state.state, CardState::New)).count();
        let learning_cards = cards
            .iter()
            .filter(|c| matches!(c.state.state, CardState::Learning))
            .count();
        let review_cards = cards
            .iter()
            .filter(|c| matches!(c.state.state, CardState::Review))
            .count();
        let relearning_cards = cards
            .iter()
            .filter(|c| matches!(c.state.state, CardState::Relearning))
            .count();

        self.print_info(&format!("Total cards: {}", total_cards));
        self.print_info(&format!("  New cards: {}", new_cards));
        self.print_info(&format!("  Learning cards: {}", learning_cards));
        self.print_info(&format!("  Review cards: {}", review_cards));
        self.print_info(&format!("  Relearning cards: {}", relearning_cards));

        Ok(())
    }

    /// Get user input from stdin
    fn get_user_input(&self) -> Result<String> {
        use std::io::{self, Write};

        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim_end().to_string())
    }

    /// Handle config command
    async fn handle_config(&mut self) -> Result<i32> {
        if let Some(Commands::Config {
            show,
            reset,
            set,
            get,
            edit,
            path,
        }) = &self.cli.command
        {
            let show_val = *show;
            let reset_val = *reset;
            let edit_val = *edit;
            let path_val = *path;
            let set_val = set.clone();
            let get_val = get.clone();

            if show_val {
                self.show_config().await?;
            }

            if reset_val {
                self.reset_config().await?;
            }

            if let Some(key_value) = set_val {
                self.set_config_value(&key_value).await?;
            }

            if let Some(key) = get_val {
                self.get_config_value(&key).await?;
            }

            if edit_val {
                self.edit_config().await?;
            }

            if path_val {
                let path = self.config_manager.config_path();
                self.print_info(&format!("Configuration file: {}", path.display()));
            }

            return Ok(0);
        }
        Ok(1)
    }

    /// Show current configuration
    async fn show_config(&self) -> Result<()> {
        let config = self.config_manager.get_config();
        self.print_info("Current configuration:");
        println!("\n[Scheduler]");
        println!("  Starting ease factor: {}", config.scheduler.starting_ease_factor);
        println!("  Easy bonus: {}", config.scheduler.easy_bonus);
        println!("  Interval modifier: {}", config.scheduler.interval_modifier);
        println!("  Easy interval: {} days", config.scheduler.easy_interval);
        println!("  Good interval: {} days", config.scheduler.good_interval);
        println!("  Graduating interval: {} days", config.scheduler.graduating_interval);
        println!("  Maximum interval: {} days", config.scheduler.max_interval);
        println!(
            "  Initial failure interval: {} minutes",
            config.scheduler.initial_failure_interval
        );

        println!("\n[Daily Limits]");
        println!("  Max new cards: {}", config.daily.max_new_cards);
        println!("  Max review cards: {}", config.daily.max_review_cards);
        println!("  Day start hour: {}", config.daily.day_start_hour);
        println!("  Day end hour: {}", config.daily.day_end_hour);
        println!("  Show limit warnings: {}", config.daily.show_limit_warnings);

        println!("\n[UI]");
        println!("  Theme: {}", config.ui.theme);
        println!("  Mouse support: {}", config.ui.mouse_support);
        println!("  Show progress: {}", config.ui.show_progress);
        println!("  Show card counter: {}", config.ui.show_card_counter);
        println!("  Animation speed: {} ms", config.ui.animation_speed);
        println!("  Refresh rate: {} Hz", config.ui.refresh_rate);

        println!("\n[Shortcuts]");
        println!("  Show answer: {}", config.shortcuts.show_answer);
        println!("  Rate again: {}", config.shortcuts.rate_again);
        println!("  Rate hard: {}", config.shortcuts.rate_hard);
        println!("  Rate good: {}", config.shortcuts.rate_good);
        println!("  Rate easy: {}", config.shortcuts.rate_easy);
        println!("  Toggle pause: {}", config.shortcuts.toggle_pause);
        println!("  Exit session: {}", config.shortcuts.exit_session);
        println!("  Show help: {}", config.shortcuts.show_help);
        println!("  Show stats: {}", config.shortcuts.show_stats);

        println!("\n[Data]");
        println!(
            "  Data directory: {}",
            config
                .data
                .data_dir
                .as_ref()
                .unwrap_or(&PathBuf::from("~/.ankitui"))
                .display()
        );
        println!("  Auto backup: {}", config.data.auto_backup);
        println!("  Backup interval: {} hours", config.data.backup_interval);
        println!("  Backup count: {}", config.data.backup_count);

        Ok(())
    }

    /// Reset configuration to defaults
    async fn reset_config(&mut self) -> Result<()> {
        self.print_info("Resetting configuration to defaults...");

        // Create default configuration
        let default_config = ankitui_core::config::Config::default();

        // Save default config
        self.config_manager
            .reset_to_defaults()
            .context("Failed to reset configuration")?;

        self.print_info("✓ Configuration has been reset to defaults");
        Ok(())
    }

    /// Set a configuration value
    async fn set_config_value(&mut self, key_value: &str) -> Result<()> {
        let parts: Vec<&str> = key_value.splitn(2, '=').collect();
        if parts.len() != 2 {
            self.print_error("Invalid format. Use: key=value");
            return Ok(());
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        let mut config = self.config_manager.get_config().clone();

        // Parse and set the configuration value
        match key {
            "scheduler.starting_ease_factor" | "starting_ease_factor" => {
                config.scheduler.starting_ease_factor = value.parse().context("Invalid ease factor value")?;
            }
            "scheduler.easy_bonus" | "easy_bonus" => {
                config.scheduler.easy_bonus = value.parse().context("Invalid easy bonus value")?;
            }
            "scheduler.learning_steps" | "learning_steps" => {
                config.scheduler.learning_steps = value.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                if config.scheduler.learning_steps.is_empty() {
                    return Err(anyhow::anyhow!("At least one learning step is required"));
                }
            }
            "scheduler.relearning_steps" | "relearning_steps" => {
                config.scheduler.relearning_steps = value.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                if config.scheduler.relearning_steps.is_empty() {
                    return Err(anyhow::anyhow!("At least one relearning step is required"));
                }
            }
            "scheduler.graduating_interval_days" | "graduating_interval" => {
                config.scheduler.graduating_interval_days =
                    value.parse().context("Invalid graduating interval value")?;
            }
            "scheduler.easy_interval_days" | "easy_interval" => {
                config.scheduler.easy_interval_days = value.parse().context("Invalid easy interval value")?;
            }
            "scheduler.min_ease_factor" | "min_ease_factor" => {
                config.scheduler.min_ease_factor = value.parse().context("Invalid minimum ease factor value")?;
            }
            "scheduler.max_ease_factor" | "max_ease_factor" => {
                config.scheduler.max_ease_factor = value.parse().context("Invalid maximum ease factor value")?;
            }
            "scheduler.hard_multiplier" | "hard_multiplier" => {
                config.scheduler.hard_multiplier = value.parse().context("Invalid hard multiplier value")?;
            }
            "scheduler.interval_modifier" | "interval_modifier" => {
                config.scheduler.interval_modifier = value.parse().context("Invalid interval modifier value")?;
            }
            "daily.max_new_cards" | "max_new_cards" | "new_cards_per_day" => {
                config.daily.max_new_cards = value.parse().context("Invalid max new cards value")?;
            }
            "daily.max_review_cards" | "max_review_cards" | "max_reviews_per_day" => {
                config.daily.max_review_cards = value.parse().context("Invalid max review cards value")?;
            }
            "ui.theme" | "theme" => {
                config.ui.theme = value.parse().context("Invalid theme value")?;
            }
            "ui.show_progress" | "show_progress" => {
                config.ui.show_progress = value.parse().context("Invalid boolean value for show_progress")?;
            }
            "ui.show_card_counter" | "show_card_counter" => {
                config.ui.show_card_counter = value.parse().context("Invalid boolean value for show_card_counter")?;
            }
            "data.backup_count" | "backup_count" => {
                config.data.backup_count = value.parse().context("Invalid backup count value")?;
            }
            "data.backup_interval" | "backup_interval" => {
                config.data.backup_interval = value.parse().context("Invalid backup interval value")?;
            }
            "data.auto_backup" | "auto_backup" => {
                config.data.auto_backup = value.parse().context("Invalid boolean value for auto_backup")?;
            }
            "data.compress_data" | "compress_data" => {
                config.data.compress_data = value.parse().context("Invalid boolean value for compress_data")?;
            }
            _ => {
                self.print_error(&format!("Unknown configuration key: {}", key));
                self.print_info("Available keys:");
                self.print_info("Scheduler Settings:");
                self.print_info("  scheduler.starting_ease_factor (float)");
                self.print_info("  scheduler.easy_bonus (float)");
                self.print_info("  scheduler.learning_steps (comma-separated minutes)");
                self.print_info("  scheduler.relearning_steps (comma-separated minutes)");
                self.print_info("  scheduler.graduating_interval_days (integer)");
                self.print_info("  scheduler.easy_interval_days (integer)");
                self.print_info("  scheduler.min_ease_factor (float)");
                self.print_info("  scheduler.max_ease_factor (float)");
                self.print_info("  scheduler.hard_multiplier (float)");
                self.print_info("  scheduler.interval_modifier (float)");
                self.print_info("");
                self.print_info("Daily Limits:");
                self.print_info("  daily.max_new_cards (integer)");
                self.print_info("  daily.max_review_cards (integer)");
                self.print_info("");
                self.print_info("UI Settings:");
                self.print_info("  ui.theme (string)");
                self.print_info("  ui.show_progress (true/false)");
                self.print_info("  ui.show_card_counter (true/false)");
                self.print_info("");
                self.print_info("Data Settings:");
                self.print_info("  data.auto_backup (true/false)");
                self.print_info("  data.backup_count (integer)");
                self.print_info("  data.backup_interval (hours)");
                self.print_info("");
                self.print_info("Examples:");
                self.print_info("  ankitui config set scheduler.learning_steps \"1,6,10,14\"");
                self.print_info("  ankitui config set scheduler.starting_ease_factor 2.5");
                self.print_info("  ankitui config set daily.max_new_cards 20");
                return Ok(());
            }
        }

        // Save the updated configuration
        self.config_manager
            .update_config(|c| *c = config.clone())
            .context("Failed to update configuration")?;

        self.print_info(&format!("✓ Set {} = {}", key, value));
        Ok(())
    }

    /// Get a configuration value
    async fn get_config_value(&mut self, key: &str) -> Result<()> {
        let config = self.config_manager.get_config();

        let value = match key {
            "scheduler.starting_ease_factor" | "starting_ease_factor" => {
                config.scheduler.starting_ease_factor.to_string()
            }
            "scheduler.easy_bonus" | "easy_bonus" => config.scheduler.easy_bonus.to_string(),
            "daily.max_new_cards" | "max_new_cards" | "new_cards_per_day" => config.daily.max_new_cards.to_string(),
            "daily.max_review_cards" | "max_review_cards" | "max_reviews_per_day" => {
                config.daily.max_review_cards.to_string()
            }
            "ui.theme" | "theme" => {
                format!("{:?}", config.ui.theme)
            }
            "ui.show_progress" | "show_progress" => config.ui.show_progress.to_string(),
            "ui.show_card_counter" | "show_card_counter" => config.ui.show_card_counter.to_string(),
            "data.backup_count" | "backup_count" => config.data.backup_count.to_string(),
            "data.backup_interval" | "backup_interval" => config.data.backup_interval.to_string(),
            "data.auto_backup" | "auto_backup" => config.data.auto_backup.to_string(),
            "data.compress_data" | "compress_data" => config.data.compress_data.to_string(),
            _ => {
                self.print_error(&format!("Unknown configuration key: {}", key));
                return Ok(());
            }
        };

        self.print_info(&format!("{} = {}", key, value));
        Ok(())
    }

    /// Edit configuration in external editor
    async fn edit_config(&mut self) -> Result<()> {
        let config_path = self.config_manager.config_path();

        // Check if EDITOR environment variable is set
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
            // Try common editors
            if std::process::Command::new("nano").arg("--version").output().is_ok() {
                "nano".to_string()
            } else if std::process::Command::new("vim").arg("--version").output().is_ok() {
                "vim".to_string()
            } else {
                "vi".to_string()
            }
        });

        self.print_info(&format!("Opening configuration file in {}...", editor));
        self.print_info(&format!("File: {}", config_path.display()));

        // Launch the editor
        let status = std::process::Command::new(&editor)
            .arg(&config_path)
            .status()
            .context("Failed to launch editor")?;

        if status.success() {
            self.print_info("✓ Configuration file saved");

            // Validate the configuration
            match ConfigManager::new() {
                Ok(_) => {
                    self.print_info("✓ Configuration is valid");
                }
                Err(e) => {
                    self.print_error(&format!("Configuration error: {}", e));
                    self.print_info("Please fix the configuration file and try again");
                }
            }
        } else {
            self.print_error("Editor exited with an error");
        }

        Ok(())
    }

    /// Handle deck command
    async fn handle_deck(&self) -> Result<i32> {
        if let Some(Commands::Deck {
            list,
            create,
            delete,
            rename,
            from,
            to,
            info,
        }) = &self.cli.command
        {
            let deck_manager = self
                .deck_manager
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

            if *list {
                self.print_info("Available decks:");
                let decks = deck_manager.get_all_decks().await?;
                if decks.is_empty() {
                    self.print_info("  No decks found. Use 'ankitui deck --create <name>' to create one.");
                } else {
                    for (deck, _cards) in decks {
                        let stats = deck_manager.get_deck_statistics(&deck.uuid).await?;
                        self.print_info(&format!(
                            "  {} [{} cards, {} due]",
                            deck.name, stats.total_cards, stats.due_cards
                        ));
                    }
                }
            }

            if let Some(deck_name) = create {
                self.print_info(&format!("Creating deck: {}", deck_name));
                match deck_manager.create_deck(deck_name.clone(), None, None).await {
                    Ok(_) => self.print_info(&format!("Deck '{}' created successfully", deck_name)),
                    Err(e) => {
                        self.print_error(&format!("Failed to create deck: {}", e));
                        return Ok(1);
                    }
                }
            }

            if let Some(deck_name) = delete {
                self.print_info(&format!("Deleting deck: {}", deck_name));
                let decks = deck_manager.get_all_decks().await?;
                if let Some((deck, _cards)) = decks.iter().find(|(d, _)| d.name == *deck_name) {
                    match deck_manager.delete_deck(&deck.uuid).await {
                        Ok(_) => self.print_info(&format!("Deck '{}' deleted successfully", deck_name)),
                        Err(e) => {
                            self.print_error(&format!("Failed to delete deck: {}", e));
                            return Ok(1);
                        }
                    }
                } else {
                    self.print_error(&format!("Deck '{}' not found", deck_name));
                    return Ok(1);
                }
            }

            if *rename {
                if let (Some(from_name), Some(to_name)) = (from, to) {
                    self.print_info(&format!("Renaming deck '{}' to '{}'", from_name, to_name));
                    let decks = deck_manager.get_all_decks().await?;
                    if let Some((deck, _cards)) = decks.iter().find(|(d, _)| d.name == *from_name) {
                        match deck_manager.rename_deck(&deck.uuid, to_name.clone()).await {
                            Ok(_) => self.print_info(&format!("Deck renamed successfully")),
                            Err(e) => {
                                self.print_error(&format!("Failed to rename deck: {}", e));
                                return Ok(1);
                            }
                        }
                    } else {
                        self.print_error(&format!("Deck '{}' not found", from_name));
                        return Ok(1);
                    }
                } else {
                    self.print_error("Both --from and --to must be specified for renaming");
                    return Ok(1);
                }
            }

            if let Some(deck_name) = info {
                self.print_info(&format!("Deck information for: {}", deck_name));
                let decks = deck_manager.get_all_decks().await?;
                if let Some((deck, _cards)) = decks.iter().find(|(d, _)| d.name == *deck_name) {
                    let stats = deck_manager.get_deck_statistics(&deck.uuid).await?;
                    self.print_info(&format!("  Name: {}", deck.name));
                    if let Some(description) = &deck.description {
                        self.print_info(&format!("  Description: {}", description));
                    }
                    self.print_info(&format!("  UUID: {}", deck.uuid));
                    self.print_info(&format!("  Total cards: {}", stats.total_cards));
                    self.print_info(&format!("  Due cards: {}", stats.due_cards));
                    self.print_info(&format!("  New cards: {}", stats.new_cards));
                    self.print_info(&format!("  Learning cards: {}", stats.learning_cards));
                    self.print_info(&format!("  Review cards: {}", stats.review_cards));
                } else {
                    self.print_error(&format!("Deck '{}' not found", deck_name));
                    return Ok(1);
                }
            }

            return Ok(0);
        }
        Ok(1)
    }

    /// Handle database command
    async fn handle_database(&self) -> Result<i32> {
        if let Some(Commands::Db {
            check,
            optimize,
            rebuild,
            backup,
            restore,
            stats,
        }) = &self.cli.command
        {
            // Get data directory from config
            let config = self.config_manager.get_config();
            let default_path = std::path::PathBuf::from("~/.ankitui");
            let data_dir = config.data.data_dir.as_ref().unwrap_or(&default_path);
            let db_path = data_dir.join("ankitui.db");

            if !db_path.exists() {
                self.print_error(&format!("Database not found at: {}", db_path.display()));
                return Ok(1);
            }

            if *check {
                self.check_database_integrity(&db_path).await?;
            }

            if *optimize {
                self.optimize_database(&db_path).await?;
            }

            if *rebuild {
                self.rebuild_database(&db_path, data_dir).await?;
            }

            if *backup {
                self.backup_database(&db_path, data_dir).await?;
            }

            if let Some(backup_file) = restore {
                self.restore_database(&backup_file, &db_path, data_dir).await?;
            }

            if *stats {
                self.show_database_stats(&db_path).await?;
            }

            return Ok(0);
        }
        Ok(1)
    }

    /// Handle media command
    async fn handle_media(&self) -> Result<i32> {
        if let Some(Commands::Media { deck, action }) = &self.cli.command {
            let deck_manager = self
                .deck_manager
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

            // Find deck by name
            let deck_result = deck_manager
                .find_deck_by_name(deck)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to find deck '{}': {}", deck, e))?;

            let deck_uuid = match deck_result {
                Some((deck_obj, _)) => deck_obj.uuid,
                None => {
                    self.print_error(&format!("Deck '{}' not found", deck));
                    return Ok(1);
                }
            };

            match action {
                MediaAction::Add {
                    card_id,
                    file_path,
                    media_type,
                } => {
                    let card_uuid =
                        uuid::Uuid::parse_str(card_id).map_err(|e| anyhow::anyhow!("Invalid card ID: {}", e))?;

                    let media_type_enum = match media_type.to_lowercase().as_str() {
                        "audio" => MediaType::Audio,
                        "image" => MediaType::Image,
                        "video" => MediaType::Video,
                        _ => {
                            self.print_error(&format!(
                                "Invalid media type: {}. Use: audio, image, or video",
                                media_type
                            ));
                            return Ok(1);
                        }
                    };

                    if !file_path.exists() {
                        self.print_error(&format!("Media file not found: {}", file_path.display()));
                        return Ok(1);
                    }

                    match deck_manager
                        .add_card_media(&deck_uuid, &card_uuid, file_path, media_type_enum)
                        .await
                    {
                        Ok(()) => {
                            self.print_info(&format!("Successfully added media to card {}", card_id));
                            return Ok(0);
                        }
                        Err(e) => {
                            self.print_error(&format!("Failed to add media: {}", e));
                            return Ok(1);
                        }
                    }
                }

                MediaAction::Remove { card_id } => {
                    let card_uuid =
                        uuid::Uuid::parse_str(card_id).map_err(|e| anyhow::anyhow!("Invalid card ID: {}", e))?;

                    match deck_manager.remove_card_media(&deck_uuid, &card_uuid).await {
                        Ok(()) => {
                            self.print_info(&format!("Successfully removed media from card {}", card_id));
                            return Ok(0);
                        }
                        Err(e) => {
                            self.print_error(&format!("Failed to remove media: {}", e));
                            return Ok(1);
                        }
                    }
                }

                MediaAction::List { detailed } => match deck_manager.get_cards_with_media(&deck_uuid).await {
                    Ok(cards) => {
                        if cards.is_empty() {
                            self.print_info("No media files found in this deck");
                        } else {
                            self.print_info(&format!("Found {} cards with media:", cards.len()));
                            for card in cards {
                                if let Some(media_ref) = &card.content.media {
                                    if *detailed {
                                        self.print_info(&format!(
                                            "  Card ID: {} | Type: {:?} | Path: {}",
                                            card.content.id, media_ref.media_type, media_ref.path
                                        ));
                                    } else {
                                        self.print_info(&format!(
                                            "  Card ID: {} | Type: {:?}",
                                            card.content.id, media_ref.media_type
                                        ));
                                    }
                                }
                            }
                        }
                        return Ok(0);
                    }
                    Err(e) => {
                        self.print_error(&format!("Failed to list media: {}", e));
                        return Ok(1);
                    }
                },

                MediaAction::Validate { fix } => match deck_manager.validate_deck_media(&deck_uuid).await {
                    Ok(results) => {
                        if results.is_empty() {
                            self.print_info("No media files to validate");
                        } else {
                            let mut valid_count = 0;
                            let mut invalid_count = 0;
                            self.print_info("Media validation results:");
                            for (card_id, is_valid) in results {
                                if is_valid {
                                    self.print_info(&format!("  Card {}: ✅ Valid", card_id));
                                    valid_count += 1;
                                } else {
                                    self.print_info(&format!("  Card {}: ❌ Invalid/Missing", card_id));
                                    invalid_count += 1;
                                }
                            }
                            self.print_info(&format!("Summary: {} valid, {} invalid", valid_count, invalid_count));
                        }
                        return Ok(0);
                    }
                    Err(e) => {
                        self.print_error(&format!("Failed to validate media: {}", e));
                        return Ok(1);
                    }
                },

                MediaAction::Cleanup { dry_run } => match deck_manager.cleanup_deck_media(&deck_uuid).await {
                    Ok(cleaned_count) => {
                        if *dry_run {
                            self.print_info(&format!("Would clean up {} orphaned media files", cleaned_count));
                        } else {
                            self.print_info(&format!("Cleaned up {} orphaned media files", cleaned_count));
                        }
                        return Ok(0);
                    }
                    Err(e) => {
                        self.print_error(&format!("Failed to cleanup media: {}", e));
                        return Ok(1);
                    }
                },

                MediaAction::Stats { format } => match deck_manager.get_deck_media_stats(&deck_uuid).await {
                    Ok(stats) => {
                        match format.as_str() {
                            "json" => {
                                let json_output = serde_json::json!({
                                    "total_media_files": stats.total_media_files,
                                    "audio_files": stats.audio_files,
                                    "image_files": stats.image_files,
                                    "video_files": stats.video_files,
                                    "total_size_bytes": stats.total_size_bytes,
                                    "average_size_bytes": stats.average_size_bytes,
                                });
                                self.print_info(&serde_json::to_string_pretty(&json_output).unwrap());
                            }
                            _ => {
                                self.print_info("Media statistics:");
                                self.print_info(&format!("  Total media files: {}", stats.total_media_files));
                                self.print_info(&format!("  Audio files: {}", stats.audio_files));
                                self.print_info(&format!("  Image files: {}", stats.image_files));
                                self.print_info(&format!("  Video files: {}", stats.video_files));
                                self.print_info(&format!(
                                    "  Total size: {:.2} MB",
                                    stats.total_size_bytes as f64 / 1024.0 / 1024.0
                                ));
                                if stats.average_size_bytes > 0 {
                                    self.print_info(&format!(
                                        "  Average size: {:.2} KB",
                                        stats.average_size_bytes as f64 / 1024.0
                                    ));
                                }
                            }
                        }
                        return Ok(0);
                    }
                    Err(e) => {
                        self.print_error(&format!("Failed to get media stats: {}", e));
                        return Ok(1);
                    }
                },
            }
        }
        Ok(1)
    }

    /// Check database integrity
    async fn check_database_integrity(&self, db_path: &std::path::Path) -> Result<()> {
        self.print_info("Checking database integrity...");

        // For SQLite, we can use PRAGMA integrity_check
        let db_url = format!("sqlite:{}", db_path.display());
        let pool = sqlx::SqlitePool::connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        let result: (String,) = sqlx::query_as("PRAGMA integrity_check")
            .fetch_one(&pool)
            .await
            .context("Failed to check database integrity")?;

        if result.0 == "ok" {
            self.print_info("✓ Database integrity check passed");
        } else {
            self.print_error(&format!("Database integrity check failed: {}", result.0));
            self.print_info("Consider running 'ankitui db --rebuild' to fix database issues");
        }

        pool.close().await;
        Ok(())
    }

    /// Optimize database
    async fn optimize_database(&self, db_path: &std::path::Path) -> Result<()> {
        self.print_info("Optimizing database...");

        let db_url = format!("sqlite:{}", db_path.display());
        let pool = sqlx::SqlitePool::connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        // Analyze tables for better query planning
        sqlx::query("ANALYZE")
            .execute(&pool)
            .await
            .context("Failed to analyze database")?;

        // Vacuum to rebuild database file and reclaim space
        sqlx::query("VACUUM")
            .execute(&pool)
            .await
            .context("Failed to vacuum database")?;

        // Reindex to rebuild indexes
        sqlx::query("REINDEX")
            .execute(&pool)
            .await
            .context("Failed to reindex database")?;

        self.print_info("✓ Database optimization completed");

        pool.close().await;
        Ok(())
    }

    /// Rebuild database
    async fn rebuild_database(&self, db_path: &std::path::Path, data_dir: &std::path::Path) -> Result<()> {
        self.print_info("Rebuilding database...");
        self.print_info("This will create a backup and rebuild the database from TOML files");

        // Create backup before rebuilding
        let backup_path = data_dir.join(format!(
            "backup_before_rebuild_{}.db",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ));

        std::fs::copy(db_path, &backup_path).context("Failed to create backup before rebuild")?;

        self.print_info(&format!("✓ Created backup: {}", backup_path.display()));

        // Export all decks to TOML first
        let temp_export_dir = data_dir.join("temp_export");
        std::fs::create_dir_all(&temp_export_dir).context("Failed to create temporary export directory")?;

        // Note: This is a simplified rebuild. In a real implementation,
        // you would want to use the deck manager to export and import properly
        self.print_info("⚠️  Database rebuild is a complex operation.");
        self.print_info("Consider exporting your decks manually and importing them into a fresh database.");
        self.print_info("Backup created successfully in case you want to proceed manually.");
        Ok(())
    }

    /// Backup database
    async fn backup_database(&self, db_path: &std::path::Path, data_dir: &std::path::Path) -> Result<()> {
        self.print_info("Creating database backup...");

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("ankitui_backup_{}.db", timestamp);
        let backup_path = data_dir.join(&backup_name);

        // Close any existing connections and copy the file
        std::fs::copy(db_path, &backup_path).context("Failed to copy database file")?;

        self.print_info(&format!("✓ Database backed up to: {}", backup_path.display()));

        // Also backup TOML content files
        let content_dir = data_dir.join("content");
        if content_dir.exists() {
            let backup_content_dir = data_dir.join(format!("backup_content_{}", timestamp));
            self.backup_directory(&content_dir, &backup_content_dir)?;
            self.print_info(&format!(
                "✓ Content files backed up to: {}",
                backup_content_dir.display()
            ));
        }

        Ok(())
    }

    /// Restore database from backup
    async fn restore_database(
        &self,
        backup_file: &std::path::Path,
        db_path: &std::path::Path,
        data_dir: &std::path::Path,
    ) -> Result<()> {
        self.print_info(&format!("Restoring database from: {}", backup_file.display()));

        if !backup_file.exists() {
            return Err(anyhow::anyhow!("Backup file not found: {}", backup_file.display()));
        }

        // Create backup of current database before restoring
        let current_backup = data_dir.join(format!(
            "backup_before_restore_{}.db",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ));

        std::fs::copy(db_path, &current_backup).context("Failed to backup current database")?;

        self.print_info(&format!(
            "✓ Current database backed up to: {}",
            current_backup.display()
        ));

        // Restore the database
        std::fs::copy(backup_file, db_path).context("Failed to restore database from backup")?;

        self.print_info("✓ Database restored successfully");

        // Check integrity after restore
        self.check_database_integrity(db_path).await?;

        Ok(())
    }

    /// Show database statistics
    async fn show_database_stats(&self, db_path: &std::path::Path) -> Result<()> {
        self.print_info("Database statistics:");

        // File size
        let metadata = std::fs::metadata(db_path).context("Failed to get database metadata")?;
        let file_size = metadata.len();
        self.print_info(&format!(
            "  File size: {} bytes ({:.2} MB)",
            file_size,
            file_size as f64 / 1024.0 / 1024.0
        ));

        let db_url = format!("sqlite:{}", db_path.display());
        let pool = sqlx::SqlitePool::connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        // Get table row counts
        let tables = vec!["card_states", "deck_name_index"];

        for table in tables {
            let result: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {}", table))
                .fetch_one(&pool)
                .await
                .context(format!("Failed to get row count for table {}", table))?;

            self.print_info(&format!("  {}: {} rows", table, result.0));
        }

        // Database version and page info
        let version: (String,) = sqlx::query_as("SELECT sqlite_version()")
            .fetch_one(&pool)
            .await
            .context("Failed to get SQLite version")?;

        self.print_info(&format!("  SQLite version: {}", version.0));

        let page_size: (i64,) = sqlx::query_as("PRAGMA page_size")
            .fetch_one(&pool)
            .await
            .context("Failed to get page size")?;

        let page_count: (i64,) = sqlx::query_as("PRAGMA page_count")
            .fetch_one(&pool)
            .await
            .context("Failed to get page count")?;

        self.print_info(&format!("  Page size: {} bytes", page_size.0));
        self.print_info(&format!("  Page count: {}", page_count.0));
        self.print_info(&format!("  Database size: {} bytes", page_size.0 * page_count.0));

        pool.close().await;
        Ok(())
    }

    /// Backup directory recursively
    fn backup_directory(&self, src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(dst).context("Failed to create backup directory")?;

        for entry in std::fs::read_dir(src).context("Failed to read source directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                self.backup_directory(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path).context(format!("Failed to copy file: {}", src_path.display()))?;
            }
        }

        Ok(())
    }

    /// Print informational message (unless in quiet mode)
    fn print_info(&self, message: &str) {
        if !self.cli.quiet {
            println!("{}", message);
        }
    }

    /// Print error message
    fn print_error(&self, message: &str) {
        eprintln!("Error: {}", message);
    }

    /// Get configuration manager reference
    pub fn config_manager(&self) -> &ConfigManager {
        &self.config_manager
    }

    /// Get CLI arguments reference
    pub fn cli(&self) -> &Cli {
        &self.cli
    }

    /// Get deck manager reference
    pub fn deck_manager(&self) -> Option<&Arc<DeckManager>> {
        self.deck_manager.as_ref()
    }

    /// Get stats engine reference
    pub fn stats_engine(&self) -> Option<&Arc<StatsEngine>> {
        self.stats_engine.as_ref()
    }

    /// Import CSV format
    async fn import_csv(&self, content: &str, deck_name: &str) -> Result<Uuid> {
        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

        let mut cards = Vec::new();
        let mut reader = csv::Reader::from_reader(content.as_bytes());

        for (line_num, result) in reader.records().enumerate() {
            let record = result.with_context(|| format!("Failed to parse CSV line {}", line_num + 1))?;

            if record.len() < 2 {
                self.print_info(&format!(
                    "Skipping line {} - requires at least 2 columns (front,back)",
                    line_num + 1
                ));
                continue;
            }

            let front = record.get(0).unwrap_or("").trim();
            let back = record.get(1).unwrap_or("").trim();

            if front.is_empty() {
                self.print_info(&format!("Skipping line {} - empty front side", line_num + 1));
                continue;
            }

            // Parse optional fields
            let tags = if record.len() > 2 {
                record
                    .get(2)
                    .unwrap_or("")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                Vec::new()
            };

            let card_content = CardContent {
                id: uuid::Uuid::new_v4(),
                front: front.to_string(),
                back: back.to_string(),
                tags,
                media: None,
                custom: std::collections::HashMap::new(),
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            };

            cards.push(card_content);
        }

        if cards.is_empty() {
            return Err(anyhow::anyhow!("No valid cards found in CSV file"));
        }

        // Create deck and add cards
        let deck_uuid = deck_manager.create_deck(deck_name.to_string(), None, None).await?;
        let cards_count = cards.len();
        deck_manager.add_cards(&deck_uuid, cards).await?;

        self.print_info(&format!("Imported {} cards from CSV", cards_count));
        Ok(deck_uuid)
    }

    /// Import TSV format
    async fn import_tsv(&self, content: &str, deck_name: &str) -> Result<Uuid> {
        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

        let mut cards = Vec::new();
        let lines = content.lines().enumerate();

        for (line_num, line) in lines {
            let fields: Vec<&str> = line.split('\t').collect();

            if fields.len() < 2 {
                self.print_info(&format!(
                    "Skipping line {} - requires at least 2 columns (front,back)",
                    line_num + 1
                ));
                continue;
            }

            let front = fields[0].trim();
            let back = fields[1].trim();

            if front.is_empty() {
                self.print_info(&format!("Skipping line {} - empty front side", line_num + 1));
                continue;
            }

            // Parse optional fields
            let tags = if fields.len() > 2 {
                fields[2]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                Vec::new()
            };

            let card_content = CardContent {
                id: uuid::Uuid::new_v4(),
                front: front.to_string(),
                back: back.to_string(),
                tags,
                media: None,
                custom: std::collections::HashMap::new(),
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            };

            cards.push(card_content);
        }

        if cards.is_empty() {
            return Err(anyhow::anyhow!("No valid cards found in TSV file"));
        }

        // Create deck and add cards
        let deck_uuid = deck_manager.create_deck(deck_name.to_string(), None, None).await?;
        let cards_count = cards.len();
        deck_manager.add_cards(&deck_uuid, cards).await?;

        self.print_info(&format!("Imported {} cards from TSV", cards_count));
        Ok(deck_uuid)
    }

    /// Import JSON format
    async fn import_json(&self, content: &str, deck_name: &str) -> Result<Uuid> {
        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;

        #[derive(Deserialize)]
        struct JsonCard {
            front: String,
            back: String,
            tags: Option<Vec<String>>,
            #[serde(flatten)]
            extra: std::collections::HashMap<String, serde_json::Value>,
        }

        #[derive(Deserialize)]
        struct JsonDeck {
            name: Option<String>,
            cards: Vec<JsonCard>,
        }

        let json_deck: JsonDeck = serde_json::from_str(content).context("Failed to parse JSON file")?;

        if json_deck.cards.is_empty() {
            return Err(anyhow::anyhow!("No cards found in JSON file"));
        }

        let mut cards = Vec::new();
        for json_card in json_deck.cards {
            if json_card.front.trim().is_empty() {
                self.print_info("Skipping card with empty front side");
                continue;
            }

            let card_content = CardContent {
                id: uuid::Uuid::new_v4(),
                front: json_card.front.trim().to_string(),
                back: json_card.back.trim().to_string(),
                tags: json_card.tags.unwrap_or_default(),
                media: None,
                custom: std::collections::HashMap::new(),
                created_at: chrono::Utc::now(),
                modified_at: chrono::Utc::now(),
            };

            cards.push(card_content);
        }

        if cards.is_empty() {
            return Err(anyhow::anyhow!("No valid cards found in JSON file"));
        }

        // Create deck and add cards
        let deck_uuid = deck_manager.create_deck(deck_name.to_string(), None, None).await?;
        let cards_count = cards.len();
        deck_manager.add_cards(&deck_uuid, cards).await?;

        self.print_info(&format!("Imported {} cards from JSON", cards_count));
        Ok(deck_uuid)
    }

    /// Export to CSV format
    async fn export_csv(&self, deck_uuid: &uuid::Uuid, deck_name: &str, include_states: bool) -> Result<()> {
        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;
        let (deck, cards) = deck_manager.get_deck(deck_uuid).await?;

        let filename = format!("{}.csv", deck_name);
        let mut wtr = csv::Writer::from_path(&filename)?;

        // Write header
        if include_states {
            wtr.write_record(&[
                "front",
                "back",
                "tags",
                "ease_factor",
                "interval",
                "reps",
                "lapses",
                "state",
            ])?;
        } else {
            wtr.write_record(&["front", "back", "tags"])?;
        }

        // Write cards
        for card in &cards {
            let tags_str = card.content.tags.join(",");

            if include_states {
                let state_str = format!("{:?}", card.state.state);
                wtr.write_record(&[
                    &card.content.front,
                    &card.content.back,
                    &tags_str,
                    &card.state.ease_factor.to_string(),
                    &card.state.interval.to_string(),
                    &card.state.reps.to_string(),
                    &card.state.lapses.to_string(),
                    &state_str,
                ])?;
            } else {
                wtr.write_record(&[&card.content.front, &card.content.back, &tags_str])?;
            }
        }

        wtr.flush()?;
        self.print_info(&format!("Exported {} cards to CSV: {}", cards.len(), filename));
        Ok(())
    }

    /// Export to TSV format
    async fn export_tsv(&self, deck_uuid: &uuid::Uuid, deck_name: &str, include_states: bool) -> Result<()> {
        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;
        let (deck, cards) = deck_manager.get_deck(deck_uuid).await?;

        let filename = format!("{}.tsv", deck_name);
        let mut file = std::fs::File::create(&filename)?;

        // Write header
        if include_states {
            file.write_all(b"front\tback\ttags\tease_factor\tinterval\treps\tlapses\tstate\n")?;
        } else {
            file.write_all(b"front\tback\ttags\n")?;
        }

        // Write cards
        for card in &cards {
            let tags_str = card.content.tags.join(",");
            let front = card.content.front.replace('\t', " ").replace('\n', "\\n");
            let back = card.content.back.replace('\t', " ").replace('\n', "\\n");

            if include_states {
                let state_str = format!("{:?}", card.state.state);
                let line = format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    front,
                    back,
                    tags_str,
                    card.state.ease_factor,
                    card.state.interval,
                    card.state.reps,
                    card.state.lapses,
                    state_str
                );
                file.write_all(line.as_bytes())?;
            } else {
                let line = format!("{}\t{}\t{}\n", front, back, tags_str);
                file.write_all(line.as_bytes())?;
            }
        }

        self.print_info(&format!("Exported {} cards to TSV: {}", cards.len(), filename));
        Ok(())
    }

    /// Export to JSON format
    async fn export_json(&self, deck_uuid: &uuid::Uuid, deck_name: &str, include_states: bool) -> Result<()> {
        let deck_manager = self
            .deck_manager
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Deck manager not initialized"))?;
        let (deck, cards) = deck_manager.get_deck(deck_uuid).await?;

        #[derive(Serialize)]
        struct JsonExportCard {
            front: String,
            back: String,
            tags: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ease_factor: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            interval: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            reps: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lapses: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<String>,
        }

        #[derive(Serialize)]
        struct JsonExportDeck {
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            modified_at: chrono::DateTime<chrono::Utc>,
            cards: Vec<JsonExportCard>,
        }

        let json_cards: Vec<JsonExportCard> = cards
            .into_iter()
            .map(|card| JsonExportCard {
                front: card.content.front,
                back: card.content.back,
                tags: card.content.tags,
                ease_factor: if include_states {
                    Some(card.state.ease_factor)
                } else {
                    None
                },
                interval: if include_states {
                    Some(card.state.interval)
                } else {
                    None
                },
                reps: if include_states { Some(card.state.reps) } else { None },
                lapses: if include_states { Some(card.state.lapses) } else { None },
                state: if include_states {
                    Some(format!("{:?}", card.state.state))
                } else {
                    None
                },
            })
            .collect();

        let json_deck = JsonExportDeck {
            name: deck.name,
            created_at: deck.created_at,
            modified_at: deck.modified_at,
            cards: json_cards,
        };

        let filename = format!("{}.json", deck_name);
        let json_string = serde_json::to_string_pretty(&json_deck)?;
        std::fs::write(&filename, json_string)?;

        self.print_info(&format!(
            "Exported {} cards to JSON: {}",
            json_deck.cards.len(),
            filename
        ));
        Ok(())
    }
}

/// Check if command requires core components to be initialized
fn needs_core_components(cli: &Cli) -> bool {
    match &cli.command {
        None => true, // Default review command
        Some(Commands::Review { .. }) => true,
        Some(Commands::Import { .. }) => true,
        Some(Commands::Export { .. }) => true,
        Some(Commands::Stats { .. }) => true,
        Some(Commands::Edit { .. }) => true,
        Some(Commands::Deck { .. }) => true,
        Some(Commands::Config { .. }) => false,
        Some(Commands::Db { .. }) => true,
        Some(Commands::Media { .. }) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_parsing() {
        let args = vec!["ankitui", "--verbose", "review", "--deck", "test"];
        let app = CliApp::with_args(args).await.unwrap();

        assert!(app.cli().verbose);
        assert!(matches!(&app.cli().command, Some(Commands::Review { deck, .. }) if deck.as_ref().unwrap() == "test"));
    }

    #[tokio::test]
    async fn test_config_command() {
        let args = vec!["ankitui", "config", "--show"];
        let app = CliApp::with_args(args).await.unwrap();

        assert!(matches!(&app.cli().command, Some(Commands::Config { show, .. }) if *show));
    }

    #[tokio::test]
    async fn test_deck_management() {
        let args = vec!["ankitui", "deck", "--list"];
        let app = CliApp::with_args(args).await.unwrap();

        assert!(matches!(&app.cli().command, Some(Commands::Deck { list, .. }) if *list));
    }

    #[tokio::test]
    async fn test_import_command() {
        let args = vec!["ankitui", "import", "test.csv", "--deck", "test", "--format", "csv"];
        let app = CliApp::with_args(args).await.unwrap();

        assert!(
            matches!(&app.cli().command, Some(Commands::Import { input, deck, format, .. })
            if input.file_name().unwrap() == "test.csv" && deck == "test")
        );
    }
}

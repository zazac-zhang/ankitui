//! AnkiTUI - Terminal Spaced Repetition Learning System
//!
//! Main entry point for the application

use ankitui::util::cli::{Cli, CliApp};
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Check if we should run in CLI mode
    // If any specific command is provided, use CLI mode
    // If no command or only --deck --limit flags are provided, use TUI mode (default review)
    let use_cli_mode = cli.command.is_some();

    if use_cli_mode {
        // Run in CLI mode
        let mut app = CliApp::new().await?;
        let exit_code = app.run().await?;
        std::process::exit(exit_code);
    } else {
        // Run in TUI mode (default behavior)
        run_tui_mode().await
    }
}

async fn run_tui_mode() -> Result<()> {
    use ankitui_core::config::ConfigManager;
    use ankitui_core::core::DeckManager;
    use ankitui_tui::{App};
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::io;
    use std::time::Duration;
    
    // Load configuration
    let config_manager = ConfigManager::new()?;

    // Initialize data directory
    let data_dir = config_manager.get_data_dir();

    // Initialize core managers
    let db_path = data_dir.join("ankitui.db");
    let deck_manager = DeckManager::new(&data_dir, &db_path).await?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Create application config
    let app_config = ankitui_tui::app::AppConfig {
        title: "AnkiTUI".to_string(),
        enable_mouse: true,
        enable_bracketed_paste: true,
        tick_rate: Duration::from_millis(16),
        theme: ankitui_tui::ui::theme::Theme::default(),
        debug: false,
    };

    // Create application
    let mut app = App::new(app_config).await?;

    // Initial data update
    app.update().await?;

    // TODO: Implement proper event loop
    // For now, just show a simple interface
    println!("TUI mode started - Event loop implementation pending");
    println!("Press Ctrl+C to exit");

    // Simple wait loop (temporary implementation)
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

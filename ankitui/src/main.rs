//! AnkiTUI - Terminal Spaced Repetition Learning System
//!
//! Main entry point for the application

use ankitui::util::cli::{Cli, CliApp};
use anyhow::Result;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

/// Initialize logging with both console and file output
fn init_logging() {
    // Load .env file if it exists
    let _ = dotenv::dotenv();

    // Use default log level - will be configurable via config file in future
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // Create/open log file in current directory
    let log_file_path = "./ankitui.log";

    // Write session start marker to file
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_file_path) {
        let _ = writeln!(
            file,
            "\n=== AnkiTUI Session Started at {} ===",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f")
        );
    }

    // Initialize logger with custom format that writes to both stderr and file
    let log_file_path_clone = log_file_path.to_string();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&log_level))
        .format(move |_buf, record| {
            let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let log_message = format!(
                "{} [{}] {}:{} - {}",
                timestamp,
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            );

            // Write to console (stderr)
            eprintln!("{}", log_message);

            // Also write to file
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file_path_clone) {
                let _ = writeln!(file, "{}", log_message);
            }

            Ok(())
        })
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init_logging();

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
    use ankitui_tui::App;
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

    // Run the main event loop
    ankitui_tui::run_event_loop_with_app(&mut app, None).await?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

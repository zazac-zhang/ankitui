//! Terminal management for the TUI application

use crate::utils::error::{TuiError, TuiResult};
use crossterm::{
    cursor::{Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

/// Terminal configuration
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub enable_mouse: bool,
    pub enable_bracketed_paste: bool,
    pub title: Option<String>,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            enable_mouse: true,
            enable_bracketed_paste: true,
            title: Some("AnkiTUI V2".to_string()),
        }
    }
}

/// Terminal manager for handling terminal setup and cleanup
pub struct TerminalManager {
    config: TerminalConfig,
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
    initialized: bool,
}

impl TerminalManager {
    /// Create a new terminal manager with configuration
    pub fn new(config: TerminalConfig) -> Self {
        Self {
            config,
            terminal: None,
            initialized: false,
        }
    }

    /// Get terminal configuration
    pub fn config(&self) -> &TerminalConfig {
        &self.config
    }

    /// Check if terminal is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Initialize the terminal for TUI operation
    pub fn initialize(&mut self) -> TuiResult<()> {
        if self.initialized {
            return Ok(());
        }

        log::debug!("Initializing terminal");

        // Enable raw mode
        enable_raw_mode().map_err(|e| TuiError::render(format!("Failed to enable raw mode: {}", e)))?;

        // Enter alternate screen
        execute!(io::stdout(), EnterAlternateScreen, Hide, Clear(ClearType::All))
            .map_err(|e| TuiError::render(format!("Failed to setup terminal: {}", e)))?;

        // Enable mouse capture if configured
        if self.config.enable_mouse {
            execute!(io::stdout(), EnableMouseCapture)
                .map_err(|e| TuiError::render(format!("Failed to enable mouse capture: {}", e)))?;
        }

        // Create terminal
        let backend = CrosstermBackend::new(io::stdout());
        self.terminal = Some(Terminal::new(backend)?);

        // Set terminal title if provided
        if let Some(title) = &self.config.title {
            self.set_title(title)?;
        }

        self.initialized = true;
        log::debug!("Terminal initialized successfully");

        Ok(())
    }

    /// Cleanup terminal state
    pub fn cleanup(&mut self) -> TuiResult<()> {
        if !self.initialized {
            return Ok(());
        }

        log::debug!("Cleaning up terminal");

        // Restore terminal state
        if self.config.enable_mouse {
            let _ = execute!(io::stdout(), DisableMouseCapture);
        }

        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);

        let _ = disable_raw_mode();

        // Flush terminal
        if let Some(terminal) = &mut self.terminal {
            let _ = terminal.show_cursor();
        }

        self.terminal = None;
        self.initialized = false;

        log::debug!("Terminal cleanup completed");
        Ok(())
    }

    /// Get terminal reference
    pub fn terminal(&mut self) -> TuiResult<&mut Terminal<CrosstermBackend<io::Stdout>>> {
        if !self.initialized {
            return Err(TuiError::render("Terminal not initialized"));
        }

        self.terminal
            .as_mut()
            .ok_or_else(|| TuiError::render("Terminal not available"))
    }

    /// Set terminal title
    pub fn set_title(&self, title: &str) -> TuiResult<()> {
        // Set terminal title using escape sequences
        // This works on most terminals
        execute!(io::stdout(), crossterm::terminal::SetTitle(title))
            .map_err(|e| TuiError::render(format!("Failed to set terminal title: {}", e)))?;

        Ok(())
    }

    /// Clear the terminal screen
    pub fn clear(&mut self) -> TuiResult<()> {
        if let Some(terminal) = &mut self.terminal {
            terminal
                .clear()
                .map_err(|e| TuiError::render(format!("Failed to clear terminal: {}", e)))?;
        }
        Ok(())
    }

    /// Hide the cursor
    pub fn hide_cursor(&mut self) -> TuiResult<()> {
        if let Some(terminal) = &mut self.terminal {
            terminal
                .hide_cursor()
                .map_err(|e| TuiError::render(format!("Failed to hide cursor: {}", e)))?;
        }
        Ok(())
    }

    /// Show the cursor
    pub fn show_cursor(&mut self) -> TuiResult<()> {
        if let Some(terminal) = &mut self.terminal {
            terminal
                .show_cursor()
                .map_err(|e| TuiError::render(format!("Failed to show cursor: {}", e)))?;
        }
        Ok(())
    }

    /// Get terminal size
    pub fn size(&self) -> TuiResult<ratatui::layout::Size> {
        if !self.initialized {
            return Err(TuiError::render("Terminal not initialized"));
        }

        crossterm::terminal::size()
            .map(|(width, height)| ratatui::layout::Size { width, height })
            .map_err(|e| TuiError::render(format!("Failed to get terminal size: {}", e)))
    }

    /// Check if terminal supports Unicode
    pub fn supports_unicode(&self) -> bool {
        // Check if terminal supports Unicode by checking locale
        std::env::var("LANG")
            .ok()
            .and_then(|lang| {
                if lang.contains("UTF-8") || lang.contains("utf-8") {
                    Some(true)
                } else {
                    Some(false)
                }
            })
            .unwrap_or(false)
    }

    /// Check if terminal supports colors
    pub fn supports_colors(&self) -> bool {
        // Most modern terminals support colors
        true
    }

    /// Enable or disable mouse capture
    pub fn set_mouse_capture(&mut self, enable: bool) -> TuiResult<()> {
        if !self.initialized {
            return Ok(());
        }

        if enable && !self.config.enable_mouse {
            execute!(io::stdout(), EnableMouseCapture)
                .map_err(|e| TuiError::render(format!("Failed to enable mouse capture: {}", e)))?;
        } else if !enable && self.config.enable_mouse {
            execute!(io::stdout(), DisableMouseCapture)
                .map_err(|e| TuiError::render(format!("Failed to disable mouse capture: {}", e)))?;
        }

        self.config.enable_mouse = enable;
        Ok(())
    }

    /// Suspend terminal (for external programs)
    pub fn suspend(&mut self) -> TuiResult<()> {
        if !self.initialized {
            return Ok(());
        }

        // Leave raw mode and alternate screen temporarily
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
        let _ = disable_raw_mode();

        // Send SIGTSTP to suspend the process
        #[cfg(unix)]
        unsafe {
            libc::raise(libc::SIGTSTP);
        }

        // Re-initialize when resumed
        self.initialize()?;
        Ok(())
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new(TerminalConfig::default())
    }
}

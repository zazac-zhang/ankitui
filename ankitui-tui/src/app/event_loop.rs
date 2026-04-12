//! Event loop implementation with stateful event handling

use crate::domain::CardRating;
use crate::ui::event::{Command, CommandType, Event};
use crate::ui::render::Renderer;
use crate::ui::state::{AppState, StateStore};
use crate::utils::error::TuiResult;
use crossterm::event::{poll, read, Event as CrosstermEvent, KeyEvent, MouseEvent};
use crossterm::event::{KeyCode, KeyModifiers, MouseEvent as CrosstermMouseEvent, MouseEventKind};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid;

/// Event loop configuration
#[derive(Debug, Clone)]
pub struct EventLoopConfig {
    pub tick_rate: Duration,
    pub enable_mouse: bool,
    pub enable_bracketed_paste: bool,
    pub capture_keyboard: bool,
}

impl Default for EventLoopConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(16), // ~60 FPS
            enable_mouse: true,
            enable_bracketed_paste: true,
            capture_keyboard: true,
        }
    }
}

/// Event loop with stateful event handling
pub struct EventLoop {
    config: EventLoopConfig,
    last_tick: Instant,
    state_store: Arc<RwLock<StateStore>>,
}

impl EventLoop {
    /// Create a new event loop with configuration and state store
    pub fn new(config: EventLoopConfig, state_store: Arc<RwLock<StateStore>>) -> Self {
        Self {
            config,
            last_tick: Instant::now(),
            state_store,
        }
    }

    /// Get event loop configuration
    pub fn config(&self) -> &EventLoopConfig {
        &self.config
    }

    /// Run the event loop with stateful event handling
    pub async fn run(&mut self) -> TuiResult<Option<Command>> {
        // Check for tick timeout
        if self.last_tick.elapsed() >= self.config.tick_rate {
            self.last_tick = Instant::now();

            // Handle periodic tasks
            if let Some(command) = self.handle_tick().await? {
                return Ok(Some(command));
            }
        }

        // Poll for events
        if poll(Duration::from_millis(0))? {
            let crossterm_event = read()?;

            // Convert to our Event type
            let event = match crossterm_event {
                CrosstermEvent::Key(key_event) => Event::Key(key_event),
                CrosstermEvent::Mouse(mouse_event) => Event::Mouse(mouse_event),
                CrosstermEvent::Resize(width, height) => Event::Resize(width, height),
                CrosstermEvent::FocusGained => Event::FocusGained,
                CrosstermEvent::FocusLost => Event::FocusLost,
                CrosstermEvent::Paste(content) => Event::Paste(content),
            };

            // Get current state
            let current_state = self.get_current_state().await;

            // Handle event with context awareness using current state
            let command = handle_event_with_state(event, &current_state);

            return Ok(Some(command));
        }

        Ok(None)
    }

    /// Run the event loop until a quit command is received
    pub async fn run_until_quit(&mut self) -> TuiResult<()> {
        loop {
            if let Some(command) = self.run().await? {
                if matches!(command.command_type, CommandType::Quit) {
                    break;
                }
            }

            // Small delay to prevent busy waiting
            tokio::time::sleep(Duration::from_millis(1)).await;
        }

        Ok(())
    }

    /// Handle periodic tick events
    async fn handle_tick(&mut self) -> TuiResult<Option<Command>> {
        // Handle periodic tasks like:
        // - Auto-save
        // - Background synchronization
        // - Progress indicators
        // - Timer updates

        // For now, return no command on tick
        Ok(None)
    }

    /// Get current application state from state store
    async fn get_current_state(&self) -> AppState {
        let state_store = self.state_store.read().await;
        state_store.get_state()
    }

    /// Update event loop configuration
    pub fn update_config(&mut self, config: EventLoopConfig) {
        self.config = config;
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new(
            EventLoopConfig::default(),
            Arc::new(RwLock::new(StateStore::new())),
        )
    }
}

/// Event processor trait for command execution
pub trait EventProcessor: Send + Sync {
    /// Process a command and return whether to continue the event loop
    fn process_command(&mut self, command: Command) -> TuiResult<bool>;
}

/// Application event processor that connects commands to application logic
pub struct ApplicationEventProcessor {
    app_controller: crate::app::AppController<'static>,
}

impl ApplicationEventProcessor {
    pub fn new(app_controller: crate::app::AppController<'static>) -> Self {
        Self { app_controller }
    }
}

impl EventProcessor for ApplicationEventProcessor {
    fn process_command(&mut self, command: Command) -> TuiResult<bool> {
        // Save command type before moving command
        let command_type = command.command_type.clone();

        // Handle the command using the app controller
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.app_controller.handle_command(command).await })
        })?;

        // Continue event loop unless it's a quit command
        Ok(!matches!(command_type, CommandType::Quit))
    }
}

/// Convenience function to run the event loop with an application
pub async fn run_event_loop_with_app(
    mut app: &mut crate::app::App,
    config: Option<EventLoopConfig>,
) -> TuiResult<()> {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::backend::CrosstermBackend;
    use std::io;

    let event_loop_config = config.unwrap_or_default();

    // Use the app's existing state store to maintain state consistency
    let state_store = Arc::clone(&app.state_store);

    let mut event_loop = EventLoop::new(event_loop_config, state_store);

    // Initialize the application state
    app.initialize().await?;

    // Setup terminal for rendering
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Run the event loop and execute commands
    loop {
        // Sync renderer with current state
        let current_state = app.state_store.read().await.get_state();
        let current_screen = current_state.current_screen().clone();

        // Render the UI with state information
        let state_clone = current_state.clone();

        terminal.draw(|f| {
            let area = f.area();
            // Split the borrow by taking the renderer first
            let app_ptr = app as *const crate::app::main_app::App;
            let renderer = app.renderer_mut();

            // Now we can safely create an immutable reference
            let app_ref = unsafe { &*app_ptr };
            renderer.render_with_app_and_state(f, area, app_ref, &state_clone);
        })?;

        if let Some(command) = event_loop.run().await? {
            // Check for quit command
            if matches!(command.command_type, CommandType::Quit) {
                break;
            }

            // Execute the command using the app
            app.execute_command(command).await?;
        }

        // Small delay to prevent busy waiting
        tokio::time::sleep(Duration::from_millis(16)).await; // ~60 FPS
    }

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

/// Handle event with state-aware processing
fn handle_event_with_state(event: Event, current_state: &AppState) -> Command {
    match event {
        Event::Key(key_event) => handle_key_event_contextual(key_event, current_state),
        Event::Mouse(mouse_event) => handle_mouse_event_contextual(mouse_event, current_state),
        Event::Resize(width, height) => Command::system(CommandType::Resize(width, height)),
        Event::FocusGained => Command::system(CommandType::FocusGained),
        Event::FocusLost => Command::system(CommandType::FocusLost),
        Event::Paste(content) => handle_paste_contextual(content, current_state),
    }
}

/// Context-aware keyboard event handling
fn handle_key_event_contextual(event: KeyEvent, current_state: &AppState) -> Command {
    let screen = current_state.current_screen();

    match (event.code, event.modifiers) {
        // Navigation keys - context dependent
        (KeyCode::Up, KeyModifiers::NONE) => handle_navigation_up(screen, current_state),
        (KeyCode::Down, KeyModifiers::NONE) => handle_navigation_down(screen, current_state),
        (KeyCode::Left, KeyModifiers::NONE) => handle_navigation_left(screen, current_state),
        (KeyCode::Right, KeyModifiers::NONE) => handle_navigation_right(screen, current_state),

        // Page navigation keys
        (KeyCode::PageUp, KeyModifiers::NONE) => handle_page_up(screen, current_state),
        (KeyCode::PageDown, KeyModifiers::NONE) => handle_page_down(screen, current_state),
        (KeyCode::Home, KeyModifiers::NONE) => handle_home(screen, current_state),
        (KeyCode::End, KeyModifiers::NONE) => handle_end(screen, current_state),

        // Selection keys - context dependent
        (KeyCode::Enter, KeyModifiers::NONE) => handle_select_contextual(screen, current_state),
        (KeyCode::Char(' '), KeyModifiers::NONE) => handle_space_contextual(screen, current_state),

        // Tab navigation
        (KeyCode::Tab, KeyModifiers::NONE) => handle_tab(screen, current_state),
        (KeyCode::BackTab, KeyModifiers::SHIFT) => handle_shift_tab(screen, current_state),

        // Number shortcuts for main menu
        (KeyCode::Char('1'), KeyModifiers::NONE) if screen == crate::ui::state::Screen::MainMenu => {
            Command::user(CommandType::NavigateTo(crate::ui::state::Screen::DeckSelection))
        }
        (KeyCode::Char('2'), KeyModifiers::NONE) if screen == crate::ui::state::Screen::MainMenu => {
            Command::user(CommandType::NavigateTo(crate::ui::state::Screen::DeckManagement))
        }
        (KeyCode::Char('3'), KeyModifiers::NONE) if screen == crate::ui::state::Screen::MainMenu => {
            Command::user(CommandType::NavigateTo(crate::ui::state::Screen::Statistics))
        }
        (KeyCode::Char('4'), KeyModifiers::NONE) if screen == crate::ui::state::Screen::MainMenu => {
            Command::user(CommandType::NavigateTo(crate::ui::state::Screen::Settings))
        }
        (KeyCode::Char('5'), KeyModifiers::NONE) if screen == crate::ui::state::Screen::MainMenu => {
            Command::user(CommandType::Quit)
        }
        (KeyCode::Char('/'), KeyModifiers::NONE) if screen == crate::ui::state::Screen::MainMenu => {
            Command::user(CommandType::StartSearch)
        }

        // Study session keys - only active in study mode
        (KeyCode::Char('1'), KeyModifiers::NONE)
            if screen == crate::ui::state::Screen::StudySession =>
        {
            Command::user(CommandType::RateCurrentCard(CardRating::Again))
        }
        (KeyCode::Char('2'), KeyModifiers::NONE)
            if screen == crate::ui::state::Screen::StudySession =>
        {
            Command::user(CommandType::RateCurrentCard(CardRating::Hard))
        }
        (KeyCode::Char('3'), KeyModifiers::NONE)
            if screen == crate::ui::state::Screen::StudySession =>
        {
            Command::user(CommandType::RateCurrentCard(CardRating::Good))
        }
        (KeyCode::Char('4'), KeyModifiers::NONE)
            if screen == crate::ui::state::Screen::StudySession =>
        {
            Command::user(CommandType::RateCurrentCard(CardRating::Easy))
        }

        // Escape key - context dependent
        (KeyCode::Esc, KeyModifiers::NONE) => handle_escape_contextual(screen, current_state),

        // Quit keys - global
        (KeyCode::Char('q'), KeyModifiers::CONTROL)
        | (KeyCode::Char('c'), KeyModifiers::CONTROL) => Command::user(CommandType::Quit),

        // Help key - global
        (KeyCode::F(1), KeyModifiers::NONE) | (KeyCode::Char('?'), KeyModifiers::NONE) => {
            Command::user(CommandType::ShowHelp)
        }

        // Refresh keys - context dependent
        (KeyCode::F(5), KeyModifiers::NONE) => handle_refresh_contextual(screen, current_state),

        // Search key - context dependent
        (KeyCode::Char('/'), KeyModifiers::NONE) => handle_search_contextual(screen, current_state),

        // Create key - context dependent
        (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
            handle_create_contextual(screen, current_state)
        }

        // Delete key - context dependent
        (KeyCode::Delete, KeyModifiers::NONE) | (KeyCode::Backspace, KeyModifiers::CONTROL) => {
            handle_delete_contextual(screen, current_state)
        }

        // Character input for search screen
        (KeyCode::Char(c), KeyModifiers::NONE) if screen == crate::ui::state::Screen::Search => {
            Command::user(CommandType::SearchDecks(c.to_string()))
        }

        // Backspace for search screen
        (KeyCode::Backspace, KeyModifiers::NONE) if screen == crate::ui::state::Screen::Search => {
            Command::user(CommandType::Unknown) // Will be handled by input handler
        }

        _ => Command::user(CommandType::Unknown),
    }
}

/// Context-aware mouse event handling
fn handle_mouse_event_contextual(event: CrosstermMouseEvent, current_state: &AppState) -> Command {
    let screen = current_state.current_screen();

    match event.kind {
        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            handle_left_click_contextual(event.column, event.row, screen, current_state)
        }
        MouseEventKind::Down(crossterm::event::MouseButton::Right) => {
            handle_right_click_contextual(event.column, event.row, screen, current_state)
        }
        MouseEventKind::ScrollUp => {
            handle_scroll_up_contextual(event.column, event.row, screen, current_state)
        }
        MouseEventKind::ScrollDown => {
            handle_scroll_down_contextual(event.column, event.row, screen, current_state)
        }
        MouseEventKind::Moved => Command::user(CommandType::MouseMove(event.column, event.row)),
        _ => Command::user(CommandType::Unknown),
    }
}

// Context-specific navigation handlers
fn handle_navigation_up(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::SelectPreviousDeck),
        crate::ui::state::Screen::MainMenu => Command::user(CommandType::NavigateUp),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigateUp),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::NavigateUp),
        crate::ui::state::Screen::Search => Command::user(CommandType::NavigateUp),
        crate::ui::state::Screen::Help => Command::user(CommandType::NavigateUp),
        _ => Command::user(CommandType::NavigateUp),
    }
}

fn handle_navigation_down(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::SelectNextDeck),
        crate::ui::state::Screen::MainMenu => Command::user(CommandType::NavigateDown),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigateDown),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::NavigateDown),
        crate::ui::state::Screen::Search => Command::user(CommandType::NavigateDown),
        crate::ui::state::Screen::Help => Command::user(CommandType::NavigateDown),
        _ => Command::user(CommandType::NavigateDown),
    }
}

fn handle_navigation_left(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::Settings => Command::user(CommandType::NavigateLeft),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigateLeft),
        crate::ui::state::Screen::MainMenu => Command::user(CommandType::NavigateLeft),
        _ => Command::user(CommandType::NavigateLeft),
    }
}

fn handle_navigation_right(
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::Settings => Command::user(CommandType::NavigateRight),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigateRight),
        crate::ui::state::Screen::MainMenu => Command::user(CommandType::NavigateRight),
        _ => Command::user(CommandType::NavigateRight),
    }
}

// Context-specific selection handlers
fn handle_select_contextual(screen: crate::ui::state::Screen, current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::MainMenu => Command::user(CommandType::Confirm),
        crate::ui::state::Screen::DeckSelection => {
            Command::user(CommandType::StartStudySessionDefault)
        }
        crate::ui::state::Screen::StudySession => {
            if current_state.is_showing_answer() {
                Command::user(CommandType::RateCurrentCard(CardRating::Good))
            } else {
                Command::user(CommandType::ShowAnswer)
            }
        }
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::SaveCard),
        crate::ui::state::Screen::Settings => Command::user(CommandType::ConfirmSetting),
        _ => Command::user(CommandType::Select),
    }
}

fn handle_space_contextual(screen: crate::ui::state::Screen, current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::StudySession => {
            if current_state.is_showing_answer() {
                Command::user(CommandType::RateCurrentCard(CardRating::Good))
            } else {
                Command::user(CommandType::ShowAnswer)
            }
        }
        crate::ui::state::Screen::DeckSelection => {
            Command::user(CommandType::SelectDeck(uuid::Uuid::nil()))
        }
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::ToggleCardSide),
        _ => Command::user(CommandType::Select),
    }
}

// Context-specific escape handlers
fn handle_escape_contextual(screen: crate::ui::state::Screen, current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::StudySession => {
            if current_state.is_study_session_active() {
                Command::user(CommandType::EndStudySession)
            } else {
                Command::user(CommandType::NavigateToMainMenu)
            }
        }
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::CancelEdit),
        crate::ui::state::Screen::Settings => Command::user(CommandType::NavigateToMainMenu),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::NavigateToMainMenu),
        crate::ui::state::Screen::Help => Command::user(CommandType::NavigateToMainMenu),
        crate::ui::state::Screen::Search => Command::user(CommandType::NavigateToMainMenu),
        _ => Command::user(CommandType::NavigateBack),
    }
}

// Context-specific refresh handlers
fn handle_refresh_contextual(
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::LoadDecks),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::RefreshStatistics),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::RefreshSession),
        _ => Command::user(CommandType::RefreshScreen),
    }
}

// Context-specific search handlers
fn handle_search_contextual(
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => {
            Command::user(CommandType::SearchDecks(String::new()))
        }
        crate::ui::state::Screen::StudySession => {
            Command::user(CommandType::SearchCards(String::new()))
        }
        _ => Command::user(CommandType::StartSearch),
    }
}

// Context-specific create handlers
fn handle_create_contextual(
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::CreateDeckPrompt),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::CreateCardPrompt),
        _ => Command::user(CommandType::CreateDeckPrompt),
    }
}

// Context-specific delete handlers
fn handle_delete_contextual(
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::DeleteDeckPrompt),
        crate::ui::state::Screen::CardEditor => {
            Command::user(CommandType::DeleteCard(uuid::Uuid::nil()))
        }
        _ => Command::user(CommandType::DeleteCard(uuid::Uuid::nil())),
    }
}

// Mouse event handlers
fn handle_left_click_contextual(
    x: u16,
    y: u16,
    screen: crate::ui::state::Screen,
    current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::StudySession => {
            if current_state.is_showing_answer() {
                // Check if click is on a rating button
                if y >= 10 && y <= 14 {
                    // Rating button area
                    let rating = match x {
                        10..=15 => CardRating::Again,
                        17..=22 => CardRating::Hard,
                        24..=29 => CardRating::Good,
                        31..=36 => CardRating::Easy,
                        _ => return Command::user(CommandType::ShowAnswer),
                    };
                    Command::user(CommandType::RateCurrentCard(rating))
                } else {
                    Command::user(CommandType::ShowAnswer)
                }
            } else {
                Command::user(CommandType::ShowAnswer)
            }
        }
        _ => Command::user(CommandType::Click(x, y)),
    }
}

fn handle_right_click_contextual(
    x: u16,
    y: u16,
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => {
            Command::user(CommandType::ShowDeckContextMenu(x, y))
        }
        crate::ui::state::Screen::StudySession => {
            Command::user(CommandType::ShowCardContextMenu(x, y))
        }
        _ => Command::user(CommandType::RightClick(x, y)),
    }
}

fn handle_scroll_up_contextual(
    _x: u16,
    _y: u16,
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::SelectPreviousDeck),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::ScrollUp),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::ScrollStatsUp),
        _ => Command::user(CommandType::ScrollUp),
    }
}

fn handle_scroll_down_contextual(
    _x: u16,
    _y: u16,
    screen: crate::ui::state::Screen,
    _current_state: &AppState,
) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::SelectNextDeck),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::ScrollDown),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::ScrollStatsDown),
        _ => Command::user(CommandType::ScrollDown),
    }
}

fn handle_paste_contextual(content: String, current_state: &AppState) -> Command {
    let screen = current_state.current_screen();

    match screen {
        crate::ui::state::Screen::CardEditor => {
            Command::user(CommandType::PasteCardContent(content))
        }
        crate::ui::state::Screen::DeckSelection if content.contains('\n') => {
            // Try to import cards from pasted content
            Command::user(CommandType::ImportCards(content))
        }
        _ => Command::system(CommandType::Paste(content)),
    }
}

// Page navigation handlers
fn handle_page_up(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::NavigatePageUp),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::ScrollStatsUp),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::NavigatePageUp),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigatePageUp),
        _ => Command::user(CommandType::NavigatePageUp),
    }
}

fn handle_page_down(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::NavigatePageDown),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::ScrollStatsDown),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::NavigatePageDown),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigatePageDown),
        _ => Command::user(CommandType::NavigatePageDown),
    }
}

fn handle_home(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::NavigateHome),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::NavigateHome),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::NavigateHome),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigateHome),
        _ => Command::user(CommandType::NavigateHome),
    }
}

fn handle_end(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::DeckSelection => Command::user(CommandType::NavigateEnd),
        crate::ui::state::Screen::Statistics => Command::user(CommandType::NavigateEnd),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::NavigateEnd),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::NavigateEnd),
        _ => Command::user(CommandType::NavigateEnd),
    }
}

// Tab navigation handlers
fn handle_tab(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::Settings => Command::user(CommandType::NavigateRight),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::ToggleCardSide),
        crate::ui::state::Screen::StudySession => Command::user(CommandType::SkipCurrentCard),
        crate::ui::state::Screen::Search => Command::user(CommandType::ToggleCardSide), // Toggle search type
        _ => Command::user(CommandType::NavigateDown),
    }
}

fn handle_shift_tab(screen: crate::ui::state::Screen, _current_state: &AppState) -> Command {
    match screen {
        crate::ui::state::Screen::Settings => Command::user(CommandType::NavigateLeft),
        crate::ui::state::Screen::CardEditor => Command::user(CommandType::ToggleCardSide),
        _ => Command::user(CommandType::NavigateUp),
    }
}

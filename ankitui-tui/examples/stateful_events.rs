//! Stateful Event Handling Example
//!
//! Demonstrates how the same events can have different meanings based on application state

use ankitui_core::data::models::Rating;
use ankitui_tui_v2::{
    ui::event::stateful_handler::StatefulEventHandler,
    ui::event::{Command, CommandType, Event},
    ui::state::{AppState, Screen, StateStore},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent as CrosstermMouseEvent, MouseEventKind};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Stateful Event Handling Example");
    println!("================================\n");

    // Create state store
    let state_store = StateStore::new();

    // Example 1: Same Event, Different Screens = Different Commands
    println!("📍 Example 1: Navigation Context");
    println!("--------------------------------");

    let enter_key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);

    // Context: Main Menu
    let mut main_menu_state = AppState::new();
    main_menu_state.current_screen = Screen::MainMenu;
    let handler = StatefulEventHandler::new(main_menu_state);
    let command = handler.handle_event(Event::Key(enter_key.clone()))?;
    println!("Enter key in Main Menu -> {:?}", command.command_type);

    // Context: Deck Selection
    let mut deck_selection_state = AppState::new();
    deck_selection_state.current_screen = Screen::DeckSelection;
    let handler = StatefulEventHandler::new(deck_selection_state);
    let command = handler.handle_event(Event::Key(enter_key.clone()))?;
    println!("Enter key in Deck Selection -> {:?}", command.command_type);

    // Context: Study Session
    let mut study_state = AppState::new();
    study_state.current_screen = Screen::StudySession;
    study_state.set_ui_state("showing_answer".to_string(), "false".to_string());
    let handler = StatefulEventHandler::new(study_state);
    let command = handler.handle_event(Event::Key(enter_key.clone()))?;
    println!("Enter key in Study Session (question) -> {:?}", command.command_type);

    // Context: Study Session showing answer
    let mut study_answer_state = AppState::new();
    study_answer_state.current_screen = Screen::StudySession;
    study_answer_state.set_ui_state("showing_answer".to_string(), "true".to_string());
    let handler = StatefulEventHandler::new(study_answer_state);
    let command = handler.handle_event(Event::Key(enter_key.clone()))?;
    println!("Enter key in Study Session (answer) -> {:?}", command.command_type);

    // Example 2: Number Keys in Study Session
    println!("\n🔢 Example 2: Rating Context");
    println!("-----------------------------");

    let number_keys = vec![
        ('1', Rating::Again),
        ('2', Rating::Hard),
        ('3', Rating::Good),
        ('4', Rating::Easy),
    ];

    for (key_char, expected_rating) in number_keys {
        let key_event = KeyEvent::new(KeyCode::Char(key_char), KeyModifiers::NONE);

        // In Study Session
        let mut study_state = AppState::new();
        study_state.current_screen = Screen::StudySession;
        let handler = StatefulEventHandler::new(study_state);
        let command = handler.handle_event(Event::Key(key_event))?;
        println!("Key '{}' in Study Session -> {:?}", key_char, command.command_type);

        // In Main Menu (should be ignored)
        let mut menu_state = AppState::new();
        menu_state.current_screen = Screen::MainMenu;
        let handler = StatefulEventHandler::new(menu_state);
        let command = handler.handle_event(Event::Key(KeyEvent::new(KeyCode::Char(key_char), KeyModifiers::NONE)))?;
        println!("Key '{}' in Main Menu -> {:?}", key_char, command.command_type);
    }

    // Example 3: Escape Key Context
    println!("\n🚪 Example 3: Escape Context");
    println!("----------------------------");

    let escape_key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

    // Context: Study Session active
    let mut active_study_state = AppState::new();
    active_study_state.current_screen = Screen::StudySession;
    active_study_state.set_ui_state("card_study_mode".to_string(), "active".to_string());
    let handler = StatefulEventHandler::new(active_study_state);
    let command = handler.handle_event(Event::Key(escape_key.clone()))?;
    println!("Escape in active Study Session -> {:?}", command.command_type);

    // Context: Study Session inactive
    let mut inactive_study_state = AppState::new();
    inactive_study_state.current_screen = Screen::StudySession;
    inactive_study_state.set_ui_state("card_study_mode".to_string(), "inactive".to_string());
    let handler = StatefulEventHandler::new(inactive_study_state);
    let command = handler.handle_event(Event::Key(escape_key.clone()))?;
    println!("Escape in inactive Study Session -> {:?}", command.command_type);

    // Context: Card Editor
    let mut editor_state = AppState::new();
    editor_state.current_screen = Screen::CardEditor;
    let handler = StatefulEventHandler::new(editor_state);
    let command = handler.handle_event(Event::Key(escape_key.clone()))?;
    println!("Escape in Card Editor -> {:?}", command.command_type);

    // Example 4: Space Bar Context
    println!("\n⎵ Example 4: Space Bar Context");
    println!("------------------------------");

    let space_key = KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE);

    // Context: Study Session (question)
    let mut question_state = AppState::new();
    question_state.current_screen = Screen::StudySession;
    question_state.set_ui_state("showing_answer".to_string(), "false".to_string());
    let handler = StatefulEventHandler::new(question_state);
    let command = handler.handle_event(Event::Key(space_key.clone()))?;
    println!("Space in Study Session (question) -> {:?}", command.command_type);

    // Context: Study Session (answer)
    let mut answer_state = AppState::new();
    answer_state.current_screen = Screen::StudySession;
    answer_state.set_ui_state("showing_answer".to_string(), "true".to_string());
    let handler = StatefulEventHandler::new(answer_state);
    let command = handler.handle_event(Event::Key(space_key.clone()))?;
    println!("Space in Study Session (answer) -> {:?}", command.command_type);

    // Context: Deck Selection
    let mut deck_state = AppState::new();
    deck_state.current_screen = Screen::DeckSelection;
    let handler = StatefulEventHandler::new(deck_state);
    let command = handler.handle_event(Event::Key(space_key.clone()))?;
    println!("Space in Deck Selection -> {:?}", command.command_type);

    // Example 5: Mouse Click Context
    println!("\n🖱️  Example 5: Mouse Click Context");
    println!("---------------------------------");

    let left_click = CrosstermMouseEvent {
        kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
        column: 20,
        row: 12,
        modifiers: KeyModifiers::NONE,
    };

    // Context: Study Session (different areas mean different actions)
    let mut study_click_state = AppState::new();
    study_click_state.current_screen = Screen::StudySession;
    study_click_state.set_ui_state("showing_answer".to_string(), "true".to_string());
    let handler = StatefulEventHandler::new(study_click_state);

    // Click on rating area (y: 10-14)
    let rating_click = CrosstermMouseEvent {
        kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
        column: 25, // Good rating area
        row: 12,
        modifiers: KeyModifiers::NONE,
    };
    let command = handler.handle_event(Event::Mouse(rating_click))?;
    println!("Left click on rating area -> {:?}", command.command_type);

    // Click outside rating area
    let other_click = CrosstermMouseEvent {
        kind: MouseEventKind::Down(crossterm::event::MouseButton::Left),
        column: 5,
        row: 5,
        modifiers: KeyModifiers::NONE,
    };
    let command = handler.handle_event(Event::Mouse(other_click))?;
    println!("Left click outside rating area -> {:?}", command.command_type);

    // Example 6: Search Context
    println!("\n🔍 Example 6: Search Context");
    println!("---------------------------");

    let search_key = KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE);

    // Context: Deck Selection
    let mut deck_search_state = AppState::new();
    deck_search_state.current_screen = Screen::DeckSelection;
    let handler = StatefulEventHandler::new(deck_search_state);
    let command = handler.handle_event(Event::Key(search_key.clone()))?;
    println!("'/' in Deck Selection -> {:?}", command.command_type);

    // Context: Study Session
    let mut study_search_state = AppState::new();
    study_search_state.current_screen = Screen::StudySession;
    let handler = StatefulEventHandler::new(study_search_state);
    let command = handler.handle_event(Event::Key(search_key.clone()))?;
    println!("'/' in Study Session -> {:?}", command.command_type);

    // Context: Settings
    let mut settings_state = AppState::new();
    settings_state.current_screen = Screen::Settings;
    let handler = StatefulEventHandler::new(settings_state);
    let command = handler.handle_event(Event::Key(search_key.clone()))?;
    println!("'/' in Settings -> {:?}", command.command_type);

    println!("\n✅ Stateful event handling examples completed!");
    println!("\nKey Benefits:");
    println!("• Same event can have different meanings based on context");
    println!("• More intuitive user experience");
    println!("• Reduced cognitive load for users");
    println!("• Context-aware UI behavior");

    Ok(())
}

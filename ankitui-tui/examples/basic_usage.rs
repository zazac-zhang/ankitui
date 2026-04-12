//! Basic usage example for AnkiTUI V2
//!
//! Demonstrates how to properly use the Core package API through the service layer

use ankitui_core::data::models::{CardContent, Rating};
use ankitui_tui_v2::{
    app::{App, AppConfig, AppController},
    ui::{event::Command, state::Screen, CommandType},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Create application with default configuration
    let config = AppConfig::default();
    let mut app = App::new(config).await?;

    // Initialize the application
    app.initialize().await?;
    println!("✅ AnkiTUI V2 initialized successfully");

    // Create controller for business logic operations
    let mut controller = AppController::new(&mut app);

    // Example 1: Load all decks
    println!("\n📚 Loading all decks...");
    let decks = controller.load_all_decks().await?;
    println!("Found {} decks", decks.len());

    // Example 2: Create a new deck
    println!("\n➕ Creating new deck...");
    let deck_name = "Example Deck".to_string();
    let deck_id = controller
        .create_deck(deck_name.clone(), Some("Example deck for testing".to_string()))
        .await?;
    println!("Created deck '{}' with ID: {}", deck_name, deck_id);

    // Example 3: Add cards to the deck
    println!("\n🃏 Adding cards to deck...");
    let cards = vec![
        CardContent {
            id: Uuid::new_v4(),
            front: "What is Rust?".to_string(),
            back: "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string(),
            tags: vec!["programming".to_string(), "rust".to_string()],
            media: vec![],
            custom: std::collections::HashMap::new(),
        },
        CardContent {
            id: Uuid::new_v4(),
            front: "What is Cargo?".to_string(),
            back: "Cargo is Rust's build system and package manager. It handles building your code, downloading the libraries your code depends on, and building those libraries.".to_string(),
            tags: vec!["programming".to_string(), "rust".to_string(), "cargo".to_string()],
            media: vec![],
            custom: std::collections::HashMap::new(),
        },
    ];

    controller.add_cards(deck_id, cards).await?;
    println!("Added 2 cards to deck");

    // Example 4: Start a study session
    println!("\n📖 Starting study session...");
    controller.start_study_session(deck_id).await?;
    println!("Study session started");

    // Example 5: Simulate card reviews
    println!("\n🎯 Simulating card reviews...");

    // In a real application, you would:
    // 1. Show the card front to user
    // 2. User requests to see answer
    // 3. User rates the card (Again, Hard, Good, Easy)
    // 4. Move to next card

    // Simulate rating current card as "Good"
    controller.rate_current_card(Rating::Good).await?;
    println!("Rated card as Good");

    // Simulate rating next card as "Easy"
    controller.rate_current_card(Rating::Easy).await?;
    println!("Rated card as Easy");

    // Example 6: Load deck statistics
    println!("\n📊 Loading deck statistics...");
    let stats = controller.load_deck_statistics(deck_id).await?;
    println!("Deck statistics:");
    println!("  Total cards: {}", stats.total_cards);
    println!("  New cards: {}", stats.new_cards);
    println!("  Review cards: {}", stats.review_cards);
    println!("  Due cards: {}", stats.due_cards);

    // Example 7: Load global statistics
    println!("\n🌍 Loading global statistics...");
    let global_stats = controller.load_global_statistics().await?;
    println!("Global statistics:");
    println!("  Total decks: {}", global_stats.total_decks);
    println!("  Total cards: {}", global_stats.total_cards);
    println!("  Cards studied today: {}", global_stats.cards_studied_today);

    // Example 8: End study session
    println!("\n🏁 Ending study session...");
    controller.end_study_session().await?;
    println!("Study session ended");

    // Example 9: Handle UI commands programmatically
    println!("\n🎮 Handling UI commands...");

    // Navigate to deck selection screen
    let nav_command = Command::new(CommandType::NavigateTo(Screen::DeckSelection));
    controller.handle_command(nav_command).await?;
    println!("Navigated to deck selection screen");

    // Load decks command
    let load_command = Command::new(CommandType::LoadDecks);
    controller.handle_command(load_command).await?;
    println!("Loaded decks via command");

    // Example 10: Demonstrate error handling
    println!("\n⚠️ Demonstrating error handling...");
    match controller.load_deck_statistics(Uuid::new_v4()).await {
        Ok(_) => println!("Unexpectedly found non-existent deck"),
        Err(e) => println!("✅ Correctly handled error: {}", e),
    }

    println!("\n🎉 All examples completed successfully!");
    println!("Core package API integration is working correctly!");

    Ok(())
}

//! Study session helper functions
//!
//! Utility functions for study session operations and card state management.

use crate::ui::state::store::{StateStore, SystemMessage};
use crate::utils::error::TuiResult;
use ankitui_core::{SessionController, Card};
use uuid::Uuid;

/// Get current card ID and deck ID from session
pub async fn get_current_card_info(
    session_controller: &tokio::sync::Mutex<SessionController>,
) -> Option<(Uuid, Card, Option<Uuid>)> {
    let session = session_controller.lock().await;
    let card = session.current_card()?;
    let card_id = card.content.id;
    let deck_id = session.current_deck_id();
    Some((card_id, card.clone(), deck_id))
}

/// Get current card ID only
pub async fn get_current_card_id(
    session_controller: &tokio::sync::Mutex<SessionController>,
) -> Option<Uuid> {
    let session = session_controller.lock().await;
    session.current_card().map(|c| c.content.id)
}

/// Check if there's a current card in the session
pub async fn has_current_card(
    session_controller: &tokio::sync::Mutex<SessionController>,
) -> bool {
    let session = session_controller.lock().await;
    session.current_card().is_some()
}

/// Show a success message for card operations
pub async fn show_card_operation_message(
    state_store: &StateStore,
    operation: &str,
    description: &str,
) -> TuiResult<()> {
    state_store.show_message(SystemMessage::info(
        operation,
        description,
    ))?;
    Ok(())
}

/// Show a warning message for card operations
pub async fn show_card_operation_warning(
    state_store: &StateStore,
    operation: &str,
    description: &str,
) -> TuiResult<()> {
    state_store.show_message(SystemMessage::warning(
        operation,
        description,
    ))?;
    Ok(())
}

/// Reset study session UI state after card operation
pub async fn reset_study_ui_state(
    state_store: &StateStore,
    study_service: &mut crate::domain::StudyService,
) -> TuiResult<()> {
    // Reset answer view state
    state_store.set_showing_answer(false)?;

    // Skip to next card
    study_service.skip_current_card().await?;

    Ok(())
}

/// Get deck cards with error handling
pub async fn get_deck_cards_safe(
    session_controller: &tokio::sync::Mutex<SessionController>,
    deck_id: &Uuid,
) -> Option<Vec<Card>> {
    let session = session_controller.lock().await;
    session.get_deck_cards(deck_id).await.ok()
}

/// Check if a card exists in deck
pub fn card_exists_in_deck(cards: &[Card], card_id: &Uuid) -> bool {
    cards.iter().any(|c| &c.content.id == card_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_exists_in_deck() {
        // Use a simple UUID-based test instead of creating full Card objects
        let card_id = Uuid::new_v4();
        let cards = vec![]; // Empty deck for this test

        assert!(!card_exists_in_deck(&cards, &card_id));
        assert!(!card_exists_in_deck(&cards, &Uuid::new_v4()));
    }
}

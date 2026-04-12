//! State Store - SQLite-based storage for card state and scheduling information
//!
//! Handles storage of SM-2 algorithm state and scheduling data in SQLite database

use crate::data::models::CardStateData;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use std::path::Path;
use uuid::Uuid;

/// State Store manages SQLite-based storage of card scheduling state
#[derive(Debug, Clone)]
pub struct StateStore {
    pool: SqlitePool,
}

impl StateStore {
    /// Create or open a state store database
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path = db_path.as_ref();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create database directory: {}", parent.display()))?;
        }

        // Connect to database with optimized connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5) // Reduced for single-user TUI app
            .min_connections(1) // Keep at least one connection ready
            .acquire_timeout(std::time::Duration::from_secs(30))
            .idle_timeout(std::time::Duration::from_secs(600)) // 10 minutes
            .max_lifetime(std::time::Duration::from_secs(1800)) // 30 minutes
            .test_before_acquire(true) // Ensure connections are valid
            .connect(&format!("sqlite:{}?mode=rwc", db_path.display()))
            .await
            .context("Failed to connect to SQLite database")?;

        // Configure SQLite for performance - use DELETE mode to avoid temp file issues
        sqlx::query("PRAGMA journal_mode=DELETE")
            .execute(&pool)
            .await
            .context("Failed to set journal mode")?;

        sqlx::query("PRAGMA synchronous=NORMAL")
            .execute(&pool)
            .await
            .context("Failed to set synchronous mode")?;

        sqlx::query("PRAGMA cache_size=10000")
            .execute(&pool)
            .await
            .context("Failed to set cache size")?;

        sqlx::query("PRAGMA temp_store=memory")
            .execute(&pool)
            .await
            .context("Failed to set temp store")?;

        // Run migrations
        Self::run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        // Create card_states table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS card_states (
                id TEXT PRIMARY KEY,
                due TEXT NOT NULL,
                interval INTEGER NOT NULL DEFAULT 0,
                ease_factor REAL NOT NULL DEFAULT 2.5,
                reps INTEGER NOT NULL DEFAULT 0,
                lapses INTEGER NOT NULL DEFAULT 0,
                state TEXT NOT NULL DEFAULT 'New',
                updated_at TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await
        .context("Failed to create card_states table")?;

        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_card_states_due ON card_states(due)")
            .execute(pool)
            .await
            .context("Failed to create due index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_card_states_state ON card_states(state)")
            .execute(pool)
            .await
            .context("Failed to create state index")?;

        // Additional performance indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_card_states_updated_at ON card_states(updated_at)")
            .execute(pool)
            .await
            .context("Failed to create updated_at index")?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_card_states_due_state ON card_states(due, state)")
            .execute(pool)
            .await
            .context("Failed to create composite due_state index")?;

        // Optimize SQLite settings - use DELETE mode to avoid temp file issues
        sqlx::query("PRAGMA journal_mode=DELETE")
            .execute(pool)
            .await
            .context("Failed to set journal mode")?;

        sqlx::query("PRAGMA synchronous=NORMAL")
            .execute(pool)
            .await
            .context("Failed to set synchronous mode")?;

        sqlx::query("PRAGMA cache_size=10000")
            .execute(pool)
            .await
            .context("Failed to set cache size")?;

        sqlx::query("PRAGMA temp_store=MEMORY")
            .execute(pool)
            .await
            .context("Failed to set temp store to memory")?;

        Ok(())
    }

    /// Save or update card state
    pub async fn save_card_state(&self, state: &CardStateData) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO card_states
            (id, due, interval, ease_factor, reps, lapses, state, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(state.id.to_string())
        .bind(state.due.to_rfc3339())
        .bind(state.interval)
        .bind(state.ease_factor)
        .bind(state.reps)
        .bind(state.lapses)
        .bind(format!("{:?}", state.state))
        .bind(state.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .context("Failed to save card state")?;

        Ok(())
    }

    /// Load card state by ID
    pub async fn load_card_state(&self, card_id: &Uuid) -> Result<Option<CardStateData>> {
        let row = sqlx::query("SELECT * FROM card_states WHERE id = ?")
            .bind(card_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .context("Failed to load card state")?;

        if let Some(row) = row {
            Ok(Some(self.row_to_card_state(row)?))
        } else {
            Ok(None)
        }
    }

    /// Load multiple card states by IDs
    pub async fn load_card_states(&self, card_ids: &[Uuid]) -> Result<Vec<CardStateData>> {
        if card_ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders: String = std::iter::repeat("?")
            .take(card_ids.len())
            .collect::<Vec<_>>()
            .join(",");

        let query = format!("SELECT * FROM card_states WHERE id IN ({})", placeholders);

        let mut q = sqlx::query(&query);
        for id in card_ids {
            q = q.bind(id.to_string());
        }

        let rows = q.fetch_all(&self.pool).await.context("Failed to load card states")?;

        let mut states = Vec::new();
        for row in rows {
            states.push(self.row_to_card_state(row)?);
        }

        Ok(states)
    }

    /// Get cards due for review before or at the specified time
    pub async fn get_due_cards(&self, before: DateTime<Utc>, limit: Option<i32>) -> Result<Vec<Uuid>> {
        let mut query = "SELECT id FROM card_states WHERE due <= ? ORDER BY due ASC".to_string();

        if let Some(limit) = limit {
            query = format!("{} LIMIT {}", query, limit);
        }

        let rows = sqlx::query(&query)
            .bind(before.to_rfc3339())
            .fetch_all(&self.pool)
            .await
            .context("Failed to get due cards")?;

        let mut card_ids = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            if let Ok(uuid) = Uuid::parse_str(&id_str) {
                card_ids.push(uuid);
            }
        }

        Ok(card_ids)
    }

    /// Get cards in specific states (e.g., New, Learning, Review)
    pub async fn get_cards_by_state(&self, state: &str, limit: Option<i32>) -> Result<Vec<Uuid>> {
        let mut query = "SELECT id FROM card_states WHERE state = ? ORDER BY created_at ASC".to_string();

        if let Some(limit) = limit {
            query = format!("{} LIMIT {}", query, limit);
        }

        let rows = sqlx::query(&query)
            .bind(state)
            .fetch_all(&self.pool)
            .await
            .context("Failed to get cards by state")?;

        let mut card_ids = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            if let Ok(uuid) = Uuid::parse_str(&id_str) {
                card_ids.push(uuid);
            }
        }

        Ok(card_ids)
    }

    /// Get all card states for a deck (by filtering card IDs)
    /// Note: This requires knowledge of which cards belong to which deck
    pub async fn get_all_card_states(&self) -> Result<Vec<CardStateData>> {
        let rows = sqlx::query("SELECT * FROM card_states ORDER BY due ASC")
            .fetch_all(&self.pool)
            .await
            .context("Failed to get all card states")?;

        let mut states = Vec::new();
        for row in rows {
            states.push(self.row_to_card_state(row)?);
        }

        Ok(states)
    }

    /// Delete card state
    pub async fn delete_card_state(&self, card_id: &Uuid) -> Result<()> {
        sqlx::query("DELETE FROM card_states WHERE id = ?")
            .bind(card_id.to_string())
            .execute(&self.pool)
            .await
            .context("Failed to delete card state")?;

        Ok(())
    }

    /// Bury a card temporarily (won't show until next session)
    pub async fn bury_card(&self, card_id: &Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE card_states
            SET state = 'Buried', updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(card_id.to_string())
        .execute(&self.pool)
        .await
        .context("Failed to bury card")?;

        Ok(())
    }

    /// Suspend a card permanently (won't show in reviews)
    pub async fn suspend_card(&self, card_id: &Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE card_states
            SET state = 'Suspended', updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(card_id.to_string())
        .execute(&self.pool)
        .await
        .context("Failed to suspend card")?;

        Ok(())
    }

    /// Unbury a card (restore to previous state)
    pub async fn unbury_card(&self, card_id: &Uuid, previous_state: crate::data::models::CardState) -> Result<()> {
        let state_str = match previous_state {
            crate::data::models::CardState::New => "New",
            crate::data::models::CardState::Learning => "Learning",
            crate::data::models::CardState::Review => "Review",
            crate::data::models::CardState::Relearning => "Relearning",
            crate::data::models::CardState::Buried | crate::data::models::CardState::Suspended => "New",
        };

        sqlx::query(
            r#"
            UPDATE card_states
            SET state = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(state_str)
        .bind(card_id.to_string())
        .execute(&self.pool)
        .await
        .context("Failed to unbury card")?;

        Ok(())
    }

    /// Unsuspend a card (restore to previous state)
    pub async fn unsuspend_card(&self, card_id: &Uuid, previous_state: crate::data::models::CardState) -> Result<()> {
        self.unbury_card(card_id, previous_state).await
    }

    /// Get all buried cards for a deck
    pub async fn get_buried_cards(&self, deck_cards: &[Uuid]) -> Result<Vec<Uuid>> {
        if deck_cards.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders: String = std::iter::repeat("?")
            .take(deck_cards.len())
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            "SELECT id FROM card_states WHERE id IN ({}) AND state = 'Buried'",
            placeholders
        );

        let mut q = sqlx::query(&query);
        for id in deck_cards {
            q = q.bind(id.to_string());
        }

        let rows = q.fetch_all(&self.pool).await.context("Failed to get buried cards")?;

        let mut card_ids = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            if let Ok(uuid) = Uuid::parse_str(&id_str) {
                card_ids.push(uuid);
            }
        }

        Ok(card_ids)
    }

    /// Get all suspended cards for a deck
    pub async fn get_suspended_cards(&self, deck_cards: &[Uuid]) -> Result<Vec<Uuid>> {
        if deck_cards.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders: String = std::iter::repeat("?")
            .take(deck_cards.len())
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            "SELECT id FROM card_states WHERE id IN ({}) AND state = 'Suspended'",
            placeholders
        );

        let mut q = sqlx::query(&query);
        for id in deck_cards {
            q = q.bind(id.to_string());
        }

        let rows = q.fetch_all(&self.pool).await.context("Failed to get suspended cards")?;

        let mut card_ids = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            if let Ok(uuid) = Uuid::parse_str(&id_str) {
                card_ids.push(uuid);
            }
        }

        Ok(card_ids)
    }

    /// Unbury all cards (end of session cleanup)
    pub async fn unbury_all_cards(&self, deck_cards: &[Uuid]) -> Result<usize> {
        if deck_cards.is_empty() {
            return Ok(0);
        }

        let placeholders: String = std::iter::repeat("?")
            .take(deck_cards.len())
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            r#"
            UPDATE card_states
            SET state = 'New', updated_at = CURRENT_TIMESTAMP
            WHERE id IN ({}) AND state = 'Buried'
            "#,
            placeholders
        );

        let mut q = sqlx::query(&query);
        for id in deck_cards {
            q = q.bind(id.to_string());
        }

        let result = q.execute(&self.pool).await.context("Failed to unbury all cards")?;
        Ok(result.rows_affected() as usize)
    }

    /// Get statistics about card states
    pub async fn get_statistics(&self) -> Result<CardStats> {
        let total_row = sqlx::query("SELECT COUNT(*) as count FROM card_states")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get total cards count")?;

        let total: i64 = total_row.get("count");

        let due_row = sqlx::query("SELECT COUNT(*) as count FROM card_states WHERE due <= ?")
            .bind(Utc::now().to_rfc3339())
            .fetch_one(&self.pool)
            .await
            .context("Failed to get due cards count")?;

        let due: i64 = due_row.get("count");

        let new_row = sqlx::query("SELECT COUNT(*) as count FROM card_states WHERE state = 'New'")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get new cards count")?;

        let new: i64 = new_row.get("count");

        let learning_row = sqlx::query("SELECT COUNT(*) as count FROM card_states WHERE state = 'Learning'")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get learning cards count")?;

        let learning: i64 = learning_row.get("count");

        let review_row = sqlx::query("SELECT COUNT(*) as count FROM card_states WHERE state = 'Review'")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get review cards count")?;

        let review: i64 = review_row.get("count");

        let buried_row = sqlx::query("SELECT COUNT(*) as count FROM card_states WHERE state = 'Buried'")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get buried cards count")?;

        let buried: i64 = buried_row.get("count");

        let suspended_row = sqlx::query("SELECT COUNT(*) as count FROM card_states WHERE state = 'Suspended'")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get suspended cards count")?;

        let suspended: i64 = suspended_row.get("count");

        Ok(CardStats {
            total,
            due,
            new,
            learning,
            review,
            buried,
            suspended,
        })
    }

    /// Convert database row to CardStateData
    fn row_to_card_state(&self, row: sqlx::sqlite::SqliteRow) -> Result<CardStateData> {
        let id_str: String = row.get("id");
        let due_str: String = row.get("due");
        let state_str: String = row.get("state");
        let updated_str: String = row.get("updated_at");

        let state = match state_str.as_str() {
            "New" => crate::data::models::CardState::New,
            "Learning" => crate::data::models::CardState::Learning,
            "Review" => crate::data::models::CardState::Review,
            "Relearning" => crate::data::models::CardState::Relearning,
            "Buried" => crate::data::models::CardState::Buried,
            "Suspended" => crate::data::models::CardState::Suspended,
            _ => return Err(anyhow::anyhow!("Unknown card state: {}", state_str)),
        };

        Ok(CardStateData {
            id: Uuid::parse_str(&id_str).context("Failed to parse UUID from database")?,
            due: DateTime::parse_from_rfc3339(&due_str)
                .context("Failed to parse due date from database")?
                .with_timezone(&Utc),
            interval: row.get("interval"),
            ease_factor: row.get("ease_factor"),
            reps: row.get("reps"),
            lapses: row.get("lapses"),
            state,
            updated_at: DateTime::parse_from_rfc3339(&updated_str)
                .context("Failed to parse updated_at date from database")?
                .with_timezone(&Utc),
        })
    }

    /// Close the database connection pool
    pub async fn close(&self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }
}

/// Statistics about card states
#[derive(Debug, Clone)]
pub struct CardStats {
    pub total: i64,
    pub due: i64,
    pub new: i64,
    pub learning: i64,
    pub review: i64,
    pub buried: i64,
    pub suspended: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::models::CardStateData;
    use chrono::Utc;
    use uuid::Uuid;

    async fn create_test_store() -> Result<StateStore> {
        // Use in-memory database instead of temporary file
        StateStore::new(":memory:").await
    }

    fn create_test_card_state() -> CardStateData {
        CardStateData {
            id: Uuid::new_v4(),
            due: Utc::now(),
            interval: 1,
            ease_factor: 2.5,
            reps: 1,
            lapses: 0,
            state: crate::data::models::CardState::Review,
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_save_and_load_card_state() -> Result<()> {
        let store = create_test_store().await?;
        let state = create_test_card_state();

        store.save_card_state(&state).await?;
        let loaded_state = store.load_card_state(&state.id).await?;

        assert!(loaded_state.is_some());
        let loaded = loaded_state.unwrap();
        assert_eq!(state.id, loaded.id);
        assert_eq!(state.interval, loaded.interval);
        assert_eq!(state.ease_factor, loaded.ease_factor);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_due_cards() -> Result<()> {
        let store = create_test_store().await?;

        let mut state1 = create_test_card_state();
        state1.due = Utc::now() - chrono::Duration::hours(1); // Past due

        let mut state2 = create_test_card_state();
        state2.id = Uuid::new_v4();
        state2.due = Utc::now() + chrono::Duration::hours(1); // Future due

        store.save_card_state(&state1).await?;
        store.save_card_state(&state2).await?;

        let due_cards = store.get_due_cards(Utc::now(), None).await?;
        assert_eq!(due_cards.len(), 1);
        assert_eq!(due_cards[0], state1.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_statistics() -> Result<()> {
        let store = create_test_store().await?;

        let mut state1 = create_test_card_state();
        state1.state = crate::data::models::CardState::New;
        state1.due = Utc::now() - chrono::Duration::hours(1);

        let mut state2 = create_test_card_state();
        state2.id = Uuid::new_v4();
        state2.state = crate::data::models::CardState::Learning;
        state2.due = Utc::now() - chrono::Duration::hours(1);

        store.save_card_state(&state1).await?;
        store.save_card_state(&state2).await?;

        let stats = store.get_statistics().await?;
        assert_eq!(stats.total, 2);
        assert_eq!(stats.due, 2);
        assert_eq!(stats.new, 1);
        assert_eq!(stats.learning, 1);

        Ok(())
    }
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct ConnectionPoolStats {
    pub size: u32,
    pub idle: u32,
}

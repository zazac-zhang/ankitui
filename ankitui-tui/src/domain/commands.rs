//! Domain Commands - CQRS implementation for AnkiTUI
//!
//! Command definitions following Command Query Responsibility Segregation pattern

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Base command trait
pub trait Command: Send + Sync {
    type Result: Send + Sync;

    fn id(&self) -> Uuid;
    fn timestamp(&self) -> DateTime<Utc>;
}

// Deck Commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeckCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
}

impl Command for CreateDeckCommand {
    type Result = Uuid;

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeckCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub deck_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Command for UpdateDeckCommand {
    type Result = ();

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDeckCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub deck_id: Uuid,
}

impl Command for DeleteDeckCommand {
    type Result = ();

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

// Card Commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub deck_id: Uuid,
    pub front: String,
    pub back: String,
    pub tags: Vec<String>,
}

impl Command for CreateCardCommand {
    type Result = Uuid;

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub card_id: Uuid,
    pub front: Option<String>,
    pub back: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Command for UpdateCardCommand {
    type Result = ();

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCardCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub card_id: Uuid,
}

impl Command for DeleteCardCommand {
    type Result = ();

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

// Study Session Commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartStudySessionCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub deck_id: Uuid,
    pub max_new_cards: Option<i32>,
    pub max_review_cards: Option<i32>,
}

impl Command for StartStudySessionCommand {
    type Result = Uuid;

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateCardCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub card_id: Uuid,
    pub rating: i32, // 0=Again, 1=Hard, 2=Good, 3=Easy
}

impl Command for RateCardCommand {
    type Result = ();

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndStudySessionCommand {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub session_id: Uuid,
}

impl Command for EndStudySessionCommand {
    type Result = crate::domain::models::SessionStats;

    fn id(&self) -> Uuid {
        self.id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
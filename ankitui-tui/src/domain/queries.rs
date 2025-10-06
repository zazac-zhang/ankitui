//! Domain Queries - CQRS implementation for AnkiTUI
//!
//! Query definitions following Command Query Responsibility Segregation pattern

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Base query trait
pub trait Query: Send + Sync {
    type Result: Send + Sync;

    fn id(&self) -> Uuid;
}

// Deck Queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllDecksQuery {
    pub id: Uuid,
}

impl Query for GetAllDecksQuery {
    type Result = Vec<crate::domain::models::Deck>;

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeckQuery {
    pub id: Uuid,
    pub deck_id: Uuid,
}

impl Query for GetDeckQuery {
    type Result = Option<crate::domain::models::Deck>;

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindDeckByNameQuery {
    pub id: Uuid,
    pub name: String,
}

impl Query for FindDeckByNameQuery {
    type Result = Option<crate::domain::models::Deck>;

    fn id(&self) -> Uuid {
        self.id
    }
}

// Card Queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCardsInDeckQuery {
    pub id: Uuid,
    pub deck_id: Uuid,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Query for GetCardsInDeckQuery {
    type Result = Vec<crate::domain::models::Card>;

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCardQuery {
    pub id: Uuid,
    pub card_id: Uuid,
}

impl Query for GetCardQuery {
    type Result = Option<crate::domain::models::Card>;

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCardsQuery {
    pub id: Uuid,
    pub deck_id: Option<Uuid>,
    pub query: String,
    pub limit: Option<i32>,
}

impl Query for SearchCardsQuery {
    type Result = Vec<crate::domain::models::Card>;

    fn id(&self) -> Uuid {
        self.id
    }
}

// Study Queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDueCardsQuery {
    pub id: Uuid,
    pub deck_id: Option<Uuid>,
    pub limit: Option<i32>,
}

impl Query for GetDueCardsQuery {
    type Result = Vec<crate::domain::models::Card>;

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNewCardsQuery {
    pub id: Uuid,
    pub deck_id: Option<Uuid>,
    pub limit: Option<i32>,
}

impl Query for GetNewCardsQuery {
    type Result = Vec<crate::domain::models::Card>;

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNextCardQuery {
    pub id: Uuid,
    pub deck_id: Uuid,
}

impl Query for GetNextCardQuery {
    type Result = Option<crate::domain::models::Card>;

    fn id(&self) -> Uuid {
        self.id
    }
}

// Statistics Queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeckStatisticsQuery {
    pub id: Uuid,
    pub deck_id: Uuid,
}

impl Query for GetDeckStatisticsQuery {
    type Result = crate::domain::models::DeckStatistics;

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGlobalStatisticsQuery {
    pub id: Uuid,
}

impl Query for GetGlobalStatisticsQuery {
    type Result = crate::domain::models::GlobalStatistics;

    fn id(&self) -> Uuid {
        self.id
    }
}

// User Preferences Queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPreferencesQuery {
    pub id: Uuid,
}

impl Query for GetUserPreferencesQuery {
    type Result = HashMap<String, String>;

    fn id(&self) -> Uuid {
        self.id
    }
}
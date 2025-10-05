//! Core data models for cards and decks
//!
//! Separates explicit fields (user-defined, stored in TOML) from
//! implicit fields (system-maintained, stored in SQLite)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Card state for SM-2 algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardState {
    /// New card not yet seen in review
    New,
    /// Currently in learning phase (short intervals)
    Learning,
    /// Regular review card
    Review,
    /// Card being relearned after lapse
    Relearning,
    /// Card is temporarily buried (won't show until next session)
    Buried,
    /// Card is permanently suspended (won't show in reviews)
    Suspended,
}

/// Explicit card fields (user-defined content, stored in TOML)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardContent {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub tags: Vec<String>,
    pub media: Option<MediaRef>,
    pub custom: HashMap<String, toml::Value>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

/// Implicit card fields (system-maintained state, stored in SQLite)
#[derive(Debug, Clone)]
pub struct CardStateData {
    pub id: Uuid,
    pub due: DateTime<Utc>,
    pub interval: i32,
    pub ease_factor: f32,
    pub reps: i32,
    pub lapses: i32,
    pub state: CardState,
    pub updated_at: DateTime<Utc>,
}

/// Complete card with both content and state
#[derive(Debug, Clone)]
pub struct Card {
    pub content: CardContent,
    pub state: CardStateData,
}

/// Media reference for audio/images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaRef {
    pub path: String,
    pub media_type: MediaType,
}

/// Reason for burying a card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuryReason {
    /// User manually buried the card
    UserBury,
    /// Card buried after review (sibling bury)
    ReviewBury,
    /// Card buried due to related cards
    SiblingBury,
    /// Card buried until next session
    SessionBury,
}

/// Record for buried cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuriedCardRecord {
    pub card_id: Uuid,
    pub bury_reason: BuryReason,
    pub bury_until: DateTime<Utc>,
    pub original_state: CardState,
    pub created_at: DateTime<Utc>,
}

/// Record for suspended cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendedCardRecord {
    pub card_id: Uuid,
    pub suspend_reason: String,
    pub created_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaType {
    Audio,
    Image,
    Video,
}

/// Enhanced media reference with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMediaRef {
    pub id: Uuid,
    pub path: String,
    pub media_type: MediaType,
    pub metadata: MediaMetadata,
    pub status: MediaStatus,
    pub local_cache_path: Option<String>,
    pub remote_url: Option<String>,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Media file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub file_size: u64,
    pub mime_type: Option<String>,
    pub duration_seconds: Option<f64>, // For audio/video
    pub dimensions: Option<(u32, u32)>, // For images (width, height)
    pub checksum: Option<String>,
    pub filename: Option<String>,
    pub duration: Option<f64>,
    pub created_at: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
}

/// Media file status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaStatus {
    Available,
    Missing,
    Corrupted,
    Processing,
}

/// Card template for different card types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTemplate {
    pub name: String,
    pub card_type: CardType,
    pub front_template: String,
    pub back_template: String,
    pub fields: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

/// Rendered card with processed content
#[derive(Debug, Clone)]
pub struct RenderedCard {
    pub content: String,
    pub media_refs: Vec<MediaRef>,
    pub has_input: bool,
    pub expected_answer: Option<String>,
    pub multiple_choice_options: Option<Vec<String>>,
}

/// Card rendering context
#[derive(Debug, Clone)]
pub struct CardRenderingContext {
    pub card_type: CardType,
    pub side: CardSide,
    pub fields: HashMap<String, String>,
    pub media_refs: Vec<MediaRef>,
    pub extra: HashMap<String, String>,
}

/// Cloze deletion data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClozeData {
    pub text: String,
    pub clozes: Vec<ClozeItem>,
    pub cloze_number: usize,
    pub hints: Option<Vec<String>>,
}

/// Individual cloze item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClozeItem {
    pub answer: String,
    pub hint: Option<String>,
}

/// Input field data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputData {
    pub question: String,
    pub answer: String,
    pub input_type: InputType,
    pub hint: Option<String>,
    pub case_sensitive: bool,
    pub strict: bool,
}

/// Input field types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Text,
    Number,
    Date,
    Formula,
}

/// Multiple choice data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoiceData {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub explanation: Option<String>,
}

/// Image occlusion data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageOcclusionData {
    pub image_path: String,
    pub occlusions: Vec<Occlusion>,
    pub question: Option<String>,
    pub answer: Option<String>,
}

/// Occlusion area for image occlusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occlusion {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub shape: OcclusionShape,
}

/// Occlusion shape types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OcclusionShape {
    Rectangle,
    Ellipse,
    Polygon(Vec<(f32, f32)>),
}

impl Default for MediaMetadata {
    fn default() -> Self {
        Self {
            file_size: 0,
            mime_type: None,
            duration_seconds: None,
            dimensions: None,
            checksum: None,
            filename: None,
            duration: None,
            created_at: None,
            tags: None,
        }
    }
}

/// Tag for card organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: Option<TagColor>,
    pub description: Option<String>,
    pub priority: TagPriority,
    pub parent_tag: Option<Uuid>,
    pub usage_count: Option<usize>,
    pub shortcuts: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tag color for visual identification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TagColor {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Orange,
    Pink,
    Cyan,
    Magenta,
    Gray,
    White,
    Black,
    Custom(String),
}


/// Tag priority for sorting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TagPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Tag filter for searching
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TagFilter {
    Exact(String),
    StartsWith(String),
    Contains(String),
    ByColor(TagColor),
    ByPriority(TagPriority),
    ByParent(Uuid),
    HasParent,
    NoParent,
}

/// Tag statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagStats {
    pub tag_id: Uuid,
    pub name: String,
    pub card_count: usize,
    pub review_cards: usize,
    pub new_cards: usize,
    pub last_used: Option<DateTime<Utc>>,
    pub average_retention: f32,
    pub retention_rate: f32,
}

/// Card side (front/back)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardSide {
    Front,
    Back,
}

/// Card type for different learning modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardType {
    Basic,
    BasicReversed,
    Cloze,
    Input,
    MultipleChoice,
    ImageOcclusion,
}

/// Extended card content with advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedCardContent {
    pub id: Uuid,
    pub card_type: CardType,
    pub tags: Vec<String>,
    pub media: Vec<MediaRef>,
    pub custom: HashMap<String, toml::Value>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl ExtendedCardContent {
    pub fn basic(front: String, back: String) -> Self {
        let now = Utc::now();
        let mut custom = HashMap::new();
        custom.insert("front".to_string(), toml::Value::String(front.clone()));
        custom.insert("back".to_string(), toml::Value::String(back.clone()));

        Self {
            id: Uuid::new_v4(),
            card_type: CardType::Basic,
            tags: vec![],
            media: vec![],
            custom,
            created_at: now,
            modified_at: now,
        }
    }

    pub fn input(question: String, answer: String, hint: Option<String>) -> Self {
        let now = Utc::now();
        let input_data = InputData {
            question: question.clone(),
            answer: answer.clone(),
            input_type: InputType::Text,
            hint,
            case_sensitive: false,
            strict: false,
        };

        let mut custom = HashMap::new();
        custom.insert("question".to_string(), toml::Value::String(question));
        custom.insert("answer".to_string(), toml::Value::String(answer));
        custom.insert("input_data".to_string(), toml::Value::String(serde_json::to_string(&input_data).unwrap()));

        Self {
            id: Uuid::new_v4(),
            card_type: CardType::Input,
            tags: vec![],
            media: vec![],
            custom,
            created_at: now,
            modified_at: now,
        }
    }

    pub fn multiple_choice(question: String, options: Vec<String>, correct_answer: usize, explanation: Option<String>) -> Self {
        let now = Utc::now();
        let mc_data = MultipleChoiceData {
            question: question.clone(),
            options: options.clone(),
            correct_answer,
            explanation,
        };

        let mut custom = HashMap::new();
        custom.insert("question".to_string(), toml::Value::String(question));
        custom.insert("mc_data".to_string(), toml::Value::String(serde_json::to_string(&mc_data).unwrap()));

        Self {
            id: Uuid::new_v4(),
            card_type: CardType::MultipleChoice,
            tags: vec![],
            media: vec![],
            custom,
            created_at: now,
            modified_at: now,
        }
    }

    pub fn front(&self) -> String {
        self.custom.get("front")
            .or_else(|| self.custom.get("question"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn back(&self) -> String {
        if let Some(back) = self.custom.get("back").and_then(|v| v.as_str()) {
            return back.to_string();
        }

        // For input cards, show answer and hint
        if self.card_type == CardType::Input {
            if let Some(input_str) = self.custom.get("input_data").and_then(|v| v.as_str()) {
                if let Ok(input_data) = serde_json::from_str::<InputData>(input_str) {
                    let mut result = format!("Answer: {}", input_data.answer);
                    if let Some(hint) = input_data.hint {
                        result += &format!("\nHint: {}", hint);
                    }
                    return result;
                }
            }
        }

        // For multiple choice cards, show correct answer and explanation
        if self.card_type == CardType::MultipleChoice {
            if let Some(mc_str) = self.custom.get("mc_data").and_then(|v| v.as_str()) {
                if let Ok(mc_data) = serde_json::from_str::<MultipleChoiceData>(mc_str) {
                    let mut result = format!("Correct Answer: {}\n", mc_data.correct_answer + 1);
                    for (i, option) in mc_data.options.iter().enumerate() {
                        result += &format!("{}. {}\n", i + 1, option);
                    }
                    if let Some(explanation) = mc_data.explanation {
                        result += &format!("\nExplanation: {}", explanation);
                    }
                    return result;
                }
            }
        }

        "".to_string()
    }
}

/// Deck metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub scheduler_config: Option<SchedulerConfig>,
}

/// Scheduler configuration (can override global settings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub new_cards_per_day: Option<i32>,
    pub max_reviews_per_day: Option<i32>,
    pub easy_interval: i32,
    pub starting_ease_factor: f32,
    pub minimum_ease_factor: f32,
    pub easy_bonus: f32,
    pub interval_multiplier: f32,
    pub hard_interval_multiplier: f32,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            new_cards_per_day: Some(20),
            max_reviews_per_day: Some(200),
            easy_interval: 4,
            starting_ease_factor: 2.5,
            minimum_ease_factor: 1.3,
            easy_bonus: 1.3,
            interval_multiplier: 2.5,
            hard_interval_multiplier: 1.2,
        }
    }
}

impl Card {
    pub fn new(content: CardContent) -> Self {
        let now = Utc::now();
        Self {
            state: CardStateData {
                id: content.id,
                due: now,
                interval: 0,
                ease_factor: 2.5,
                reps: 0,
                lapses: 0,
                state: CardState::New,
                updated_at: now,
            },
            content,
        }
    }
}

impl ImportDeck {
    /// Convert ImportDeck to internal Deck model, generating missing fields
    pub fn to_internal_deck(&self) -> Deck {
        let now = Utc::now();
        Deck {
            uuid: self.uuid.unwrap_or_else(Uuid::new_v4),
            name: self.name.clone(),
            description: self.description.clone(),
            created_at: self.created_at.unwrap_or(now),
            modified_at: self.modified_at.unwrap_or(now),
            scheduler_config: self.scheduler_config.clone(),
        }
    }

    /// Convert ImportCardContent to internal CardContent models
    pub fn to_internal_cards(&self) -> Vec<CardContent> {
        let now = Utc::now();
        self.cards.iter().map(|import_card| {
            import_card.to_internal_card_content()
        }).collect()
    }
}

impl ImportCardContent {
    /// Convert ImportCardContent to internal CardContent model, generating missing fields
    pub fn to_internal_card_content(&self) -> CardContent {
        let now = Utc::now();
        CardContent {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            front: self.front.clone(),
            back: self.back.clone(),
            tags: self.tags.clone().unwrap_or_default(),
            media: self.media.clone(),
            custom: self.custom.clone().unwrap_or_default(),
            created_at: self.created_at.unwrap_or(now),
            modified_at: self.modified_at.unwrap_or(now),
        }
    }
}

// Export format types for backup/debug functionality
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDeck {
    pub deck: Deck,
    pub cards: Vec<ExportCard>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportCard {
    #[serde(flatten)]
    pub content: CardContent,
    pub ease_factor: f32,
    pub interval: i32,
    pub reps: i32,
    pub lapses: i32,
    pub due: String,
    pub state: String,
}

// Import format types - fields are optional for flexible import
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportDeck {
    pub name: String,
    pub description: Option<String>,
    pub uuid: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub scheduler_config: Option<SchedulerConfig>,
    pub cards: Vec<ImportCardContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportCardContent {
    pub front: String,
    pub back: String,
    pub id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub media: Option<MediaRef>,
    pub custom: Option<std::collections::HashMap<String, toml::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
}

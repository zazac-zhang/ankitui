//! TUI Search and Filter System
//!
//! Provides powerful search capabilities for decks, cards, and content

use ankitui_core::{Card, CardContent, CardState, CardStateData, Deck};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Search system for decks and cards
pub struct SearchEngine {
    deck_index: DeckIndex,
    card_index: CardIndex,
    search_history: VecDeque<SearchQuery>,
    saved_searches: HashMap<String, SavedSearch>,
    config: SearchConfig,
}

/// Index for deck searching
#[derive(Debug, Clone)]
pub struct DeckIndex {
    decks_by_name: HashMap<String, Vec<Uuid>>,
    decks_by_tag: HashMap<String, Vec<Uuid>>,
    decks_by_date: HashMap<String, Vec<Uuid>>,
    decks_by_stats: HashMap<String, Vec<Uuid>>,
    full_text_index: HashMap<String, Vec<Uuid>>,
}

/// Index for card searching
#[derive(Debug, Clone)]
pub struct CardIndex {
    cards_by_front: HashMap<String, Vec<Uuid>>,
    cards_by_back: HashMap<String, Vec<Uuid>>,
    cards_by_tag: HashMap<String, Vec<Uuid>>,
    cards_by_deck: HashMap<Uuid, Vec<Uuid>>,
    full_text_index: HashMap<String, Vec<Uuid>>,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub id: String,
    pub query: String,
    pub filters: Vec<SearchFilter>,
    pub sort_by: SearchSort,
    pub timestamp: DateTime<Utc>,
    pub result_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchFilter {
    ByDeck(Vec<Uuid>),
    ByTag(Vec<String>),
    ByDateRange(DateRange),
    ByCardCount(CardCountRange),
    ByDifficulty(DifficultyRange),
    ByStatus(CardStatusFilter),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CardCountRange {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DifficultyRange {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardStatusFilter {
    New,
    Learning,
    Review,
    Suspended,
    Buried,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchSort {
    Relevance,
    NameAsc,
    NameDesc,
    CreatedAsc,
    CreatedDesc,
    ModifiedAsc,
    ModifiedDesc,
    CardCountAsc,
    CardCountDesc,
    DueCountAsc,
    DueCountDesc,
}

#[derive(Debug, Clone)]
pub struct SavedSearch {
    pub name: String,
    pub query: SearchQuery,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u32,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub items: Vec<SearchItem>,
    pub total_count: usize,
    pub query_id: String,
    pub search_time: std::time::Duration,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum SearchItem {
    Deck(DeckSearchResult),
    Card(CardSearchResult),
}

#[derive(Debug, Clone)]
pub struct DeckSearchResult {
    pub deck: Deck,
    pub relevance_score: f32,
    pub matched_fields: Vec<String>,
    pub preview_text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CardSearchResult {
    pub card: Card,
    pub deck_id: Uuid,
    pub deck_name: String,
    pub relevance_score: f32,
    pub matched_fields: Vec<String>,
    pub preview_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchConfig {
    pub max_results: usize,
    pub enable_fuzzy_search: bool,
    pub min_query_length: usize,
    pub highlight_matches: bool,
    pub cache_results: bool,
    pub cache_duration: std::time::Duration,
    pub indexing_depth: IndexingDepth,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndexingDepth {
    Basic,    // Name and description only
    Standard, // Include tags and basic metadata
    Deep,     // Include full text content
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            deck_index: DeckIndex::new(),
            card_index: CardIndex::new(),
            search_history: VecDeque::new(),
            saved_searches: HashMap::new(),
            config: SearchConfig::default(),
        }
    }

    pub fn index_decks(&mut self, decks: &[Deck]) -> Result<()> {
        self.deck_index.clear();
        for deck in decks {
            self.deck_index.add_deck(deck.clone());
        }
        Ok(())
    }

    pub fn index_cards(&mut self, cards: &[Card], deck_id: Uuid) -> Result<()> {
        self.card_index.clear();
        for card in cards {
            self.card_index.add_card(card.clone(), deck_id);
        }
        Ok(())
    }

    pub fn search_decks(
        &mut self,
        query: &str,
        filters: &[SearchFilter],
        sort: SearchSort,
    ) -> Result<SearchResult> {
        let start_time = std::time::Instant::now();

        // Create search query
        let search_query = SearchQuery {
            id: Uuid::new_v4().to_string(),
            query: query.to_string(),
            filters: filters.to_vec(),
            sort_by: sort,
            timestamp: Utc::now(),
            result_count: 0,
        };

        // Execute search
        let mut results = Vec::new();

        // Text search
        if !query.is_empty() {
            let text_results = self.deck_index.text_search(query, &self.config)?;
            results.extend(text_results);
        }

        // Apply filters
        results = self.apply_deck_filters(results, filters);

        // Sort results
        self.sort_deck_results(&mut results, sort.clone());

        // Limit results
        if results.len() > self.config.max_results {
            results.truncate(self.config.max_results);
        }

        // Generate suggestions
        let suggestions = self.generate_deck_suggestions(query, &results);

        let total_count = results.len();
        let search_result = SearchResult {
            items: results.into_iter().map(SearchItem::Deck).collect(),
            total_count,
            query_id: search_query.id.clone(),
            search_time: start_time.elapsed(),
            suggestions,
        };

        // Store in history
        self.add_to_history(search_query);

        Ok(search_result)
    }

    pub fn search_cards(
        &mut self,
        query: &str,
        filters: &[SearchFilter],
        sort: SearchSort,
    ) -> Result<SearchResult> {
        let start_time = std::time::Instant::now();

        let search_query = SearchQuery {
            id: Uuid::new_v4().to_string(),
            query: query.to_string(),
            filters: filters.to_vec(),
            sort_by: sort,
            timestamp: Utc::now(),
            result_count: 0,
        };

        let mut results = Vec::new();

        // Text search
        if !query.is_empty() {
            let text_results = self.card_index.text_search(query, &self.config)?;
            results.extend(text_results);
        }

        // Apply filters
        results = self.apply_card_filters(results, filters);

        // Sort results
        self.sort_card_results(&mut results, sort.clone());

        // Limit results
        if results.len() > self.config.max_results {
            results.truncate(self.config.max_results);
        }

        let suggestions = self.generate_suggestions(query, &results);

        let total_count = results.len();
        let search_result = SearchResult {
            items: results.into_iter().map(SearchItem::Card).collect(),
            total_count,
            query_id: search_query.id.clone(),
            search_time: start_time.elapsed(),
            suggestions,
        };

        self.add_to_history(search_query);

        Ok(search_result)
    }

    pub fn save_search(&mut self, name: String, query: SearchQuery, description: Option<String>) {
        let saved_search = SavedSearch {
            name: name.clone(),
            query,
            description,
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
        };

        self.saved_searches.insert(name, saved_search);
    }

    pub fn load_saved_search(&mut self, name: &str) -> Option<&SearchQuery> {
        if let Some(saved_search) = self.saved_searches.get_mut(name) {
            saved_search.last_used = Some(Utc::now());
            saved_search.usage_count += 1;
            Some(&saved_search.query)
        } else {
            None
        }
    }

    pub fn get_search_history(&self) -> Vec<SearchQuery> {
        self.search_history.iter().cloned().collect()
    }

    pub fn get_saved_searches(&self) -> HashMap<String, &SavedSearch> {
        self.saved_searches
            .iter()
            .map(|(k, v)| (k.clone(), v))
            .collect()
    }

    pub fn clear_history(&mut self) {
        self.search_history.clear();
    }

    pub fn set_config(&mut self, config: SearchConfig) {
        self.config = config;
    }

    fn add_to_history(&mut self, query: SearchQuery) {
        self.search_history.push_back(query);
        if self.search_history.len() > 100 {
            self.search_history.pop_front();
        }
    }

    fn apply_deck_filters(
        &self,
        results: Vec<DeckSearchResult>,
        filters: &[SearchFilter],
    ) -> Vec<DeckSearchResult> {
        results
            .into_iter()
            .filter(|result| {
                filters.iter().all(|filter| {
                    match filter {
                        SearchFilter::ByDeck(_) => true, // Not applicable for deck search
                        SearchFilter::ByTag(_) => true,  // Not applicable for deck search
                        SearchFilter::ByDateRange(range) => {
                            if let (Some(start), Some(end)) = (range.start, range.end) {
                                result.deck.created_at >= start && result.deck.created_at <= end
                            } else {
                                true
                            }
                        }
                        SearchFilter::ByCardCount(range) => {
                            // Would need deck stats to implement properly
                            true
                        }
                        SearchFilter::ByDifficulty(_) => true, // Not applicable for decks
                        SearchFilter::ByStatus(_) => true,     // Not applicable for decks
                        SearchFilter::Custom(_) => true,       // Would need custom logic
                    }
                })
            })
            .collect()
    }

    fn apply_card_filters(
        &self,
        results: Vec<CardSearchResult>,
        filters: &[SearchFilter],
    ) -> Vec<CardSearchResult> {
        results
            .into_iter()
            .filter(|result| {
                filters.iter().all(|filter| {
                    match filter {
                        SearchFilter::ByDeck(deck_ids) => deck_ids.contains(&result.deck_id),
                        SearchFilter::ByTag(tags) => {
                            let card_tags: HashSet<&str> = result
                                .card
                                .content
                                .tags
                                .iter()
                                .map(|s| s.as_str())
                                .collect();
                            tags.iter().any(|tag| card_tags.contains(tag.as_str()))
                        }
                        SearchFilter::ByDateRange(range) => {
                            if let (Some(start), Some(end)) = (range.start, range.end) {
                                result.card.content.created_at >= start
                                    && result.card.content.created_at <= end
                            } else {
                                true
                            }
                        }
                        SearchFilter::ByCardCount(_) => true, // Not applicable for cards
                        SearchFilter::ByDifficulty(range) => {
                            if let (Some(min), Some(max)) = (range.min, range.max) {
                                let ease_factor = result.card.state.ease_factor;
                                ease_factor >= min && ease_factor <= max
                            } else {
                                true
                            }
                        }
                        SearchFilter::ByStatus(status) => {
                            match status {
                                CardStatusFilter::New => {
                                    matches!(result.card.state.state, CardState::New)
                                }
                                CardStatusFilter::Learning => {
                                    matches!(result.card.state.state, CardState::Learning)
                                }
                                CardStatusFilter::Review => {
                                    matches!(result.card.state.state, CardState::Review)
                                }
                                CardStatusFilter::Suspended => false, // Would need additional field
                                CardStatusFilter::Buried => false,    // Would need additional field
                            }
                        }
                        SearchFilter::Custom(_) => true, // Would need custom logic
                    }
                })
            })
            .collect()
    }

    fn sort_deck_results(&self, results: &mut [DeckSearchResult], sort: SearchSort) {
        match sort {
            SearchSort::Relevance => results.sort_by(|a, b| {
                b.relevance_score
                    .partial_cmp(&a.relevance_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            SearchSort::NameAsc => results.sort_by(|a, b| a.deck.name.cmp(&b.deck.name)),
            SearchSort::NameDesc => results.sort_by(|a, b| b.deck.name.cmp(&a.deck.name)),
            SearchSort::CreatedAsc => {
                results.sort_by(|a, b| a.deck.created_at.cmp(&b.deck.created_at))
            }
            SearchSort::CreatedDesc => {
                results.sort_by(|a, b| b.deck.created_at.cmp(&a.deck.created_at))
            }
            SearchSort::ModifiedAsc => {
                results.sort_by(|a, b| a.deck.modified_at.cmp(&b.deck.modified_at))
            }
            SearchSort::ModifiedDesc => {
                results.sort_by(|a, b| b.deck.modified_at.cmp(&a.deck.modified_at))
            }
            SearchSort::CardCountAsc | SearchSort::CardCountDesc => {
                // Would need deck stats to implement properly
            }
            SearchSort::DueCountAsc | SearchSort::DueCountDesc => {
                // Would need deck stats to implement properly
            }
        }
    }

    fn sort_card_results(&self, results: &mut [CardSearchResult], sort: SearchSort) {
        match sort {
            SearchSort::Relevance => results.sort_by(|a, b| {
                b.relevance_score
                    .partial_cmp(&a.relevance_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            SearchSort::NameAsc => results.sort_by(|a, b| a.deck_name.cmp(&b.deck_name)),
            SearchSort::NameDesc => results.sort_by(|a, b| b.deck_name.cmp(&a.deck_name)),
            SearchSort::CreatedAsc => {
                results.sort_by(|a, b| a.card.content.created_at.cmp(&b.card.content.created_at))
            }
            SearchSort::CreatedDesc => {
                results.sort_by(|_a, b| b.card.content.created_at.cmp(&b.card.content.created_at))
            }
            SearchSort::ModifiedAsc | SearchSort::ModifiedDesc => {
                // Would need modified timestamp on cards
            }
            SearchSort::CardCountAsc
            | SearchSort::CardCountDesc
            | SearchSort::DueCountAsc
            | SearchSort::DueCountDesc => {
                // Not applicable for cards
            }
        }
    }

    fn generate_suggestions(&self, query: &str, results: &[CardSearchResult]) -> Vec<String> {
        let mut suggestions = Vec::new();

        // If query is short, suggest common terms
        if query.len() < 3 {
            suggestions.extend(
                ["front", "back", "tag", "new", "review"]
                    .iter()
                    .map(|s| s.to_string()),
            );
        }

        // Add suggestions based on results
        if results.is_empty() {
            // No results - suggest checking spelling or trying different terms
            suggestions.push("Try different search terms".to_string());
            suggestions.push("Check spelling".to_string());
        }

        suggestions
    }

    fn generate_deck_suggestions(&self, query: &str, results: &[DeckSearchResult]) -> Vec<String> {
        let mut suggestions = Vec::new();

        // If query is short, suggest common terms
        if query.len() < 3 {
            suggestions.extend(
                ["deck", "card", "learn", "study", "review"]
                    .iter()
                    .map(|s| s.to_string()),
            );
        }

        // Add suggestions based on results
        if results.is_empty() {
            // No results - suggest checking spelling or trying different terms
            suggestions.push("Try different search terms".to_string());
            suggestions.push("Check spelling".to_string());
        }

        suggestions
    }
}

impl DeckIndex {
    pub fn new() -> Self {
        Self {
            decks_by_name: HashMap::new(),
            decks_by_tag: HashMap::new(),
            decks_by_date: HashMap::new(),
            decks_by_stats: HashMap::new(),
            full_text_index: HashMap::new(),
        }
    }

    pub fn add_deck(&mut self, deck: Deck) {
        // Index by name
        let name_words: Vec<String> = deck
            .name
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for word in name_words {
            self.decks_by_name
                .entry(word)
                .or_insert_with(Vec::new)
                .push(deck.uuid);
        }

        // Index by description
        if let Some(description) = &deck.description {
            let desc_words: Vec<String> = description
                .to_lowercase()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            for word in desc_words {
                self.full_text_index
                    .entry(word)
                    .or_insert_with(Vec::new)
                    .push(deck.uuid);
            }
        }

        // Index by date
        let date_key = deck.created_at.format("%Y-%m-%d").to_string();
        self.decks_by_date
            .entry(date_key)
            .or_insert_with(Vec::new)
            .push(deck.uuid);
    }

    pub fn text_search(
        &self,
        query: &str,
        _config: &SearchConfig,
    ) -> Result<Vec<DeckSearchResult>> {
        let query_words: Vec<String> = query
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if query_words.is_empty() {
            return Ok(Vec::new());
        }

        let mut matching_decks: HashMap<Uuid, f32> = HashMap::new();

        for word in query_words {
            // Search in names
            if let Some(decks) = self.decks_by_name.get(&word) {
                for deck_id in decks {
                    *matching_decks.entry(*deck_id).or_insert(0.0) += 1.0;
                }
            }

            // Search in full text
            if let Some(decks) = self.full_text_index.get(&word) {
                for deck_id in decks {
                    *matching_decks.entry(*deck_id).or_insert(0.0) += 0.5;
                }
            }
        }

        // Convert to search results
        let mut results = Vec::new();
        for (deck_id, score) in matching_decks {
            // This would need the actual deck data to create proper results
            // For now, we'll return placeholder results
            results.push(DeckSearchResult {
                deck: Deck {
                    uuid: deck_id,
                    name: format!("Deck {}", deck_id.to_string()[..8].to_uppercase()),
                    description: Some("Search result".to_string()),
                    created_at: Utc::now(),
                    modified_at: Utc::now(),
                    scheduler_config: None,
                },
                relevance_score: score,
                matched_fields: vec!["name".to_string()],
                preview_text: None,
            });
        }

        Ok(results)
    }

    pub fn clear(&mut self) {
        self.decks_by_name.clear();
        self.decks_by_tag.clear();
        self.decks_by_date.clear();
        self.decks_by_stats.clear();
        self.full_text_index.clear();
    }
}

impl CardIndex {
    pub fn new() -> Self {
        Self {
            cards_by_front: HashMap::new(),
            cards_by_back: HashMap::new(),
            cards_by_tag: HashMap::new(),
            cards_by_deck: HashMap::new(),
            full_text_index: HashMap::new(),
        }
    }

    pub fn add_card(&mut self, card: Card, deck_id: Uuid) {
        // Index by front content
        let front_words: Vec<String> = card
            .content
            .front
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for word in front_words {
            self.cards_by_front
                .entry(word)
                .or_insert_with(Vec::new)
                .push(card.state.id);
        }

        // Index by back content
        let back_words: Vec<String> = card
            .content
            .back
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for word in back_words {
            self.cards_by_back
                .entry(word)
                .or_insert_with(Vec::new)
                .push(card.state.id);
        }

        // Index by tags
        for tag in &card.content.tags {
            let tag_lower = tag.to_lowercase();
            self.cards_by_tag
                .entry(tag_lower)
                .or_insert_with(Vec::new)
                .push(card.state.id);
        }

        // Index by deck
        self.cards_by_deck
            .entry(deck_id)
            .or_insert_with(Vec::new)
            .push(card.state.id);
    }

    pub fn text_search(&self, query: &str, config: &SearchConfig) -> Result<Vec<CardSearchResult>> {
        let query_words: Vec<String> = query
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if query_words.is_empty() {
            return Ok(Vec::new());
        }

        let mut matching_cards: HashMap<Uuid, f32> = HashMap::new();

        for word in query_words {
            // Search in front content
            if let Some(cards) = self.cards_by_front.get(&word) {
                for card_id in cards {
                    *matching_cards.entry(*card_id).or_insert(0.0) += 1.0;
                }
            }

            // Search in back content
            if let Some(cards) = self.cards_by_back.get(&word) {
                for card_id in cards {
                    *matching_cards.entry(*card_id).or_insert(0.0) += 1.0;
                }
            }

            // Search in tags
            if let Some(cards) = self.cards_by_tag.get(&word) {
                for card_id in cards {
                    *matching_cards.entry(*card_id).or_insert(0.0) += 0.8;
                }
            }
        }

        // Convert to search results
        let mut results = Vec::new();
        for (card_id, score) in matching_cards {
            // This would need the actual card data to create proper results
            // For now, we'll return placeholder results
            results.push(CardSearchResult {
                card: Card {
                    state: CardStateData {
                        id: card_id,
                        due: Utc::now(),
                        interval: 0,
                        ease_factor: 2.5,
                        reps: 0,
                        lapses: 0,
                        state: CardState::New,
                        updated_at: Utc::now(),
                    },
                    content: CardContent {
                        id: card_id,
                        front: "Search result card".to_string(),
                        back: "Search result answer".to_string(),
                        tags: vec!["search".to_string()],
                        media: None,
                        custom: HashMap::new(),
                        created_at: Utc::now(),
                        modified_at: Utc::now(),
                    },
                },
                deck_id: Uuid::new_v4(), // Placeholder - would need actual deck tracking
                deck_name: "Unknown Deck".to_string(),
                relevance_score: score,
                matched_fields: vec!["front".to_string()],
                preview_text: Some("Search result preview".to_string()),
            });
        }

        Ok(results)
    }

    pub fn clear(&mut self) {
        self.cards_by_front.clear();
        self.cards_by_back.clear();
        self.cards_by_tag.clear();
        self.cards_by_deck.clear();
        self.full_text_index.clear();
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_results: 50,
            enable_fuzzy_search: true,
            min_query_length: 2,
            highlight_matches: true,
            cache_results: true,
            cache_duration: std::time::Duration::from_secs(300), // 5 minutes
            indexing_depth: IndexingDepth::Standard,
        }
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::VecDeque;

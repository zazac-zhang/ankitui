//! Tag Management System
//!
//! Comprehensive tag management with metadata, statistics, and operations

use crate::data::models::{Card, CardState, Tag, TagColor, TagFilter, TagPriority, TagStats};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Tag manager for handling tag operations and analytics
#[derive(Debug)]
pub struct TagManager {
    /// Tag metadata storage
    tags: HashMap<String, Tag>,
    /// Tag statistics cache
    tag_stats: HashMap<String, TagStats>,
    /// Tag hierarchy cache (parent -> children)
    tag_hierarchy: HashMap<String, Vec<String>>,
}

impl TagManager {
    /// Create a new tag manager
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
            tag_stats: HashMap::new(),
            tag_hierarchy: HashMap::new(),
        }
    }

    /// Initialize tag manager from existing cards
    pub fn initialize_from_cards(&mut self, cards: &[Card]) -> Result<()> {
        // Clear existing data
        self.tags.clear();
        self.tag_stats.clear();
        self.tag_hierarchy.clear();

        // Process all cards to extract tags
        let mut tag_usage: HashMap<String, u32> = HashMap::new();
        let mut tag_cards: HashMap<String, Vec<&Card>> = HashMap::new();

        for card in cards {
            for tag_name in &card.content.tags {
                *tag_usage.entry(tag_name.clone()).or_insert(0) += 1;
                tag_cards.entry(tag_name.clone()).or_default().push(card);
            }
        }

        // Create tag objects and statistics
        for (tag_name, usage_count) in tag_usage {
            let tag = Tag {
                id: Uuid::new_v4(),
                name: tag_name.clone(),
                color: Some(TagColor::Blue),
                priority: TagPriority::Normal,
                description: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                usage_count: Some(usage_count as usize),
                parent_tag: None,
                shortcuts: None,
            };

            self.tags.insert(tag_name.clone(), tag);

            // Calculate statistics
            if let Some(cards) = tag_cards.get(&tag_name) {
                let stats = self.calculate_tag_stats(&tag_name, cards)?;
                self.tag_stats.insert(tag_name, stats);
            }
        }

        self.update_hierarchy();

        Ok(())
    }

    /// Create or update a tag
    pub fn create_or_update_tag(&mut self, tag: Tag) -> Result<()> {
        let tag_name = tag.name.clone();
        self.tags.insert(tag_name.clone(), tag);
        self.update_hierarchy();
        Ok(())
    }

    /// Delete a tag
    pub fn delete_tag(&mut self, tag_name: &str) -> Result<bool> {
        let removed = self.tags.remove(tag_name).is_some();
        if removed {
            self.tag_stats.remove(tag_name);
            self.update_hierarchy();
        }
        Ok(removed)
    }

    /// Get tag by name
    pub fn get_tag(&self, tag_name: &str) -> Option<&Tag> {
        self.tags.get(tag_name)
    }

    /// Get all tags
    pub fn get_all_tags(&self) -> Vec<&Tag> {
        let mut tags: Vec<&Tag> = self.tags.values().collect();
        tags.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.name.cmp(&b.name))
        });
        tags
    }

    /// Get tags by filter
    pub fn get_tags_by_filter(&self, filter: &TagFilter) -> Vec<&Tag> {
        self.tags
            .values()
            .filter(|tag| self.matches_filter(tag, filter))
            .collect()
    }

    /// Get tag statistics
    pub fn get_tag_stats(&self, tag_name: &str) -> Option<&TagStats> {
        self.tag_stats.get(tag_name)
    }

    /// Get all tag statistics
    pub fn get_all_tag_stats(&self) -> Vec<&TagStats> {
        let mut stats: Vec<&TagStats> = self.tag_stats.values().collect();
        stats.sort_by(|a, b| b.card_count.cmp(&a.card_count));
        stats
    }

    /// Update tag usage from cards
    pub fn update_tag_usage(&mut self, cards: &[Card]) -> Result<()> {
        let mut tag_usage: HashMap<String, u32> = HashMap::new();
        let mut tag_cards: HashMap<String, Vec<&Card>> = HashMap::new();

        for card in cards {
            for tag_name in &card.content.tags {
                *tag_usage.entry(tag_name.clone()).or_insert(0) += 1;
                tag_cards.entry(tag_name.clone()).or_default().push(card);
            }
        }

        // Collect tag names before consuming tag_usage
        let tag_names: Vec<String> = tag_usage.keys().cloned().collect();

        // Update existing tags and create new ones
        for (tag_name, usage_count) in tag_usage {
            if let Some(tag) = self.tags.get_mut(&tag_name) {
                tag.usage_count = Some(usage_count as usize);
            } else {
                let tag = Tag {
                    id: Uuid::new_v4(),
                    name: tag_name.clone(),
                    color: Some(TagColor::Blue),
                    priority: TagPriority::Normal,
                    description: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    usage_count: Some(usage_count as usize),
                    parent_tag: None,
                    shortcuts: None,
                };
                self.tags.insert(tag_name.clone(), tag);
            }

            // Update statistics
            if let Some(cards) = tag_cards.get(&tag_name) {
                let stats = self.calculate_tag_stats(&tag_name, cards)?;
                self.tag_stats.insert(tag_name, stats);
            }
        }

        // Remove tags that are no longer used
        self.tags.retain(|tag_name, _| tag_names.contains(tag_name));
        self.tag_stats
            .retain(|tag_name, _| tag_names.contains(tag_name));

        self.update_hierarchy();

        Ok(())
    }

    /// Search tags by query
    pub fn search_tags(&self, query: &str) -> Vec<&Tag> {
        let query_lower = query.to_lowercase();
        self.tags
            .values()
            .filter(|tag| {
                tag.name.to_lowercase().contains(&query_lower)
                    || tag
                        .description
                        .as_ref()
                        .map(|desc| desc.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .collect()
    }

    /// Get tags for card with hierarchy
    pub fn get_card_tags_with_hierarchy(&self, card: &Card) -> Vec<TagInfo> {
        card.content
            .tags
            .iter()
            .filter_map(|tag_name| {
                self.tags.get(tag_name).map(|tag| TagInfo {
                    tag: tag.clone(),
                    parent_tags: self.get_parent_tags(tag_name),
                    child_tags: self.get_child_tags(tag_name),
                })
            })
            .collect()
    }

    /// Get tag hierarchy tree
    pub fn get_tag_hierarchy(&self) -> Vec<TagNode> {
        let mut roots = Vec::new();

        for tag in self.tags.values() {
            if tag.parent_tag.is_none() {
                let node = self.build_tag_node(&tag.name);
                roots.push(node);
            }
        }

        roots
    }

    /// Rename a tag
    pub fn rename_tag(&mut self, old_name: &str, new_name: String) -> Result<bool> {
        if let Some(mut tag) = self.tags.remove(old_name) {
            tag.name = new_name.clone();
            self.tags.insert(new_name.clone(), tag);

            // Update statistics
            if let Some(mut stats) = self.tag_stats.remove(old_name) {
                stats.name = new_name.clone();
                self.tag_stats.insert(new_name, stats);
            }

            // Update hierarchy references
            self.update_hierarchy();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Set tag color
    pub fn set_tag_color(&mut self, tag_name: &str, color: Option<TagColor>) -> Result<bool> {
        if let Some(tag) = self.tags.get_mut(tag_name) {
            tag.color = color;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Set tag priority
    pub fn set_tag_priority(&mut self, tag_name: &str, priority: TagPriority) -> Result<bool> {
        if let Some(tag) = self.tags.get_mut(tag_name) {
            tag.priority = priority;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get popular tags (by usage count)
    pub fn get_popular_tags(&self, limit: usize) -> Vec<&Tag> {
        let mut tags: Vec<&Tag> = self.tags.values().collect();
        tags.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        tags.truncate(limit);
        tags
    }

    /// Get recently used tags (by card creation/modification)
    pub fn get_recent_tags(&self, cards: &[Card], limit: usize) -> Vec<&Tag> {
        let mut tag_recent_usage: HashMap<String, DateTime<Utc>> = HashMap::new();

        for card in cards {
            for tag_name in &card.content.tags {
                let current_time = tag_recent_usage
                    .entry(tag_name.clone())
                    .or_insert(card.content.modified_at);

                if card.content.modified_at > *current_time {
                    *current_time = card.content.modified_at;
                }
            }
        }

        let mut tags: Vec<(&Tag, DateTime<Utc>)> = self
            .tags
            .values()
            .filter_map(|tag| tag_recent_usage.get(&tag.name).map(|time| (tag, *time)))
            .collect();

        tags.sort_by(|a, b| b.1.cmp(&a.1));
        tags.truncate(limit);
        tags.into_iter().map(|(tag, _)| tag).collect()
    }

    // Private helper methods

    /// Calculate tag statistics from cards
    fn calculate_tag_stats(&self, tag_name: &str, cards: &[&Card]) -> Result<TagStats> {
        let mut new_cards = 0;
        let mut learning_cards = 0;
        let mut review_cards = 0;
        let mut total_ease_factor = 0.0;
        let mut correct_responses = 0;
        let mut total_responses = 0;
        let mut last_used = None;

        for card in cards {
            match card.state.state {
                CardState::New => new_cards += 1,
                CardState::Learning => learning_cards += 1,
                CardState::Review => review_cards += 1,
                _ => {}
            }

            total_ease_factor += card.state.ease_factor;

            // Simplified retention calculation
            if card.state.reps > 0 {
                let success_rate =
                    (card.state.reps - card.state.lapses) as f32 / card.state.reps as f32;
                correct_responses += (card.state.reps - card.state.lapses);
                total_responses += card.state.reps;
            }

            if card.state.due
                > last_used.unwrap_or_else(|| {
                    DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
                        .unwrap()
                        .into()
                })
            {
                last_used = Some(card.state.due);
            }
        }

        let average_ease_factor = if cards.is_empty() {
            0.0
        } else {
            total_ease_factor / cards.len() as f32
        };

        let retention_rate = if total_responses == 0 {
            0.0
        } else {
            correct_responses as f32 / total_responses as f32
        };

        Ok(TagStats {
            tag_id: Uuid::new_v4(),
            name: tag_name.to_string(),
            card_count: cards.len(),
            review_cards,
            new_cards,
            last_used,
            average_retention: retention_rate,
            retention_rate,
        })
    }

    /// Check if tag matches filter
    fn matches_filter(&self, tag: &Tag, filter: &TagFilter) -> bool {
        match filter {
            TagFilter::Exact(name) => tag.name == *name,
            TagFilter::StartsWith(prefix) => tag.name.starts_with(prefix),
            TagFilter::Contains(substring) => tag.name.contains(substring),
            TagFilter::ByColor(color) => tag.color.as_ref().map_or(false, |c| c == color),
            TagFilter::ByPriority(priority) => tag.priority == *priority,
            TagFilter::ByParent(parent) => tag.parent_tag.map_or(false, |p| p == *parent),
            TagFilter::HasParent => tag.parent_tag.is_some(),
            TagFilter::NoParent => tag.parent_tag.is_none(),
        }
    }

    /// Update tag hierarchy cache
    fn update_hierarchy(&mut self) {
        self.tag_hierarchy.clear();

        for tag in self.tags.values() {
            if let Some(parent) = &tag.parent_tag {
                self.tag_hierarchy
                    .entry(parent.to_string())
                    .or_default()
                    .push(tag.name.clone());
            }
        }
    }

    /// Get parent tags for a tag
    fn get_parent_tags(&self, tag_name: &str) -> Vec<String> {
        let mut parents = Vec::new();
        let mut current = tag_name.to_string();

        while let Some(tag) = self.tags.get(&current) {
            if let Some(parent) = &tag.parent_tag {
                parents.push(parent.to_string());
                current = parent.to_string();
            } else {
                break;
            }
        }

        parents
    }

    /// Get child tags for a tag
    fn get_child_tags(&self, tag_name: &str) -> Vec<String> {
        self.tag_hierarchy
            .get(tag_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Build tag hierarchy node
    fn build_tag_node(&self, tag_name: &str) -> TagNode {
        let tag = self.tags.get(tag_name).unwrap();
        let children = self
            .get_child_tags(tag_name)
            .into_iter()
            .map(|child_name| self.build_tag_node(&child_name))
            .collect();

        TagNode {
            tag: tag.clone(),
            children,
            stats: self.tag_stats.get(tag_name).cloned(),
        }
    }
}

impl Default for TagManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Tag information with hierarchy
#[derive(Debug, Clone)]
pub struct TagInfo {
    pub tag: Tag,
    pub parent_tags: Vec<String>,
    pub child_tags: Vec<String>,
}

/// Tag hierarchy node
#[derive(Debug, Clone)]
pub struct TagNode {
    pub tag: Tag,
    pub children: Vec<TagNode>,
    pub stats: Option<TagStats>,
}

/// Tag operations result
#[derive(Debug, Clone)]
pub struct TagOperationResult {
    pub success: bool,
    pub affected_cards: usize,
    pub message: String,
}

impl TagManager {
    /// Add tags to cards
    pub fn add_tags_to_cards(
        &mut self,
        card_ids: &[Uuid],
        tags: Vec<String>,
        cards: &mut [Card],
    ) -> Result<TagOperationResult> {
        let mut affected_cards = 0;
        let tags_set: HashSet<String> = tags.into_iter().collect();

        for card in cards.iter_mut() {
            if card_ids.contains(&card.content.id) {
                let mut added = false;
                for tag in &tags_set {
                    if !card.content.tags.contains(tag) {
                        card.content.tags.push(tag.clone());
                        added = true;
                    }
                }
                if added {
                    affected_cards += 1;
                }
            }
        }

        Ok(TagOperationResult {
            success: true,
            affected_cards,
            message: format!("Added tags to {} cards", affected_cards),
        })
    }

    /// Remove tags from cards
    pub fn remove_tags_from_cards(
        &mut self,
        card_ids: &[Uuid],
        tags: Vec<String>,
        cards: &mut [Card],
    ) -> Result<TagOperationResult> {
        let mut affected_cards = 0;
        let tags_set: HashSet<String> = tags.into_iter().collect();

        for card in cards.iter_mut() {
            if card_ids.contains(&card.content.id) {
                let original_len = card.content.tags.len();
                card.content.tags.retain(|tag| !tags_set.contains(tag));
                if card.content.tags.len() < original_len {
                    affected_cards += 1;
                }
            }
        }

        Ok(TagOperationResult {
            success: true,
            affected_cards,
            message: format!("Removed tags from {} cards", affected_cards),
        })
    }

    /// Replace tags on cards
    pub fn replace_tags_on_cards(
        &mut self,
        card_ids: &[Uuid],
        old_tags: Vec<String>,
        new_tags: Vec<String>,
        cards: &mut [Card],
    ) -> Result<TagOperationResult> {
        let mut affected_cards = 0;
        let old_tags_set: HashSet<String> = old_tags.into_iter().collect();
        let new_tags_set: HashSet<String> = new_tags.into_iter().collect();

        for card in cards.iter_mut() {
            if card_ids.contains(&card.content.id) {
                let mut changed = false;

                // Remove old tags
                card.content.tags.retain(|tag| !old_tags_set.contains(tag));

                // Add new tags
                for tag in &new_tags_set {
                    if !card.content.tags.contains(tag) {
                        card.content.tags.push(tag.clone());
                        changed = true;
                    }
                }

                if changed || !old_tags_set.is_empty() {
                    affected_cards += 1;
                }
            }
        }

        Ok(TagOperationResult {
            success: true,
            affected_cards,
            message: format!("Replaced tags on {} cards", affected_cards),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::models::{CardContent, CardStateData};

    fn create_test_card(front: &str, tags: Vec<&str>) -> Card {
        Card {
            content: CardContent {
                id: Uuid::new_v4(),
                front: front.to_string(),
                back: "back".to_string(),
                tags: tags.into_iter().map(|s| s.to_string()).collect(),
                media: None,
                custom: HashMap::new(),
                created_at: Utc::now(),
                modified_at: Utc::now(),
            },
            state: CardStateData {
                id: Uuid::new_v4(),
                due: Utc::now(),
                interval: 1,
                ease_factor: 2.5,
                reps: 1,
                lapses: 0,
                state: CardState::New,
                updated_at: todo!(),
            },
        }
    }

    #[tokio::test]
    async fn test_tag_manager_initialization() {
        let mut tag_manager = TagManager::new();
        let cards = vec![
            create_test_card("card1", vec!["tag1", "tag2"]),
            create_test_card("card2", vec!["tag2", "tag3"]),
        ];

        tag_manager.initialize_from_cards(&cards).unwrap();

        assert_eq!(tag_manager.get_all_tags().len(), 3);
        assert!(tag_manager.get_tag("tag1").is_some());
        assert!(tag_manager.get_tag("tag2").is_some());
        assert!(tag_manager.get_tag("tag3").is_some());
    }

    #[tokio::test]
    async fn test_tag_search() {
        let mut tag_manager = TagManager::new();
        let cards = vec![
            create_test_card("card1", vec!["programming", "rust"]),
            create_test_card("card2", vec!["programming", "python"]),
        ];

        tag_manager.initialize_from_cards(&cards).unwrap();

        let results = tag_manager.search_tags("prog");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "programming");
    }

    #[tokio::test]
    async fn test_tag_filtering() {
        let mut tag_manager = TagManager::new();

        let tag = Tag {
            id: Uuid::new_v4(),
            name: "important".to_string(),
            color: Some(TagColor::Red),
            priority: TagPriority::High,
            description: Some("Important cards".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            usage_count: Some(10),
            parent_tag: None,
            shortcuts: Some(vec!["imp".to_string()]),
        };

        tag_manager.create_or_update_tag(tag).unwrap();

        let red_tags = tag_manager.get_tags_by_filter(&TagFilter::ByColor(TagColor::Red));
        assert_eq!(red_tags.len(), 1);
        assert_eq!(red_tags[0].name, "important");

        let high_priority_tags =
            tag_manager.get_tags_by_filter(&TagFilter::ByPriority(TagPriority::High));
        assert_eq!(high_priority_tags.len(), 1);
    }

    #[tokio::test]
    async fn test_tag_operations() {
        let mut tag_manager = TagManager::new();
        let mut cards = vec![
            create_test_card("card1", vec!["tag1"]),
            create_test_card("card2", vec!["tag2"]),
        ];

        tag_manager.initialize_from_cards(&cards).unwrap();

        // Add tags to cards
        let card_ids = vec![cards[0].content.id];
        let result = tag_manager
            .add_tags_to_cards(&card_ids, vec!["new_tag".to_string()], &mut cards)
            .unwrap();
        assert!(result.success);
        assert_eq!(result.affected_cards, 1);

        // Remove tags from cards
        let result = tag_manager
            .remove_tags_from_cards(&card_ids, vec!["new_tag".to_string()], &mut cards)
            .unwrap();
        assert!(result.success);
        assert_eq!(result.affected_cards, 1);
    }
}

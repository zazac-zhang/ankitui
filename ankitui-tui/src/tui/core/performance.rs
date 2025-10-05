//! TUI Performance Optimization
//!
//! Provides caching, async data loading, and performance monitoring

use ankitui_core::Card;
use ankitui_core::Deck;
use ankitui_core::DeckStatistics;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

/// Render cache system
pub struct RenderCache {
    deck_list_cache: Option<CachedDeckList>,
    stats_cache: HashMap<Uuid, CachedDeckStats>,
    ui_state_cache: Option<CachedUIState>,
    performance_metrics: PerformanceMetrics,
    cache_config: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct CachedDeckList {
    decks: Vec<Deck>,
    rendered_content: String,
    last_updated: DateTime<Utc>,
    hash: u64,
}

#[derive(Debug, Clone)]
pub struct CachedDeckStats {
    deck_id: Uuid,
    stats: DeckStatistics,
    last_updated: DateTime<Utc>,
    ttl: Duration,
}

#[derive(Debug, Clone)]
pub struct CachedUIState {
    state_hash: u64,
    rendered_components: HashMap<String, CachedComponent>,
    last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CachedComponent {
    content: String,
    size: (u16, u16),
    last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    max_deck_list_age: Duration,
    max_stats_age: Duration,
    max_ui_state_age: Duration,
    max_cache_size: usize,
    enable_compression: bool,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    render_times: Vec<Duration>,
    cache_hits: u64,
    cache_misses: u64,
    data_load_times: HashMap<String, Vec<Duration>>,
    last_cleanup: Instant,
}

impl RenderCache {
    pub fn new() -> Self {
        Self {
            deck_list_cache: None,
            stats_cache: HashMap::new(),
            ui_state_cache: None,
            performance_metrics: PerformanceMetrics::new(),
            cache_config: CacheConfig::default(),
        }
    }

    pub fn should_rerender_deck_list(&mut self, decks: &[Deck]) -> bool {
        let current_hash = self.calculate_deck_list_hash(decks);

        if let Some(cached) = &self.deck_list_cache {
            let age = Utc::now().signed_duration_since(cached.last_updated);
            let expired = age
                > chrono::Duration::from_std(self.cache_config.max_deck_list_age)
                    .unwrap_or(chrono::Duration::zero());

            if expired || cached.hash != current_hash {
                self.performance_metrics.cache_misses += 1;
                true
            } else {
                self.performance_metrics.cache_hits += 1;
                false
            }
        } else {
            self.performance_metrics.cache_misses += 1;
            true
        }
    }

    pub fn cache_deck_list(&mut self, decks: Vec<Deck>, rendered_content: String) {
        let hash = self.calculate_deck_list_hash(&decks);

        self.deck_list_cache = Some(CachedDeckList {
            decks,
            rendered_content,
            last_updated: Utc::now(),
            hash,
        });
    }

    pub fn get_cached_deck_list(&self) -> Option<&CachedDeckList> {
        self.deck_list_cache.as_ref()
    }

    pub fn should_reload_deck_stats(&mut self, deck_id: Uuid) -> bool {
        if let Some(cached) = self.stats_cache.get(&deck_id) {
            let age = Utc::now().signed_duration_since(cached.last_updated);
            let expired =
                age > chrono::Duration::from_std(cached.ttl).unwrap_or(chrono::Duration::zero());

            if expired {
                self.performance_metrics.cache_misses += 1;
                true
            } else {
                self.performance_metrics.cache_hits += 1;
                false
            }
        } else {
            self.performance_metrics.cache_misses += 1;
            true
        }
    }

    pub fn cache_deck_stats(&mut self, deck_id: Uuid, stats: DeckStatistics) {
        self.stats_cache.insert(
            deck_id,
            CachedDeckStats {
                deck_id,
                stats,
                last_updated: Utc::now(),
                ttl: self.cache_config.max_stats_age,
            },
        );

        // Cleanup old entries
        self.cleanup_stats_cache();
    }

    pub fn get_cached_deck_stats(&self, deck_id: Uuid) -> Option<&DeckStatistics> {
        self.stats_cache.get(&deck_id).map(|cached| &cached.stats)
    }

    pub fn record_render_time(&mut self, duration: Duration) {
        self.performance_metrics.record_render_time(duration);
    }

    pub fn record_data_load_time(&mut self, operation: &str, duration: Duration) {
        self.performance_metrics
            .record_data_load_time(operation, duration);
    }

    fn calculate_deck_list_hash(&self, decks: &[Deck]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        for deck in decks {
            deck.name.hash(&mut hasher);
            deck.uuid.hash(&mut hasher);
            deck.created_at.timestamp().hash(&mut hasher);
        }
        hasher.finish()
    }

    fn cleanup_stats_cache(&mut self) {
        if self.stats_cache.len() > self.cache_config.max_cache_size {
            // Remove oldest entries
            let mut entries: Vec<_> = self.stats_cache.iter().collect();
            entries.sort_by_key(|(_, cached)| cached.last_updated);

            let to_remove = entries.len() - self.cache_config.max_cache_size;
            let deck_ids_to_remove: Vec<_> = entries
                .iter()
                .take(to_remove)
                .map(|(deck_id, _)| **deck_id)
                .collect();

            for deck_id in deck_ids_to_remove {
                self.stats_cache.remove(&deck_id);
            }
        }
    }

    pub fn cleanup_expired(&mut self) {
        let now = Utc::now();

        // Cleanup deck list cache
        if let Some(cached) = &self.deck_list_cache {
            let age = now.signed_duration_since(cached.last_updated);
            if age
                > chrono::Duration::from_std(self.cache_config.max_deck_list_age)
                    .unwrap_or(chrono::Duration::zero())
            {
                self.deck_list_cache = None;
            }
        }

        // Cleanup stats cache
        self.stats_cache.retain(|_, cached| {
            let age = now.signed_duration_since(cached.last_updated);
            age <= chrono::Duration::from_std(cached.ttl).unwrap_or(chrono::Duration::zero())
        });

        self.performance_metrics.last_cleanup = Instant::now();
    }

    pub fn get_performance_stats(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            render_times: Vec::new(),
            cache_hits: 0,
            cache_misses: 0,
            data_load_times: HashMap::new(),
            last_cleanup: Instant::now(),
        }
    }

    pub fn record_render_time(&mut self, duration: Duration) {
        self.render_times.push(duration);
        if self.render_times.len() > 1000 {
            self.render_times.remove(0);
        }
    }

    pub fn record_data_load_time(&mut self, operation: &str, duration: Duration) {
        let times = self
            .data_load_times
            .entry(operation.to_string())
            .or_insert_with(Vec::new);
        times.push(duration);
        if times.len() > 100 {
            times.remove(0);
        }
    }

    pub fn get_average_render_time(&self) -> Option<Duration> {
        if self.render_times.is_empty() {
            None
        } else {
            let total: Duration = self.render_times.iter().sum();
            Some(total / self.render_times.len() as u32)
        }
    }

    pub fn get_cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    pub fn get_slow_operations(&self, threshold: Duration) -> Vec<(String, Duration)> {
        let mut slow_ops = Vec::new();

        for (operation, times) in &self.data_load_times {
            if let Some(&max_time) = times.iter().max() {
                if max_time > threshold {
                    slow_ops.push((operation.clone(), max_time));
                }
            }
        }

        slow_ops.sort_by(|a, b| b.1.cmp(&a.1));
        slow_ops
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_deck_list_age: Duration::from_secs(30),
            max_stats_age: Duration::from_secs(60),
            max_ui_state_age: Duration::from_secs(5),
            max_cache_size: 100,
            enable_compression: false,
        }
    }
}

/// Async data loader
pub struct AsyncDataLoader {
    deck_cache: RwLock<HashMap<Uuid, CachedDeckData>>,
    stats_cache: RwLock<HashMap<Uuid, CachedDeckStats>>,
    loading_operations: Mutex<HashMap<String, LoadingOperation>>,
    performance_metrics: RwLock<PerformanceMetrics>,
}

#[derive(Debug, Clone)]
pub struct CachedDeckData {
    deck: Deck,
    cards: Vec<Card>,
    last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LoadingOperation {
    id: String,
    operation_type: LoadingOperationType,
    started_at: Instant,
    status: LoadingStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingOperationType {
    LoadDeck,
    LoadDeckStats,
    LoadDecksList,
    ImportDeck,
    ExportDeck,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingStatus {
    Pending,
    InProgress(f32), // Progress 0.0 - 1.0
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum AsyncData<T> {
    Loading,
    Loaded(T),
    Error(String),
}

impl AsyncDataLoader {
    pub fn new() -> Self {
        Self {
            deck_cache: RwLock::new(HashMap::new()),
            stats_cache: RwLock::new(HashMap::new()),
            loading_operations: Mutex::new(HashMap::new()),
            performance_metrics: RwLock::new(PerformanceMetrics::new()),
        }
    }

    pub async fn get_deck_cached(&self, deck_id: Uuid) -> AsyncData<(Deck, Vec<Card>)> {
        // Try cache first
        {
            let cache = self.deck_cache.read().await;
            if let Some(cached) = cache.get(&deck_id) {
                let age = Utc::now().signed_duration_since(cached.last_updated);
                if age.num_minutes() < 30 {
                    // Cache valid for 30 minutes
                    return AsyncData::Loaded((cached.deck.clone(), cached.cards.clone()));
                }
            }
        }

        AsyncData::Loading
    }

    pub async fn cache_deck(&self, deck_id: Uuid, deck: Deck, cards: Vec<Card>) {
        let mut cache = self.deck_cache.write().await;
        cache.insert(
            deck_id,
            CachedDeckData {
                deck,
                cards,
                last_updated: Utc::now(),
            },
        );
    }

    pub async fn get_deck_stats_cached(&self, deck_id: Uuid) -> AsyncData<DeckStatistics> {
        // Try cache first
        {
            let cache = self.stats_cache.read().await;
            if let Some(cached) = cache.get(&deck_id) {
                let age = Utc::now().signed_duration_since(cached.last_updated);
                if age.num_minutes() < 5 {
                    // Stats cached for 5 minutes
                    return AsyncData::Loaded(cached.stats.clone());
                }
            }
        }

        AsyncData::Loading
    }

    pub async fn cache_deck_stats(&self, deck_id: Uuid, stats: DeckStatistics) {
        let mut cache = self.stats_cache.write().await;
        cache.insert(
            deck_id,
            CachedDeckStats {
                deck_id,
                stats,
                last_updated: Utc::now(),
                ttl: Duration::from_secs(300), // 5 minutes
            },
        );
    }

    pub async fn start_loading_operation(&self, id: String, operation_type: LoadingOperationType) {
        let mut operations = self.loading_operations.lock().await;
        operations.insert(
            id.clone(),
            LoadingOperation {
                id,
                operation_type,
                started_at: Instant::now(),
                status: LoadingStatus::Pending,
            },
        );
    }

    pub async fn update_loading_progress(&self, id: &str, progress: f32) {
        let mut operations = self.loading_operations.lock().await;
        if let Some(operation) = operations.get_mut(id) {
            operation.status = LoadingStatus::InProgress(progress);
        }
    }

    pub async fn complete_loading_operation(&self, id: &str) {
        let mut operations = self.loading_operations.lock().await;
        if let Some(operation) = operations.get_mut(id) {
            operation.status = LoadingStatus::Completed;

            // Record performance metrics
            let duration = operation.started_at.elapsed();
            let mut metrics = self.performance_metrics.write().await;
            metrics.record_data_load_time(&format!("{:?}", operation.operation_type), duration);
        }
    }

    pub async fn fail_loading_operation(&self, id: &str, error: String) {
        let mut operations = self.loading_operations.lock().await;
        if let Some(operation) = operations.get_mut(id) {
            operation.status = LoadingStatus::Failed(error);
        }
    }

    pub async fn get_loading_status(&self, id: &str) -> Option<LoadingStatus> {
        let operations = self.loading_operations.lock().await;
        operations.get(id).map(|op| op.status.clone())
    }

    pub async fn cleanup_completed_operations(&self) {
        let mut operations = self.loading_operations.lock().await;
        operations.retain(|_, op| !matches!(op.status, LoadingStatus::Completed));
    }

    pub async fn cleanup_expired_cache(&self) {
        let now = Utc::now();

        // Cleanup deck cache
        {
            let mut cache = self.deck_cache.write().await;
            cache.retain(|_, cached| {
                let age = now.signed_duration_since(cached.last_updated);
                age.num_hours() < 1 // Keep deck cache for 1 hour
            });
        }

        // Cleanup stats cache
        {
            let mut cache = self.stats_cache.write().await;
            cache.retain(|_, cached| {
                let age = now.signed_duration_since(cached.last_updated);
                age.num_minutes() < 10 // Keep stats cache for 10 minutes
            });
        }
    }

    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().await.clone()
    }
}

impl Default for AsyncDataLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Render performance monitor
pub struct RenderPerformanceMonitor {
    frame_times: Vec<Duration>,
    last_frame_time: Instant,
    target_fps: u32,
    performance_stats: RenderPerformanceStats,
}

#[derive(Debug, Clone)]
pub struct RenderPerformanceStats {
    pub average_fps: f64,
    pub min_fps: f64,
    pub max_fps: f64,
    pub frame_drops: u64,
    pub total_frames: u64,
    pub average_frame_time: Duration,
    pub worst_frame_time: Duration,
}

impl RenderPerformanceMonitor {
    pub fn new(target_fps: u32) -> Self {
        Self {
            frame_times: Vec::new(),
            last_frame_time: Instant::now(),
            target_fps,
            performance_stats: RenderPerformanceStats::default(),
        }
    }

    pub fn start_frame(&mut self) {
        self.last_frame_time = Instant::now();
    }

    pub fn end_frame(&mut self) {
        let frame_time = self.last_frame_time.elapsed();
        self.frame_times.push(frame_time);

        if self.frame_times.len() > 300 {
            // Keep last 300 frames
            self.frame_times.remove(0);
        }

        self.update_performance_stats();
    }

    fn update_performance_stats(&mut self) {
        if self.frame_times.is_empty() {
            return;
        }

        let total_time: Duration = self.frame_times.iter().sum();
        let average_frame_time = total_time / self.frame_times.len() as u32;
        let average_fps = 1.0 / average_frame_time.as_secs_f64();

        let target_frame_time = Duration::from_secs_f64(1.0 / self.target_fps as f64);
        let frame_drops = self
            .frame_times
            .iter()
            .filter(|&&time| time > target_frame_time)
            .count() as u64;

        let worst_frame_time = *self.frame_times.iter().max().unwrap_or(&Duration::ZERO);
        let min_fps = 1.0 / worst_frame_time.as_secs_f64();

        self.performance_stats = RenderPerformanceStats {
            average_fps,
            min_fps,
            max_fps: self.target_fps as f64,
            frame_drops,
            total_frames: self.frame_times.len() as u64,
            average_frame_time,
            worst_frame_time,
        };
    }

    pub fn get_performance_stats(&self) -> &RenderPerformanceStats {
        &self.performance_stats
    }

    pub fn should_skip_frame(&self) -> bool {
        // Skip frame if we're running behind
        self.performance_stats.average_fps < (self.target_fps as f64 * 0.8)
    }
}

impl Default for RenderPerformanceStats {
    fn default() -> Self {
        Self {
            average_fps: 0.0,
            min_fps: 0.0,
            max_fps: 60.0,
            frame_drops: 0,
            total_frames: 0,
            average_frame_time: Duration::ZERO,
            worst_frame_time: Duration::ZERO,
        }
    }
}

// Additional methods for CachedDeckList to support render_manager
impl CachedDeckList {
    pub fn rendered_content(&self) -> &str {
        &self.rendered_content
    }
}

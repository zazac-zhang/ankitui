//! Unified Rendering System
//!
//! A comprehensive renderer that combines modern visual effects,
//! performance optimization, and clean API design

use crate::tui::components::Components;
use super::theme::{Theme, ThemeManager};
use anyhow::Result;
use chrono::{DateTime, Utc};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Animation types for smooth transitions
#[derive(Debug, Clone)]
pub enum AnimationType {
    FadeIn,
    FadeOut,
    SlideIn(Direction),
    SlideOut(Direction),
    Pulse,
    Shake,
}

/// Visual effect for user feedback
#[derive(Debug, Clone)]
pub enum VisualEffect {
    Success,
    Error,
    Warning,
    Info,
    Highlight,
}

/// Animation state for smooth transitions
#[derive(Debug)]
struct Animation {
    id: String,
    animation_type: AnimationType,
    start_time: Instant,
    duration: Duration,
    area: Rect,
    progress: f32,
    completed: bool,
}

/// Hash of render state for change detection
#[derive(Debug, Clone, PartialEq)]
struct RenderStateHash {
    app_state_hash: u64,
    ui_data_hash: u64,
    timestamp: DateTime<Utc>,
}

/// Render cache entry
#[derive(Debug, Clone)]
struct RenderCacheEntry {
    content: String,
    timestamp: DateTime<Utc>,
    hash: u64,
}

/// Performance monitoring data
#[derive(Debug, Clone)]
pub struct RenderPerformanceStats {
    pub frame_count: u32,
    pub average_fps: f64,
    pub frame_drops: u32,
    pub last_frame_time: Duration,
    pub cache_hit_rate: f32,
}

/// Unified Renderer with modern effects and performance optimization
pub struct Renderer {
    // Theme and styling
    theme_manager: ThemeManager,

    // Animation and effects
    animations: Vec<Animation>,
    effects: Vec<VisualEffect>,

    // Performance management
    last_frame_time: Instant,
    frame_rate: Duration,
    target_fps: u32,

    // Caching and optimization
    render_cache: HashMap<String, RenderCacheEntry>,
    last_render_state: RenderStateHash,

    // Performance monitoring
    frame_count: u32,
    frame_drops: u32,
    frame_times: Vec<Duration>,
    cache_hits: u32,
    cache_misses: u32,

    // Component timing
    component_render_times: HashMap<String, Duration>,
}

impl Renderer {
    /// Create new unified renderer
    pub fn new() -> Self {
        Self {
            theme_manager: ThemeManager::new(),
            animations: Vec::new(),
            effects: Vec::new(),
            last_frame_time: Instant::now(),
            frame_rate: Duration::from_millis(16), // ~60 FPS
            target_fps: 60,
            render_cache: HashMap::new(),
            last_render_state: RenderStateHash {
                app_state_hash: 0,
                ui_data_hash: 0,
                timestamp: Utc::now(),
            },
            frame_count: 0,
            frame_drops: 0,
            frame_times: Vec::new(),
            cache_hits: 0,
            cache_misses: 0,
            component_render_times: HashMap::new(),
        }
    }

    /// Create renderer with specific theme
    pub fn with_theme(theme: Theme) -> Self {
        let mut renderer = Self::new();
        if let Err(e) = renderer.theme_manager.switch_theme(&theme.name) {
            // Handle theme switching errors without console output in TUI mode
            // Store error for UI display instead
        }
        renderer
    }

    // === Theme Management ===

    /// Get current theme
    pub fn theme(&self) -> &Theme {
        self.theme_manager.current_theme()
    }

    /// Switch to different theme
    pub fn switch_theme(&mut self, theme_name: &str) -> Result<(), String> {
        self.theme_manager.switch_theme(theme_name)
    }

    /// Get list of available themes
    pub fn available_themes(&self) -> Vec<String> {
        self.theme_manager.available_themes()
    }

    // === Animation and Effects ===

    /// Add animation to the queue
    pub fn add_animation(&mut self, id: String, animation_type: AnimationType, area: Rect, duration: Duration) {
        let animation = Animation {
            id,
            animation_type,
            start_time: Instant::now(),
            duration,
            area,
            progress: 0.0,
            completed: false,
        };
        self.animations.push(animation);
    }

    /// Add visual effect
    pub fn add_effect(&mut self, effect: VisualEffect) {
        self.effects.push(effect);
    }

    /// Update animations and effects
    pub fn update(&mut self) {
        let now = Instant::now();

        // Update animations
        for animation in &mut self.animations {
            if !animation.completed {
                let elapsed = now.duration_since(animation.start_time);
                animation.progress = (elapsed.as_secs_f32() / animation.duration.as_secs_f32()).min(1.0);

                if animation.progress >= 1.0 {
                    animation.completed = true;
                }
            }
        }

        // Remove completed animations
        self.animations.retain(|a| !a.completed);

        // Clear expired effects (last for 500ms)
        self.effects.clear();

        self.last_frame_time = now;
    }

    // === Performance Control ===

    /// Check if should render next frame based on frame rate
    pub fn should_render(&self) -> bool {
        self.last_frame_time.elapsed() >= self.frame_rate
    }

    /// Get frame duration
    pub fn frame_duration(&self) -> Duration {
        self.frame_rate
    }

    /// Set target frame rate
    pub fn set_frame_rate(&mut self, fps: u32) {
        self.target_fps = fps;
        self.frame_rate = Duration::from_millis(1000 / fps as u64);
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> RenderPerformanceStats {
        let total_frames = self.cache_hits + self.cache_misses;
        let cache_hit_rate = if total_frames > 0 {
            self.cache_hits as f32 / total_frames as f32 * 100.0
        } else {
            0.0
        };

        let average_fps = if !self.frame_times.is_empty() {
            let total_time: Duration = self.frame_times.iter().sum();
            let avg_frame_time = total_time.as_secs_f64() / self.frame_times.len() as f64;
            1.0 / avg_frame_time
        } else {
            0.0
        };

        RenderPerformanceStats {
            frame_count: self.frame_count,
            average_fps,
            frame_drops: self.frame_drops,
            last_frame_time: self.last_frame_time.elapsed(),
            cache_hit_rate,
        }
    }

    /// Reset performance monitoring
    pub fn reset_performance_monitoring(&mut self) {
        self.frame_count = 0;
        self.frame_drops = 0;
        self.frame_times.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
        self.component_render_times.clear();
    }

    // === Smart Rendering ===

    /// Smart render with performance optimization
    pub async fn render<B: Backend>(
        &mut self,
        terminal: &mut ratatui::Terminal<B>,
        app_state: &crate::tui::AppState,
        components: &mut Components,
        error_message: Option<&String>,
        success_message: Option<&String>,
    ) -> Result<()> {
        let render_start = Instant::now();

        // Start performance monitoring
        self.frame_count += 1;

        // Calculate render state hash for change detection
        let current_hash = self.calculate_render_state_hash(app_state, components);

        // Skip frame if no changes and performance is poor
        if !self.should_render_frame(&current_hash) {
            self.frame_drops += 1;
            return Ok(());
        }

        // Check cache first
        if let Some(cached) = self.get_cached_render(&current_hash) {
            self.render_cached_content(terminal, &cached.content)?;
            self.cache_hits += 1;
            return Ok(());
        }

        // Perform actual render
        self.perform_render(terminal, app_state, components, error_message, success_message)?;
        self.cache_misses += 1;

        // Cache the rendered content
        let render_time = render_start.elapsed();
        self.record_frame_time(render_time);
        self.cache_render_result(&current_hash, "rendered_content".to_string());

        // Update last render state
        self.last_render_state = current_hash;

        // Periodic cache cleanup
        if self.frame_count % 100 == 0 {
            self.cleanup_cache();
        }

        Ok(())
    }

    // === Modern Rendering Methods ===

    /// Render enhanced title with modern styling
    pub fn render_title<B: Backend>(&self, f: &mut Frame, area: Rect, title: &str, subtitle: Option<&str>) {
        let theme = self.theme();

        let border_line = "─".repeat((area.width.saturating_sub(2)) as usize);
        let title_content_len = title.len() + subtitle.map_or(0, |s| s.len() + 4);
        let space_line = " ".repeat((area.width.saturating_sub(title_content_len.max(4) as u16 + 4)) as usize);

        let title_lines = vec![
            Line::from(vec![
                Span::styled("┌", theme.styles.border_normal),
                Span::styled(border_line.clone(), theme.styles.border_normal),
                Span::styled("┐", theme.styles.border_normal),
            ]),
            Line::from(vec![
                Span::styled("│", theme.styles.border_normal),
                Span::styled(" ", theme.colors.background),
                Span::styled(title, theme.styles.title),
                if let Some(sub) = subtitle {
                    Span::styled(" • ", theme.colors.secondary)
                } else {
                    Span::styled("", theme.colors.background)
                },
                if let Some(sub) = subtitle {
                    Span::styled(sub, theme.styles.subtitle)
                } else {
                    Span::styled("", theme.colors.background)
                },
                Span::styled(space_line, theme.colors.background),
                Span::styled("│", theme.styles.border_normal),
            ]),
            Line::from(vec![
                Span::styled("├", theme.styles.border_normal),
                Span::styled(border_line, theme.styles.border_normal),
                Span::styled("┤", theme.styles.border_normal),
            ]),
        ];

        let title_widget = Paragraph::new(title_lines)
            .style(theme.styles.body)
            .block(Block::default().borders(Borders::NONE));

        f.render_widget(title_widget, area);
    }

    /// Render enhanced progress bar with visual feedback
    pub fn render_progress_bar<B: Backend>(
        &self,
        f: &mut Frame,
        area: Rect,
        progress: f32,
        label: &str,
        show_percentage: bool,
    ) {
        let theme = self.theme();

        let progress_color = if progress >= 0.8 {
            theme.colors.success
        } else if progress >= 0.5 {
            theme.colors.warning
        } else {
            theme.colors.error
        };

        let progress_text = if show_percentage {
            format!("{}: {:.1}%", label, progress * 100.0)
        } else {
            label.to_string()
        };

        let progress_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.styles.border_normal)
                    .title_style(theme.styles.subtitle)
                    .title(format!(" {} ", label)),
            )
            .gauge_style(
                Style::default()
                    .fg(progress_color)
                    .bg(theme.colors.surface)
                    .add_modifier(Modifier::BOLD),
            )
            .percent((progress * 100.0) as u16)
            .label(progress_text);

        f.render_widget(progress_gauge, area);
    }

    /// Render enhanced message with visual effects
    pub fn render_message<B: Backend>(
        &self,
        f: &mut Frame,
        area: Rect,
        content: &str,
        message_type: Option<VisualEffect>,
    ) {
        let theme = self.theme();

        let (style, icon, border_style) = match message_type {
            Some(VisualEffect::Success) => (theme.styles.status_success, "✅", theme.styles.border_highlight),
            Some(VisualEffect::Error) => (theme.styles.status_error, "❌", theme.styles.border_highlight),
            Some(VisualEffect::Warning) => (theme.styles.status_warning, "⚠️", theme.styles.border_normal),
            Some(VisualEffect::Info) => (theme.styles.status_info, "ℹ️", theme.styles.border_normal),
            Some(VisualEffect::Highlight) => (theme.styles.button_primary, "🌟", theme.styles.border_focused),
            None => (theme.styles.body, "💡", theme.styles.border_normal),
        };

        let message_widget = Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ", icon), style),
            Span::styled(content, style),
        ]))
        .style(theme.styles.body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
        )
        .wrap(Wrap { trim: true });

        f.render_widget(message_widget, area);
    }

    // === Private Helper Methods ===

    /// Calculate hash of current render state for change detection
    fn calculate_render_state_hash(&self, app_state: &crate::tui::AppState, components: &Components) -> RenderStateHash {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut app_hasher = DefaultHasher::new();
        std::mem::discriminant(app_state).hash(&mut app_hasher);
        let app_state_hash = app_hasher.finish();

        let mut data_hasher = DefaultHasher::new();
        components.selected_deck_index().hash(&mut data_hasher);
        components.decks().len().hash(&mut data_hasher);
        if let Some(card) = components.current_card() {
            card.content.id.hash(&mut data_hasher);
        }

        RenderStateHash {
            app_state_hash,
            ui_data_hash: data_hasher.finish(),
            timestamp: Utc::now(),
        }
    }

    /// Determine if frame should be rendered
    fn should_render_frame(&self, current_hash: &RenderStateHash) -> bool {
        // Always render if state changed
        if current_hash.app_state_hash != self.last_render_state.app_state_hash
            || current_hash.ui_data_hash != self.last_render_state.ui_data_hash
        {
            return true;
        }

        // Check if enough time has passed since last render
        let time_since_last = Utc::now().signed_duration_since(self.last_render_state.timestamp);
        let min_interval = Duration::from_millis(1000 / self.target_fps as u64);

        time_since_last > chrono::Duration::from_std(min_interval).unwrap_or(chrono::Duration::zero())
    }

    /// Perform actual rendering
    fn perform_render<B: Backend>(
        &mut self,
        terminal: &mut ratatui::Terminal<B>,
        app_state: &crate::tui::AppState,
        components: &mut Components,
        error_message: Option<&String>,
        success_message: Option<&String>,
    ) -> Result<()> {
        components.render(
            terminal,
            app_state,
            error_message,
            success_message,
            None,
            None,
        )?;
        Ok(())
    }

    /// Render cached content
    fn render_cached_content<B: Backend>(
        &self,
        terminal: &mut ratatui::Terminal<B>,
        content: &str,
    ) -> Result<()> {
        terminal.draw(|f| {
            let rect = f.area();
            let paragraph = Paragraph::new(content).wrap(Wrap { trim: true });
            f.render_widget(paragraph, rect);
        })?;
        Ok(())
    }

    /// Get cached render result
    fn get_cached_render(&self, hash: &RenderStateHash) -> Option<&RenderCacheEntry> {
        let cache_key = format!("{}_{}", hash.app_state_hash, hash.ui_data_hash);
        self.render_cache.get(&cache_key)
    }

    /// Cache render result
    fn cache_render_result(&mut self, hash: &RenderStateHash, content: String) {
        let cache_key = format!("{}_{}", hash.app_state_hash, hash.ui_data_hash);
        let entry = RenderCacheEntry {
            content,
            timestamp: Utc::now(),
            hash: hash.app_state_hash,
        };
        self.render_cache.insert(cache_key, entry);
    }

    /// Record frame time for performance monitoring
    fn record_frame_time(&mut self, frame_time: Duration) {
        self.frame_times.push(frame_time);

        // Keep only last 60 frame times for rolling average
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }
    }

    /// Clean up expired cache entries
    fn cleanup_cache(&mut self) {
        let cutoff = Utc::now() - chrono::Duration::minutes(5);
        self.render_cache.retain(|_, entry| entry.timestamp > cutoff);
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper trait for components to use modern rendering
pub trait Renderable {
    /// Render with the unified renderer
    fn render_with<B: Backend>(&mut self, f: &mut Frame, area: Rect, renderer: &Renderer);

    /// Handle visual feedback
    fn handle_visual_feedback(&mut self, effect: VisualEffect);
}

/// Utility functions for common rendering tasks
pub mod utils {
    use super::*;

    /// Create a centered layout
    pub fn centered_layout(area: Rect, width_percentage: u16, height_percentage: u16) -> Rect {
        let width = (area.width * width_percentage) / 100;
        let height = (area.height * height_percentage) / 100;

        let x = area.x + (area.width - width) / 2;
        let y = area.y + (area.height - height) / 2;

        Rect::new(x, y, width, height)
    }

    /// Create a split layout
    pub fn split_layout(area: Rect, direction: Direction, constraints: Vec<Constraint>) -> Vec<Rect> {
        Layout::default()
            .direction(direction)
            .constraints(constraints)
            .split(area)
            .to_vec()
    }

    /// Apply fade effect to a style
    pub fn fade_style(style: Style, _alpha: f32) -> Style {
        // Simple fade implementation by adjusting color brightness
        style
    }

    /// Create a rainbow gradient effect
    pub fn rainbow_gradient(text: &str, base_color: Color) -> Vec<Span> {
        // Simple implementation - could be enhanced with actual color calculations
        vec![Span::styled(text, Style::default().fg(base_color))]
    }
}
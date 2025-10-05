//! Rendering Manager
//!
//! Advanced rendering system integration with autonomous components

use super::app_context::AppContext;
use super::component_registry::ComponentRegistry;
use super::event_bus::AppEvent;
use super::state_manager::RenderContext;
use crate::tui::app::AppState;
use crate::tui::rendering::{
    renderer::{Renderer, RenderPerformanceStats},
    interaction_feedback::{InteractionFeedback, InteractionType},
    layout::LayoutPresets,
};
use anyhow::Result;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;

/// Rendering manager for advanced TUI rendering
pub struct RenderingManager {
    /// Advanced renderer
    renderer: Renderer,
    /// Interaction feedback system
    feedback_system: InteractionFeedback,
    /// Render performance stats
    performance_stats: Arc<Mutex<RenderPerformanceStats>>,
    /// Cache for render contexts
    context_cache: HashMap<String, RenderContext>,
}

impl RenderingManager {
    /// Create new rendering manager
    pub fn new() -> Result<Self> {
        let renderer = Renderer::new();
        let feedback_system = InteractionFeedback::new(renderer.theme().clone());
        let performance_stats = Arc::new(Mutex::new(RenderPerformanceStats {
            frame_count: 0,
            average_fps: 0.0,
            frame_drops: 0,
            last_frame_time: Duration::from_millis(0),
            cache_hit_rate: 0.0,
        }));
        let context_cache = HashMap::new();

        Ok(Self {
            renderer,
            feedback_system,
            performance_stats,
            context_cache,
        })
    }

    /// Render the entire application
    pub fn render<B: Backend>(
        &mut self,
        frame: &mut Frame,
        app_state: &AppState,
        component_registry: &mut ComponentRegistry,
        app_context: &AppContext,
    ) -> Result<()> {
        let start_time = std::time::Instant::now();

        // Create render context
        let render_context = self.create_render_context(frame.area(), app_context, app_state);

        // Set current component based on state
        component_registry.set_current_component(app_state.clone());

        // Get layout areas for current state
        let layout_areas = self.get_layout_areas(frame.area(), app_state);

        // Render main content area
        if let Some(main_area) = layout_areas.get("main") {
            self.render_main_content::<B>(
                frame,
                *main_area,
                app_state,
                component_registry,
                &render_context,
            )?;
        }

        // Render header if configured
        if let Some(header_area) = layout_areas.get("header") {
            self.render_header::<B>(frame, *header_area, app_state)?;
        }

        // Render footer if configured
        if let Some(footer_area) = layout_areas.get("footer") {
            self.render_footer::<B>(frame, *footer_area, app_state)?;
        }

        // Render interaction feedback (if any)
        // Note: This depends on the InteractionFeedback API

        // Update performance stats
        {
            let mut stats = self.performance_stats.lock().unwrap();
            stats.frame_count += 1;
            stats.last_frame_time = start_time.elapsed();
            // Simple FPS calculation
            if stats.frame_count > 1 {
                stats.average_fps = (stats.average_fps * (stats.frame_count - 1) as f64 + 1_000_000.0 / stats.last_frame_time.as_micros() as f64) / stats.frame_count as f64;
            }
        }

        Ok(())
    }

    /// Create render context
    fn create_render_context(&self, area: Rect, _app_context: &AppContext, current_state: &AppState) -> RenderContext {
        let mut data = HashMap::new();
        data.insert("area_width".to_string(), area.width.to_string());
        data.insert("area_height".to_string(), area.height.to_string());

        RenderContext {
            state: current_state.clone(),
            data,
            focused: true,
        }
    }

    /// Get layout areas for app state
    fn get_layout_areas(&self, area: Rect, app_state: &AppState) -> HashMap<String, Rect> {
        let mut areas = HashMap::new();

        match app_state {
            AppState::MainMenu | AppState::DeckSelection | AppState::DeckManagement |
            AppState::Statistics | AppState::Settings | AppState::Help => {
                // Layout with header, main content, and footer
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),  // Header
                        Constraint::Min(0),  // Main content
                        Constraint::Length(1),  // Footer
                    ])
                    .split(area);

                areas.insert("header".to_string(), chunks[0]);
                areas.insert("main".to_string(), chunks[1]);
                areas.insert("footer".to_string(), chunks[2]);
            }
            AppState::Learning | AppState::CardReview => {
                // Learning mode - full content area with minimal UI
                areas.insert("main".to_string(), area);
            }
            AppState::ConfirmExit => {
                // Dialog layout - centered confirmation
                let dialog_area = LayoutPresets::centered_dialog(area, 60, 30);
                areas.insert("main".to_string(), dialog_area);
            }
        }

        areas
    }

    /// Render main content area
    fn render_main_content<B: Backend>(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        component_registry: &mut ComponentRegistry,
        render_context: &RenderContext,
    ) -> Result<()> {
        // Use the component registry to render the current component
        let result = component_registry.with_current_component(|component| {
            // Create a new render context with the specific area
            let mut component_data = render_context.data.clone();
            component_data.insert("area_width".to_string(), area.width.to_string());
            component_data.insert("area_height".to_string(), area.height.to_string());

            let component_context = RenderContext {
                state: render_context.state.clone(),
                data: component_data,
                focused: render_context.focused,
            };

            component.render(frame, component_context)
        });

        // If no current component, render placeholder
        if result.is_none() {
            self.render_placeholder::<B>(frame, area, app_state)?;
        }

        Ok(())
    }

    /// Render header
    fn render_header<B: Backend>(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let theme = self.renderer.theme();

        let header_text = vec![
            Line::from(vec![
                Span::styled("AnkiTUI", Style::default().fg(theme.colors.primary).add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::styled(format!("{}", app_state), Style::default().fg(theme.colors.secondary)),
            ])
        ];

        let header = Paragraph::new(header_text)
            .style(Style::default().bg(theme.colors.background))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(theme.colors.border))
            );

        frame.render_widget(header, area);
        Ok(())
    }

    /// Render footer
    fn render_footer<B: Backend>(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let theme = self.renderer.theme();

        let help_text = self.get_help_text_for_state(app_state);

        let footer_text = vec![
            Line::from(vec![
                Span::styled(help_text, Style::default().fg(theme.colors.secondary)),
            ])
        ];

        let footer = Paragraph::new(footer_text)
            .style(Style::default().bg(theme.colors.background))
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(theme.colors.border))
            );

        frame.render_widget(footer, area);
        Ok(())
    }

    /// Render status bar
    fn render_status_bar<B: Backend>(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let theme = self.renderer.theme();
        let stats = self.performance_stats.lock().unwrap();
        let status_text = format!("FPS: {:.1} | Frames: {}", stats.average_fps, stats.frame_count);

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(theme.colors.text).bg(theme.colors.background));

        frame.render_widget(status, area);
        Ok(())
    }

    /// Render placeholder for missing components
    fn render_placeholder<B: Backend>(&mut self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let theme = self.renderer.theme();

        let placeholder_text = vec![
            Line::from(vec![
                Span::styled(format!("{} Component", app_state), Style::default().fg(theme.colors.primary).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Component not implemented yet", Style::default().fg(theme.colors.text)),
            ]),
        ];

        let placeholder = Paragraph::new(placeholder_text)
            .style(Style::default().fg(theme.colors.text).bg(theme.colors.background))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{}", app_state))
                    .border_style(Style::default().fg(theme.colors.border))
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(placeholder, area);
        Ok(())
    }

    /// Get help text for current state
    fn get_help_text_for_state(&self, app_state: &AppState) -> &'static str {
        match app_state {
            AppState::MainMenu => "↑↓ Navigate | Enter Select | q Quit | ? Help",
            AppState::DeckSelection => "↑↓ Navigate | Enter Select | Back/Escape Cancel | ? Help",
            AppState::DeckManagement => "↑↓ Navigate | Enter Select | Back/Escape Cancel | ? Help",
            AppState::Learning => "1-4 Rate Card | Space Show Answer | W Switch Deck | Back/Escape Exit | ? Help",
            AppState::CardReview => "1-4 Rate Card | Space Show Answer | W Switch Deck | Back/Escape Exit | ? Help",
            AppState::Statistics => "↑↓ Navigate | Enter Select | Back/Escape Cancel | ? Help",
            AppState::Settings => "↑↓ Navigate | Enter Toggle | Back/Escape Cancel | ? Help",
            AppState::Help => "↑↓ Scroll | Back/Escape Close | q Quit",
            AppState::ConfirmExit => "Enter Confirm | Back/Escape Cancel",
        }
    }

    /// Show interaction feedback
    pub fn show_feedback(&mut self, feedback_type: InteractionType, message: &str) {
        self.feedback_system.add_feedback(message.to_string(), feedback_type);
    }

    /// Handle app events for rendering
    pub fn handle_event(&mut self, event: &AppEvent) -> Result<()> {
        match event {
            AppEvent::ThemeChanged { theme } => {
                self.renderer.switch_theme(theme).map_err(|e| anyhow::anyhow!("Theme switch error: {}", e))?;
            }
            AppEvent::Success { message } => {
                self.show_feedback(InteractionType::Success, message);
            }
            AppEvent::ErrorOccurred { error, context } => {
                self.show_feedback(InteractionType::Error, &format!("{}: {}", context, error));
            }
            _ => {}
        }
        Ok(())
    }

    /// Get render metrics
    pub fn get_metrics(&self) -> RenderPerformanceStats {
        self.performance_stats.lock().unwrap().clone()
    }

    /// Clear render cache
    pub fn clear_cache(&mut self) -> Result<()> {
        // Clear the context cache
        self.context_cache.clear();
        Ok(())
    }

    /// Get renderer reference
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    /// Get renderer mutably
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}

impl Default for RenderingManager {
    fn default() -> Self {
        Self::new().expect("Failed to create RenderingManager")
    }
}
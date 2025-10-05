//! Component Registry
//!
//! Centralized component management with clean interfaces

use super::event_handler::Action;
use super::state_manager::RenderContext;
use super::app_context::{AppContext, ComponentResult};
use crate::tui::core::AppEvent;
use anyhow::Result;

/// Legacy component trait - for backward compatibility
pub trait UIComponent: Send + Sync {
    /// Render the component
    fn render(
        &mut self,
        frame: &mut ratatui::Frame,
        context: RenderContext,
    ) -> Result<()>;

    /// Handle user action
    fn handle_action(&mut self, action: Action) -> Result<Option<crate::tui::app::AppState>>;

    /// Update component state
    fn update(&mut self) -> Result<()>;

    /// Component name for identification
    fn name(&self) -> &str;
}

/// Enhanced component trait for autonomous state management
pub trait AutonomousComponent: UIComponent {
    /// Handle user action with autonomous state management
    fn handle_action_enhanced(&mut self, action: Action, context: &AppContext) -> Result<ComponentResult>;

    /// Handle application events
    fn handle_event(&mut self, event: &AppEvent, context: &AppContext) -> Result<ComponentResult>;

    /// Update component state with context
    fn update_with_context(&mut self, context: &AppContext) -> Result<()>;

    /// Check if component can handle the current app state
    fn can_handle_state(&self, state: crate::tui::app::AppState) -> bool;

    /// Initialize component with context
    fn initialize(&mut self, context: &AppContext) -> Result<()> {
        // Default implementation - can be overridden
        Ok(())
    }

    /// Cleanup component resources
    fn cleanup(&mut self) -> Result<()> {
        // Default implementation - can be overridden
        Ok(())
    }
}

/// Component registry for managing all UI components
pub struct ComponentRegistry {
    components: std::collections::HashMap<String, Box<dyn UIComponent>>,
    current_component: Option<String>,
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        let mut registry = Self {
            components: std::collections::HashMap::new(),
            current_component: None,
        };

        // Initialize all components
        registry.initialize_components();
        registry
    }

    /// Initialize all UI components
    fn initialize_components(&mut self) {
        // Import and register modern components

        // Main menu component
        self.register_component(
            "main_menu",
            Box::new(crate::tui::components::menu::Menu::new()),
        );

        // Deck selection component
        self.register_component(
            "deck_selector",
            Box::new(crate::tui::components::deck::DeckSelector::new()),
        );

        // Learning component
        self.register_component(
            "learning",
            Box::new(crate::tui::components::study::Study::new()),
        );

        // Statistics component
        self.register_component(
            "statistics",
            Box::new(crate::tui::components::stats::Stats::new()),
        );

        // Settings component with real functionality
        self.register_component(
            "settings",
            Box::new(crate::tui::components::settings::Settings::new()),
        );

        // Help component with comprehensive documentation
        self.register_component("help", Box::new(crate::tui::components::help::Help::new()));
    }

    /// Register a component
    pub fn register_component(&mut self, name: &str, component: Box<dyn UIComponent>) {
        self.components.insert(name.to_string(), component);
    }

    /// Get component name for current state
    pub fn get_component_name_for_state(&self, state: crate::tui::app::AppState) -> &'static str {
        match state {
            crate::tui::app::AppState::MainMenu => "main_menu",
            crate::tui::app::AppState::DeckSelection => "deck_selector",
            crate::tui::app::AppState::DeckManagement => "deck_selector", // Use deck selector for management
            crate::tui::app::AppState::Learning => "learning",
            crate::tui::app::AppState::CardReview => "learning", // Use learning component for card review
            crate::tui::app::AppState::Statistics => "statistics",
            crate::tui::app::AppState::Settings => "settings",
            crate::tui::app::AppState::Help => "help",
            crate::tui::app::AppState::ConfirmExit => "main_menu", // ConfirmExit uses main menu for confirmation
        }
    }

    /// Set current component by state
    pub fn set_current_component(&mut self, state: crate::tui::app::AppState) {
        let component_name = self.get_component_name_for_state(state);
        self.current_component = Some(component_name.to_string());
    }

    /// Get current active component name
    pub fn get_current_component_name(&self) -> Option<&String> {
        self.current_component.as_ref()
    }

    /// Update all components
    pub fn update_all(&mut self) -> Result<()> {
        for component in self.components.values_mut() {
            component.update()?;
        }
        Ok(())
    }

    /// Execute operation on current component
    pub fn with_current_component<F, R>(&mut self, mut f: F) -> Option<R>
    where
        F: FnMut(&mut dyn UIComponent) -> R,
    {
        if let Some(ref name) = self.current_component {
            self.components.get_mut(name).map(|c| f(c.as_mut()))
        } else {
            None
        }
    }

    /// Execute operation on component by name
    pub fn with_component_by_name<F, R>(&mut self, name: &str, mut f: F) -> Option<R>
    where
        F: FnMut(&mut dyn UIComponent) -> R,
    {
        self.components.get_mut(name).map(|c| f(c.as_mut()))
    }
}

/// Placeholder component for initial implementation
struct PlaceholderComponent {
    name: String,
}

impl PlaceholderComponent {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl UIComponent for PlaceholderComponent {
    fn render(
        &mut self,
        frame: &mut ratatui::Frame,
        _context: RenderContext,
    ) -> Result<()> {
        use ratatui::{
            text::{Line, Span},
            widgets::{Block, Borders, Paragraph, Wrap},
        };

        let area = frame.area();
        let text = vec![
            Line::from(Span::raw(format!("{} Component", self.name))),
            Line::from(Span::raw("Placeholder implementation")),
        ];

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.name.clone()),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
        Ok(())
    }

    fn handle_action(&mut self, _action: Action) -> Result<Option<crate::tui::app::AppState>> {
        // Placeholder implementation - no state changes
        Ok(None)
    }

    fn update(&mut self) -> Result<()> {
        // Placeholder implementation - nothing to update
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

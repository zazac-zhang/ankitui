//! Base component traits and structures

use crate::utils::error::{TuiError, TuiResult};
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Component trait for all UI components
pub trait Component: Send + Sync {
    /// Render the component
    fn render(&self, f: &mut Frame, area: Rect, focused: bool);

    /// Handle user input
    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool>;

    /// Update component state
    fn update(&mut self) -> TuiResult<()>;

    /// Check if component can receive focus
    fn can_focus(&self) -> bool {
        false
    }

    /// Get component identifier
    fn id(&self) -> &str;

    /// Get component state
    fn state(&self) -> &ComponentState;

    /// Get mutable component state
    fn state_mut(&mut self) -> &mut ComponentState;

    /// Mark component as dirty (needs redraw)
    fn mark_dirty(&mut self) {
        self.state_mut().mark_dirty();
    }

    /// Check if component needs redraw
    fn is_dirty(&self) -> bool {
        self.state().dirty
    }

    /// Clear dirty flag
    fn clear_dirty(&mut self) {
        self.state_mut().mark_clean();
    }
}

/// Component state management
#[derive(Debug, Clone)]
pub struct ComponentState {
    pub focused: bool,
    pub visible: bool,
    pub enabled: bool,
    pub dirty: bool,
    pub bounds: Option<Rect>,
    pub z_index: u32,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self {
            focused: false,
            visible: true,
            enabled: true,
            dirty: true,
            bounds: None,
            z_index: 0,
        }
    }
}

impl ComponentState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_focus(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_bounds(mut self, bounds: Rect) -> Self {
        self.bounds = Some(bounds);
        self
    }

    pub fn with_z_index(mut self, z_index: u32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        if self.bounds != Some(bounds) {
            self.bounds = Some(bounds);
            self.mark_dirty();
        }
    }

    pub fn get_bounds(&self) -> Option<Rect> {
        self.bounds
    }

    pub fn contains_point(&self, x: u16, y: u16) -> bool {
        if let Some(bounds) = self.bounds {
            x >= bounds.x && x < bounds.x + bounds.width && y >= bounds.y && y < bounds.y + bounds.height
        } else {
            false
        }
    }
}

/// Interactive component trait
pub trait InteractiveComponent: Component {
    /// Handle keyboard events
    fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> TuiResult<bool>;

    /// Handle mouse events
    fn handle_mouse_event(&mut self, event: crossterm::event::MouseEvent) -> TuiResult<bool>;

    /// Get action bindings
    fn get_keybindings(&self) -> Vec<(crossterm::event::KeyEvent, String)>;

    /// Handle focus events
    fn on_focus_gained(&mut self) -> TuiResult<()> {
        self.state_mut().focused = true;
        self.mark_dirty();
        Ok(())
    }

    /// Handle blur events
    fn on_focus_lost(&mut self) -> TuiResult<()> {
        self.state_mut().focused = false;
        self.mark_dirty();
        Ok(())
    }
}

/// Container component trait
pub trait ContainerComponent: Component {
    /// Add a child component
    fn add_child(&mut self, component: Box<dyn Component>);

    /// Remove a child component by ID
    fn remove_child(&mut self, id: &str) -> Option<Box<dyn Component>>;

    /// Get child component by ID
    fn get_child(&self, id: &str) -> Option<&dyn Component>;

    /// Get mutable child component by ID
    fn get_child_mut(&mut self, id: &str) -> Option<&mut dyn Component>;

    /// Get all child IDs
    fn child_ids(&self) -> Vec<&str>;

    /// Get children in rendering order
    fn children(&self) -> Vec<&dyn Component>;

    /// Get mutable children in rendering order
    fn children_mut(&mut self) -> Vec<&mut dyn Component>;

    /// Layout children within the container
    fn layout_children(&mut self, area: Rect) -> TuiResult<()>;
}

/// Focusable component trait
pub trait FocusableComponent: Component {
    /// Focus the component
    fn focus(&mut self) -> TuiResult<()>;

    /// Unfocus the component
    fn unfocus(&mut self) -> TuiResult<()>;

    /// Check if component is focused
    fn is_focused(&self) -> bool;

    /// Move to next focusable element
    fn focus_next(&mut self) -> bool;

    /// Move to previous focusable element
    fn focus_previous(&mut self) -> bool;

    /// Get focus order hint (lower numbers get focus first)
    fn focus_order(&self) -> u32 {
        0
    }
}

/// Event handling result
#[derive(Debug, Clone)]
pub enum EventResult {
    /// Event was handled
    Handled,
    /// Event was not handled
    NotHandled,
    /// Event was handled and component needs redraw
    NeedsRedraw,
    /// Event was handled and navigation should occur
    Navigation(Direction),
}

/// Navigation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Next,
    Previous,
    First,
    Last,
}

/// Component registry for managing component instances
pub struct ComponentRegistry {
    components: HashMap<String, Arc<dyn Component>>,
    focus_order: Vec<String>,
}

impl std::fmt::Debug for ComponentRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentRegistry")
            .field("component_count", &self.components.len())
            .field("focus_order", &self.focus_order)
            .finish()
    }
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            focus_order: Vec::new(),
        }
    }

    pub fn register<C: Component + 'static>(&mut self, component: C) {
        let id = component.id().to_string();
        let component = Arc::new(component);
        self.components.insert(id.clone(), component);

        // Update focus order if component can focus
        if self.components[&id].can_focus() {
            if !self.focus_order.contains(&id) {
                self.focus_order.push(id);
            }
        }
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn Component>> {
        self.components.get(id).cloned()
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<dyn Component>> {
        let component = self.components.remove(id)?;
        self.focus_order.retain(|focus_id| focus_id != id);
        Some(component)
    }

    pub fn get_focused(&self) -> Option<Arc<dyn Component>> {
        for component in self.components.values() {
            if component.state().focused {
                return Some(Arc::clone(component));
            }
        }
        None
    }

    pub fn focus_next(&self) -> Option<String> {
        if self.focus_order.is_empty() {
            return None;
        }

        let current_focused = self.get_focused().map(|c| c.id().to_string());

        if let Some(current_id) = current_focused {
            if let Some(current_index) = self.focus_order.iter().position(|id| id == &current_id) {
                let next_index = (current_index + 1) % self.focus_order.len();
                return Some(self.focus_order[next_index].clone());
            }
        }

        // No current focus, focus first focusable component
        self.focus_order.first().cloned()
    }

    pub fn focus_previous(&self) -> Option<String> {
        if self.focus_order.is_empty() {
            return None;
        }

        let current_focused = self.get_focused().map(|c| c.id().to_string());

        if let Some(current_id) = current_focused {
            if let Some(current_index) = self.focus_order.iter().position(|id| id == &current_id) {
                let prev_index = if current_index == 0 {
                    self.focus_order.len() - 1
                } else {
                    current_index - 1
                };
                return Some(self.focus_order[prev_index].clone());
            }
        }

        // No current focus, focus last focusable component
        self.focus_order.last().cloned()
    }

    pub fn get_all(&self) -> Vec<Arc<dyn Component>> {
        self.components.values().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.components.clear();
        self.focus_order.clear();
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

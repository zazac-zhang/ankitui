//! Layout components for arranging UI elements

use crate::ui::components::base::{Component, ComponentState, ContainerComponent};
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction as LayoutDirection, Rect},
    widgets::Block,
    Frame,
};

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

/// Container for arranging components in a specific layout
pub struct Container {
    id: String,
    state: ComponentState,
    direction: Direction,
    constraints: Vec<Constraint>,
    children: Vec<Box<dyn Component>>,
    focused_child_index: Option<usize>,
}

impl Container {
    pub fn new(id: String, direction: Direction) -> Self {
        Self {
            id,
            state: ComponentState::new(),
            direction,
            constraints: Vec::new(),
            children: Vec::new(),
            focused_child_index: None,
        }
    }

    pub fn with_constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.mark_dirty();
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    pub fn set_constraints(&mut self, constraints: Vec<Constraint>) {
        self.constraints = constraints;
        self.mark_dirty();
    }

    pub fn add_child<C: Component + 'static>(&mut self, child: C) {
        self.children.push(Box::new(child));
        self.mark_dirty();
    }

    pub fn remove_child(&mut self, index: usize) -> Option<Box<dyn Component>> {
        if index >= self.children.len() {
            return None;
        }
        let child = self.children.remove(index);
        if self.focused_child_index.is_some()
            && self.focused_child_index.unwrap() >= self.children.len()
        {
            self.focused_child_index = None;
        }
        self.mark_dirty();
        Some(child)
    }

    pub fn children(&self) -> &[Box<dyn Component>] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.children
    }

    fn calculate_layout(&self, area: Rect) -> Vec<Rect> {
        if self.children.is_empty() {
            return Vec::new();
        }

        let ratatui_direction = match self.direction {
            Direction::Horizontal => LayoutDirection::Horizontal,
            Direction::Vertical => LayoutDirection::Vertical,
        };

        let constraints = if self.constraints.len() == self.children.len() {
            self.constraints.clone()
        } else {
            // Default constraints if not provided
            std::iter::repeat(Constraint::Percentage(100 / self.children.len() as u16))
                .take(self.children.len())
                .collect()
        };

        ratatui::layout::Layout::default()
            .direction(ratatui_direction)
            .constraints(constraints)
            .split(area)
            .to_vec()
    }
}

impl Component for Container {
    fn render(&self, f: &mut Frame, area: Rect, focused: bool) {
        if !self.state.visible {
            return;
        }

        // Calculate layout for children
        let child_areas = self.calculate_layout(area);

        // Render each child
        for (i, child) in self.children.iter().enumerate() {
            if let Some(&child_area) = child_areas.get(i) {
                let child_focused = focused && self.focused_child_index == Some(i);
                child.render(f, child_area, child_focused);
            }
        }
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        if !self.state.enabled {
            return Ok(false);
        }

        // Forward input to focused child
        if let Some(focused_index) = self.focused_child_index {
            if let Some(child) = self.children.get_mut(focused_index) {
                return child.handle_input(event);
            }
        }

        Ok(false)
    }

    fn update(&mut self) -> TuiResult<()> {
        for child in &mut self.children {
            child.update()?;
        }
        Ok(())
    }

    fn can_focus(&self) -> bool {
        self.children.iter().any(|child| child.can_focus())
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn state(&self) -> &ComponentState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
}

impl ContainerComponent for Container {
    fn add_child(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
        self.mark_dirty();
    }

    fn remove_child(&mut self, id: &str) -> Option<Box<dyn Component>> {
        for (i, child) in self.children.iter().enumerate() {
            if child.id() == id {
                return Some(self.children.remove(i));
            }
        }
        None
    }

    fn get_child(&self, id: &str) -> Option<&dyn Component> {
        self.children
            .iter()
            .find(|child| child.id() == id)
            .map(|child| child.as_ref())
    }

    fn get_child_mut(&mut self, id: &str) -> Option<&mut (dyn Component + '_)> {
        for child in &mut self.children {
            if child.id() == id {
                return Some(child.as_mut());
            }
        }
        None
    }

    fn child_ids(&self) -> Vec<&str> {
        self.children.iter().map(|child| child.id()).collect()
    }

    fn children(&self) -> Vec<&dyn Component> {
        self.children.iter().map(|child| child.as_ref()).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut (dyn Component + '_)> {
        Vec::new()
    }

    fn layout_children(&mut self, area: Rect) -> TuiResult<()> {
        let child_areas = self.calculate_layout(area);

        for (i, child) in self.children.iter_mut().enumerate() {
            if let Some(&child_area) = child_areas.get(i) {
                child.state_mut().set_bounds(child_area);
            }
        }

        Ok(())
    }
}

//! Widget components for the TUI application

use crate::ui::components::base::{Component, ComponentState, InteractiveComponent};
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List as RatatuiList, ListItem, Paragraph},
    Frame,
};

// Placeholder widget types - to be implemented fully
pub struct Button {
    id: String,
    state: ComponentState,
    label: String,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Button {
    pub fn new(id: String, label: String) -> Self {
        Self {
            id,
            state: ComponentState::new(),
            label,
            on_click: None,
        }
    }

    pub fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
        self.mark_dirty();
    }

    pub fn click(&mut self) {
        if let Some(callback) = &self.on_click {
            callback();
        }
    }
}

impl Component for Button {
    fn render(&self, f: &mut Frame, area: Rect, focused: bool) {
        if !self.state.visible {
            return;
        }

        let style = if focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::REVERSED)
        } else if self.state.enabled {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default().borders(Borders::ALL).style(style);

        let paragraph = Paragraph::new(self.label.as_str())
            .block(block)
            .style(style);

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        if !self.state.enabled {
            return Ok(false);
        }

        match event {
            crossterm::event::Event::Key(key) if key.code == crossterm::event::KeyCode::Enter => {
                self.click();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn update(&mut self) -> TuiResult<()> {
        Ok(())
    }

    fn can_focus(&self) -> bool {
        self.state.enabled
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

impl InteractiveComponent for Button {
    fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> TuiResult<bool> {
        match event.code {
            crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Char(' ') => {
                self.click();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn handle_mouse_event(&mut self, event: crossterm::event::MouseEvent) -> TuiResult<bool> {
        if let Some(bounds) = self.state.bounds {
            if event.column >= bounds.x
                && event.column < bounds.x + bounds.width
                && event.row >= bounds.y
                && event.row < bounds.y + bounds.height
            {
                if event.kind
                    == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
                {
                    self.click();
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn get_keybindings(&self) -> Vec<(crossterm::event::KeyEvent, String)> {
        vec![
            (
                crossterm::event::KeyEvent::new(
                    crossterm::event::KeyCode::Enter,
                    crossterm::event::KeyModifiers::NONE,
                ),
                "Activate".to_string(),
            ),
            (
                crossterm::event::KeyEvent::new(
                    crossterm::event::KeyCode::Char(' '),
                    crossterm::event::KeyModifiers::NONE,
                ),
                "Activate".to_string(),
            ),
        ]
    }
}

// Other placeholder widget types
pub struct Input {
    id: String,
    state: ComponentState,
    placeholder: String,
    value: String,
}

impl Input {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: ComponentState::new(),
            placeholder: "Enter text...".to_string(),
            value: String::new(),
        }
    }
}

impl Component for Input {
    fn render(&self, f: &mut Frame, area: Rect, focused: bool) {
        let display_text = if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
        };

        let style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let block = Block::default().borders(Borders::ALL).style(style);

        let paragraph = Paragraph::new(display_text.as_str()).block(block).style(style);

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> {
        Ok(false)
    }

    fn update(&mut self) -> TuiResult<()> {
        Ok(())
    }

    fn can_focus(&self) -> bool {
        true
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

// Placeholder list widget
pub struct List {
    id: String,
    state: ComponentState,
    items: Vec<String>,
    selected_index: usize,
}

impl List {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: ComponentState::new(),
            items: Vec::new(),
            selected_index: 0,
        }
    }
}

impl Component for List {
    fn render(&self, f: &mut Frame, area: Rect, focused: bool) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, text)| {
                let style = if i == self.selected_index {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ListItem::new(text.as_str()).style(style)
            })
            .collect();

        let list = RatatuiList::new(items)
            .block(Block::default().borders(Borders::ALL))
            .style(if focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        f.render_widget(list, area);
    }

    fn handle_input(&mut self, _event: crossterm::event::Event) -> TuiResult<bool> {
        Ok(false)
    }

    fn update(&mut self) -> TuiResult<()> {
        Ok(())
    }

    fn can_focus(&self) -> bool {
        !self.items.is_empty()
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

// Placeholder types for other widgets
pub struct Table;
pub struct Dialog;

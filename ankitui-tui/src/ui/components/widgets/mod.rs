//! Widget components for the TUI application

use crate::ui::components::base::{Component, ComponentState, InteractiveComponent};
use crate::utils::error::TuiResult;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Gauge, List as RatatuiList, ListItem, Paragraph, Row},
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
            Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED)
        } else if self.state.enabled {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default().borders(Borders::ALL).style(style);

        let paragraph = Paragraph::new(self.label.as_str()).block(block).style(style);

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
                if event.kind == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left) {
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
                crossterm::event::KeyEvent::new(crossterm::event::KeyCode::Enter, crossterm::event::KeyModifiers::NONE),
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
    cursor_pos: usize,
}

impl Input {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: ComponentState::new(),
            placeholder: "Enter text...".to_string(),
            value: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.cursor_pos = value.len();
        self.value = value;
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_pos = 0;
        self.mark_dirty();
    }

    fn insert_char(&mut self, ch: char) {
        let pos = self.cursor_pos.min(self.value.len());
        self.value.insert(pos, ch);
        self.cursor_pos += 1;
        self.mark_dirty();
    }

    fn delete_char_before(&mut self) {
        if self.cursor_pos > 0 && !self.value.is_empty() {
            let pos = self.cursor_pos.min(self.value.len());
            self.value.remove(pos - 1);
            self.cursor_pos -= 1;
            self.mark_dirty();
        }
    }
}

impl Component for Input {
    fn render(&self, f: &mut Frame, area: Rect, focused: bool) {
        let display_value = if self.value.is_empty() {
            self.placeholder.clone()
        } else {
            self.value.clone()
        };

        let style = if focused {
            Style::default().fg(Color::Yellow)
        } else if self.value.is_empty() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };

        let block = Block::default().borders(Borders::ALL).style(style);

        let paragraph = Paragraph::new(display_value).block(block).style(style);

        f.render_widget(paragraph, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        if !self.state.enabled {
            return Ok(false);
        }

        match event {
            crossterm::event::Event::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => match key.code {
                crossterm::event::KeyCode::Char(c) => {
                    self.insert_char(c);
                    Ok(true)
                }
                crossterm::event::KeyCode::Backspace => {
                    self.delete_char_before();
                    Ok(true)
                }
                crossterm::event::KeyCode::Left => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                        self.mark_dirty();
                    }
                    Ok(true)
                }
                crossterm::event::KeyCode::Right => {
                    if self.cursor_pos < self.value.len() {
                        self.cursor_pos += 1;
                        self.mark_dirty();
                    }
                    Ok(true)
                }
                crossterm::event::KeyCode::Home => {
                    self.cursor_pos = 0;
                    self.mark_dirty();
                    Ok(true)
                }
                crossterm::event::KeyCode::End => {
                    self.cursor_pos = self.value.len();
                    self.mark_dirty();
                    Ok(true)
                }
                crossterm::event::KeyCode::Esc => Ok(false),
                _ => Ok(false),
            },
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

    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.selected_index = if items.is_empty() { 0 } else { 0 };
        self.items = items;
        self
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn selected_item(&self) -> Option<&String> {
        self.items.get(self.selected_index)
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.selected_index = 0;
        self.items = items;
        self.mark_dirty();
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.mark_dirty();
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index + 1 < self.items.len() {
            self.selected_index += 1;
            self.mark_dirty();
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

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        if self.items.is_empty() || !self.state.enabled {
            return Ok(false);
        }

        match event {
            crossterm::event::Event::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => match key.code {
                crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                    self.move_up();
                    Ok(true)
                }
                crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                    self.move_down();
                    Ok(true)
                }
                crossterm::event::KeyCode::Home => {
                    self.selected_index = 0;
                    self.mark_dirty();
                    Ok(true)
                }
                crossterm::event::KeyCode::End => {
                    self.selected_index = self.items.len().saturating_sub(1);
                    self.mark_dirty();
                    Ok(true)
                }
                crossterm::event::KeyCode::PageUp => {
                    self.selected_index = self.selected_index.saturating_sub(10);
                    self.mark_dirty();
                    Ok(true)
                }
                crossterm::event::KeyCode::PageDown => {
                    self.selected_index = (self.selected_index + 10).min(self.items.len().saturating_sub(1));
                    self.mark_dirty();
                    Ok(true)
                }
                _ => Ok(false),
            },
            _ => Ok(false),
        }
    }

    fn update(&mut self) -> TuiResult<()> {
        Ok(())
    }

    fn can_focus(&self) -> bool {
        self.state.enabled && !self.items.is_empty()
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

/// Table widget with column headers and row data
pub struct Table {
    id: String,
    state: ComponentState,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    selected_row: usize,
    column_widths: Vec<ratatui::layout::Constraint>,
}

impl Table {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: ComponentState::new(),
            headers: Vec::new(),
            rows: Vec::new(),
            selected_row: 0,
            column_widths: Vec::new(),
        }
    }

    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self
    }

    pub fn with_column_widths(mut self, widths: Vec<ratatui::layout::Constraint>) -> Self {
        self.column_widths = widths;
        self
    }

    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    pub fn set_data(&mut self, headers: Vec<String>, rows: Vec<Vec<String>>) {
        self.headers = headers;
        self.rows = rows;
        self.selected_row = 0;
        self.mark_dirty();
    }

    pub fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
            self.mark_dirty();
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_row + 1 < self.rows.len() {
            self.selected_row += 1;
            self.mark_dirty();
        }
    }
}

impl Component for Table {
    fn render(&self, f: &mut Frame, area: Rect, focused: bool) {
        if self.headers.is_empty() && self.rows.is_empty() {
            let paragraph = Paragraph::new("No data available")
                .block(Block::default().borders(Borders::ALL).title("Table"))
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(paragraph, area);
            return;
        }

        let header_cells = self
            .headers
            .iter()
            .map(|h| Cell::from(h.clone()).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        let header_row = Row::new(header_cells).style(Style::default().bg(Color::DarkGray));

        let data_rows = self.rows.iter().enumerate().map(|(i, row)| {
            let style = if i == self.selected_row {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            let cells = row.iter().map(|c| Cell::from(c.clone()));
            Row::new(cells).style(style)
        });

        let widths: Vec<_> = if self.column_widths.is_empty() {
            let pct = 100 / self.headers.len().max(1) as u16;
            self.headers
                .iter()
                .map(|_| ratatui::layout::Constraint::Percentage(pct))
                .collect()
        } else {
            self.column_widths.clone()
        };

        let table_widget = ratatui::widgets::Table::new(data_rows, widths)
            .header(header_row)
            .block(Block::default().borders(Borders::ALL).style(if focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }));

        f.render_widget(table_widget, area);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        if self.rows.is_empty() || !self.state.enabled {
            return Ok(false);
        }

        match event {
            crossterm::event::Event::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => match key.code {
                crossterm::event::KeyCode::Up => {
                    self.move_up();
                    Ok(true)
                }
                crossterm::event::KeyCode::Down => {
                    self.move_down();
                    Ok(true)
                }
                crossterm::event::KeyCode::Home => {
                    self.selected_row = 0;
                    self.mark_dirty();
                    Ok(true)
                }
                crossterm::event::KeyCode::End => {
                    self.selected_row = self.rows.len().saturating_sub(1);
                    self.mark_dirty();
                    Ok(true)
                }
                _ => Ok(false),
            },
            _ => Ok(false),
        }
    }

    fn update(&mut self) -> TuiResult<()> {
        Ok(())
    }

    fn can_focus(&self) -> bool {
        self.state.enabled && !self.rows.is_empty()
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

/// Dialog widget for confirmations and alerts
pub struct Dialog {
    id: String,
    state: ComponentState,
    title: String,
    message: String,
    buttons: Vec<String>,
    selected_button: usize,
}

impl Dialog {
    pub fn new(id: String, title: String, message: String) -> Self {
        Self {
            id,
            state: ComponentState::new(),
            title,
            message,
            buttons: vec!["OK".to_string()],
            selected_button: 0,
        }
    }

    pub fn with_buttons(mut self, buttons: Vec<String>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn selected_button(&self) -> usize {
        self.selected_button
    }

    pub fn confirm_button_text(&self) -> &str {
        self.buttons.first().map(|s| s.as_str()).unwrap_or("OK")
    }

    pub fn move_left(&mut self) {
        if self.selected_button > 0 {
            self.selected_button -= 1;
            self.mark_dirty();
        }
    }

    pub fn move_right(&mut self) {
        if self.selected_button + 1 < self.buttons.len() {
            self.selected_button += 1;
            self.mark_dirty();
        }
    }
}

impl Component for Dialog {
    fn render(&self, f: &mut Frame, area: Rect, _focused: bool) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let title_paragraph =
            Paragraph::new(self.title.as_str()).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        f.render_widget(title_paragraph, chunks[0]);

        let message_paragraph = Paragraph::new(self.message.as_str()).style(Style::default());
        f.render_widget(message_paragraph, chunks[1]);

        let button_text: String = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, b)| {
                if i == self.selected_button {
                    format!("[ {} ]", b)
                } else {
                    format!("  {}  ", b)
                }
            })
            .collect::<Vec<_>>()
            .join("   ");

        let button_paragraph = Paragraph::new(button_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Actions"));
        f.render_widget(button_paragraph, chunks[2]);
    }

    fn handle_input(&mut self, event: crossterm::event::Event) -> TuiResult<bool> {
        if !self.state.enabled {
            return Ok(false);
        }

        match event {
            crossterm::event::Event::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => match key.code {
                crossterm::event::KeyCode::Left => {
                    self.move_left();
                    Ok(true)
                }
                crossterm::event::KeyCode::Right => {
                    self.move_right();
                    Ok(true)
                }
                crossterm::event::KeyCode::Enter => Ok(true),
                crossterm::event::KeyCode::Esc => Ok(false),
                _ => Ok(false),
            },
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

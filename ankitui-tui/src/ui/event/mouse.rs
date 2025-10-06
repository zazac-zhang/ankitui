//! Mouse event handling

use crossterm::event::{KeyEvent, MouseEvent as CrosstermMouseEvent, MouseEventKind, MouseButton as CrosstermMouseButton, KeyModifiers as CrosstermKeyModifiers};
use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};

/// Serializable key modifiers wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModifiers {
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
    pub super_: bool,
    pub hyper: bool,
}

impl From<CrosstermKeyModifiers> for KeyModifiers {
    fn from(mods: CrosstermKeyModifiers) -> Self {
        Self {
            control: mods.contains(CrosstermKeyModifiers::CONTROL),
            alt: mods.contains(CrosstermKeyModifiers::ALT),
            shift: mods.contains(CrosstermKeyModifiers::SHIFT),
            meta: mods.contains(CrosstermKeyModifiers::META),
            super_: mods.contains(CrosstermKeyModifiers::SUPER),
            hyper: mods.contains(CrosstermKeyModifiers::HYPER),
        }
    }
}

impl From<KeyModifiers> for CrosstermKeyModifiers {
    fn from(mods: KeyModifiers) -> Self {
        let mut result = CrosstermKeyModifiers::NONE;
        if mods.control {
            result |= CrosstermKeyModifiers::CONTROL;
        }
        if mods.alt {
            result |= CrosstermKeyModifiers::ALT;
        }
        if mods.shift {
            result |= CrosstermKeyModifiers::SHIFT;
        }
        if mods.meta {
            result |= CrosstermKeyModifiers::META;
        }
        if mods.super_ {
            result |= CrosstermKeyModifiers::SUPER;
        }
        if mods.hyper {
            result |= CrosstermKeyModifiers::HYPER;
        }
        result
    }
}

impl KeyModifiers {
    pub fn contains(&self, other: CrosstermKeyModifiers) -> bool {
        match other {
            CrosstermKeyModifiers::CONTROL => self.control,
            CrosstermKeyModifiers::ALT => self.alt,
            CrosstermKeyModifiers::SHIFT => self.shift,
            CrosstermKeyModifiers::META => self.meta,
            CrosstermKeyModifiers::SUPER => self.super_,
            CrosstermKeyModifiers::HYPER => self.hyper,
            _ => false, // For other combinations
        }
    }
}

/// Mouse event wrapper
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MouseEvent {
    pub kind: MouseKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Mouse event kinds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Moved,
    ScrolledUp,
    ScrolledDown,
    ScrolledLeft,
    ScrolledRight,
}

/// Mouse button types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseEvent {
    pub fn new(
        kind: MouseKind,
        column: u16,
        row: u16,
        modifiers: crossterm::event::KeyModifiers,
    ) -> Self {
        Self {
            kind,
            column,
            row,
            modifiers: modifiers.into(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn is_click(&self) -> bool {
        matches!(self.kind, MouseKind::Down(_))
    }

    pub fn is_left_click(&self) -> bool {
        matches!(self.kind, MouseKind::Down(MouseButton::Left))
    }

    pub fn is_right_click(&self) -> bool {
        matches!(self.kind, MouseKind::Down(MouseButton::Right))
    }

    pub fn is_middle_click(&self) -> bool {
        matches!(self.kind, MouseKind::Down(MouseButton::Middle))
    }

    pub fn is_drag(&self) -> bool {
        matches!(self.kind, MouseKind::Drag(_))
    }

    pub fn is_scroll_up(&self) -> bool {
        matches!(self.kind, MouseKind::ScrolledUp)
    }

    pub fn is_scroll_down(&self) -> bool {
        matches!(self.kind, MouseKind::ScrolledDown)
    }

    pub fn is_scroll_left(&self) -> bool {
        matches!(self.kind, MouseKind::ScrolledLeft)
    }

    pub fn is_scroll_right(&self) -> bool {
        matches!(self.kind, MouseKind::ScrolledRight)
    }

    pub fn is_movement(&self) -> bool {
        matches!(self.kind, MouseKind::Moved)
    }

    pub fn position(&self) -> (u16, u16) {
        (self.column, self.row)
    }

    pub fn x(&self) -> u16 {
        self.column
    }

    pub fn y(&self) -> u16 {
        self.row
    }

    pub fn has_ctrl(&self) -> bool {
        self.modifiers
            .contains(crossterm::event::KeyModifiers::CONTROL)
    }

    pub fn has_alt(&self) -> bool {
        self.modifiers.contains(crossterm::event::KeyModifiers::ALT)
    }

    pub fn has_shift(&self) -> bool {
        self.modifiers
            .contains(crossterm::event::KeyModifiers::SHIFT)
    }

    /// Check if mouse event is within a given rectangle
    pub fn is_within(&self, area: Rect) -> bool {
        self.column >= area.x
            && self.column < area.x + area.width
            && self.row >= area.y
            && self.row < area.y + area.height
    }

    /// Check if mouse event is at a specific position
    pub fn is_at(&self, x: u16, y: u16) -> bool {
        self.column == x && self.row == y
    }

    /// Check if mouse event is near a position (within tolerance)
    pub fn is_near(&self, x: u16, y: u16, tolerance: u16) -> bool {
        let dx = if self.column > x {
            self.column - x
        } else {
            x - self.column
        };
        let dy = if self.row > y {
            self.row - y
        } else {
            y - self.row
        };
        dx <= tolerance && dy <= tolerance
    }

    pub fn description(&self) -> String {
        let mut parts = Vec::new();

        if self.has_ctrl() {
            parts.push("Ctrl");
        }
        if self.has_alt() {
            parts.push("Alt");
        }
        if self.has_shift() {
            parts.push("Shift");
        }

        let action_desc = match &self.kind {
            MouseKind::Down(button) => format!("{} click", button.description()),
            MouseKind::Up(button) => format!("{} release", button.description()),
            MouseKind::Drag(button) => format!("{} drag", button.description()),
            MouseKind::Moved => "Move".to_string(),
            MouseKind::ScrolledUp => "Scroll up".to_string(),
            MouseKind::ScrolledDown => "Scroll down".to_string(),
            MouseKind::ScrolledLeft => "Scroll left".to_string(),
            MouseKind::ScrolledRight => "Scroll right".to_string(),
        };

        if parts.is_empty() {
            format!("{} at ({}, {})", action_desc, self.column, self.row)
        } else {
            format!(
                "{} {} at ({}, {})",
                parts.join("+"),
                action_desc,
                self.column,
                self.row
            )
        }
    }
}

impl MouseButton {
    pub fn description(&self) -> &'static str {
        match self {
            MouseButton::Left => "Left",
            MouseButton::Right => "Right",
            MouseButton::Middle => "Middle",
        }
    }
}

impl From<CrosstermMouseEvent> for MouseEvent {
    fn from(event: CrosstermMouseEvent) -> Self {
        let kind = match event.kind {
            MouseEventKind::Down(button) => MouseKind::Down(MouseButton::from(button)),
            MouseEventKind::Up(button) => MouseKind::Up(MouseButton::from(button)),
            MouseEventKind::Drag(button) => MouseKind::Drag(MouseButton::from(button)),
            MouseEventKind::Moved => MouseKind::Moved,
            MouseEventKind::ScrollUp => MouseKind::ScrolledUp,
            MouseEventKind::ScrollDown => MouseKind::ScrolledDown,
            MouseEventKind::ScrollLeft => MouseKind::ScrolledLeft,
            MouseEventKind::ScrollRight => MouseKind::ScrolledRight,
        };

        Self::new(kind, event.column, event.row, event.modifiers)
    }
}

impl From<CrosstermMouseButton> for MouseButton {
    fn from(button: CrosstermMouseButton) -> Self {
        match button {
            CrosstermMouseButton::Left => MouseButton::Left,
            CrosstermMouseButton::Right => MouseButton::Right,
            CrosstermMouseButton::Middle => MouseButton::Middle,
        }
    }
}

/// Mouse action mapping
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseAction {
    Click(MouseButton),
    DoubleClick(MouseButton),
    TripleClick(MouseButton),
    DragStart(MouseButton),
    DragMove(MouseButton),
    DragEnd(MouseButton),
    Hover,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,

    // Context-specific actions
    SelectItem,
    OpenMenu,
    CloseMenu,
    Resize,
    Move,
    Copy,
    Paste,

    // Custom
    Custom(String),
}

impl MouseAction {
    pub fn description(&self) -> String {
        match self {
            MouseAction::Click(_) => "Click".to_string(),
            MouseAction::DoubleClick(_) => "Double click".to_string(),
            MouseAction::TripleClick(_) => "Triple click".to_string(),
            MouseAction::DragStart(_) => "Start drag".to_string(),
            MouseAction::DragMove(_) => "Drag".to_string(),
            MouseAction::DragEnd(_) => "End drag".to_string(),
            MouseAction::Hover => "Hover".to_string(),
            MouseAction::ScrollUp => "Scroll up".to_string(),
            MouseAction::ScrollDown => "Scroll down".to_string(),
            MouseAction::ScrollLeft => "Scroll left".to_string(),
            MouseAction::ScrollRight => "Scroll right".to_string(),
            MouseAction::SelectItem => "Select item".to_string(),
            MouseAction::OpenMenu => "Open menu".to_string(),
            MouseAction::CloseMenu => "Close menu".to_string(),
            MouseAction::Resize => "Resize".to_string(),
            MouseAction::Move => "Move".to_string(),
            MouseAction::Copy => "Copy".to_string(),
            MouseAction::Paste => "Paste".to_string(),
            MouseAction::Custom(desc) => desc.clone(),
        }
    }
}

//! UI components for the TUI application

pub mod base;
pub mod layout;

// Placeholder modules - to be implemented
pub mod screens;
pub mod widgets;

// Re-export key component types
pub use base::{
    Component, ComponentRegistry, ComponentState, ContainerComponent, FocusableComponent, InteractiveComponent,
};
pub use layout::{Container, Direction};

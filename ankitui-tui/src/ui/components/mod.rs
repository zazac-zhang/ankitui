//! UI components for the TUI application

pub mod base;
pub mod layout;

// Placeholder modules - to be implemented
pub mod widgets;
pub mod screens;

// Re-export key component types
pub use base::{Component, ComponentState, ComponentRegistry, InteractiveComponent, ContainerComponent, FocusableComponent};
pub use layout::{Container, Direction};
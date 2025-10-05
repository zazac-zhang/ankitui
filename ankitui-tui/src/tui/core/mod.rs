//! Core TUI Modules
//!
//! Modern clean architecture with clear separation of concerns

// Modern core modules
pub mod app_context;
pub mod action_dispatcher;
pub mod component_registry;
pub mod config_manager;
pub mod event_bus;
pub mod event_handler;
pub mod rendering_manager;
pub mod state_manager;

// Performance monitoring
pub mod performance;

// State management
pub mod state_management;

// Error management - 统一错误处理系统
pub mod error_management;

// Modern re-exports
pub use app_context::{AppContext, ComponentResult};
pub use action_dispatcher::ActionDispatcher;
pub use component_registry::{ComponentRegistry, UIComponent, AutonomousComponent};
pub use config_manager::{TUIConfig, TUIConfigManager};
pub use event_bus::{global_event_bus, publish_global, AppEvent, EventBus};
pub use event_handler::{Action, Events};
pub use rendering_manager::RenderingManager;
pub use state_manager::{RenderContext, StateManager};

// Performance re-exports
pub use performance::*;

// State management re-exports - 使用新的清晰架构
pub use state_management::{
    ApplicationState, ApplicationStateManager, MessageLevel, SystemMessage, UIState, UIStateManager,
};

// Error management re-exports - 统一错误处理系统
pub use error_management::*;

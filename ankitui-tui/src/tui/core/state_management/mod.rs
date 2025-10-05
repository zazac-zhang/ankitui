//! State Management Module
//!
//! 清晰的状态管理架构，遵循"一个state只管理一类型state"原则
//!
//! 架构设计：
//! - ui_state.rs: 专门管理UI相关的状态（菜单、选择、界面交互等）
//! - app_state.rs: 专门管理应用级别的状态（服务、配置、运行时等）
//! - unified_state_manager.rs: 统一的状态管理器，协调不同类型的状态

pub mod app_state;
pub mod ui_state;

pub use app_state::{ApplicationState, ApplicationStateManager};
pub use ui_state::{MessageLevel, SystemMessage, UIState, UIStateManager};

//! Event Bus System
//!
//! Centralized event management for clean component communication

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Application-wide events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    /// Navigation events
    NavigateTo {
        destination: String,
    },
    NavigateBack,
    NavigateForward,

    /// State change events
    StateChanged {
        old_state: String,
        new_state: String,
    },
    StateUpdated {
        component: String,
        data: HashMap<String, String>,
    },

    /// Data events
    DataLoaded {
        data_type: String,
        count: usize,
    },
    DataChanged {
        data_type: String,
        id: String,
    },
    DataError {
        data_type: String,
        error: String,
    },

    /// User action events
    UserAction {
        action: String,
        target: String,
    },
    KeyPressed {
        key: String,
        modifiers: Vec<String>,
    },

    /// System events
    ThemeChanged {
        theme: String,
    },
    ConfigurationChanged {
        key: String,
        value: String,
    },
    ErrorOccurred {
        error: String,
        context: String,
    },
    Success {
        message: String,
    },

    /// Timer events
    TimerTick {
        interval_ms: u64,
    },
    AnimationUpdate {
        component: String,
        progress: f32,
    },

    /// Focus events
    FocusChanged {
        component: String,
        focused: bool,
    },
    FocusRequested {
        component: String,
    },

    /// Modal events
    ModalOpened {
        modal_type: String,
    },
    ModalClosed {
        result: Option<String>,
    },

    /// Application lifecycle
    ApplicationStarted,
    ApplicationExiting,
    ApplicationResumed,
    ApplicationPaused,
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    /// Handle an event
    fn handle_event(&mut self, event: &AppEvent) -> Result<()>;

    /// Get handler name
    fn name(&self) -> &str;
}

/// Event bus for centralized event management
pub struct EventBus {
    /// Registered event handlers
    handlers: Arc<Mutex<HashMap<String, Vec<Box<dyn EventHandler>>>>>,
    /// Event history (for debugging)
    event_history: Arc<Mutex<Vec<AppEvent>>>,
    max_history_size: usize,
}

impl EventBus {
    /// Create new event bus
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            event_history: Arc::new(Mutex::new(Vec::new())),
            max_history_size: 1000,
        }
    }

    /// Register an event handler for specific event types
    pub fn register_handler<F>(&self, event_types: Vec<String>, handler: F)
    where
        F: Fn(&AppEvent) -> Result<()> + Send + Sync + 'static,
    {
        let wrapper = HandlerWrapper::new(event_types.clone(), handler);

        let mut handlers = self.handlers.lock().unwrap();
        for event_type in event_types {
            handlers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(Box::new(wrapper.clone()));
        }
    }

    /// Register a typed event handler
    pub fn register_typed_handler(&self, handler: Box<dyn EventHandler>) {
        // For now, register handler for all event types
        // In a real implementation, we'd have type-based registration
        let mut handlers = self.handlers.lock().unwrap();
        handlers
            .entry("*".to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    /// Publish an event
    pub fn publish(&self, event: AppEvent) -> Result<()> {
        // Add to history
        self.add_to_history(event.clone());

        // Determine event type for routing
        let event_type = self.get_event_type(&event);

        // Handle event publishing without console output in TUI mode
        // TODO: Implement proper handler execution with mutable references

        Ok(())
    }

    /// Get event type from event
    fn get_event_type(&self, event: &AppEvent) -> String {
        match event {
            AppEvent::NavigateTo { .. } => "navigate_to".to_string(),
            AppEvent::NavigateBack => "navigate_back".to_string(),
            AppEvent::NavigateForward => "navigate_forward".to_string(),
            AppEvent::StateChanged { .. } => "state_changed".to_string(),
            AppEvent::StateUpdated { .. } => "state_updated".to_string(),
            AppEvent::DataLoaded { .. } => "data_loaded".to_string(),
            AppEvent::DataChanged { .. } => "data_changed".to_string(),
            AppEvent::DataError { .. } => "data_error".to_string(),
            AppEvent::UserAction { .. } => "user_action".to_string(),
            AppEvent::KeyPressed { .. } => "key_pressed".to_string(),
            AppEvent::ThemeChanged { .. } => "theme_changed".to_string(),
            AppEvent::ConfigurationChanged { .. } => "configuration_changed".to_string(),
            AppEvent::ErrorOccurred { .. } => "error_occurred".to_string(),
            AppEvent::Success { .. } => "success".to_string(),
            AppEvent::TimerTick { .. } => "timer_tick".to_string(),
            AppEvent::AnimationUpdate { .. } => "animation_update".to_string(),
            AppEvent::FocusChanged { .. } => "focus_changed".to_string(),
            AppEvent::FocusRequested { .. } => "focus_requested".to_string(),
            AppEvent::ModalOpened { .. } => "modal_opened".to_string(),
            AppEvent::ModalClosed { .. } => "modal_closed".to_string(),
            AppEvent::ApplicationStarted => "application_started".to_string(),
            AppEvent::ApplicationExiting => "application_exiting".to_string(),
            AppEvent::ApplicationResumed => "application_resumed".to_string(),
            AppEvent::ApplicationPaused => "application_paused".to_string(),
        }
    }

    /// Add event to history
    fn add_to_history(&self, event: AppEvent) {
        let mut history = self.event_history.lock().unwrap();
        history.push(event);
        if history.len() > self.max_history_size {
            history.remove(0);
        }
    }

    /// Get event history
    pub fn get_history(&self) -> Vec<AppEvent> {
        self.event_history.lock().unwrap().clone()
    }

    /// Clear event history
    pub fn clear_history(&self) {
        self.event_history.lock().unwrap().clear();
    }

    /// Get recent events of specific type
    pub fn get_recent_events(&self, event_type: &str, count: usize) -> Vec<AppEvent> {
        self.event_history
            .lock()
            .unwrap()
            .iter()
            .rev()
            .filter(|event| self.get_event_type(event) == event_type)
            .take(count)
            .cloned()
            .collect()
    }
}

/// Wrapper for function-based handlers
#[derive(Clone)]
struct HandlerWrapper {
    event_types: Vec<String>,
    handler_fn: Arc<dyn Fn(&AppEvent) -> Result<()> + Send + Sync>,
}

impl HandlerWrapper {
    fn new<F>(event_types: Vec<String>, handler_fn: F) -> Self
    where
        F: Fn(&AppEvent) -> Result<()> + Send + Sync + 'static,
    {
        Self {
            event_types,
            handler_fn: Arc::new(handler_fn),
        }
    }
}

impl EventHandler for HandlerWrapper {
    fn handle_event(&mut self, event: &AppEvent) -> Result<()> {
        (self.handler_fn)(event)
    }

    fn name(&self) -> &str {
        "function_handler"
    }
}


impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            handlers: self.handlers.clone(),
            event_history: self.event_history.clone(),
            max_history_size: self.max_history_size,
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macros for common events
#[macro_export]
macro_rules! publish_navigation {
    ($bus:expr, $destination:expr) => {
        $bus.publish($crate::tui::core::event_bus::AppEvent::NavigateTo {
            destination: $destination.to_string(),
        })
    };
}

#[macro_export]
macro_rules! publish_state_change {
    ($bus:expr, $old:expr, $new:expr) => {
        $bus.publish($crate::tui::core::event_bus::AppEvent::StateChanged {
            old_state: $old.to_string(),
            new_state: $new.to_string(),
        })
    };
}

#[macro_export]
macro_rules! publish_error {
    ($bus:expr, $error:expr, $context:expr) => {
        $bus.publish($crate::tui::core::event_bus::AppEvent::ErrorOccurred {
            error: $error.to_string(),
            context: $context.to_string(),
        })
    };
}

#[macro_export]
macro_rules! publish_success {
    ($bus:expr, $message:expr) => {
        $bus.publish($crate::tui::core::event_bus::AppEvent::Success {
            message: $message.to_string(),
        })
    };
}

/// Global event bus instance (singleton pattern)
static mut GLOBAL_EVENT_BUS: Option<EventBus> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Get global event bus instance
pub fn global_event_bus() -> &'static EventBus {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_EVENT_BUS = Some(EventBus::new());
        });
        GLOBAL_EVENT_BUS.as_ref().unwrap()
    }
}

/// Publish to global event bus
pub fn publish_global(event: AppEvent) -> Result<()> {
    global_event_bus().publish(event)
}

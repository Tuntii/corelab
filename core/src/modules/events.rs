//! Event System
//! 
//! Pub/sub event bus for inter-app communication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    PersonCreated,
    PersonUpdated,
    ConversationCreated,
    MemoryExtracted,
    AIRequestCompleted,
    Custom(String),
}

/// Event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub source: String,
    pub data: serde_json::Value,
    pub timestamp: String,
}

/// Event handler callback type
pub type EventHandler = Arc<dyn Fn(&Event) + Send + Sync>;

/// Event Bus - pub/sub system
pub struct EventBus {
    handlers: RwLock<HashMap<String, Vec<EventHandler>>>,
    log: RwLock<Vec<Event>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
            log: RwLock::new(Vec::new()),
        }
    }

    /// Subscribe to an event type
    pub fn subscribe(&self, event_key: &str, handler: EventHandler) {
        let mut handlers = self.handlers.write().unwrap();
        handlers
            .entry(event_key.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    /// Emit an event
    pub fn emit(&self, event: Event) {
        let event_key = format!("{:?}", event.event_type);
        
        // Log the event
        {
            let mut log = self.log.write().unwrap();
            log.push(event.clone());
        }

        // Call handlers
        let handlers = self.handlers.read().unwrap();
        if let Some(event_handlers) = handlers.get(&event_key) {
            for handler in event_handlers {
                handler(&event);
            }
        }
    }

    /// Get event log
    pub fn get_log(&self, limit: usize) -> Vec<Event> {
        let log = self.log.read().unwrap();
        log.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

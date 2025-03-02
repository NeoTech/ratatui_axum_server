use std::collections::HashMap;
use std::sync::Arc;

use crate::handler::{Handler, HealthCheckHandler, StaticResponseHandler, StatusHandler};

/// Registry for storing and retrieving handlers
pub struct HandlerRegistry {
    handlers: HashMap<String, Arc<dyn Handler>>,
}

impl HandlerRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a handler with a name
    pub fn register<H: Handler>(&mut self, name: &str, handler: H) {
        self.handlers.insert(name.to_string(), Arc::new(handler));
    }

    /// Get a handler by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>> {
        self.handlers.get(name).cloned()
    }

    /// Check if a handler exists
    pub fn contains(&self, name: &str) -> bool {
        self.handlers.contains_key(name)
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        register_default_handlers(&mut registry);
        registry
    }
}

/// Register the default handlers in the registry
pub fn register_default_handlers(registry: &mut HandlerRegistry) {
    // Register the static response handler as the default fallback
    registry.register("default", StaticResponseHandler);
    
    // Register specialized handlers
    registry.register("health", HealthCheckHandler);
    registry.register("status", StatusHandler);
} 
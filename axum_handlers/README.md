# Axum Handlers

A flexible handler system for Axum web applications that allows mapping endpoint configurations to handler implementations.

## Features

- **Handler Registry**: Map handler names to implementations
- **Default Handlers**: Built-in handlers for common use cases
- **Custom Handlers**: Easily create and register custom handlers
- **Context-Based Execution**: Pass context to handlers for flexible execution

## Installation

Add the library to your Cargo.toml:

```toml
[dependencies]
axum_handlers = { path = "../axum_handlers" }
```

## Basic Usage

```rust
use axum_handlers::{HandlerRegistry, register_default_handlers, Handler, HandlerContext, HandlerResult};
use async_trait::async_trait;

// Create and initialize the handler registry
let mut registry = HandlerRegistry::new();
register_default_handlers(&mut registry);

// Create a custom handler
pub struct MyCustomHandler;

#[async_trait]
impl Handler for MyCustomHandler {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
        Ok(format!("Custom response for {}", ctx.path))
    }
}

// Register the custom handler
registry.register("my_custom", MyCustomHandler);
```

## API Reference

### `Handler` Trait

```rust
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult;
}
```

The core trait that all handlers must implement. It takes a `HandlerContext` and returns a `HandlerResult`.

### `HandlerContext`

```rust
pub struct HandlerContext {
    pub path: String,
    pub params: HashMap<String, String>,
    pub static_response: Option<String>,
}
```

Contains information about the request and configuration that is passed to handlers.

### `HandlerResult`

```rust
pub type HandlerResult = Result<String, (StatusCode, String)>;
```

The result type for handlers. It can be either a successful response or an error with a status code and message.

### `HandlerRegistry`

```rust
pub struct HandlerRegistry {
    handlers: HashMap<String, Arc<dyn Handler>>,
}

impl HandlerRegistry {
    pub fn new() -> Self;
    pub fn register<H: Handler>(&mut self, name: &str, handler: H);
    pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>>;
    pub fn contains(&self, name: &str) -> bool;
}
```

A registry for storing and retrieving handlers by name.

### Default Handlers

The library includes the following default handlers:

- `StaticResponseHandler`: Returns the static response from the configuration
- `HealthCheckHandler`: Returns "OK" for health checks
- `StatusHandler`: Returns a JSON response with server status information

## Integration with Axum

This library is designed to work seamlessly with Axum web applications:

```rust
use axum::{Router, routing::get, extract::State};
use std::sync::Arc;
use axum_handlers::{HandlerRegistry, HandlerContext, Handler};

// Create app state with handler registry
struct AppState {
    handler_registry: HandlerRegistry,
}

// Create a route handler that uses the registry
async fn handle_request(
    State(state): State<Arc<AppState>>,
    handler_name: String,
) -> impl IntoResponse {
    if let Some(handler) = state.handler_registry.get(&handler_name) {
        let ctx = HandlerContext {
            path: "/example".to_string(),
            params: HashMap::new(),
            static_response: None,
        };
        
        match handler.handle(ctx).await {
            Ok(response) => response.into_response(),
            Err((status, message)) => (status, message).into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Handler not found").into_response()
    }
}
```

## Extending the Library

### Creating Custom Handlers

You can create custom handlers by implementing the `Handler` trait:

```rust
use axum_handlers::{Handler, HandlerContext, HandlerResult};
use async_trait::async_trait;

pub struct DatabaseHandler;

#[async_trait]
impl Handler for DatabaseHandler {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
        // Access a database using parameters from ctx.params
        let user_id = ctx.params.get("user_id").unwrap_or(&"0".to_string());
        
        // Perform database operations
        let result = format!("User data for ID: {}", user_id);
        
        Ok(result)
    }
}
```

### Adding Middleware Support

You can extend the library to support middleware by modifying the handler context:

```rust
pub struct HandlerContextWithMiddleware {
    pub path: String,
    pub params: HashMap<String, String>,
    pub static_response: Option<String>,
    pub middleware: Vec<String>,
}
```

## License

MIT 
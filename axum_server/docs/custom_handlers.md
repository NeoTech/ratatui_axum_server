# Creating Custom Handlers

This guide explains how to create and register custom handlers for the Axum TUI application.

## Overview

The application uses a dynamic handler system that allows you to define HTTP endpoints in a configuration file (`config.yaml`) and implement their behavior in Rust code. This separation provides flexibility and makes it easy to add new endpoints without changing the core application code.

## The Connection Between Configuration and Handlers

Custom handlers are connected to configuration through the handler name. Here's how it works:

1. In `config.yaml`, you define an endpoint with a specific handler name
2. In your Rust code, you implement a handler and register it with the same name
3. When a request comes in, the application looks up the handler by name and executes it

## Example: User Handler

Here's a complete example showing the connection between configuration and handler implementation:

### 1. Configuration in `config.yaml`

```yaml
- path: /api/user/{id}
  method: GET
  handler: user
  response: ""
  description: "Get user data by ID"
  params:
    database: "users"
    format: "json"
```

This configuration defines:
- A path with a parameter: `/api/user/{id}`
- The HTTP method: `GET`
- A handler name: `user`
- Parameters that will be passed to the handler

### 2. Implementation in Rust

```rust
use axum_handlers::{Handler, HandlerContext, HandlerResult, HandlerRegistry};
use async_trait::async_trait;
use axum::http::StatusCode;

// Define the handler struct
pub struct UserHandler;

// Implement the Handler trait
#[async_trait]
impl Handler for UserHandler {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
        // Extract user ID from the path (assuming path is like /api/user/{id})
        let user_id = ctx.path.split('/').last().unwrap_or("unknown");

        // Get database and format from params defined in config.yaml
        let default_db = "default".to_string();
        let default_format = "json".to_string();
        let database = ctx.params.get("database").unwrap_or(&default_db);
        let format = ctx.params.get("format").unwrap_or(&default_format);
        
        // Return formatted response based on the format parameter
        match format.as_str() {
            "json" => Ok(format!("{{ \"id\": \"{}\", \"name\": \"User {}\", \"database\": \"{}\" }}", 
                user_id, user_id, database)),
            "text" => Ok(format!("User ID: {}, Database: {}", user_id, database)),
            _ => Err((StatusCode::BAD_REQUEST, format!("Unsupported format: {}", format))),
        }
    }
}

// Register the handler with the registry
pub fn register_custom_handlers(registry: &mut HandlerRegistry) {
    registry.register("user", UserHandler);
    // Register other handlers here...
}
```

### 3. How It Works

When a request comes in to `/api/user/456`:
1. The routing system looks up the handler name `"user"` from the config
2. It finds the `UserHandler` registered with the name `"user"` in the registry
3. It passes the path (`/api/user/456`) and params (`{"database": "users", "format": "json"}`) to the handler
4. The handler extracts the ID (`456`), uses the params, and returns the response

## Creating Your Own Custom Handler

Follow these steps to create your own custom handler:

1. **Define the endpoint in `config.yaml`**:
   ```yaml
   - path: /api/your-endpoint/{param}
     method: GET
     handler: your_handler_name
     response: ""
     description: "Description of your endpoint"
     params:
       param1: "value1"
       param2: "value2"
   ```

2. **Create a handler struct**:
   ```rust
   pub struct YourHandler;
   ```

3. **Implement the `Handler` trait**:
   ```rust
   #[async_trait]
   impl Handler for YourHandler {
       async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
           // Extract parameters from the path
           let param = ctx.path.split('/').last().unwrap_or("default");
           
           // Get custom parameters from config
           let param1 = ctx.params.get("param1").unwrap_or(&"default".to_string());
           
           // Your business logic here
           
           // Return a successful response
           Ok(format!("Your response with {param} and {param1}"))
           
           // Or return an error
           // Err((StatusCode::BAD_REQUEST, "Error message".to_string()))
       }
   }
   ```

4. **Register your handler**:
   ```rust
   pub fn register_custom_handlers(registry: &mut HandlerRegistry) {
       // ... existing registrations
       registry.register("your_handler_name", YourHandler);
   }
   ```

## Handler Context

The `HandlerContext` provides:

- `path`: The full path of the request
- `params`: Parameters from the configuration
- `static_response`: Optional static response from configuration

## Best Practices

1. **Extract path parameters carefully**: Use proper error handling when extracting parameters from the path.
2. **Provide default values**: Always provide default values for optional parameters.
3. **Return appropriate status codes**: Use appropriate HTTP status codes for different scenarios.
4. **Keep handlers focused**: Each handler should have a single responsibility.
5. **Use descriptive handler names**: Names should reflect the handler's purpose.

## Advanced Example: Resource Handler with Path Parameters

Here's a more advanced example that handles both endpoints with and without path parameters:

### Configuration

```yaml
# Endpoint without path parameter
- path: /api/resource
  method: GET
  handler: resource_get
  response: ""
  description: "Get a resource"

# Endpoint with path parameter
- path: /api/resource/{id}
  method: GET
  handler: resource_get
  response: ""
  description: "Get a resource by ID"
```

### Implementation

```rust
pub struct ResourceHandler {
    operation: String,
}

impl ResourceHandler {
    pub fn new(operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
        }
    }
}

#[async_trait]
impl Handler for ResourceHandler {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
        // Extract resource ID from the path if available
        let resource_id = ctx.path.split('/').last()
            .and_then(|id| if id.is_empty() || id == "resource" { None } else { Some(id) })
            .unwrap_or("1"); // Default to ID 1 if not provided
        
        // Check if validation is required
        let validate = ctx.params.get("validate")
            .map(|v| v == "true")
            .unwrap_or(false);
        
        // Return response based on operation and resource ID
        match self.operation.as_str() {
            "get" => Ok(format!("{{ \"id\": {}, \"name\": \"Resource {}\", \"status\": \"active\" }}", 
                resource_id, resource_id)),
            // Other operations...
            _ => Err((StatusCode::BAD_REQUEST, format!("Unsupported operation: {}", self.operation))),
        }
    }
}

// Registration
pub fn register_custom_handlers(registry: &mut HandlerRegistry) {
    // ... other registrations
    registry.register("resource_get", ResourceHandler::new("get"));
}
```

This handler can handle both `/api/resource` and `/api/resource/123` endpoints, adapting its behavior based on whether a path parameter is present. 
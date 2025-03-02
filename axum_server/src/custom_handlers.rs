use axum_handlers::{Handler, HandlerContext, HandlerResult, HandlerRegistry};
use async_trait::async_trait;
use axum::http::StatusCode;

// Example of a custom handler for user data
pub struct UserHandler;

#[async_trait]
impl Handler for UserHandler {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
        // Extract user ID from the path (assuming path is like /api/user/{id})
        let user_id = ctx.path.split('/').last().unwrap_or("unknown");

        // Get database and format from params
        let default_db = "default".to_string();
        let default_format = "json".to_string();
        let database = ctx.params.get("database").unwrap_or(&default_db);
        let format = ctx.params.get("format").unwrap_or(&default_format);
        
        // In a real application, you would query a database here
        // For this example, we'll just return a formatted response
        match format.as_str() {
            "json" => Ok(format!("{{ \"id\": \"{}\", \"name\": \"User {}\", \"database\": \"{}\" }}", 
                user_id, user_id, database)),
            "text" => Ok(format!("User ID: {}, Database: {}", user_id, database)),
            _ => Err((StatusCode::BAD_REQUEST, format!("Unsupported format: {}", format))),
        }
    }
}

// Example of a custom handler for resource operations
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
        // Check if validation is required
        let validate = ctx.params.get("validate")
            .map(|v| v == "true")
            .unwrap_or(false);
        
        // Extract resource ID from the path if available
        let resource_id = ctx.path.split('/').last()
            .and_then(|id| if id.is_empty() || id == "resource" { None } else { Some(id) })
            .unwrap_or("1"); // Default to ID 1 if not provided
        
        // In a real application, you would perform the operation here
        // For this example, we'll just return a formatted response
        match self.operation.as_str() {
            "get" => Ok(format!("{{ \"id\": {}, \"name\": \"Resource {}\", \"status\": \"active\" }}", 
                resource_id, resource_id)),
            "create" => {
                if validate {
                    // Perform validation logic here
                    Ok("{ \"status\": \"created\", \"id\": 2, \"validated\": true }".to_string())
                } else {
                    Ok("{ \"status\": \"created\", \"id\": 2 }".to_string())
                }
            },
            "update" => {
                if validate {
                    // Perform validation logic here
                    Ok("{ \"status\": \"updated\", \"validated\": true }".to_string())
                } else {
                    Ok("{ \"status\": \"updated\" }".to_string())
                }
            },
            "delete" => Ok("{ \"status\": \"deleted\" }".to_string()),
            _ => Err((StatusCode::BAD_REQUEST, format!("Unsupported operation: {}", self.operation))),
        }
    }
}

// Function to register custom handlers
pub fn register_custom_handlers(registry: &mut HandlerRegistry) {
    // Register user handler
    registry.register("user", UserHandler);
    
    // Register resource handlers
    registry.register("resource_get", ResourceHandler::new("get"));
    registry.register("resource_create", ResourceHandler::new("create"));
    registry.register("resource_update", ResourceHandler::new("update"));
    registry.register("resource_delete", ResourceHandler::new("delete"));
} 
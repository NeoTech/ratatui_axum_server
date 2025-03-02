use async_trait::async_trait;
use axum::http::StatusCode;
use std::collections::HashMap;

/// Result type for handlers
pub type HandlerResult = Result<String, (StatusCode, String)>;

/// Context passed to handlers
pub struct HandlerContext {
    /// Path of the request
    pub path: String,
    /// Parameters from the YAML configuration
    pub params: HashMap<String, String>,
    /// Static response from the YAML configuration (if any)
    pub static_response: Option<String>,
}

/// Trait for implementing custom handlers
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    /// Handle a request and return a response
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult;
}

/// Simple handler that returns a static response
pub struct StaticResponseHandler;

#[async_trait]
impl Handler for StaticResponseHandler {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
        if let Some(response) = ctx.static_response {
            Ok(response)
        } else {
            Err((StatusCode::INTERNAL_SERVER_ERROR, "No static response configured".to_string()))
        }
    }
}

/// Handler for health checks
pub struct HealthCheckHandler;

#[async_trait]
impl Handler for HealthCheckHandler {
    async fn handle(&self, _ctx: HandlerContext) -> HandlerResult {
        Ok("OK".to_string())
    }
}

/// Handler for API status
pub struct StatusHandler;

#[async_trait]
impl Handler for StatusHandler {
    async fn handle(&self, _ctx: HandlerContext) -> HandlerResult {
        Ok("{ \"status\": \"running\", \"version\": \"1.0.0\" }".to_string())
    }
} 
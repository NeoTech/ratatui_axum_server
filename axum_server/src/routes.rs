use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::{mpsc, Arc};
use std::collections::HashMap;
use axum_handlers::{HandlerRegistry, HandlerContext};
use crate::config::Config;

pub struct AppState {
    pub log_sender: mpsc::Sender<String>,
    pub config: Config,
    pub handler_registry: HandlerRegistry,
}

// Define handler functions for each HTTP method
async fn handle_request(
    State(state): State<Arc<AppState>>,
    path: String,
    method: String,
    handler_name: String,
    response: String,
    params: HashMap<String, String>,
) -> impl IntoResponse {
    state.log_sender.send(format!("Received {} request to {}", method, path)).unwrap();
    
    // Check if the handler exists in the registry
    if let Some(handler) = state.handler_registry.get(&handler_name) {
        // Create the handler context
        let ctx = HandlerContext {
            path,
            params,
            static_response: Some(response),
        };
        
        // Execute the handler
        match handler.handle(ctx).await {
            Ok(response) => response.into_response(),
            Err((status, message)) => (status, message).into_response(),
        }
    } else {
        // If the handler doesn't exist, use the default handler
        if let Some(default_handler) = state.handler_registry.get("default") {
            let ctx = HandlerContext {
                path,
                params,
                static_response: Some(response),
            };
            
            match default_handler.handle(ctx).await {
                Ok(response) => response.into_response(),
                Err((status, message)) => (status, message).into_response(),
            }
        } else {
            // If there's no default handler, return a 500 error
            (StatusCode::INTERNAL_SERVER_ERROR, "No handler found").into_response()
        }
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let mut router = Router::new();
    
    // Add routes dynamically based on the configuration
    for endpoint in &state.config.endpoints {
        let path = endpoint.path.clone();
        let response = endpoint.response.clone();
        let method = endpoint.method.to_uppercase();
        let handler_name = endpoint.handler.clone();
        let params = endpoint.params.clone();
        
        // Check if the path contains path parameters
        let has_path_params = path.contains('{') && path.contains('}');
        
        // Add the route based on the HTTP method
        match method.as_str() {
            "GET" => {
                if has_path_params {
                    // For paths with parameters
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, get(move |state: State<Arc<AppState>>, path_params: Path<String>| {
                        let actual_path = path_params.0.clone();
                        handle_request(state, actual_path, "GET".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                } else {
                    // For paths without parameters
                    let path_clone = path.clone();
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, get(move |state: State<Arc<AppState>>| {
                        handle_request(state, path_clone, "GET".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                }
            },
            "POST" => {
                if has_path_params {
                    // For paths with parameters
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, post(move |state: State<Arc<AppState>>, path_params: Path<String>| {
                        let actual_path = path_params.0.clone();
                        handle_request(state, actual_path, "POST".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                } else {
                    // For paths without parameters
                    let path_clone = path.clone();
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, post(move |state: State<Arc<AppState>>| {
                        handle_request(state, path_clone, "POST".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                }
            },
            "PUT" => {
                if has_path_params {
                    // For paths with parameters
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, put(move |state: State<Arc<AppState>>, path_params: Path<String>| {
                        let actual_path = path_params.0.clone();
                        handle_request(state, actual_path, "PUT".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                } else {
                    // For paths without parameters
                    let path_clone = path.clone();
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, put(move |state: State<Arc<AppState>>| {
                        handle_request(state, path_clone, "PUT".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                }
            },
            "DELETE" => {
                if has_path_params {
                    // For paths with parameters
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, delete(move |state: State<Arc<AppState>>, path_params: Path<String>| {
                        let actual_path = path_params.0.clone();
                        handle_request(state, actual_path, "DELETE".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                } else {
                    // For paths without parameters
                    let path_clone = path.clone();
                    let response_clone = response.clone();
                    let handler_name_clone = handler_name.clone();
                    let params_clone = params.clone();
                    router = router.route(&endpoint.path, delete(move |state: State<Arc<AppState>>| {
                        handle_request(state, path_clone, "DELETE".to_string(), handler_name_clone, response_clone, params_clone)
                    }));
                }
            },
            _ => {
                eprintln!("Unsupported HTTP method: {}", method);
            }
        };
    }
    
    router.with_state(state)
} 
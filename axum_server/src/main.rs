mod config;
mod routes;
mod custom_handlers;

use tokio::sync::watch;
use std::thread;
use std::sync::{mpsc, Arc};
use clap::Parser;
use crate::config::Config;
use crate::routes::{AppState, create_router};
use axum_tui::{run_ui, ServerInfo, EndpointInfo};
use axum_handlers::{HandlerRegistry, register_default_handlers};
use crate::custom_handlers::register_custom_handlers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Load configuration
    let config = Config::load(&args.config)?;
    
    // Create a channel for sending logs from the server to the UI
    let (log_tx, log_rx) = mpsc::channel::<String>();
    
    // Create a channel for graceful shutdown
    let (shutdown_tx, mut shutdown_rx) = watch::channel::<()>(());

    // Log server startup
    log_tx.send(format!("Server starting with configuration from {}", args.config))?;
    log_tx.send(format!("Server will listen on {}:{}", config.server.host, config.server.port))?;
    
    // Create and initialize the handler registry
    let mut handler_registry = HandlerRegistry::new();
    register_default_handlers(&mut handler_registry);
    register_custom_handlers(&mut handler_registry);
    
    // Log configured endpoints
    for endpoint in &config.endpoints {
        log_tx.send(format!("Configured endpoint: {} {} - {}", 
            endpoint.method, 
            endpoint.path, 
            endpoint.description))?;
        
        // Log whether the endpoint uses a custom handler or the default
        if handler_registry.contains(&endpoint.handler) {
            log_tx.send(format!("  Using custom handler: {}", endpoint.handler))?;
        } else {
            log_tx.send(format!("  Using default handler with static response"))?;
        }
    }

    // Create server info for the UI
    let server_info = ServerInfo {
        host: config.server.host.clone(),
        port: config.server.port,
        endpoints: config.endpoints.iter().map(|e| EndpointInfo {
            path: e.path.clone(),
            method: e.method.clone(),
            description: e.description.clone(),
        }).collect(),
    };

    // Share the log sender, config, and handler registry across routes using Arc
    let state = Arc::new(AppState {
        log_sender: log_tx,
        config: config.clone(),
        handler_registry,
    });

    // Create the router with dynamic routes
    let app = create_router(state);

    // Set up the server to listen on the configured address
    let addr = config.get_socket_addr()?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    // Spawn the UI thread with server info
    let ui_thread = thread::spawn(move || {
        run_ui(log_rx, shutdown_tx, Some(server_info));
    });

    // Run the server in the main thread with graceful shutdown
    axum::serve(listener, app.into_make_service()).with_graceful_shutdown(async move {
        shutdown_rx.changed().await.ok();
    }).await?;

    // Wait for the UI thread to finish
    ui_thread.join().unwrap();
    
    Ok(())
} 
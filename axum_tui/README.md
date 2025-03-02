# Axum TUI

A Terminal User Interface (TUI) library for Axum web applications, built with Ratatui and Crossterm.

## Features

- **Multi-tab Interface**:
  - Logs tab for real-time log display
  - Server Info tab for endpoint documentation
  
- **Log Management**:
  - Real-time log display
  - Log filtering with search
  - Timestamp toggling
  - Log export to file
  
- **Server Information Display**:
  - Host and port information
  - Endpoint documentation
  - Server statistics
  
- **Interactive UI**:
  - Keyboard navigation
  - Tab switching
  - Help screen

## Installation

Add the library to your Cargo.toml:

```toml
[dependencies]
axum_tui = { git = "https://github.com/yourusername/ratatui_axos_app" }
```

## Basic Usage

```rust
use axum_tui::{run_ui, ServerInfo, EndpointInfo};
use std::sync::mpsc;
use tokio::sync::watch;

// Create channels for logs and shutdown
let (log_tx, log_rx) = mpsc::channel::<String>();
let (shutdown_tx, shutdown_rx) = watch::channel::<()>(());

// Create server info (optional)
let server_info = ServerInfo {
    host: "127.0.0.1".to_string(),
    port: 3000,
    endpoints: vec![
        EndpointInfo {
            path: "/".to_string(),
            method: "GET".to_string(),
            description: "Root endpoint".to_string(),
        },
        // Add more endpoints as needed
    ],
};

// Run the UI in a separate thread
std::thread::spawn(move || {
    run_ui(log_rx, shutdown_tx, Some(server_info));
});

// Send logs from your application
log_tx.send("Server started".to_string()).unwrap();

// Wait for shutdown signal in your main application
shutdown_rx.changed().await.ok();
```

## API Reference

### `run_ui`

```rust
pub fn run_ui(
    log_rx: mpsc::Receiver<String>, 
    shutdown_tx: watch::Sender<()>,
    server_info: Option<ServerInfo>
)
```

Starts the terminal UI with the given parameters:

- `log_rx`: Receives log messages to display in the UI
- `shutdown_tx`: Sends a shutdown signal when the user presses 'q'
- `server_info`: Optional server configuration information to display in the Server Info tab

### `ServerInfo`

```rust
pub struct ServerInfo {
    pub host: String,
    pub port: u16,
    pub endpoints: Vec<EndpointInfo>,
}
```

Contains information about the server configuration.

### `EndpointInfo`

```rust
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
}
```

Contains information about an endpoint.

### `AppUi`

```rust
pub struct AppUi {
    // Fields omitted for brevity
}

impl AppUi {
    pub fn new() -> Self { /* ... */ }
    pub fn add_log(&mut self, message: String) { /* ... */ }
    pub fn clear_logs(&mut self) { /* ... */ }
    pub fn save_logs_to_file(&self) -> Result<(), std::io::Error> { /* ... */ }
    // Other methods...
}
```

The main UI state struct. You can use this directly for more control over the UI.

## Keyboard Controls

- **Tab Navigation**:
  - `Tab` - Next tab
  - `Shift+Tab` - Previous tab

- **Log Controls**:
  - `f` - Filter logs
  - `c` - Clear logs
  - `t` - Toggle timestamps
  - `Ctrl+s` - Save logs to file
  - `Up/Down` - Navigate logs
  - `PgUp/PgDown` - Scroll up/down 10 entries
  - `Home/End` - Jump to first/last log

- **General Controls**:
  - `h` - Toggle help screen
  - `q` - Quit application
  - `Esc` - Cancel filter/Close help

## Integration with Axum

This library is designed to work seamlessly with Axum web applications:

```rust
use axum::{Router, routing::get, extract::State};
use std::sync::{mpsc, Arc};
use tokio::sync::watch;
use axum_tui::{run_ui, ServerInfo, EndpointInfo};

// Create channels
let (log_tx, log_rx) = mpsc::channel::<String>();
let (shutdown_tx, mut shutdown_rx) = watch::channel::<()>(());

// Create app state with log sender
struct AppState {
    log_sender: mpsc::Sender<String>,
}

let state = Arc::new(AppState {
    log_sender: log_tx.clone(),
});

// Create server info
let server_info = ServerInfo {
    host: "127.0.0.1".to_string(),
    port: 3000,
    endpoints: vec![
        EndpointInfo {
            path: "/".to_string(),
            method: "GET".to_string(),
            description: "Root endpoint".to_string(),
        },
    ],
};

// Define routes
let app = Router::new()
    .route("/", get(root))
    .with_state(state);

// Run the UI in a separate thread
let ui_thread = std::thread::spawn(move || {
    run_ui(log_rx, shutdown_tx, Some(server_info));
});

// Define handler function
async fn root(State(state): State<Arc<AppState>>) -> &'static str {
    state.log_sender.send("Received request to /".to_string()).unwrap();
    "Hello, World!"
}

// Run the server with graceful shutdown
axum::serve(listener, app.into_make_service())
    .with_graceful_shutdown(async move {
        shutdown_rx.changed().await.ok();
    })
    .await
    .unwrap();

// Wait for the UI thread to finish
ui_thread.join().unwrap();
```

## Customization

### Custom Log Formatting

You can format your logs before sending them to the UI:

```rust
// Add timestamp and level
let formatted_log = format!("[INFO] {}: User logged in", chrono::Local::now());
log_tx.send(formatted_log).unwrap();
```

### Custom Server Info

You can update the server info dynamically:

```rust
let mut app = AppUi::new();

// Update server info
app.server_info = Some(ServerInfo {
    host: "updated-host".to_string(),
    port: 8080,
    endpoints: vec![/* ... */],
});
```

## License

MIT 
# Axum Server with Terminal UI

A configurable web server built with Axum that includes a Terminal User Interface (TUI) for real-time log monitoring and server information display.

## Overview

This application demonstrates a modern approach to building web servers in Rust with a focus on:

1. **Configuration-driven development**: Define your server endpoints in YAML without changing code
2. **Real-time monitoring**: View logs and server information in a terminal interface
3. **Separation of concerns**: UI components are in a reusable library
4. **Dynamic handler system**: Map endpoints to custom handlers or use static responses

## How It Works

### Architecture

The application consists of three main components:

1. **Web Server (Axum)**: 
   - Dynamically creates routes based on YAML configuration
   - Handles HTTP requests and generates responses
   - Sends log messages to the Terminal UI

2. **Terminal UI (Ratatui)**:
   - Displays real-time logs from the server
   - Shows server configuration and endpoint information
   - Provides filtering and navigation capabilities

3. **Handler System**:
   - Maps endpoint configurations to handler implementations
   - Supports both static responses and dynamic handlers
   - Allows for easy extension with custom handlers

### Key Components

- **Configuration System**: Loads server settings and endpoint definitions from YAML
- **Dynamic Router**: Creates routes based on the configuration
- **Handler Registry**: Maps handler names to implementations
- **Terminal UI**: Split into tabs for logs and server information
- **Event System**: Handles keyboard input and updates the UI

## Configuration

The application uses a YAML configuration file to define the server settings and endpoints.

### Server Configuration

```yaml
server:
  host: 127.0.0.1  # Server host address
  port: 3000       # Server port
```

### Endpoint Configuration

Each endpoint is defined with the following properties:

```yaml
endpoints:
  - path: /example           # URL path
    method: GET              # HTTP method (GET, POST, PUT, DELETE)
    handler: example_handler # Handler name (for reference)
    response: "Example"      # Response content (used by default handler)
    description: "Example endpoint" # Description for documentation
    params:                  # Optional parameters for custom handlers
      key: value
```

### Example Configuration

```yaml
server:
  host: 127.0.0.1
  port: 3000

endpoints:
  - path: /
    method: GET
    handler: root
    response: "Hello, World!"
    description: "Root endpoint that returns a simple greeting"
    
  - path: /health
    method: GET
    handler: health
    response: "OK"
    description: "Health check endpoint for monitoring"
    
  - path: /api/status
    method: GET
    handler: status
    response: "{ \"status\": \"running\", \"version\": \"1.0.0\" }"
    description: "API status endpoint that returns JSON"
```

## Handler System

The application includes a flexible handler system that allows you to map endpoints to custom handlers or use static responses.

### Default Handlers

The following handlers are included by default:

- **default**: Returns the static response from the configuration
- **health**: Returns "OK" for health checks
- **status**: Returns a JSON response with server status information

### Creating Custom Handlers

You can create custom handlers by implementing the `Handler` trait:

```rust
use axum_handlers::{Handler, HandlerContext, HandlerResult};
use async_trait::async_trait;

pub struct MyCustomHandler;

#[async_trait]
impl Handler for MyCustomHandler {
    async fn handle(&self, ctx: HandlerContext) -> HandlerResult {
        // Access path, params, and static response from ctx
        // Return a response or an error
        Ok(format!("Custom response for {}", ctx.path))
    }
}
```

### Registering Custom Handlers

Register your custom handlers in the registry:

```rust
let mut registry = HandlerRegistry::new();
registry.register("my_custom", MyCustomHandler);
```

## Running the Application

### Prerequisites

- Rust and Cargo (latest stable version)

### Building

```bash
# Clone the repository
git clone https://github.com/NeoTech/ratatui_axum_server.git
cd ratatui_axum_server

# Build the application
cargo build
```

### Running

```bash
# Run with default configuration
cargo run -p ratatui_axos_app

# Run with a specific configuration file
cargo run -p ratatui_axos_app -- --config path/to/config.yaml
```

### Testing the Server

While the application is running, you can test the endpoints:

```bash
# Using curl
curl http://localhost:3000/
curl http://localhost:3000/health
curl http://localhost:3000/api/status

# Using PowerShell
Invoke-WebRequest -Uri http://localhost:3000/ -UseBasicParsing
```

Each request will appear in the Terminal UI logs tab.

## Using the Terminal UI

The Terminal UI provides a real-time view of your server's activity and configuration.

### Tabs

- **Logs**: Displays real-time logs of server activity
- **Server Info**: Shows server configuration and endpoint details

### Keyboard Controls

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

## Extending the Application

### Adding Custom Handlers

To add custom handlers:

1. Create a new handler that implements the `Handler` trait
2. Register the handler in the registry
3. Reference the handler in your YAML configuration

### Adding Middleware

To add middleware support:

1. Add middleware configuration to the YAML schema
2. Update the router creation in `routes.rs` to apply middleware based on configuration

## License

MIT 
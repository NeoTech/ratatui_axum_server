pub mod app;
pub mod ui;
mod util;

pub use app::{AppUi, ServerInfo, EndpointInfo};

use std::io::stdout;
use std::sync::mpsc;
use tokio::sync::watch;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

pub fn run_ui(
    log_rx: mpsc::Receiver<String>, 
    shutdown_tx: watch::Sender<()>,
    server_info: Option<ServerInfo>
) {
    // Set up the terminal with Crossterm backend
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear().unwrap();

    // Initialize app state
    let mut app = AppUi::new();
    
    // Set server info if provided
    if let Some(info) = server_info {
        app.server_info = Some(info);
    }

    // Main UI loop
    loop {
        // Draw the UI
        terminal
            .draw(|f| ui::draw_ui(f, &mut app))
            .unwrap();

        // Handle events, break the loop if handle_events returns false
        if !ui::handle_events(&mut app, &log_rx, &shutdown_tx) {
            break;
        }
    }

    // Clean up the terminal
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
} 
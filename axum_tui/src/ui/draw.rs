use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph, Tabs, Wrap, ListItem, Clear};
use ratatui::layout::{Layout, Direction, Constraint};
use crate::app::{AppUi, AppMode};
use crate::util::centered_rect;

pub fn draw_ui(f: &mut Frame, app: &mut AppUi) {
    let size = f.size();
    
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title bar
            Constraint::Min(1),     // Logs area
            Constraint::Length(1),  // Status bar
        ])
        .split(size);
    
    // Title bar with tabs
    let titles = vec!["Logs", "Server Info"];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Axum Server Monitor"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.selected_tab);
    f.render_widget(tabs, chunks[0]);
    
    // Main content area
    match app.mode {
        AppMode::Normal | AppMode::Filter => {
            // Display content based on selected tab
            match app.selected_tab {
                0 => draw_logs(f, app, chunks[1]),
                1 => draw_server_info(f, chunks[1], app),
                _ => {}
            }
            
            // If in filter mode, show the filter input
            if let AppMode::Filter = app.mode {
                let filter_area = centered_rect(60, 3, size);
                let filter_text = Paragraph::new(format!("Filter: {}", app.filter_input))
                    .block(Block::default().title("Enter Filter").borders(Borders::ALL))
                    .style(Style::default().fg(Color::Yellow));
                f.render_widget(Clear, filter_area);
                f.render_widget(filter_text, filter_area);
            }
        },
        AppMode::Help => {
            // First render the content based on selected tab
            match app.selected_tab {
                0 => draw_logs(f, app, chunks[1]),
                1 => draw_server_info(f, chunks[1], app),
                _ => {}
            }
            
            // Then render help screen as an overlay
            draw_help(f, size);
        }
    }
    
    // Status bar
    draw_status_bar(f, app, chunks[2]);
}

fn draw_logs(f: &mut Frame, app: &mut AppUi, area: Rect) {
    let log_items: Vec<ListItem> = app.filtered_logs.iter()
        .map(|&idx| {
            let log = &app.logs[idx];
            let content = if app.show_timestamps {
                format!("[{}] {}", app.format_timestamp(log.timestamp), log.message)
            } else {
                log.message.clone()
            };
            ListItem::new(content)
        })
        .collect();
    
    let logs_list = List::new(log_items)
        .block(Block::default()
            .title(format!("Logs ({}/{})", app.filtered_logs.len(), app.logs.len()))
            .borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">> ");
    
    let mut list_state = ListState::default();
    if !app.filtered_logs.is_empty() {
        list_state.select(Some(app.scroll));
    }
    
    f.render_stateful_widget(logs_list, area, &mut list_state);
}

fn draw_help(f: &mut Frame, size: Rect) {
    let help_text = vec![
        "Keyboard Controls:",
        "",
        "q       - Quit application",
        "h       - Toggle help screen",
        "c       - Clear logs",
        "t       - Toggle timestamps",
        "f       - Filter logs",
        "Tab     - Next tab",
        "Shift+Tab - Previous tab",
        "Ctrl+s  - Save logs to file",
        "Esc     - Cancel filter/Close help",
        "Up/Down - Navigate logs",
        "PgUp    - Scroll up 10 entries",
        "PgDown  - Scroll down 10 entries",
        "Home    - Jump to first log",
        "End     - Jump to last log",
    ].join("\n");
    
    // Create a floating help panel
    let help_area = {
        let popup_width = 60;
        let popup_height = 16;
        
        let x = (size.width.saturating_sub(popup_width)) / 2;
        let y = (size.height.saturating_sub(popup_height)) / 2;
        
        Rect {
            x,
            y,
            width: popup_width,
            height: popup_height,
        }
    };
    
    f.render_widget(Clear, help_area);
    
    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default().title("Help").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    
    f.render_widget(help_paragraph, help_area);
}

fn draw_status_bar(f: &mut Frame, app: &AppUi, area: Rect) {
    let status = match app.mode {
        AppMode::Normal => {
            match app.selected_tab {
                0 => format!("Logs Tab | Logs: {} | Filter: {} | Press 'h' for help", 
                          app.logs.len(), 
                          if app.filter.is_empty() { "None".to_string() } else { app.filter.clone() }),
                1 => "Server Info Tab | Server: localhost:3000 | Press 'h' for help".to_string(),
                _ => "Press 'h' for help".to_string(),
            }
        },
        AppMode::Help => "Help Mode | Press Esc to return".to_string(),
        AppMode::Filter => "Filter Mode | Enter filter text, press Enter to apply, Esc to cancel".to_string(),
    };
    
    let status_bar = Paragraph::new(status)
        .style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(status_bar, area);
}

fn draw_server_info(f: &mut Frame, area: Rect, app: &AppUi) {
    let mut info_lines = Vec::new();
    
    info_lines.push("Server Information".to_string());
    info_lines.push("".to_string());
    
    if let Some(server_info) = &app.server_info {
        info_lines.push(format!("Host: {}", server_info.host));
        info_lines.push(format!("Port: {}", server_info.port));
        info_lines.push("Status: Running".to_string());
        info_lines.push("".to_string());
        
        info_lines.push("Available Endpoints:".to_string());
        for endpoint in &server_info.endpoints {
            info_lines.push(format!("  - {} {} : {}", 
                endpoint.method, 
                endpoint.path, 
                endpoint.description));
        }
    } else {
        info_lines.push("Host: 127.0.0.1".to_string());
        info_lines.push("Port: 3000".to_string());
        info_lines.push("Status: Running".to_string());
        info_lines.push("".to_string());
        
        info_lines.push("Available Endpoints:".to_string());
        info_lines.push("  - GET / : Returns 'Hello, World!'".to_string());
        info_lines.push("  - GET /health : Returns 'OK'".to_string());
        info_lines.push("  - GET /helloworld : Returns 'Hello, World!'".to_string());
    }
    
    info_lines.push("".to_string());
    info_lines.push("Server Statistics:".to_string());
    info_lines.push(format!("  - Uptime: Since application start"));
    info_lines.push(format!("  - Total Logs: {}", app.logs.len()));
    
    let server_info = info_lines.join("\n");
    
    let info_paragraph = Paragraph::new(server_info)
        .block(Block::default().title("Server Info").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    
    f.render_widget(info_paragraph, area);
} 
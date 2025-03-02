use crossterm::event::{self, Event, KeyCode, KeyModifiers, KeyEventKind};
use tokio::sync::watch;
use std::sync::mpsc;
use std::time::Duration;

use crate::app::{AppUi, AppMode};

pub fn handle_events(
    app: &mut AppUi, 
    log_rx: &mpsc::Receiver<String>,
    shutdown_tx: &watch::Sender<()>
) -> bool {
    // Collect new log messages
    while let Ok(msg) = log_rx.try_recv() {
        app.add_log(msg);
    }

    // Handle keyboard events
    if event::poll(Duration::from_millis(100)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            // Only process key press events, not key release events
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char('q') => {
                            shutdown_tx.send(()).ok();
                            return false; // Signal to exit the loop
                        },
                        KeyCode::Char('h') => {
                            app.mode = AppMode::Help;
                        },
                        KeyCode::Char('c') => {
                            app.clear_logs();
                        },
                        KeyCode::Char('t') => {
                            app.show_timestamps = !app.show_timestamps;
                        },
                        KeyCode::Char('f') => {
                            app.mode = AppMode::Filter;
                            app.filter_input = app.filter.clone();
                        },
                        KeyCode::Tab => {
                            app.selected_tab = (app.selected_tab + 1) % 2; // Cycle through tabs
                        },
                        KeyCode::BackTab => {
                            app.selected_tab = if app.selected_tab > 0 { app.selected_tab - 1 } else { 1 };
                        },
                        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            match app.save_logs_to_file() {
                                Ok(_) => {
                                    app.add_log("Logs saved to server_logs.txt".to_string());
                                },
                                Err(e) => {
                                    app.add_log(format!("Error saving logs: {}", e));
                                }
                            }
                        },
                        KeyCode::Up => {
                            if app.scroll > 0 {
                                app.scroll -= 1;
                            }
                        },
                        KeyCode::Down => {
                            if !app.filtered_logs.is_empty() && app.scroll < app.filtered_logs.len() - 1 {
                                app.scroll += 1;
                            }
                        },
                        KeyCode::PageUp => {
                            if app.scroll > 10 {
                                app.scroll -= 10;
                            } else {
                                app.scroll = 0;
                            }
                        },
                        KeyCode::PageDown => {
                            if !app.filtered_logs.is_empty() {
                                if app.scroll + 10 < app.filtered_logs.len() - 1 {
                                    app.scroll += 10;
                                } else {
                                    app.scroll = app.filtered_logs.len() - 1;
                                }
                            }
                        },
                        KeyCode::Home => {
                            app.scroll = 0;
                        },
                        KeyCode::End => {
                            if !app.filtered_logs.is_empty() {
                                app.scroll = app.filtered_logs.len() - 1;
                            }
                        },
                        _ => {}
                    },
                    AppMode::Help => {
                        if key.code == KeyCode::Esc || key.code == KeyCode::Char('h') {
                            app.mode = AppMode::Normal;
                        }
                    },
                    AppMode::Filter => match key.code {
                        KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                            app.filter_input = String::new();
                        },
                        KeyCode::Enter => {
                            app.apply_filter();
                        },
                        KeyCode::Char(c) => {
                            app.filter_input.push(c);
                        },
                        KeyCode::Backspace => {
                            app.filter_input.pop();
                        },
                        _ => {}
                    },
                }
            }
        }
    }
    
    true // Continue the loop
} 
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::io::Write;
use std::fs::File;

pub struct LogEntry {
    pub timestamp: u64,
    pub message: String,
}

pub enum AppMode {
    Normal,
    Help,
    Filter,
}

#[derive(Clone)]
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
}

#[derive(Clone)]
pub struct ServerInfo {
    pub host: String,
    pub port: u16,
    pub endpoints: Vec<EndpointInfo>,
}

pub struct AppUi {
    pub logs: Vec<LogEntry>,
    pub filtered_logs: Vec<usize>,
    pub scroll: usize,
    pub filter: String,
    pub filter_input: String,
    pub mode: AppMode,
    pub show_timestamps: bool,
    pub selected_tab: usize,
    pub server_info: Option<ServerInfo>,
}

impl AppUi {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            filtered_logs: Vec::new(),
            scroll: 0,
            filter: String::new(),
            filter_input: String::new(),
            mode: AppMode::Normal,
            show_timestamps: true,
            selected_tab: 0,
            server_info: None,
        }
    }

    pub fn add_log(&mut self, message: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let log_entry = LogEntry {
            timestamp,
            message,
        };
        
        self.logs.push(log_entry);
        
        if self.logs.len() > 1000 {
            self.logs.remove(0);
        }
        
        self.update_filtered_logs();
    }
    
    pub fn update_filtered_logs(&mut self) {
        if self.filter.is_empty() {
            self.filtered_logs = (0..self.logs.len()).collect();
        } else {
            self.filtered_logs = self.logs.iter().enumerate()
                .filter(|(_, log)| log.message.to_lowercase().contains(&self.filter.to_lowercase()))
                .map(|(i, _)| i)
                .collect();
        }
        
        if !self.filtered_logs.is_empty() && self.scroll >= self.filtered_logs.len() {
            self.scroll = self.filtered_logs.len() - 1;
        }
    }
    
    pub fn apply_filter(&mut self) {
        self.filter = self.filter_input.clone();
        self.update_filtered_logs();
        self.mode = AppMode::Normal;
    }
    
    pub fn clear_logs(&mut self) {
        self.logs.clear();
        self.filtered_logs.clear();
        self.scroll = 0;
    }
    
    pub fn format_timestamp(&self, timestamp: u64) -> String {
        let secs = timestamp % 60;
        let mins = (timestamp / 60) % 60;
        let hours = (timestamp / 3600) % 24;
        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    }
    
    pub fn save_logs_to_file(&self) -> Result<(), std::io::Error> {
        let mut file = File::create("server_logs.txt")?;
        
        writeln!(file, "Server Logs Export")?;
        writeln!(file, "===================")?;
        writeln!(file, "")?;
        
        for idx in &self.filtered_logs {
            let log = &self.logs[*idx];
            if self.show_timestamps {
                writeln!(file, "[{}] {}", self.format_timestamp(log.timestamp), log.message)?;
            } else {
                writeln!(file, "{}", log.message)?;
            }
        }
        
        Ok(())
    }
} 
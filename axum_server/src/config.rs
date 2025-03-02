use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndpointConfig {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub response: String,
    pub description: String,
    #[serde(default)]
    pub params: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub endpoints: Vec<EndpointConfig>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
    
    pub fn get_socket_addr(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        let ip = IpAddr::from_str(&self.server.host)?;
        Ok(SocketAddr::new(ip, self.server.port))
    }
} 
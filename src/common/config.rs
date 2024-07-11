use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub log: LogSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct LogSettings {
    pub level: String,
}

impl Settings {
    pub fn new(config_file: &str) -> Self {
        let config_str = fs::read_to_string(config_file)
            .unwrap_or_else(|_| panic!("Failed to read configuration file: {}", config_file));
        toml::from_str(&config_str).unwrap_or_else(|_| panic!("Failed to parse configuration file: {}", config_file))
    }
}

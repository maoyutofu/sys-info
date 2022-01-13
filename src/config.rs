use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sys: Sys,
    pub http: Http,
    pub notification: Notification,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub timer: u64,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub bind: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Notification {
    pub enable: bool,
    pub url: String,
    pub interval: u64,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;
        Ok(config)
    }
}

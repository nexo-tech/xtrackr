use serde::Deserialize;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub privacy: PrivacyConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PrivacyConfig {
    pub data_retention_days: i64,
    pub anonymize_ip: bool,
    pub cookie_lifetime_days: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {

        let cfg = config::Config::builder()
        .add_source(config::Environment::default().separator("__"))
        .build()?;

        cfg.try_deserialize()
    }
} 

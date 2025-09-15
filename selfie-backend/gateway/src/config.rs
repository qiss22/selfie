use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub metrics_addr: String,
    pub service_discovery: ServiceDiscoveryConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServiceDiscoveryConfig {
    pub auth_service: String,
    pub user_service: String,
    pub post_service: String,
    pub media_service: String,
    pub chat_service: String,
    // Add other services as needed
}

pub fn load_config() -> Result<Config> {
    let config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()?;

    Ok(config.try_deserialize()?)
}
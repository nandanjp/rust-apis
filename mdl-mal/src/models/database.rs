use crate::config::MongoConfig;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct DatabaseConfig {
    pub uri: String,
    pub connection_timeout: Duration,
    pub min_pool_size: u32,
    pub max_pool_size: u32,
}

impl DatabaseConfig {
    pub fn new(config: &MongoConfig) -> Self {
        Self {
            uri: config.get_uri().clone(),
            connection_timeout: Duration::from_secs(config.get_connection() as u64),
            min_pool_size: config.get_min_pool(),
            max_pool_size: config.get_max_pool(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Order {
    Asc,
    Desc,
}
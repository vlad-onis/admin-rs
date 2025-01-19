use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub connection_url: String,
    pub max_connections: u32,
}

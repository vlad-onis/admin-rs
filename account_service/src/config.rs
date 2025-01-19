use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
use thiserror::Error;

use crate::storage::config::DatabaseConfig;

pub const CONFIG_FILE_PATH: &str = "./config.toml";
pub const CARGO_MANIFEST_DIRECTORY_KEY: &str = "CARGO_MANIFEST_DIR";

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("ENV error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Config file is missing")]
    MissingConfig,

    #[error("Failed to deserialize the config: {0}")]
    ConfigDeserialization(toml::de::Error),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
}

pub fn extract_config(config_path: PathBuf) -> Result<Config, Error> {
    let cargo_dir =
        PathBuf::from_str(std::env::var(CARGO_MANIFEST_DIRECTORY_KEY)?.as_str()).unwrap();

    let config_file_path = cargo_dir.join(config_path);

    if !config_file_path.is_file() {
        return Err(Error::MissingConfig);
    }

    let content = std::fs::read_to_string(config_file_path)?;
    let config = toml::from_str::<Config>(&content).map_err(Error::ConfigDeserialization)?;
    Ok(config)
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn config_extraction_will_not_panic() {
        assert!(matches!(
            extract_config(PathBuf::from_str("🦀").unwrap()),
            Err(Error::MissingConfig)
        ));
    }

    #[test]
    fn config_extraction_works() {
        assert!(extract_config(PathBuf::from_str("./config.toml").unwrap()).is_ok());
    }
}

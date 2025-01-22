use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use thiserror::Error;

use super::config::DatabaseConfig;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database Error: {0}")]
    DB(#[from] sqlx::Error),
}

pub struct Storage<DB: sqlx::Database> {
    pub connection_pool: Pool<DB>,
    pub max_connections: u32,
}

impl Storage<Postgres> {
    pub async fn new(config: DatabaseConfig) -> Result<Storage<Postgres>, Error> {
        let connection_pool =
            Storage::<Postgres>::connect(&config.connection_url, config.max_connections).await?;

        Ok(Storage {
            connection_pool,
            max_connections: config.max_connections,
        })
    }

    async fn connect(connection_url: &str, max_connections: u32) -> Result<Pool<Postgres>, Error> {
        println!("Attempting to connect to: {}", connection_url);

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(connection_url)
            .await?;

        println!("Connected successfully");

        Ok(pool)
    }
}

#[cfg(test)]
pub mod tests {

    use sqlx::Sqlite;
    use tempfile::{Builder, NamedTempFile};

    use crate::storage::{config::DatabaseConfig, Storage};

    // taggin a function with this macro will make a file path available called temp_db
    // in your scope, and you can use that file path as an sqlite instance only for that test
    use test_setup_macro::with_temp_db;

    #[with_temp_db]
    async fn connection() {
        let config = DatabaseConfig {
            connection_url: temp_db.path().to_string_lossy().to_string(),
            max_connections: 5,
        };

        let _storage = Storage::<Sqlite>::new(config).await.unwrap();
    }
}

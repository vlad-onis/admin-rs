pub mod account_row;
pub mod account_storage;
pub mod config;
pub mod db_storage;

#[cfg(test)]
use crate::storage::{
    config::DatabaseConfig,
    db_storage::{Error, Storage},
};
#[cfg(test)]
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[cfg(test)]
impl Storage<Sqlite> {
    pub async fn new(config: DatabaseConfig) -> Result<Storage<Sqlite>, Error> {
        let connection_pool =
            Storage::<Sqlite>::connect(&config.connection_url, config.max_connections).await?;

        Ok(Storage {
            connection_pool,
            max_connections: config.max_connections,
        })
    }

    async fn connect(connection_url: &str, max_connections: u32) -> Result<Pool<Sqlite>, Error> {
        println!("Attempting to connect to: {}", connection_url);

        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(connection_url)
            .await?;

        println!("Connected successfully");

        Ok(pool)
    }
}

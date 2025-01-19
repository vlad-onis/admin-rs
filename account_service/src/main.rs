use config::extract_config;

use clap::Parser;
use cli_args::Args;

mod cli_args;
mod config;
mod storage;

use sqlx::Postgres;
use storage::db_storage::Storage;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = extract_config(args.config_file_path).expect("Failed to obtain config");

    println!("{config:?}");

    Storage::<Postgres>::new(config.database).await.unwrap();
}

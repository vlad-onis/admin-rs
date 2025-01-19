use crate::config::CONFIG_FILE_PATH;
use std::{convert::Infallible, path::PathBuf, str::FromStr};

use clap::Parser;

fn validate_path(p: &str) -> Result<PathBuf, Infallible> {
    // This is infallible
    let path_buf = PathBuf::from_str(p).unwrap();
    if path_buf.exists() && path_buf.is_file() {
        return Ok(path_buf);
    }

    // todo: this should be a log
    println!(
        "Failed to parse the config path provided: \
        it does not exist or it is not a file, using default value {CONFIG_FILE_PATH}"
    );

    // from_str is Infallible here
    Ok(PathBuf::from_str(CONFIG_FILE_PATH).unwrap())
}

#[derive(Parser)]
pub struct Args {
    #[clap(
        short,
        long,
        value_parser = validate_path,
        default_value = CONFIG_FILE_PATH
    )]
    pub config_file_path: PathBuf,
}

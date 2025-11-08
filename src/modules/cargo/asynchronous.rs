use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::modules::cargo::models::cargo::Cargo;
use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcherError as CargoFileWatcherError;

pub type CargoFileWatcher = LiveJsonFileWatcher<Cargo>;

/// Read the contents of the 'Cargo.json' file at the given path.
pub async fn read_cargo_file<P: AsRef<Path>>(path: P) -> Result<Cargo, ReadCargoFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path).await?)?)
}

#[derive(Debug, Error)]
pub enum ReadCargoFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse cargo file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

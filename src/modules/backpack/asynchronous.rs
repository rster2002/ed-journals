use std::path::Path;
use thiserror::Error;
use tokio::fs;
use crate::backpack::models::backpack::Backpack;
use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcher;

pub type BackpackFileWatcher = LiveJsonFileWatcher<Backpack>;
pub use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcherError as BackpackFileWatcherError;

pub async fn read_backpack_file<P: AsRef<Path>>(path: P) -> Result<Backpack, ReadBackpackFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)
        .await?)?)
}

#[derive(Debug, Error)]
pub enum ReadBackpackFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse market file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

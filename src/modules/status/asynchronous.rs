use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::modules::shared::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::live_json_file_watcher::LiveJsonFileWatcherError as StatusFileWatcherError;
use crate::status::Status;

pub type StatusFileWatcher = LiveJsonFileWatcher<Status>;

pub async fn read_status_file<P: AsRef<Path>>(path: P) -> Result<Status, ReadStatusFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path).await?)?)
}

#[derive(Debug, Error)]
pub enum ReadStatusFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse status file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

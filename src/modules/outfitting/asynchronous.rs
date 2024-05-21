use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::modules::outfitting::Outfitting;
use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcherError as OutfittingFileWatcherError;

pub type OutfittingFileWatcher = LiveJsonFileWatcher<Outfitting>;

pub async fn read_outfitting_file<P: AsRef<Path>>(
    path: P,
) -> Result<Outfitting, ReadOutfittingFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path).await?)?)
}

#[derive(Debug, Error)]
pub enum ReadOutfittingFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse outfitting file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcher;
use std::path::Path;
use thiserror::Error;
use tokio::fs;

pub type ShipLockerFileWatcher = LiveJsonFileWatcher<ShipLocker>;
pub use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcherError as ShipLockerFileWatcherError;
use crate::ship_locker::ShipLocker;

pub async fn read_ship_locker_file<P: AsRef<Path>>(
    path: P,
) -> Result<ShipLocker, ReadShipLockerFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path).await?)?)
}

#[derive(Debug, Error)]
pub enum ReadShipLockerFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse ship locker file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

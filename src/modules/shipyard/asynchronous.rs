use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::modules::shared::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::live_json_file_watcher::LiveJsonFileWatcherError as ShipyardFileWatcherError;
use crate::modules::shipyard::models::shipyard::Shipyard;

pub type ShipyardFileWatcher = LiveJsonFileWatcher<Shipyard>;

pub async fn read_shipyard_file<P: AsRef<Path>>(
    path: P,
) -> Result<Shipyard, ReadShipyardFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path).await?)?)
}

#[derive(Debug, Error)]
pub enum ReadShipyardFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse shipyard file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

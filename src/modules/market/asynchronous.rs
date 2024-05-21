use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::modules::market::Market;
use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcherError as MarketFileWatcherError;

pub type MarketFileWatcher = LiveJsonFileWatcher<Market>;

pub async fn read_market_file<P: AsRef<Path>>(path: P) -> Result<Market, ReadMarketFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path).await?)?)
}

#[derive(Debug, Error)]
pub enum ReadMarketFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse market file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

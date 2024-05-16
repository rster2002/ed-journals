use std::path::Path;
use thiserror::Error;
use tokio::fs;
use crate::modules::nav_route::models::nav_route::NavRoute;
use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcher;

pub type NavRouteFileWatcher = LiveJsonFileWatcher<NavRoute>;
pub use crate::modules::shared::asynchronous::live_json_file_watcher::LiveJsonFileWatcherError as NavRouteFileWatcherError;

pub async fn read_nav_route_file<P: AsRef<Path>>(path: P) -> Result<NavRoute, ReadNavRouteFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)
        .await?)?)
}

#[derive(Debug, Error)]
pub enum ReadNavRouteFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse nav route file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

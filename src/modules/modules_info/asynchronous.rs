use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::modules::shared::live_json_file_watcher::LiveJsonFileWatcher;
pub use crate::modules::shared::live_json_file_watcher::LiveJsonFileWatcherError as ModulesInfoFileWatcherError;
use crate::modules_info::models::modules_info::ModulesInfo;

pub type ModulesInfoFileWatcher = LiveJsonFileWatcher<ModulesInfo>;

pub async fn read_modules_info_file<P: AsRef<Path>>(
    path: P,
) -> Result<ModulesInfo, ReadModulesInfoFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path).await?)?)
}

#[derive(Debug, Error)]
pub enum ReadModulesInfoFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse modules info file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

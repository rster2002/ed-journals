pub mod live_iter;
pub mod log_iter;

#[cfg(feature = "asynchronous")]
pub mod async_iter;

use std::path::{Path, PathBuf};
use crate::modules::logs2::error::LogError;
use crate::modules::logs2::models::log_dir::log_path::LogPath;
use crate::modules::logs2::models::log_file::live_iter::LiveIter;
use crate::modules::logs2::models::log_file::log_iter::LogIter;

#[cfg(feature = "asynchronous")]
use crate::modules::logs2::models::log_file::async_iter::AsyncIter;

#[derive(Debug)]
pub struct LogFile {
    path: PathBuf,
}

impl LogFile {
    pub fn new<P: AsRef<Path>>(path: P) -> LogFile {
        LogFile {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn iter(&self) -> Result<LogIter<std::io::BufReader<std::fs::File>>, LogError> {
        let file = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(file);

        Ok(LogIter::from(reader))
    }

    pub fn live_iter(&self) -> Result<LiveIter, LogError> {
        LiveIter::open(&self.path)
    }

    #[cfg(feature = "asynchronous")]
    pub async fn async_iter(&self) -> Result<AsyncIter<tokio_util::compat::Compat<tokio::io::BufReader<tokio::fs::File>>>, LogError> {
        let file = tokio::fs::File::open(&self.path)
            .await?;

        let reader = tokio::io::BufReader::new(file);

        Ok(AsyncIter::from(reader))
    }
}

impl From<LogPath> for LogFile {
    fn from(value: LogPath) -> Self {
        LogFile {
            path: value.into(),
        }
    }
}
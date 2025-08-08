use std::io;
use std::path::Path;
use thiserror::Error;

use crate::logs::asynchronous::LogFileReaderError;
use crate::logs::content::LogEvent;

use super::RawLiveLogFileReader;

#[derive(Debug)]
pub struct LiveLogFileReader {
    inner: RawLiveLogFileReader,
}

#[derive(Debug, Error)]
pub enum LiveLogFileReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),

    #[error(transparent)]
    LogFileReaderError(#[from] LogFileReaderError),
}

impl LiveLogFileReader {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveLogFileReaderError> {
        Ok(LiveLogFileReader {
            inner: RawLiveLogFileReader::open(path).await?,
        })
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LogFileReaderError>> {
        let result = match self.inner.next().await? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };

        Some(serde_json::from_value(result).map_err(|e| e.into()))
    }
}

use std::path::Path;

use thiserror::Error;

use crate::logs::asynchronous::LogFileReaderError;
use crate::logs::content::LogEvent;
use crate::logs::{LogDirError, LogFileError};

use super::RawLogDirReader;

#[derive(Debug)]
pub struct LogDirReader {
    inner: RawLogDirReader,
}

#[derive(Debug, Error)]
pub enum LogDirReaderError {
    #[error(transparent)]
    LogFileReaderError(#[from] LogFileReaderError),

    #[error(transparent)]
    LogDirError(#[from] LogDirError),

    #[error(transparent)]
    LogFileError(#[from] LogFileError),
}

impl LogDirReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        LogDirReader {
            inner: RawLogDirReader::open(path),
        }
    }

    pub fn is_reading_latest(&self) -> bool {
        self.inner.is_live()
    }

    pub fn is_failing(&self) -> bool {
        self.inner.is_failing()
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LogDirReaderError>> {
        let result = match self.inner.next().await? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        let raw_json = result.to_string();
        Some(
            serde_json::from_value(result)
                .map_err(|error| LogFileReaderError::FailedToParseLine { error, raw_json }.into()),
        )
    }
}

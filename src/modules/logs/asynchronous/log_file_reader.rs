use std::io;
use std::path::Path;

use thiserror::Error;

use crate::logs::content::LogEvent;

use super::RawLogFileReader;

#[derive(Debug)]
pub struct LogFileReader {
    inner: RawLogFileReader,
}

#[derive(Debug, Error)]
pub enum LogFileReaderError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("Failed to parse log line: {0}")]
    FailedToParseLine(#[from] serde_json::Error),
}

impl LogFileReader {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self, LogFileReaderError> {
        Ok(LogFileReader {
            inner: RawLogFileReader::open(path).await?,
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

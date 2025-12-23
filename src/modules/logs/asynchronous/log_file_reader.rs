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

    #[error("Failed to parse log line: {error}\nRaw JSON: {raw_json}")]
    FailedToParseLine {
        error: serde_json::Error,
        raw_json: String,
    },
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

        let raw_json = result.to_string();
        Some(serde_json::from_value(result).map_err(|error| {
            LogFileReaderError::FailedToParseLine { error, raw_json }
        }))
    }
}

use std::path::Path;

use thiserror::Error;

use crate::logs::blocking::LogFileReaderError;
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
    LogFileError(#[from] LogFileError),

    #[error(transparent)]
    LogDirError(#[from] LogDirError),

    #[error(transparent)]
    LogFileReaderError(#[from] LogFileReaderError),
}

impl LogDirReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        LogDirReader {
            inner: RawLogDirReader::open(path),
        }
    }

    pub fn is_reading_latest(&self) -> bool {
        self.inner.is_reading_latest()
    }

    pub fn is_failing(&self) -> bool {
        self.inner.is_failing()
    }
}

impl Iterator for LogDirReader {
    type Item = Result<LogEvent, LogDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.inner.next()? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        Some(
            serde_json::from_value(result)
                .map_err(|e| LogFileReaderError::FailedToParseLine(e).into()),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::logs::blocking::LogDirReader;

    #[test]
    fn all_entries_are_read_correctly() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let reader = LogDirReader::open(dir_path);

        let mut count = 0;
        for _ in reader {
            count += 1;
        }

        assert!(count >= 870929);
    }
}

use std::path::Path;
use thiserror::Error;

use crate::logs::asynchronous::log_dir_reader::LogDirReaderError;
use crate::logs::content::LogEvent;

use super::{LogFileReaderError, RawLiveLogDirReader};

/// The async variant of [super::blocking::LiveLogDirReader]. Watches the whole journal dir and
/// reads all files. Once all historic files have been read the current read will only resolve once
/// the newest log file is changed at which it will read the active log file and return the entry.
///
/// ```rust
/// # use std::env::current_dir;
/// use std::path::PathBuf;
/// use ed_journals::logs::asynchronous::LiveLogDirReader;
///
/// # tokio_test::block_on(async {
/// let path = PathBuf::from("somePath");
/// # let path = current_dir()
/// #    .unwrap()
/// #    .join("test-files")
/// #    .join("journals");
/// let mut live_dir_reader = LiveLogDirReader::open(path)
///     .unwrap();
///
/// // At first this will read all existing lines from the journal logs, after which it will wait
/// // until it detects new entries in the latest log file.
/// while let Some(entry) = live_dir_reader.next().await {
///     // Do something with the entry
///     # break;
/// }
/// # });
/// ```
#[derive(Debug)]
pub struct LiveLogDirReader {
    inner: RawLiveLogDirReader,
}

#[derive(Debug, Error)]
pub enum LiveLogDirReaderError {
    #[error(transparent)]
    LogDirReaderError(#[from] LogDirReaderError),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveLogDirReader {
    pub fn open<P: AsRef<Path>>(dir_path: P) -> Result<LiveLogDirReader, LiveLogDirReaderError> {
        Ok(LiveLogDirReader {
            inner: RawLiveLogDirReader::open(dir_path)?,
        })
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LiveLogDirReaderError>> {
        let result = match self.inner.next().await? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        Some(serde_json::from_value(result).map_err(|e| {
            LiveLogDirReaderError::LogDirReaderError(
                LogFileReaderError::FailedToParseLine(e).into(),
            )
        }))
    }
}

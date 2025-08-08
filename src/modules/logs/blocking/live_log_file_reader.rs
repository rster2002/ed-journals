use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use thiserror::Error;

use crate::logs::content::LogEvent;
use crate::modules::logs::blocking::LogFileReaderError;
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

use super::RawLiveLogFileReader;

/// Allows you to iterate over a journal log file and blocks when there are no entries to read, then
/// when the file changes it will unblock and return the new line(s).
///
/// ```no_run
/// use std::path::PathBuf;
/// use ed_journals::logs::blocking::LiveLogFileReader;
///
/// let path = PathBuf::from("somePath");
///
/// let live_reader = LiveLogFileReader::open(path)
///     .unwrap();
///
/// // This will block the current thread until there are new entries.
/// for entry in live_reader {
///     // Do something with the entry
/// }
/// ```
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
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveLogFileReaderError> {
        Ok(LiveLogFileReader {
            inner: RawLiveLogFileReader::open(path)?,
        })
    }

    pub fn handle(&self) -> LiveLogFileHandle {
        self.inner.handle()
    }
}

#[derive(Debug, Clone)]
pub struct LiveLogFileHandle {
    pub(super) active: Arc<AtomicBool>,
    pub(super) blocker: SyncBlocker,
}

impl LiveLogFileHandle {
    pub fn stop(&self) {
        self.active.swap(false, Ordering::Relaxed);
        self.blocker.unblock();
    }
}

impl Iterator for LiveLogFileReader {
    type Item = Result<LogEvent, LogFileReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.inner.next()? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };

        Some(serde_json::from_value(result).map_err(|e| e.into()))
    }
}

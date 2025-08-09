use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use thiserror::Error;

use crate::logs::blocking::log_dir_reader::LogDirReaderError;
use crate::logs::content::LogEvent;
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

use super::RawLiveLogDirReader;

/// Watches the whole journal dir and reads all files. Once all historic files have been read it
/// will block the current thread until the newest log file is changed at which it will read the
/// active log file and return the entry.
///
/// ```no_run
/// use std::path::PathBuf;
/// use ed_journals::logs::blocking::LiveLogDirReader;
///
/// let path = PathBuf::from("somePath");
///
/// let live_dir_reader = LiveLogDirReader::open(path)
///     .unwrap();
///
/// // At first this will read all existing lines from the journal logs, after which it will block
/// // the current thread until it detects new entries in the latest log file.
/// for entry in live_dir_reader {
///     // Do something with the entry
/// }
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
        Ok(Self {
            inner: RawLiveLogDirReader::open(dir_path)?,
        })
    }

    pub fn handle(&self) -> LiveLogDirHandle {
        self.inner.handle()
    }
}

pub struct LiveLogDirHandle {
    active: Arc<AtomicBool>,
    blocker: SyncBlocker,
}

impl LiveLogDirHandle {
    pub fn new(active: Arc<AtomicBool>, blocker: SyncBlocker) -> Self {
        LiveLogDirHandle { active, blocker }
    }

    pub fn close(&self) {
        self.active.swap(false, Ordering::Relaxed);
        self.blocker.unblock();
    }
}

impl Iterator for LiveLogDirReader {
    type Item = Result<LogEvent, LiveLogDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.inner.next()? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        Some(serde_json::from_value(result).map_err(|e| {
            LiveLogDirReaderError::LogDirReaderError(LogDirReaderError::LogFileReaderError(
                super::LogFileReaderError::FailedToParseLine(e),
            ))
        }))
    }
}

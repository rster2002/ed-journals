use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::{path::Path, sync::atomic::Ordering};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

use super::{LiveLogFileHandle, LiveLogFileReaderError, LogFileReaderError, RawLogFileReader};

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
pub struct RawLiveLogFileReader {
    blocker: SyncBlocker,
    log_file_reader: RawLogFileReader,
    _watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
}

impl RawLiveLogFileReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveLogFileReaderError> {
        let journal_file_reader = RawLogFileReader::open(&path)?;

        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(RawLiveLogFileReader {
            blocker,
            log_file_reader: journal_file_reader,
            _watcher: watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    pub fn handle(&self) -> LiveLogFileHandle {
        LiveLogFileHandle::new(
            self.active.clone(),
            self.blocker.clone(),
        )
    }
}

impl Iterator for RawLiveLogFileReader {
    type Item = Result<serde_json::Value, LogFileReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) {
                return None;
            }

            match self.log_file_reader.next() {
                Some(value) => return Some(value),
                None => self.blocker.block(),
            }
        }
    }
}

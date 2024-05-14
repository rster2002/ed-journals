use std::collections::VecDeque;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::{io, thread};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use crate::logs::content::LogEvent;
use crate::modules::blockers::sync_blocker::SyncBlocker;
use crate::modules::logs::blocking::{LogFileReader, LogFileReaderError};

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
    blocker: SyncBlocker,
    log_file_reader: LogFileReader,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
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
        let journal_file_reader = LogFileReader::open(&path)?;

        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveLogFileReader {
            blocker,
            log_file_reader: journal_file_reader,
            watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    pub fn handle(&self) -> LiveLogFileHandle {
        LiveLogFileHandle {
            active: self.active.clone(),
            blocker: self.blocker.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiveLogFileHandle {
    active: Arc<AtomicBool>,
    blocker: SyncBlocker,
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

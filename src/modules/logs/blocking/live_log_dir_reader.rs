
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;
use thiserror::Error;
use crate::logs::{LogDir, LogDirError};
use crate::logs::blocking::log_dir_reader::{LogDirReader, LogDirReaderError};
use crate::logs::blocking::LogFileReader;
use crate::logs::content::LogEvent;
use crate::logs::blocking::LogFileReaderError;
use crate::logs::log_file::{LogFile, LogFileError};
use crate::modules::blockers::sync_blocker::SyncBlocker;

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
    blocker: SyncBlocker,
    log_dir_reader: LogDirReader,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
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
        let log_dir_reader = LogDirReader::open(&dir_path);

        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
        })?;

        watcher.watch(dir_path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(Self {
            blocker,
            log_dir_reader,
            active: Arc::new(AtomicBool::new(true)),
            watcher,
        })
    }

    pub fn handle(&self) -> LiveLogDirHandle {
        LiveLogDirHandle {
            active: self.active.clone(),
            blocker: self.blocker.clone(),
        }
    }
}

pub struct LiveLogDirHandle {
    active: Arc<AtomicBool>,
    blocker: SyncBlocker,
}

impl LiveLogDirHandle {
    pub fn close(&self) {
        self.active.swap(false, Ordering::Relaxed);
        self.blocker.unblock();
    }
}

impl Iterator for LiveLogDirReader {
    type Item = Result<LogEvent, LiveLogDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) || self.log_dir_reader.is_failing() {
                return None;
            }

            let Some(result) = self.log_dir_reader.next() else {
                self.blocker.block();
                continue;
            };

            return Some(result.map_err(|e| e.into()));
        }
    }
}

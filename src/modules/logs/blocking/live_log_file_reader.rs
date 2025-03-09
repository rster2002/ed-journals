use std::io;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;

use crate::logs::content::LogEvent;
use crate::modules::logs::blocking::{LogFileReader, LogFileReaderError};
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

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
pub struct LiveLogFileReader<T>
where T : Read + Seek,
{
    inner: LogFileReader<T>,
    blocker: SyncBlocker,
    _watcher: RecommendedWatcher,
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

impl<T> LiveLogFileReader<T>
where T : Read + Seek,
{
    pub fn new<P: AsRef<Path>>(path: P, inner: LogFileReader<T>) -> Result<Self, LiveLogFileReaderError> {
        // let journal_file_reader = LogFileReader::new(&path)?;

        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        // This is stopped when it is dropped
        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveLogFileReader {
            blocker,
            inner,
            _watcher: watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    // pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveLogFileReaderError> {

    // }

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

impl<T> Iterator for LiveLogFileReader<T>
where T : Read + Seek,
{
    type Item = Result<LogEvent, LogFileReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) {
                return None;
            }

            match self.inner.next() {
                Some(value) => return Some(value),
                None => self.blocker.block(),
            }
        }
    }
}

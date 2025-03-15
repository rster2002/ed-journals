use std::io::{Read, Seek};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;

use crate::logs::blocking::log_dir_reader::{LogDirReader, LogDirReaderError};
use crate::logs::content::LogEvent;
use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

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
pub struct LiveLogDirReader<'a, T>
where T : Read + Seek
{
    inner: LogDirReader<'a, T>,
    blocker: SyncBlocker,
    _watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
}

#[derive(Debug, Error)]
pub enum LiveLogDirReaderError {
    #[error(transparent)]
    LogDirReaderError(#[from] LogDirReaderError),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl<'a, T> LiveLogDirReader<'a, T>
where T : Read + Seek
{
    pub fn open<P: AsRef<Path>>(path: P, inner: LogDirReader<'a, T>) -> Result<LiveLogDirReader<'a, T>, LiveLogDirReaderError> {
        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(Self {
            blocker,
            inner,
            active: Arc::new(AtomicBool::new(true)),
            _watcher: watcher,
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

impl<T> Iterator for LiveLogDirReader<'_, T>
where T : Read + Seek,
{
    type Item = Result<LogEvent, LiveLogDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.active.load(Ordering::Relaxed) || self.inner.is_failing() {
                return None;
            }

            let Some(result) = self.inner.next() else {
                self.blocker.block();
                continue;
            };

            return Some(result.map_err(|e| e.into()));
        }
    }
}

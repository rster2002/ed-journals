use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{DataChange, ModifyKind};
use thiserror::Error;
use tokio::fs::File;
use tokio::sync::mpsc::{channel, Sender};
use crate::logs::{LogDir, LogDirError, LogFile};
use crate::logs::asynchronous::log_dir_reader::{LogDirReader, LogDirReaderError};
use crate::logs::content::LogEvent;
use crate::logs::asynchronous::LogFileReader;
use crate::modules::logs::asynchronous::LogFileReaderError;
use crate::modules::logs::LogFileError;
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;

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
    blocker: AsyncBlocker,
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

        let blocker = AsyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock_blocking();
        })?;

        watcher.watch(dir_path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveLogDirReader {
            blocker,
            active: Arc::new(AtomicBool::new(true)),
            watcher,
            log_dir_reader,
        })
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LiveLogDirReaderError>> {
        loop {
            if !self.active.load(Ordering::Relaxed) || self.log_dir_reader.is_failing() {
                return None;
            }

            let Some(result) = self.log_dir_reader.next().await else {
                self.blocker.block().await;
                continue;
            };

            return Some(result.map_err(|e| e.into()));
        }
    }
}

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
use crate::logs::content::LogEvent;
use crate::logs::asynchronous::LogFileReader;
use crate::modules::blockers::async_blocker::AsyncBlocker;
use crate::modules::logs::asynchronous::LogFileReaderError;
use crate::modules::logs::LogFileError;

/// The async variant of [super::blocking::LiveLogDirReader]. Watches the whole journal dir and
/// reads all files. Once all historic files have been read the current read will only resolve once
/// the newest log file is changed at which it will read the active log file and return the entry.
///
/// ```no_run
/// use std::path::PathBuf;
/// use ed_journals::logs::asynchronous::LiveLogDirReader;
///
/// let path = PathBuf::from("somePath");
///
/// let mut live_dir_reader = LiveLogDirReader::open(path)
///     .unwrap();
///
/// // At first this will read all existing lines from the journal logs, after which it will wait
/// // until it detects new entries in the latest log file.
/// while let Some(entry) = live_dir_reader.next().await {
///     // Do something with the entry
/// }
/// ```
#[derive(Debug)]
pub struct LiveLogDirReader {
    blocker: AsyncBlocker,
    dir: LogDir,
    current_file: Option<LogFile>,
    current_reader: Option<LogFileReader>,
    watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
    failing: bool,
}

#[derive(Debug, Error)]
pub enum LiveLogDirReaderError {
    #[error("Log path is not a directory")]
    PathIsNotADirectory,

    #[error(transparent)]
    JournalFileError(#[from] LogFileError),

    #[error(transparent)]
    JournalReaderError(#[from] LogFileReaderError),

    #[error(transparent)]
    JournalDirError(#[from] LogDirError),

    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl LiveLogDirReader {
    pub fn open<P: AsRef<Path>>(dir_path: P) -> Result<LiveLogDirReader, LiveLogDirReaderError> {
        let blocker = AsyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock_blocking();
        })?;

        watcher.watch(dir_path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveLogDirReader {
            blocker,
            dir: LogDir::new(dir_path.as_ref().to_path_buf())?,
            current_file: None,
            current_reader: None,
            active: Arc::new(AtomicBool::new(true)),
            watcher,
            failing: false,
        })
    }

    async fn set_current_file(
        &mut self,
        journal_file: LogFile,
    ) -> Result<(), LiveLogDirReaderError> {
        self.current_reader = Some(journal_file.create_async_reader().await?);
        self.current_file = Some(journal_file);

        Ok(())
    }

    async fn set_next_file(&mut self) -> Result<(), LiveLogDirReaderError> {
        let files = self.dir.journal_logs_oldest_first()?;

        for file in files {
            let Some(current) = &self.current_file else {
                self.set_current_file(file)
                    .await?;
                return Ok(());
            };

            if &file > current {
                self.set_current_file(file)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LiveLogDirReaderError>> {
        loop {
            if !self.active.load(Ordering::Relaxed) || self.failing {
                return None;
            }

            let result = self.set_next_file()
                .await;

            if let Err(error) = result {
                self.failing = true;
                return Some(Err(error));
            }

            let reader = self.current_reader.as_mut()?;

            let Some(result) = reader.next().await else {
                self.blocker.block().await;
                continue;
            };

            return Some(result.map_err(|e| e.into()));
        }
    }
}

use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use tokio::fs::File;
use crate::logs::asynchronous::LogFileReaderError;
use crate::logs::content::LogEvent;
use crate::modules::logs::asynchronous::LogFileReader;
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;

#[derive(Debug)]
pub struct LiveLogFileReader {
    blocker: AsyncBlocker,
    journal_file_reader: LogFileReader,
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
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveLogFileReaderError> {
        let journal_file_reader = LogFileReader::open(path.as_ref()).await?;

        let blocker = AsyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |res| {
            local_blocker.unblock_blocking();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveLogFileReader {
            blocker,
            journal_file_reader,
            watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LogFileReaderError>> {
        loop {
            if !self.active.load(Ordering::Relaxed) {
                return None;
            }

            match self.journal_file_reader.next().await {
                Some(value) => return Some(value),
                None => self.blocker.block().await,
            }
        }
    }
}

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::logs::asynchronous::LogFileReaderError;
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;

use super::{LiveLogFileReaderError, RawLogFileReader};

#[derive(Debug)]
pub struct RawLiveLogFileReader {
    blocker: AsyncBlocker,
    journal_file_reader: RawLogFileReader,
    _watcher: RecommendedWatcher,
    active: Arc<AtomicBool>,
}

impl RawLiveLogFileReader {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveLogFileReaderError> {
        let journal_file_reader = RawLogFileReader::open(path.as_ref()).await?;

        let blocker = AsyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock_blocking();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(RawLiveLogFileReader {
            blocker,
            journal_file_reader,
            _watcher: watcher,
            active: Arc::new(AtomicBool::new(true)),
        })
    }

    pub async fn next(&mut self) -> Option<Result<serde_json::Value, LogFileReaderError>> {
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

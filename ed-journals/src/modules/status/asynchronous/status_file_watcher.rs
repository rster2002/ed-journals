use std::path::PathBuf;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use crate::modules::blockers::async_blocker::AsyncBlocker;
use crate::status::asynchronous::read_status_file;
use crate::status::asynchronous::read_status_file::ReadStatusFileError;
use crate::status::Status;

#[derive(Debug)]
pub struct StatusFileWatcher {
    blocker: AsyncBlocker,
    path: PathBuf,
    watcher: RecommendedWatcher,
    first: bool,
}

#[derive(Debug, Error)]
pub enum StatusFileWatcherError {
    #[error(transparent)]
    NotifyError(#[from] notify::Error),
}

impl StatusFileWatcher {
    pub fn new(path: PathBuf) -> Result<Self, StatusFileWatcherError> {
        let blocker = AsyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock_blocking();
        })?;

        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(StatusFileWatcher {
            blocker,
            path,
            watcher,
            first: true,
        })
    }

    pub async fn next(&mut self) -> Option<Result<Status, ReadStatusFileError>> {
        if !self.first {
            self.blocker.block().await;
        }

        self.first = false;
        return Some(read_status_file(&self.path).await);
    }
}

use std::path::PathBuf;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use crate::modules::blockers::sync_blocker::SyncBlocker;
use crate::status::blocking::read_status_file;
use crate::status::blocking::read_status_file::ReadStatusFileError;
use crate::status::Status;

#[derive(Debug)]
pub struct StatusFileWatcher {
    blocker: SyncBlocker,
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
        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
        })?;

        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok(StatusFileWatcher {
            blocker,
            path,
            watcher,
            first: true,
        })
    }
}

impl Iterator for StatusFileWatcher {
    type Item = Result<Status, ReadStatusFileError>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first {
            self.blocker.block();
        }

        self.first = false;
        return Some(read_status_file(&self.path));
    }
}

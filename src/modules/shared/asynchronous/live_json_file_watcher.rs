use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use thiserror::Error;
use tokio::fs;
use crate::modules::shared::asynchronous::async_blocker::AsyncBlocker;
use crate::status::Status;

#[derive(Debug)]
pub struct LiveJsonFileWatcher<T>
    where T : for<'de> Deserialize<'de>
{
    blocker: AsyncBlocker,
    path: PathBuf,
    watcher: RecommendedWatcher,
    first: bool,
    phantom_data: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum LiveJsonFileWatcherError {
    #[error(transparent)]
    NotifyError(#[from] notify::Error),

    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl<T> LiveJsonFileWatcher<T>
    where T : for<'de> Deserialize<'de>
{
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveJsonFileWatcherError> {
        let blocker = AsyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock_blocking();
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(LiveJsonFileWatcher {
            blocker,
            path: path.as_ref().to_path_buf(),
            watcher,
            first: true,
            phantom_data: PhantomData,
        })
    }

    pub async fn next(&mut self) -> Option<Result<T, LiveJsonFileWatcherError>> {
        if !self.first {
            self.blocker.block().await;
        }

        self.first = false;

        let string_content = match fs::read_to_string(&self.path).await {
            Ok(value) => value,
            Err(error) => return Some(Err(error.into())),
        };

        let value = match serde_json::from_str(&string_content) {
            Ok(value) => value,
            Err(error) => return Some(Err(error.into())),
        };

        return Some(Ok(value));
    }
}

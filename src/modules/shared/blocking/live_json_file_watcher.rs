use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::{fs, io};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use thiserror::Error;

use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

#[derive(Debug)]
pub struct LiveJsonFileWatcher<T>
where
    T: for<'de> Deserialize<'de>,
{
    blocker: SyncBlocker,
    path: PathBuf,
    watcher: RecommendedWatcher,
    first: bool,

    // This is needed so that there's a constraint for the `Iterator` trait.
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
where
    T: for<'de> Deserialize<'de>,
{
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, LiveJsonFileWatcherError> {
        let blocker = SyncBlocker::new();
        let local_blocker = blocker.clone();

        let mut watcher = notify::recommended_watcher(move |_| {
            local_blocker.unblock();
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
}

impl<T> Iterator for LiveJsonFileWatcher<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Item = Result<T, LiveJsonFileWatcherError>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first {
            self.blocker.block();
        }

        self.first = false;

        let string_content = match fs::read_to_string(&self.path) {
            Ok(value) => value,
            Err(error) => return Some(Err(error.into())),
        };

        let value = match serde_json::from_str(&string_content) {
            Ok(value) => value,
            Err(error) => return Some(Err(error.into())),
        };

        Some(Ok(value))
    }
}

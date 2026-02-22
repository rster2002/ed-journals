use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::de::DeserializeOwned;
use crate::fs::common::LogFile;
use crate::fs::{LogFSError, Unblocker};
use crate::io::{LogIOError, LogPath};
use crate::logs::LogEvent;

/// Holds a [LogFile] which is changed when [NewestFile::maybe_next] is called with a [LogPath]
/// which is newer than the current one. Calling [Iterator::next] on this type will call the inner file or
/// return [None] if the file is not yet loaded.
pub struct NewestFile<R = LogEvent>
where
    R: DeserializeOwned,
{
    current_path: Option<LogPath>,
    current_file: Option<LogFile<R>>,
    unblocker: Arc<dyn Unblocker>,
}

impl NewestFile {
    pub fn new(blocker: impl Into<Arc<dyn Unblocker>>) -> NewestFile<LogEvent> {
        NewestFile::new_typed::<LogEvent>(blocker)
    }

    pub fn new_raw(blocker: impl Into<Arc<dyn Unblocker>>) -> NewestFile<serde_json::Value> {
        NewestFile::new_typed::<serde_json::Value>(blocker)
    }

    pub fn new_typed<R>(blocker: impl Into<Arc<dyn Unblocker>>) -> NewestFile<R>
    where
        R: DeserializeOwned,
    {
        NewestFile {
            current_path: None,
            current_file: None,
            unblocker: blocker.into(),
        }
    }
}

impl<R> NewestFile<R>
where
    R: DeserializeOwned,
{
    pub fn maybe_next(&mut self, path: &LogPath) -> Result<bool, LogFSError> {
        if self.current_path.is_none() || self.current_path.as_ref().is_some_and(|current| path > current) {
            self.current_path = Some(path.clone());
            self.current_file = Some(LogFile::new_typed::<R, _>(path, self.unblocker.clone())?);

            return Ok(true);
        }

        Ok(false)
    }
}

impl<R> Iterator for NewestFile<R>
where R : DeserializeOwned,
{
    type Item = Result<R, LogIOError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_file.as_mut()?.next()
    }
}

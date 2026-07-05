use crate::fs::common::LogFile;
use crate::fs::{LogFSError, Unblocker};
use crate::io::LogIOError;
use crate::logs::LogEvent;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Holds a [LogFile] which is changed when [DifferentFile::maybe_switch] is called with a different
/// path than the current one. Calling [Iterator::next] on this type will call the inner file or
/// return [None] if the file is not yet loaded.
pub struct DifferentFile<R = LogEvent>
where
    R: DeserializeOwned,
{
    current_path: Option<PathBuf>,
    current_file: Option<LogFile<R>>,
    unblocker: Arc<dyn Unblocker>,
}

impl DifferentFile {
    pub fn new(blocker: impl Into<Arc<dyn Unblocker>>) -> DifferentFile {
        DifferentFile::new_typed::<LogEvent>(blocker)
    }

    pub fn new_raw(blocker: impl Into<Arc<dyn Unblocker>>) -> DifferentFile<serde_json::Value> {
        DifferentFile::new_typed::<serde_json::Value>(blocker)
    }

    pub fn new_typed<R>(blocker: impl Into<Arc<dyn Unblocker>>) -> DifferentFile<R>
    where
        R: DeserializeOwned,
    {
        DifferentFile {
            current_path: None,
            current_file: None,
            unblocker: blocker.into(),
        }
    }
}

impl<R> DifferentFile<R>
where
    R: DeserializeOwned,
{
    /// If the provided path is different from the path of the current file, then a new file is
    /// opened using the provided path and replaces the current file.
    pub fn maybe_switch<P: AsRef<Path>>(&mut self, path: P) -> Result<bool, LogFSError> {
        let path = path.as_ref();

        if self.current_path.is_none() || self.current_path.as_ref().is_some_and(|v| v != path) {
            self.current_path = Some(path.to_path_buf());
            self.current_file = Some(LogFile::new_typed::<R, _>(path, self.unblocker.clone())?);

            return Ok(true);
        }

        Ok(false)
    }
}

impl<R> Iterator for DifferentFile<R>
where
    R: DeserializeOwned,
{
    type Item = Result<R, LogIOError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_file.as_mut()?.next()
    }
}

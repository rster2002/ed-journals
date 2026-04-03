use crate::fs::models::common_async::AsyncLogFile;
use crate::fs::{LogFSError, Unblocker};
use crate::io::LogIOError;
use crate::logs::LogEvent;
use futures::Stream;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll};

/// Async variant of [DifferentFile](crate::fs::common::DifferentFile).
/// Holds an [AsyncLogFile] which is changed when [AsyncDifferentFile::maybe_switch] is called with a
/// different path than the current one.
pub struct AsyncDifferentFile<R = LogEvent>
where
    R: DeserializeOwned + Unpin,
{
    current_path: Option<PathBuf>,
    current_file: Option<AsyncLogFile<R>>,
    unblocker: Arc<dyn Unblocker>,
}

impl AsyncDifferentFile {
    pub fn new(blocker: impl Into<Arc<dyn Unblocker>>) -> AsyncDifferentFile {
        AsyncDifferentFile::new_typed::<LogEvent>(blocker)
    }

    pub fn new_raw(
        blocker: impl Into<Arc<dyn Unblocker>>,
    ) -> AsyncDifferentFile<serde_json::Value> {
        AsyncDifferentFile::new_typed::<serde_json::Value>(blocker)
    }

    pub fn new_typed<R>(blocker: impl Into<Arc<dyn Unblocker>>) -> AsyncDifferentFile<R>
    where
        R: DeserializeOwned + Unpin,
    {
        AsyncDifferentFile {
            current_path: None,
            current_file: None,
            unblocker: blocker.into(),
        }
    }
}

impl<R> AsyncDifferentFile<R>
where
    R: DeserializeOwned + Unpin,
{
    /// If the provided path is different from the path of the current file, then a new file is
    /// opened using the provided path and replaces the current file.
    pub async fn maybe_switch<P: AsRef<Path>>(&mut self, path: P) -> Result<bool, LogFSError> {
        let path = path.as_ref();

        if self.current_path.is_none() || self.current_path.as_ref().is_some_and(|v| v != path) {
            self.current_path = Some(path.to_path_buf());
            self.current_file =
                Some(AsyncLogFile::new_typed::<R, _>(path, self.unblocker.clone()).await?);

            return Ok(true);
        }

        Ok(false)
    }
}

impl<R> Stream for AsyncDifferentFile<R>
where
    R: DeserializeOwned + Unpin,
{
    type Item = Result<R, LogIOError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.current_file.as_mut() {
            Some(file) => pin!(file).poll_next(cx),
            None => Poll::Ready(None),
        }
    }
}

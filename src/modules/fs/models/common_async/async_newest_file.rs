use crate::fs::models::common_async::AsyncLogFile;
use crate::fs::{LogFSError, Unblocker};
use crate::io::{LogIOError, LogPath};
use crate::logs::LogEvent;
use futures::Stream;
use serde::de::DeserializeOwned;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll};

/// Async variant of [NewestFile](crate::fs::common::NewestFile).
/// Holds an [AsyncLogFile] which is changed when [AsyncNewestFile::maybe_next] is called with a
/// [LogPath] which is newer than the current one.
pub struct AsyncNewestFile<R = LogEvent>
where
    R: DeserializeOwned + Unpin,
{
    current_path: Option<LogPath>,
    current_file: Option<AsyncLogFile<R>>,
    unblocker: Arc<dyn Unblocker>,
}

impl AsyncNewestFile {
    pub fn new(blocker: impl Into<Arc<dyn Unblocker>>) -> AsyncNewestFile<LogEvent> {
        AsyncNewestFile::new_typed::<LogEvent>(blocker)
    }

    pub fn new_raw(blocker: impl Into<Arc<dyn Unblocker>>) -> AsyncNewestFile<serde_json::Value> {
        AsyncNewestFile::new_typed::<serde_json::Value>(blocker)
    }

    pub fn new_typed<R>(blocker: impl Into<Arc<dyn Unblocker>>) -> AsyncNewestFile<R>
    where
        R: DeserializeOwned + Unpin,
    {
        AsyncNewestFile {
            current_path: None,
            current_file: None,
            unblocker: blocker.into(),
        }
    }
}

impl<R> AsyncNewestFile<R>
where
    R: DeserializeOwned + Unpin,
{
    pub async fn maybe_next(&mut self, path: &LogPath) -> Result<bool, LogFSError> {
        if self.current_path.is_none()
            || self.current_path.as_ref().is_some_and(|current| path > current)
        {
            self.current_path = Some(path.clone());
            self.current_file = Some(
                AsyncLogFile::new_typed::<R, _>(path, self.unblocker.clone()).await?,
            );

            return Ok(true);
        }

        Ok(false)
    }
}

impl<R> Stream for AsyncNewestFile<R>
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

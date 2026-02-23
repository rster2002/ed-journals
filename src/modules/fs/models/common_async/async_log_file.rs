use crate::fs::{FileWatcher, LogFSError, Unblocker};
use crate::io::{AsyncIter, LogIOError};
use crate::logs::LogEvent;
use async_fs::File;
use futures::io::BufReader;
use futures::Stream;
use serde::de::DeserializeOwned;
use std::path::Path;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll};

/// Async variant of [LogFile](crate::fs::common::LogFile).
/// Holds both a watcher and an async iterator over for the given path.
#[derive(Debug)]
pub struct AsyncLogFile<R = LogEvent>
where
    R: DeserializeOwned + Unpin,
{
    watcher: FileWatcher,
    iter: AsyncIter<BufReader<File>, R>,
}

impl AsyncLogFile {
    pub async fn new<P: AsRef<Path>>(
        path: P,
        blocker: impl Into<Arc<dyn Unblocker>>,
    ) -> Result<AsyncLogFile<LogEvent>, LogFSError> {
        AsyncLogFile::new_typed::<LogEvent, _>(path, blocker).await
    }

    pub async fn new_raw<P: AsRef<Path>>(
        path: P,
        blocker: impl Into<Arc<dyn Unblocker>>,
    ) -> Result<AsyncLogFile<serde_json::Value>, LogFSError> {
        AsyncLogFile::new_typed::<serde_json::Value, _>(path, blocker).await
    }

    pub async fn new_typed<R, P>(
        path: P,
        blocker: impl Into<Arc<dyn Unblocker>>,
    ) -> Result<AsyncLogFile<R>, LogFSError>
    where
        R: DeserializeOwned + Unpin,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let watcher = FileWatcher::new(path, blocker)?;
        let file = File::open(path).await?;
        let reader = BufReader::new(file);
        let iter = AsyncIter::from(reader);

        Ok(AsyncLogFile { watcher, iter })
    }
}

impl<R> Stream for AsyncLogFile<R>
where
    R: DeserializeOwned + Unpin,
{
    type Item = Result<R, LogIOError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        pin!(&mut self.iter).poll_next(cx)
    }
}

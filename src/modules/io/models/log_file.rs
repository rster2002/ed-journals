pub mod live_iter;
pub mod log_iter;

use crate::modules::io::error::LogError;
use crate::modules::io::models::log_file::live_iter::LiveIter;
use crate::modules::io::models::log_file::log_iter::LogIter;
use crate::modules::io::models::log_path::LogPath;
use std::cmp::Ordering;
use std::path::Path;
use std::sync::Arc;

use crate::modules::shared::blocking::sync_blocker::SyncBlocker;

#[derive(Debug, Clone)]
pub struct LogFile {
    path: LogPath,
    blocker: Option<Arc<SyncBlocker>>,
}

impl LogFile {
    /// Create a new representation of a log file. The path is parsed and checked using [LogPath].
    /// If you want to read a file that doesn't pass the criteria of LogPath you can instead create
    /// iterators directly.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<LogFile, LogError> {
        let path = LogPath::try_from(path.as_ref())?;

        Ok(LogFile {
            path,
            blocker: None,
        })
    }

    // pub(crate) fn with_blocker<P: AsRef<Path>>(
    //     path: P,
    //     blocker: Arc<SyncBlocker>,
    // ) -> Result<LogFile, LogError> {
    //     let path = LogPath::try_from(path.as_ref())?;
    //
    //     Ok(LogFile {
    //         path,
    //         blocker: Some(blocker),
    //     })
    // }

    pub(crate) fn set_blocker(&mut self, blocker: Arc<SyncBlocker>) {
        self.blocker = Some(blocker);
    }

    pub fn log_path(&self) -> &LogPath {
        &self.path
    }

    /// Create a new synchronous iterator for log entries of this file.
    pub fn iter(&self) -> Result<LogIter<std::io::BufReader<std::fs::File>>, LogError> {
        let file = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(file);

        Ok(LogIter::from(reader))
    }

    /// Creates a new blocking iterator iterating over the log entries for this file. If there are
    /// no entries to read it will block until there are more entries.
    pub fn live_iter(&self) -> Result<LiveIter, LogError> {
        dbg!(&self.blocker);
        match &self.blocker {
            Some(blocker) => LiveIter::with_blocker(&self.path, blocker.clone()),
            None => LiveIter::open(&self.path),
        }
    }

    // #[cfg(feature = "asynchronous")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
    // pub async fn async_iter(
    //     &self,
    // ) -> Result<AsyncIter<futures::io::BufReader<async_fs::File>>, LogError> {
    //     let file = async_fs::File::open(&self.path).await?;
    //
    //     let reader = futures::io::BufReader::new(file);
    //
    //     Ok(AsyncIter::from(reader))
    // }
}

impl Eq for LogFile {}

impl PartialEq for LogFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialOrd for LogFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialEq<LogPath> for LogFile {
    fn eq(&self, other: &LogPath) -> bool {
        &self.path == other
    }
}

impl PartialOrd<LogPath> for LogFile {
    fn partial_cmp(&self, other: &LogPath) -> Option<Ordering> {
        self.path.partial_cmp(other)
    }
}

impl From<LogPath> for LogFile {
    fn from(value: LogPath) -> Self {
        LogFile {
            path: value,
            blocker: None,
        }
    }
}

impl From<(LogPath, Arc<SyncBlocker>)> for LogFile {
    fn from(value: (LogPath, Arc<SyncBlocker>)) -> Self {
        LogFile {
            path: value.0,
            blocker: Some(value.1),
        }
    }
}

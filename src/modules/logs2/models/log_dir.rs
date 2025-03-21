pub mod dir_iter;
pub mod live_dir_iter;

use crate::modules::logs2::models::log_dir::dir_iter::DirIter;
use crate::modules::logs2::{LogError, LogFile};
use std::path::{Path, PathBuf};

pub struct LogDir {
    path: PathBuf,
}

impl LogDir {
    pub fn new<P: AsRef<Path>>(path: P) -> LogDir {
        LogDir {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Returns an iterator for iterating over all log files in the given path in chronological
    /// order of their file names.
    pub fn iter(&self) -> Result<DirIter, LogError> {
        DirIter::new(&self.path)
    }

    /// Returns the last log file based on the timestamp in the file name.
    pub fn last(&self) -> Result<Option<LogFile>, LogError> {
        let iter = DirIter::new(&self.path)?;
        Ok(iter.last())
    }

    /// Returns a [DirIter] using async file system operations. The result is the same as the sync
    /// variant as all IO is performed when creating the iterator after which the operations are
    /// sync.
    #[cfg(feature = "asynchronous")]
    pub async fn async_iter(&self) -> Result<DirIter, LogError> {
        DirIter::new_async(&self.path).await
    }

    /// Returns the last log file based on the timestamp in the file name using async IO.
    pub async fn last_async(&self) -> Result<Option<LogFile>, LogError> {
        let iter = DirIter::new_async(&self.path).await?;
        Ok(iter.last())
    }
}

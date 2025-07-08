pub mod dir_iter;
pub mod live_dir_iter;

use crate::modules::io::models::log_dir::dir_iter::DirIter;
use crate::modules::io::{LogError, LogFile};
use std::path::{Path, PathBuf};
use crate::io::LiveDirIter;

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
    pub async fn iter(&self) -> Result<DirIter, LogError> {
        DirIter::new(&self.path).await
    }

    /// Returns the last log file based on the timestamp in the file name.
    pub async fn last(&self) -> Result<Option<LogFile>, LogError> {
        let iter = DirIter::new(&self.path).await?;
        Ok(iter.last())
    }

    pub async fn live_iter(&self) -> Result<LiveDirIter, LogError> {
        LiveDirIter::new(&self.path).await
    }

    // /// Returns an iterator for iterating over all log files in the given path in chronological
    // /// order of their file names. Unlike [LogDir::iter], this iterator will block the current
    // /// thread when there are no more files to open. Using this in combination with
    // /// [LogFile::live_iter] you can read _all_ log entries form the game and keep receiving new
    // /// entries when they are created:
    // ///
    // /// ```rust
    // /// # use std::env::current_dir;
    // /// use ed_journals::io::LogDir;
    // /// use ed_journals::io::auto_detect_journal_path;
    // ///
    // /// let path = auto_detect_journal_path();
    // /// # let path = current_dir()
    // /// #    .unwrap()
    // /// #    .join("test-files")
    // /// #    .join("journals");
    // ///
    // /// let file_iter = LogDir::new(&path)
    // ///     .live_iter()
    // ///     .expect("Failed to read directory");
    // ///
    // /// for file in file_iter {
    // ///     let live_iter = file.expect("Failed to open file")
    // ///         .live_iter()
    // ///         .expect("Failed to watch file");
    // ///
    // ///     for entry in live_iter {
    // ///         let entry = entry.expect("Failed to parse log entry");
    // ///         // ... do something with entry
    // /// #       break;
    // ///     }
    // ///
    // /// #   break;
    // /// }
    // /// ```
    // pub fn live_iter(&self) -> Result<LiveDirIter, LogError> {
    //     LiveDirIter::new(&self.path)
    // }

    // /// Returns a [DirIter] using async file system operations. The result is the same as the sync
    // /// variant as all IO is performed when creating the iterator after which the operations are
    // /// sync.
    // #[cfg(feature = "asynchronous")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
    // pub async fn async_iter(&self) -> Result<DirIter, LogError> {
    //     DirIter::new(&self.path).await
    // }

    // /// Returns the last log file based on the timestamp in the file name using async IO.
    // #[cfg(feature = "asynchronous")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
    // pub async fn last_async(&self) -> Result<Option<LogFile>, LogError> {
    //     let iter = DirIter::new(&self.path).await?;
    //     Ok(iter.last())
    // }
}

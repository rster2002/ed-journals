use crate::modules::io::models::log_path::LogPath;
use crate::modules::io::{LogError, LogFile};
use std::fs;
use std::path::Path;
use std::vec::IntoIter;

#[cfg(feature = "asynchronous")]
use futures::StreamExt;

/// Iterator over the log files in a directory in chronological order.
#[derive(Debug)]
pub struct DirIter {
    entries: IntoIter<LogPath>,
}

impl DirIter {
    /// Creates a new instance using synchronous operations.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<DirIter, LogError> {
        let read_dir = fs::read_dir(path)?;
        let mut entries = Vec::new();

        for entry in read_dir {
            let entry = entry?;

            match LogPath::try_from(entry.path().as_path()) {
                Ok(path) => entries.push(path),
                Err(LogError::IncorrectFileName) => continue,
                Err(e) => return Err(e),
            }
        }

        entries.sort();

        Ok(DirIter {
            entries: entries.into_iter(),
        })
    }

    // /// Creates a new instance using asynchronous operations.
    // #[cfg(feature = "asynchronous")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
    // pub async fn new_async<P: AsRef<Path>>(path: P) -> Result<DirIter, LogError> {
    //     let mut read_dir = async_fs::read_dir(path).await?;
    //
    //     let mut entries = Vec::new();
    //
    //     while let Some(entry) = read_dir.next().await {
    //         match LogPath::try_from(entry?.path().as_path()) {
    //             Ok(path) => entries.push(path),
    //             Err(LogError::IncorrectFileName) => continue,
    //             Err(e) => return Err(e),
    //         }
    //     }
    //
    //     entries.sort();
    //
    //     Ok(DirIter {
    //         entries: entries.into_iter(),
    //     })
    // }
}

impl Iterator for DirIter {
    type Item = LogFile;

    fn next(&mut self) -> Option<Self::Item> {
        let log_path = self.entries.next()?;
        Some(LogFile::from(log_path))
    }
}

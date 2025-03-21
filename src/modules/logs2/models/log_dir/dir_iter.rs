use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::IntoIter;
use crate::modules::logs2::{LogError, LogFile};
use crate::modules::logs2::models::log_dir::log_path::LogPath;

#[derive(Debug)]
pub struct DirIter {
    entries: IntoIter<LogPath>,
}

impl DirIter {
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

    pub async fn new_async<P: AsRef<Path>>(path: P) -> Result<DirIter, LogError> {
        let mut read_dir = tokio::fs::read_dir(path)
            .await?;

        let mut entries = Vec::new();

        while let Some(entry) = read_dir.next_entry().await? {
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
}

impl Iterator for DirIter {
    type Item = LogFile;

    fn next(&mut self) -> Option<Self::Item> {
        let log_path = self.entries.next()?;
        Some(LogFile::from(log_path))
    }
}
use std::fs;
use std::path::PathBuf;
use crate::io::{LogError, LogPath};

pub struct LogDir {
    path: PathBuf,
    files: Vec<LogPath>,
    index: usize,
}

impl LogDir {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        LogDir {
            path: path.into(),
            index: 0,
            files: Vec::new(),
        }
    }

    fn read_dir(&mut self) -> Result<(), LogError> {
        let items = fs::read_dir(&self.path)?;

        let mut new_files = Vec::new();

        for item in items {
            let item = item?;

            let path = item.path();

            let Ok(log_path) = LogPath::try_from(path) else {
                continue;
            };

            new_files.push(LogPath::from(log_path));
        }

        new_files.sort();
        self.files = new_files;

        Ok(())
    }
}

impl Iterator for LogDir {
    type Item = LogPath;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
use std::path::Path;

use thiserror::Error;

use crate::logs::{LogDir, LogDirError, LogFile, LogFileError};
use crate::logs::blocking::{LogFileReader, LogFileReaderError};
use crate::logs::content::LogEvent;

#[derive(Debug)]
pub struct LogDirReader {
    dir: LogDir,
    current_file: Option<LogFile>,
    current_reader: Option<LogFileReader>,
    is_live: bool,
    failing: bool,
}

#[derive(Debug, Error)]
pub enum LogDirReaderError {
    #[error(transparent)]
    LogFileError(#[from] LogFileError),

    #[error(transparent)]
    LogDirError(#[from] LogDirError),

    #[error(transparent)]
    LogFileReaderError(#[from] LogFileReaderError),
}

impl LogDirReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        LogDirReader {
            dir: LogDir::new(path.as_ref().to_path_buf()),
            current_file: None,
            current_reader: None,
            is_live: false,
            failing: false,
        }
    }

    fn set_current_file(&mut self, journal_file: LogFile) -> Result<(), LogDirReaderError> {
        self.current_reader = Some(journal_file.create_blocking_reader()?);
        self.current_file = Some(journal_file);

        Ok(())
    }

    pub fn is_reading_latest(&self) -> bool {
        self.is_live
    }

    fn set_next_file(&mut self) -> Result<bool, LogDirReaderError> {
        let files = self.dir.journal_logs_oldest_first()?;
        let is_empty = files.is_empty();

        let length = files.len();

        for (index, file) in files.into_iter().enumerate() {
            let Some(current) = &self.current_file else {
                self.set_current_file(file)?;
                return Ok(true);
            };

            if &file > current {
                self.set_current_file(file)?;
                return Ok(true);
            }

            self.is_live = length == index + 1;
        }

        Ok(is_empty)
    }

    pub fn is_failing(&self) -> bool {
        self.failing
    }
}

impl Iterator for LogDirReader {
    type Item = Result<LogEvent, LogDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_reader.is_none() {
                match self.set_next_file() {
                    Ok(true) => {}
                    Ok(false) => return None,
                    Err(error) => {
                        self.failing = true;
                        return Some(Err(error));
                    }
                }
            }

            let Some(reader) = &mut self.current_reader else {
                return None;
            };

            let Some(entry) = reader.next() else {
                match self.set_next_file() {
                    Ok(true) => continue,
                    Ok(false) => return None,
                    Err(error) => {
                        self.failing = true;
                        return Some(Err(error));
                    }
                }
            };

            return Some(entry.map_err(|e| e.into()));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::logs::blocking::LogDirReader;

    #[test]
    fn all_entries_are_read_correctly() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let mut reader = LogDirReader::open(dir_path);

        let mut count = 0;
        for entry in reader {
            count += 1;
        }

        assert_eq!(count, 870929);
    }
}

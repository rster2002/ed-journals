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
            failing: false,
        }
    }

    fn set_current_file(
        &mut self,
        journal_file: LogFile,
    ) -> Result<(), LogDirReaderError> {
        self.current_reader = Some(journal_file.create_blocking_reader()?);
        self.current_file = Some(journal_file);

        Ok(())
    }

    fn set_next_file(&mut self) -> Result<(), LogDirReaderError> {
        let files = self.dir.journal_logs_oldest_first()?;

        for file in files {
            let Some(current) = &self.current_file else {
                self.set_current_file(file)?;
                return Ok(());
            };

            if &file > current {
                self.set_current_file(file)?;
            }
        }

        Ok(())
    }

    pub fn is_failing(&self) -> bool {
        self.failing
    }
}

impl Iterator for LogDirReader {
    type Item = Result<LogEvent, LogDirReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.set_next_file();

        if let Err(error) = result {
            self.failing = true;
            return Some(Err(error));
        }

        let reader = self.current_reader.as_mut()?;
        Some(reader.next()?.map_err(|e| e.into()))
    }
}

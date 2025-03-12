use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use crate::modules::logs2::error::LogError;
use crate::modules::logs2::models::log_reader::LogReader;

pub struct LogFile<'a> {
    path: Cow<'a, Path>
}

impl<'a> LogFile<'a> {
    pub fn new<P: AsRef<Path>>(path: &'a P) -> LogFile<'a> {
        LogFile {
            path: Cow::Borrowed(path.as_ref())
        }
    }

    pub fn owned(path: PathBuf) -> LogFile<'static> {
        LogFile {
            path: Cow::Owned(path)
        }
    }

    pub fn iter(&self) -> Result<LogReader<BufReader<File>>, LogError> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        Ok(LogReader::new(reader))
    }
}
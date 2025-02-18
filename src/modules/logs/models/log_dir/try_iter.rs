use crate::logs::models::log_file::{LogFile, LogFileError};
use crate::logs::models::log_dir::LogDir;

pub struct TryIter<'a> {
    pub(crate) inner: &'a LogDir<'a>,
}

impl<'a> TryIter<'a> {
    pub fn new(inner: &'a LogDir) -> Self {
        TryIter { inner }
    }
}

impl<'a> Iterator for TryIter<'a> {
    type Item = Result<LogFile<'a>, LogFileError>;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.inner
            .path
            .read_dir();

        let value = match dir {
            Ok(value) => value,
            Err(error) => return Some(Err(error.into())),
        };
    }
}
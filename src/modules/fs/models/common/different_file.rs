use std::fs::File;
use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;
use crate::fs::{Blocker, LogFSError, Unblocker};
use crate::fs::common::LogFile;
use crate::io::{LogIOError, LogPath};
use crate::logs::LogEvent;

#[derive(Debug)]
pub struct DifferentFile<'a, B, R = LogEvent>
where R : DeserializeOwned,
    B : Blocker,
{
    current_path: Option<PathBuf>,
    current_file: Option<LogFile<R>>,
    blocker: &'a B,
}

impl<'a, B> DifferentFile<'a, B>
where B : Blocker,
{
    pub fn new(blocker: &'a B) -> DifferentFile<'a, B> {
        DifferentFile {
            current_path: None,
            current_file: None,
            blocker,
        }
    }

    pub fn new_typed<R>(blocker: &'a B) -> DifferentFile<'a, B, R>
    where R : DeserializeOwned,
    {
        DifferentFile {
            current_path: None,
            current_file: None,
            blocker,
        }
    }
}

impl<'a, B, R> DifferentFile<'a, B, R>
where R : DeserializeOwned,
    B : Blocker,
{
    pub fn maybe_switch<P: AsRef<Path>>(&mut self, path: P) -> Result<bool, LogFSError> {
        let path = path.as_ref();

        if self.current_path.is_none() || self.current_path.as_ref().is_some_and(|v| v != path) {
            self.current_path = Some(path.to_path_buf());
            self.current_file = Some(LogFile::new_typed::<R, _>(path, self.blocker)?);

            return Ok(true);
        }

        Ok(false)
    }
}

impl<'a, B, R> Iterator for DifferentFile<'a, B, R>
where R : DeserializeOwned,
    B : Blocker,
{
    type Item = Result<R, LogIOError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_file.as_mut()?.next()
    }
}

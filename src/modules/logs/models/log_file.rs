use std::borrow::Cow;
use std::fs::DirEntry;
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

pub struct LogFile<'a> {
    path: Cow<'a, Path>,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LogFileError {
    IO(#[from] std::io::Error),
}

#[cfg(not(feature = "legacy"))]
type RegexList = [(Regex, &'static str); 1];

#[cfg(feature = "legacy")]
type RegexList = [(Regex, &'static str); 2];

lazy_static! {
    static ref FILE_NAME_REGEXES: RegexList = [
        // Journal.YYYY-MM-DDTHHmmss.01.log
        (Regex::new(r"Journal\.(\d{4}-\d{2}-\d{2}T\d+)\.(\d{2})\.log").unwrap(), "%Y-%m-%dT%H%M%S"),

        // Journal.YYMMDDHHMMSS.01.log
        #[cfg(feature = "legacy")]
        (Regex::new(r"Journal\.(\d{12})\.(\d{2})\.log").unwrap(), "%y%m%d%H%M%S"),
    ];
}

impl<'a> TryFrom<DirEntry> for LogFile<'a> {
    type Error = LogFileError;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let file_name = value.file_name();

        // let path = value.path();
    }
}
use std::cmp::Ordering;
use std::fs::DirEntry;
use std::io;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

#[cfg(all(feature = "asynchronous", feature = "tokio"))]
use super::asynchronous;
use super::blocking;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

/// A representation of a journal log file. Can then be read using a [JournalFileReader].
#[derive(Debug)]
pub struct LogFile {
    path: PathBuf,
    naive_date_time: NaiveDateTime,
    part: u8,
}

#[derive(Debug, Error)]
pub enum LogFileError {
    #[error("Failed to represent OS string")]
    FailedToRepresentOsString,

    #[error("Incorrect file name")]
    IncorrectFileName,

    #[error("Failed to open reader: {0}")]
    FailedToOpenBlockingReader(#[from] blocking::LogFileReaderError),

    #[cfg(all(feature = "asynchronous", feature = "tokio"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
    #[error("Failed to open reader: {0}")]
    FailedToOpenAsyncReader(#[from] asynchronous::LogFileReaderError),

    #[error("Failed to parse journal date time: {0}")]
    FailedToParseDateTime(#[from] chrono::ParseError),

    #[error("Failed to parse journal part: {0}")]
    FailedToParsePart(#[from] ParseIntError),

    #[error(transparent)]
    IO(#[from] io::Error),
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

impl LogFile {
    /// Checks if the given file name (including the extension) matches that of a journal log file.
    pub fn is_match(name: &str) -> bool {
        FILE_NAME_REGEXES
            .iter()
            .any(|(regex, _)| regex.is_match(name))
    }

    /// Creates a new reader using the path of the journal log file.
    pub fn create_blocking_reader(&self) -> Result<blocking::LogFileReader, LogFileError> {
        blocking::LogFileReader::open(self.path.as_path())
            .map_err(LogFileError::FailedToOpenBlockingReader)
    }

    /// Creates a new live reader using the path of the journal log file.
    pub fn create_live_blocking_reader(
        &self,
    ) -> Result<blocking::LiveLogFileReader, blocking::LiveLogFileReaderError> {
        blocking::LiveLogFileReader::open(&self.path)
    }

    #[cfg(all(feature = "asynchronous", feature = "tokio"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
    pub async fn create_async_reader(&self) -> Result<asynchronous::LogFileReader, LogFileError> {
        asynchronous::LogFileReader::open(self.path.as_path())
            .await
            .map_err(LogFileError::FailedToOpenAsyncReader)
    }

    #[cfg(all(feature = "asynchronous", feature = "tokio"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
    pub async fn create_live_async_reader(
        &self,
    ) -> Result<asynchronous::LiveLogFileReader, asynchronous::LiveLogFileReaderError> {
        asynchronous::LiveLogFileReader::open(self.path.to_path_buf()).await
    }

    /// Returns the date time that is part of the file name of the file.
    pub fn date_time(&self) -> &NaiveDateTime {
        &self.naive_date_time
    }

    /// Returns the part number.
    pub fn part(&self) -> u8 {
        self.part
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl TryFrom<DirEntry> for LogFile {
    type Error = LogFileError;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let os_string = value.file_name();

        let file_name = os_string
            .to_str()
            .ok_or(LogFileError::FailedToRepresentOsString)?;

        FILE_NAME_REGEXES
            .iter()
            .find_map(|(regex, ts_format)| {
                regex.captures(file_name).map(|captures| {
                    let timestamp_str = captures
                        .get(1)
                        .expect("Regex should have already matched")
                        .as_str();

                    match NaiveDateTime::parse_from_str(timestamp_str, ts_format) {
                        Ok(native_date_time) => {
                            match captures
                                .get(2)
                                .expect("Regex should have already matched")
                                .as_str()
                                .parse()
                            {
                                Ok(part) => Some(Ok(LogFile {
                                    path: value.path().to_path_buf(),
                                    naive_date_time: native_date_time,
                                    part,
                                })),
                                Err(err) => Some(Err(err.into())),
                            }
                        }
                        Err(err) => Some(Err(err.into())),
                    }
                })
            })
            .flatten()
            .unwrap_or(Err(LogFileError::IncorrectFileName))
    }
}

impl Eq for LogFile {}

impl PartialEq<Self> for LogFile {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl PartialOrd<Self> for LogFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.naive_date_time
            .cmp(&other.naive_date_time)
            .then(self.part.cmp(&other.part))
    }
}

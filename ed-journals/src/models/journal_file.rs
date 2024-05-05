use std::cmp::Ordering;
use std::fs::{DirEntry, File};
use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;

use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::models::journal_reader::JournalReader;

/// A representation of a journal log file. Can then be read using a [JournalReader].
#[derive(Debug)]
pub struct JournalFile {
    path: PathBuf,
    naive_date_time: NaiveDateTime,
    part: u8,
}

#[derive(Debug, Error)]
pub enum JournalFileError {
    #[error("Failed to represent OS string")]
    FailedToRepresentOsString,

    #[error("Incorrect file name")]
    IncorrectFileName,

    #[error("Failed to parse journal date time: {0}")]
    FailedToParseDateTime(#[from] chrono::ParseError),

    #[error("Failed to parse journal part: {0}")]
    FailedToParsePart(#[from] ParseIntError),

    #[error(transparent)]
    IO(#[from] io::Error),
}

lazy_static! {
    static ref FILE_NAME_REGEX: Regex =
        Regex::new(r"Journal\.(\d{4}-\d{2}-\d{2}T\d+)\.(\d{2})\.log").unwrap();
}

impl JournalFile {
    /// Checks if the given file name (including the extension) matches that of a journal log file.
    pub fn is_match(name: &str) -> bool {
        FILE_NAME_REGEX.is_match(name)
    }

    /// Creates a new reader using the path of the journal log file.
    pub fn create_reader(&self) -> Result<JournalReader<File>, JournalFileError> {
        Ok(JournalReader::from(File::open(self.path.as_path())?))
    }

    /// Returns the date time that is part of the file name of the file.
    pub fn date_time(&self) -> &NaiveDateTime {
        &self.naive_date_time
    }

    /// Returns the part number.
    pub fn part(&self) -> u8 {
        self.part
    }
}

impl TryFrom<DirEntry> for JournalFile {
    type Error = JournalFileError;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let os_string = value.file_name();

        let file_name = os_string
            .to_str()
            .ok_or(JournalFileError::FailedToRepresentOsString)?;

        let captures = FILE_NAME_REGEX
            .captures(file_name)
            .ok_or(JournalFileError::IncorrectFileName)?;

        let timestamp_str = captures
            .get(1)
            .expect("Regex should have already matched")
            .as_str();

        let native_date_time = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%dT%H%M%S")?;
        let part = captures
            .get(2)
            .expect("Regex should have already matched")
            .as_str()
            .parse()?;

        Ok(JournalFile {
            path: value.path().to_path_buf(),
            naive_date_time: native_date_time,
            part,
        })
    }
}

impl Eq for JournalFile {}

impl PartialEq<Self> for JournalFile {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl PartialOrd<Self> for JournalFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JournalFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.naive_date_time
            .cmp(&other.naive_date_time)
            .then(self.part.cmp(&other.part))
    }
}

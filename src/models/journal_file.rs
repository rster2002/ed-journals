use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::{DirEntry, File};
use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;

use crate::models::journal_reader::JournalReader;
use thiserror::Error;

const FILE_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Journal\.(\d{4}-\d{2}-\d{2}T\d+)\.(\d{2})\.log").unwrap());

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

#[derive(Debug)]
pub struct JournalFile {
    path: PathBuf,
    naive_date_time: NaiveDateTime,
    part: u8,
}

impl JournalFile {
    pub fn is_match(name: &str) -> bool {
        FILE_NAME_REGEX.is_match(name)
    }

    pub fn create_reader(&self) -> Result<JournalReader, JournalFileError> {
        Ok(JournalReader::from(File::open(self.path.as_path())?))
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

use crate::modules::io::LogIOError;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct LogPath {
    pub path: PathBuf,
    pub timestamp: NaiveDateTime,
    pub part: u8,
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

impl TryFrom<&Path> for LogPath {
    type Error = LogIOError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let file_name = value
            .file_name()
            .ok_or(LogIOError::MissingFileName)?
            .to_str()
            .ok_or(LogIOError::FailedToRepresentOsString)?;

        for (regex, format) in FILE_NAME_REGEXES.iter() {
            let Some(captures) = regex.captures(file_name) else {
                continue;
            };

            let timestamp_str = captures
                .get(1)
                .expect("Regex should have already matched")
                .as_str();

            let timestamp = NaiveDateTime::parse_from_str(timestamp_str, format)
                .map_err(LogIOError::FailedToParseLogTime)?;

            let part = captures
                .get(2)
                .expect("Regex should have already matched")
                .as_str()
                .parse()
                .map_err(LogIOError::FailedToParsePart)?;

            return Ok(LogPath {
                path: value.to_path_buf(),
                timestamp,
                part,
            });
        }

        Err(LogIOError::IncorrectFileName)
    }
}

impl AsRef<Path> for LogPath {
    fn as_ref(&self) -> &Path {
        self.path.as_path()
    }
}

impl From<LogPath> for PathBuf {
    fn from(val: LogPath) -> Self {
        val.path
    }
}

impl Eq for LogPath {}

impl PartialEq for LogPath {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl PartialOrd for LogPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then(self.part.cmp(&other.part))
    }
}
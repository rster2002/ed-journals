use crate::modules::io::LogError;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DirEntry {
    path: PathBuf,
    timestamp: NaiveDateTime,
    part: u8,
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

impl<T> TryFrom<T> for DirEntry
where T : AsRef<Path>
{
    type Error = LogError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let file_name = value
            .file_name()
            .ok_or(LogError::MissingFileName)?
            .to_str()
            .ok_or(LogError::FailedToRepresentOsString)?;

        for (regex, format) in FILE_NAME_REGEXES.iter() {
            let Some(captures) = regex.captures(file_name) else {
                continue;
            };

            let timestamp_str = captures
                .get(1)
                .expect("Regex should have already matched")
                .as_str();

            let timestamp = NaiveDateTime::parse_from_str(timestamp_str, format)
                .map_err(LogError::FailedToParseLogTime)?;

            let part = captures
                .get(2)
                .expect("Regex should have already matched")
                .as_str()
                .parse()
                .map_err(LogError::FailedToParsePart)?;

            return Ok(DirEntry {
                path: value.to_path_buf(),
                timestamp,
                part,
            });
        }

        Err(LogError::IncorrectFileName)
    }
}

impl AsRef<Path> for DirEntry {
    fn as_ref(&self) -> &Path {
        self.path.as_path()
    }
}

impl From<DirEntry> for PathBuf {
    fn from(val: DirEntry) -> Self {
        val.path
    }
}

impl Eq for DirEntry {}

impl PartialEq for DirEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl PartialOrd for DirEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then(self.part.cmp(&other.part))
    }
}

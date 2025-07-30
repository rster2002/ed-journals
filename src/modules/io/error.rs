// use chrono::ParseError;
// use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LogError {
    IO(#[from] std::io::Error),
    SerdeJson(#[from] serde_json::Error),
    NotifyError(#[from] notify::Error),
    // #[error("Missing file name")]
    // MissingFileName,

    // #[error("Failed to represent file name")]
    // FailedToRepresentOsString,

    // #[error("Incorrect file name")]
    // IncorrectFileName,

    // #[error("Failed to parse timestamp of log file")]
    // FailedToParseLogTime(#[source] ParseError),

    // #[error("Failed to parse part number of log file")]
    // FailedToParsePart(#[source] ParseIntError),
}

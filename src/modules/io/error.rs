use chrono::ParseError;
use std::num::ParseIntError;
use std::sync::mpsc::Sender;
use std::sync::PoisonError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LogIOError {
    IO(#[from] std::io::Error),
    SerdeJson(#[from] serde_json::Error),

    #[error("Missing file name")]
    MissingFileName,

    #[error("Failed to represent file name")]
    FailedToRepresentOsString,

    #[error("Incorrect file name")]
    IncorrectFileName,

    #[error("Failed to parse timestamp of log file")]
    FailedToParseLogTime(#[source] ParseError),

    #[error("Failed to parse part number of log file")]
    FailedToParsePart(#[source] ParseIntError),
}

use std::fs;
use std::path::Path;
use thiserror::Error;
use crate::modules::status::models::status::Status;

pub fn read_status_file<P: AsRef<Path>>(path: P) -> Result<Status, ReadStatusFileError> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Error)]
pub enum ReadStatusFileError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to parse status file: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

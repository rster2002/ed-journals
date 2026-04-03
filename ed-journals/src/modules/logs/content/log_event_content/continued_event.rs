//! Fired when the log file is getting split into multiple files.

use serde::{Deserialize, Serialize};

/// Fired when the log file is getting split into multiple files. The logs will continue in a new
/// .log file with the given part number.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ContinuedEvent {
    /// The new part number for the next log file.
    pub part: u8,
}

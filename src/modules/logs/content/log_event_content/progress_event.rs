//! Includes information about the current progress of the player to the next rank.

use serde::{Deserialize, Serialize};

/// Includes information about the current progress of the player to the next rank.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ProgressEvent {
    /// Progress of the combat rank between 0-100.
    pub combat: u8,

    /// Progress of the trade rank between 0-100.
    pub trade: u8,

    /// Progress of the exploration rank between 0-100.
    pub explore: u8,

    /// Progress of the empire rank between 0-100.
    pub empire: u8,

    /// Progress of the federation rank between 0-100.
    pub federation: u8,

    #[serde(rename = "CQC")]
    pub cqc: u8,
}

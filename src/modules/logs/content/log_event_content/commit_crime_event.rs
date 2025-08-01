//! Fired when the player commits a crime.

use crate::civilization::Crime;
use serde::{Deserialize, Serialize};

/// Fired when the player commits a crime.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CommitCrimeEvent {
    /// The kind of crime the player committed.
    pub crime_type: Crime,

    /// The faction that the player has commited the crime to.
    pub faction: String,

    /// The fine that needs to be paid for the crime.
    pub fine: Option<u64>,

    /// The bounty that has been placed on the player for committing the crime.
    pub bounty: Option<u64>,
}

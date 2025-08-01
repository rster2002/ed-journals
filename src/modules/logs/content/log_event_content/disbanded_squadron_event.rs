//! Fired when the owner of a squadron disbands a squadron.

use serde::{Deserialize, Serialize};

/// Fired when the owner of a squadron disbands a squadron.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct DisbandedSquadronEvent {
    /// The name of the squadron that was disbanded.
    pub squadron_name: String,
}

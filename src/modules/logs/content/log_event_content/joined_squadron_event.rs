//! Fired when the player joins a squadron.

use serde::{Deserialize, Serialize};

/// Fired when the player joins a squadron.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct JoinedSquadronEvent {
    /// The name of the squadron joined.
    pub squadron_name: String,
}

//! Fired when the player left a squadron.

use serde::{Deserialize, Serialize};

/// Fired when the player left a squadron.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct LeftSquadronEvent {
    /// The name of the squadron left.
    pub squadron_name: String,
}

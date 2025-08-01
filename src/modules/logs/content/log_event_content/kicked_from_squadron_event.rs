//! Fired when the player was kicked from a squadron.

use serde::{Deserialize, Serialize};

/// Fired when the player was kicked from a squadron.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct KickedFromSquadronEvent {
    /// The name of the squadron the player was kicked from.
    pub squadron_name: String,
}

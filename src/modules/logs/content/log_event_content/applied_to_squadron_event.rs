//! Fired when the player applies to a squadron.

use serde::{Deserialize, Serialize};

/// Fired when the player applies to a squadron.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct AppliedToSquadronEvent {
    /// The name of the squadron the player applied to.
    pub squadron_name: String,
}

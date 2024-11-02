//! Fired when the player was invited to a squadron.

use serde::{Deserialize, Serialize};

/// Fired when the player was invited to a squadron.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InvitedToSquadronEvent {
    /// The name of the squadron the player is invited to.
    pub squadron_name: String,
}

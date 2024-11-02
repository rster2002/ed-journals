//! Fired when the current player joins a multi-crew session.

use serde::{Deserialize, Serialize};

/// Fired when the current player joins a multi-crew session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JoinACrewEvent {
    /// The CMDR name of the player that is hosting the multi-crew session.
    pub captain: String,

    #[serde(default)]
    pub telepresence: bool,
}

use serde::{Deserialize, Serialize};

/// Fired when the player left a squadron.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LeftSquadronEvent {
    /// The name of the squadron left.
    pub squadron_name: String,
}

use serde::{Serialize, Deserialize};

/// Fired whenever another player leaves the multi-crew session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberQuitsEvent {
    /// The CMDR name of the player that left the session.
    pub crew: String,

    #[serde(default)]
    pub telepresence: bool,
}

use serde::{Deserialize, Serialize};

/// Fired whenever a new player joins a multi-crew session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberJoinsEvent {
    /// The CMDR name of the player that joined the session.
    pub crew: String,

    #[serde(default)]
    pub telepresence: bool,
}

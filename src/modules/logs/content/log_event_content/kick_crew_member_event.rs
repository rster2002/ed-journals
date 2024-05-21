use serde::{Deserialize, Serialize};

/// Fired when the current player kicks another player from the multi-crew session.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KickCrewMemberEvent {
    /// The CMDR name of the player that has been kicked.
    pub crew: String,

    #[serde(default)]
    pub on_crime: bool,

    #[serde(default)]
    pub telepresence: bool,
}

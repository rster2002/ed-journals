use serde::Deserialize;

/// Fired whenever a new player joins a multi-crew session.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberJoinsEvent {
    /// The CMDR name of the player that joined the session.
    pub name: String,

    #[serde(default)]
    pub telepresence: bool,
}

use serde::Deserialize;

/// Fired when the current player leaves a multi-crew session.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct QuitACrewEvent {
    /// The CMDR name of the player that is hosting the multi-crew session.
    pub captain: String,

    #[serde(default)]
    pub telepresence: bool,
}

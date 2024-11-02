//! Fired when another player in a multi-crew session launches a fighter.

use serde::{Deserialize, Serialize};

/// Fired when another player in a multi-crew session launches a fighter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewLaunchFighterEvent {
    #[serde(rename = "ID")]
    pub id: u64,

    /// The CMDR name of the player that launched the fighter.
    pub name: String,

    #[serde(default)]
    pub telepresence: bool,
}

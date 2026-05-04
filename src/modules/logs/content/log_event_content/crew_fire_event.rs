//! Fired when the player fires a crew member.

use serde::{Deserialize, Serialize};

/// Fired when the player fires a crew member.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CrewFireEvent {
    /// The name of the crew member that was fired.
    pub name: String,

    /// The id of the crew member that was fired.
    #[serde(rename = "CrewID")]
    pub crew_id: u64,
}

//! Fired when the player abandons a mission.

use serde::{Deserialize, Serialize};

use crate::modules::station::MissionType;

/// Fired when the player abandons a mission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEvent {
    /// The kind of mission the player abandoned.
    pub name: MissionType,

    /// The id of the mission the player abandoned.
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}

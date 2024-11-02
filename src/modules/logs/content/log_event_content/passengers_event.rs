//! Fired when there is a change to the player's passenger manifest.

use serde::{Deserialize, Serialize};

/// Fired when there is a change to the player's passenger manifest.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersEvent {
    /// List of passengers that are on board.
    pub manifest: Vec<PassengersManifestEntry>,
}

/// An entry for passengers that are on board.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersManifestEntry {
    /// The id of the mission this entry is a part of.
    #[serde(rename = "MissionID")]
    mission_id: u64,

    // TODO replace with enum
    /// The kind of passenger.
    #[serde(rename = "Type")]
    kind: String,

    /// Whether the passengers are considered VIP passengers.
    #[serde(rename = "VIP")]
    vip: bool,

    /// Whether the passengers are wanted.
    wanted: bool,

    /// The number of passengers for this entry/mission.
    count: u8,
}

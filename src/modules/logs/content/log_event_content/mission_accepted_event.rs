//! Fired when the player takes on a new mission.

use serde::{Deserialize, Serialize};

use crate::modules::mixed::MixedCommodity;
use crate::modules::station::MissionType;

/// Fired when the player takes on a new mission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionAcceptedEvent {
    /// The kind of mission that was accepted.
    pub name: MissionType,

    /// The localized name of the accepted mission.
    #[serde(rename = "Localised_Name")]
    pub localized_name: Option<String>,
    pub faction: String,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    // TODO replace with model
    pub influence: String,

    // TODO replace with model
    pub reputation: String,

    // TODO this is [None] for donation missions, but should be encapsulated differently
    pub reward: Option<u64>,
    pub wing: bool,

    // TODO test when that properties are present
    pub commodity: Option<MixedCommodity>,
    pub count: Option<u16>,
    pub donation: Option<String>,
    pub donated: Option<u64>,
    pub target: Option<String>,

    // TODO replace with enum
    pub target_type: Option<String>,
    pub target_faction: Option<String>,
    pub kill_count: Option<u8>,

    // TODO replace this with a [DateTime]
    pub expiry: Option<String>,
    pub destination_system: Option<String>,
    pub destination_station: Option<String>,
    pub destination_settlement: Option<String>,
    pub new_destination_system: Option<String>,
    pub new_destination_station: Option<String>,
    pub passenger_count: Option<u8>,
    pub passenger_vips: Option<bool>,
    pub passenger_wanted: Option<bool>,

    // TODO replace with enum
    pub passenger_type: Option<String>,
}

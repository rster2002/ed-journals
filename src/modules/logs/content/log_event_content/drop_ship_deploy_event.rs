//! Fired when the player deploys from a dropship into a combat zone.

use serde::{Deserialize, Serialize};

/// Fired when the player deploys from a dropship into a combat zone.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct DropshipDeployEvent {
    /// The star system the player deployed at.
    pub star_system: String,

    /// The system address the player deployed at.
    pub system_address: u64,

    /// The name of the body the player deployed at.
    pub body: String,

    /// The body id the player deployed at.
    #[serde(rename = "BodyID")]
    pub body_id: u8,

    /// Whether the player deployed at a station.
    #[serde(default)]
    pub on_station: bool,

    /// Whether the player deployed at a planet.
    #[serde(default)]
    pub on_planet: bool,
}

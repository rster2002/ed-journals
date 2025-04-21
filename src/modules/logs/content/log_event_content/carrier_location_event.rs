//! Fired when a carrier's location is updated.

use serde::{Deserialize, Serialize};

/// Fired when a carrier's location is updated.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierLocationEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The name of the system the fleet carrier is located in.
    pub star_system: String,

    /// The address of the system the fleet carrier is located in.
    pub system_address: u64,

    /// The body id the carrier is located at.
    #[serde(rename = "BodyID")]
    pub body_id: u8,
}

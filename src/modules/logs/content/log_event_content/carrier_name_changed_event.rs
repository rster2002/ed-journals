use serde::{Deserialize, Serialize};

/// Fired when the owner changes their fleet carrier's name.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierNameChangeEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The callsign of the fleet carrier. This cannot be changed by the commander.
    pub callsign: String,

    /// The new name of the fleet carrier.
    pub name: String,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Fired when the fleet carrier owner schedules a jump.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpRequestEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The name of the system the fleet carrier will jump to.
    pub system_name: String,

    /// The name of the body the fleet carrier will jump to. Only present if a specific body was
    /// selected when planning the jump.
    pub body: Option<String>,

    /// The address of the system the fleet carrier will jump to.
    pub system_address: u64,

    /// The body id the carrier will arrive at.
    #[serde(rename = "BodyID")]
    pub body_id: u8,

    /// The departure time of the carrier. At this time the carrier will perform the jump.
    pub departure_time: DateTime<Utc>,
}

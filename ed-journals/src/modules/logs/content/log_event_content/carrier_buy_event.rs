use serde::{Serialize, Deserialize};

/// Fired when the player buys a new fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierBuyEvent {
    /// The ID of the carrier bought. This is unique to this specific carrier.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The ID of the market at which the carrier has been bought.
    pub bought_at_market: u64,

    /// The system name where the fleet carrier has been parked.
    pub location: String,

    /// The system address where the fleet carrier has been parked.
    pub system_address: u64,

    /// The price paid for the carrier. This should basically always be 5 billion.
    pub price: u64,

    // TODO not sure that this means
    pub variant: String,

    /// The call-sign or 'station name' of the carrier. This is unique to this specific carrier and
    /// cannot be changed at a later date.
    pub callsign: String,
}

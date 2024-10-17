use serde::{Deserialize, Serialize};

/// Fired when the player embarks at a fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FCEmbarkEvent {
    /// The market id of the fleet carrier.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The name of the fleet carrier.
    pub carrier_name: String,

    /// The id or 'call-sign' of the fleet carrier.
    #[serde(rename = "CarrierID")]
    pub carrier_id: String,
}

use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

/// Fired when the player starts a ship transfer between stations.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardTransferEvent {
    /// The type of the ship that is transferred.
    pub ship_type: ShipType,

    /// The localized name of the ship type that is transferred.
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    /// The id of the ship that is transferred.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// The system the ship is transferred from.
    pub system: String,

    /// The market id the ship is transferred from.
    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    /// The total distance in LY for the transfer.
    pub distance: f32,

    /// The cost of the transfer.
    pub transfer_price: u64,

    /// The time in seconds it takes for the transfer to complete.
    pub transfer_time: u64,

    /// The current market id where the action was performed. This is also where the ship will be
    /// transferred to.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

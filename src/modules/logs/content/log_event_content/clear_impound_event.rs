//! Fired when the player clears their ship after being arrested while on-foot.

use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

/// Fired when the player clears their ship after being arrested while on-foot.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ClearImpoundEvent {
    /// The ship type of the impounded ship.
    pub ship_type: ShipType,

    /// The id of the impounded ship.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// The market id where the ship is impounded.
    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    /// The market id where the player currently is.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

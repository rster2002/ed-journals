use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

/// Fired when the player swaps between two ships they own.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSwapEvent {
    /// The ship type for the retrieved ship.
    pub ship_type: ShipType,

    /// The localized ship type for the retrieved ship.
    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    /// The id of the ship that was retrieved.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// The type of the ship that was stored.
    pub store_old_ship: ShipType,

    /// The id of the ship that was stored.
    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,

    /// The current market id the action was performed. This is also where the current ship will be
    /// stored.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

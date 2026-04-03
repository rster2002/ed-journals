//! Fired when storing a module.

use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

/// Fired when storing a module.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    /// The market id the module is now stored at.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The slot that the module was equipped to.
    pub slot: ShipSlot,

    /// The module that was stored.
    pub stored_item: ShipModule,

    /// The localized name of the module that was stored.
    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: Option<String>,

    /// The type of the current active ship.
    pub ship: ShipType,

    /// The id of the current active ship.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// Whether the module is hot.
    pub hot: bool,
}

//! Fired when buying a new module and equipping it to the current active ship.

use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

/// Fired when buying a new module and equipping it to the current active ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ModuleBuyEvent {
    /// The slot the module is installed into.
    pub slot: ShipSlot,

    /// The module that was in the given slot and is now stored at the current location.
    pub stored_item: Option<ShipModule>,

    /// The localized name of the module that was stored.
    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: Option<String>,

    /// The module that was bought and installed in the given slot.
    pub buy_item: ShipModule,

    /// The localized name of the bought module.
    #[serde(rename = "BuyItem_Localised")]
    pub buy_item_localized: Option<String>,

    /// The market id where the module was bought.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The number of credits that were spent on the module.
    pub buy_price: u64,

    /// The type of the current active ship that the module has been installed onto.
    pub ship: ShipType,

    /// The id of the current active ship that the module has been installed onto.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

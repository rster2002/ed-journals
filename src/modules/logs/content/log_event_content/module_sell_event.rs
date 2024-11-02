//! Fired when the player sells a module.

use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

/// Fired when the player sells a module.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellEvent {
    /// The market id where the player sold the module.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The slot the module was installed in.
    pub slot: ShipSlot,

    /// The module that was sold.
    pub sell_item: ShipModule,

    /// The localized name of the module that was sold.
    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localized: Option<String>,

    /// The number of credits received from selling the module.
    pub sell_price: u64,

    /// The type of the current active ship that the module was installed onto.
    pub ship: ShipType,

    /// The id of the current active ship that the module was installed on.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipType};

/// Fired when selling a module that is stored at another location.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSellRemoteEvent {
    /// The storage slot the module occupied
    pub storage_slot: u8,

    /// The module that was sold.
    pub sell_item: ShipModule,

    /// The localized name of the module that was sold.
    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localized: Option<String>,

    // TODO wth is this?
    /// The id of the server the module is stored at?
    pub server_id: u64,

    /// The number of credits the module was sold for.
    pub sell_price: u64,

    /// The type of the current active ship.
    pub ship: ShipType,

    /// The id of the current active ship.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

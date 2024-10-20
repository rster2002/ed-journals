use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

/// Fired when installing a module directly from storage.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleRetrieveEvent {
    /// The market id where the module was stored at and where the player installed the module.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The slot the module is installed into.
    pub slot: ShipSlot,

    /// The module that was installed.
    pub retrieved_item: ShipModule,

    /// The localized name of the module that was installed.
    #[serde(rename = "RetrievedItem_Localised")]
    pub retrieved_item_localized: Option<String>,

    /// The type of the current active ship that the module has been installed onto.
    pub ship: ShipType,

    /// The id of the current active ship that the module has been installed onto.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// Whether the module is hot.
    pub hot: bool,
}

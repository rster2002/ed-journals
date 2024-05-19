use serde::{Serialize, Deserialize};
use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub slot: ShipSlot,
    pub stored_item: ShipModule,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: String,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub hot: bool,
}

use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipSlot, ShipType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleSwapEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    pub from_slot: ShipSlot,
    pub to_slot: ShipSlot,

    // TODO replace with enum
    pub from_item: String,

    #[serde(rename = "FromItem_Localised")]
    pub from_item_localized: Option<String>,

    pub to_item: String,

    #[serde(rename = "ToItem_Localised")]
    pub to_item_localized: Option<String>,

    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

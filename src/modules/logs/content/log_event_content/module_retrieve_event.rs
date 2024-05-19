use serde::{Serialize, Deserialize};
use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleRetrieveEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub slot: ShipSlot,
    pub retrieved_item: ShipModule,

    #[serde(rename = "RetrievedItem_Localised")]
    pub retrieved_item_localized: Option<String>,

    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub hot: bool,
}

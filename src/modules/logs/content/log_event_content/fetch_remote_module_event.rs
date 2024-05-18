use serde::{Serialize, Deserialize};
use crate::modules::ship::{ShipModule, ShipType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FetchRemoteModuleEvent {
    pub storage_slot: u8,
    pub stored_item: ShipModule,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: Option<String>,
    pub server_id: u64,
    pub transfer_cost: u64,
    pub transfer_time: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

use crate::modules::models::ship::blueprint::Blueprint;
use crate::modules::models::ship::ship_module::ShipModule;
use crate::modules::models::ship::ship_slot::ShipSlot;
use crate::modules::models::ship::ship_type::ShipType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub items: Vec<MassModuleStoreEventItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEventItem {
    pub slot: ShipSlot,
    pub name: ShipModule,
    pub hot: bool,
    pub engineer_modifications: Option<Blueprint>,
    pub level: u8,
    pub quality: u8,
}

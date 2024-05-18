use serde::{Serialize, Deserialize};
use crate::modules::ship::{Blueprint, ShipModule, ShipSlot, ShipType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
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

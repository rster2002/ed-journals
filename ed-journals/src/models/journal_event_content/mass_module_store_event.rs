use serde::Deserialize;
use crate::journal_event_content::shared::ship::blueprint::Blueprint;
use crate::journal_event_content::shared::ship::ship_module::ShipModule;
use crate::journal_event_content::shared::ship::ship_slot::ShipSlot;
use crate::journal_event_content::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub items: Vec<MassModuleStoreEventItem>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEventItem {
    pub slot: ShipSlot,
    pub name: ShipModule,
    pub hot: bool,
    pub engineer_modifications: Option<Blueprint>,
    pub level: u8,
    pub quality: u8,
}

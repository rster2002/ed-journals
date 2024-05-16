use serde::{Serialize, Deserialize};

use crate::modules::models::trading::commodity::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoDepotEvent {
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    pub update_type: CargoDepotEventUpdateType,
    pub cargo_type: Commodity,
    pub count: u16,

    #[serde(rename = "StartMarketID")]
    pub start_market_id: u64,

    #[serde(rename = "EndMarketID")]
    pub end_market_id: u64,
    pub items_collected: u16,
    pub items_delivered: u16,
    pub total_items_to_deliver: u16,
    pub progress: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CargoDepotEventUpdateType {
    Collect,
    Deliver,
    WingUpdate,
}

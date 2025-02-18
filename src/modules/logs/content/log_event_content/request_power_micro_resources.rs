//! Fired when the player requests powerplay microresources from a power contact.

use serde::{Deserialize, Serialize};

use crate::odyssey::{Item, ItemCategory};

/// Fired when the player requests powerplay microresources from a power contact.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RequestPowerMicroResourcesEvent {
    pub total_count: u16,
    pub micro_resources: Vec<PowerMicroResourceRequest>,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerMicroResourceRequest {
    pub name: Item,
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub category: ItemCategory,
    pub count: u16,
}

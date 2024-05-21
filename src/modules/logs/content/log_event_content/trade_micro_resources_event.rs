use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Item;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TradeMicroResourcesEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub received: Item,
    pub count: u8,
    pub offered: Vec<TradeMicroResourcesEventOffer>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TradeMicroResourcesEventOffer {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u8,
}

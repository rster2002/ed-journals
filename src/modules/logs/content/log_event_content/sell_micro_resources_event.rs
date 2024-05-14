use crate::modules::models::odyssey::item::Item;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellMicroResourcesEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub price: u64,
    pub micro_resources: Vec<SellMicroResourcesEventResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellMicroResourcesEventResource {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u8,
}

use serde::{Serialize, Deserialize};

use crate::modules::models::odyssey::item::Item;
use crate::modules::models::odyssey::item_type::ItemType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum BuyMicroResourceEvent {
    Single(BuyMicroResourceEventResource),
    Multiple(BuyMicroResourceEventMultiple),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyMicroResourceEventSingle {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub category: ItemType,
    pub count: u16,
    pub price: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyMicroResourceEventMultiple {
    pub total_count: u16,
    pub micro_resources: Vec<BuyMicroResourceEventResource>,
    pub price: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyMicroResourceEventResource {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub category: ItemType,
    pub count: u16,
}

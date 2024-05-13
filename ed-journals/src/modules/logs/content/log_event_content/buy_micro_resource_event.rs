use serde::{Serialize, Deserialize};

use crate::modules::shared::odyssey::item::Item;
use crate::modules::shared::odyssey::item_type::ItemType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyMicroResourceEvent {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub category: ItemType,
    pub count: u16,
    pub price: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

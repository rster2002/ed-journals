use serde::Deserialize;
use crate::modules::trading::{Commodity, CommodityCategory};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MarketEntry {
    #[serde(rename = "id")]
    pub id: u64,
    pub name: Commodity,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub category: CommodityCategory,

    #[serde(rename = "Category_Localised")]
    pub category_localized: Option<String>,
    pub buy_price: u64,
    pub sell_price: u64,
    pub mean_price: u64,
    pub stock_bracket: u64,
    pub demand_bracket: u64,
    pub stock: u64,
    pub demand: u64,
    pub consumer: bool,
    pub producer: bool,
    pub rare: bool,
}

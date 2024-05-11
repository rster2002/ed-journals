use serde::Deserialize;

use crate::modules::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MarketSellEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
    pub sell_price: u64,
    pub total_sale: u64,

    #[serde(rename = "AvgPricePaid")]
    pub average_price_paid: f32,
}

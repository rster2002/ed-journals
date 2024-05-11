use serde::Deserialize;

use crate::modules::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MarketBuyEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
    pub buy_price: u64,
    pub total_cost: u64,
}

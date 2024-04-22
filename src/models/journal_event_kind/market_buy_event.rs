use serde::Deserialize;
use crate::models::journal_event_kind::shared::trading::commodity_type::CommodityType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MarketBuyEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Type")]
    pub kind: CommodityType,
    pub count: u16,
    pub buy_price: u64,
    pub total_cost: u64,
}

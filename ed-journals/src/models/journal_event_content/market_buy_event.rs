use crate::models::journal_event_content::shared::trading::commodity::Commodity;
use serde::Deserialize;

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

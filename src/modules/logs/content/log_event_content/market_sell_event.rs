//! Fired when selling commodities at a given market.

use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when selling commodities at a given market.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MarketSellEvent {
    /// The id of the market where the player has bought the commodities.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The kind of commodity sold.
    #[serde(rename = "Type")]
    pub kind: Commodity,

    /// The amount of commodities sold.
    pub count: u16,

    /// The price per commodity in credits.
    pub sell_price: u64,

    /// The total cost of the entire transaction in credits.
    pub total_sale: u64,

    #[serde(rename = "AvgPricePaid")]
    pub average_price_paid: f32,
}

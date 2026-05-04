//! Fired when buying commodities at a given market.

use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when buying commodities at a given market.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MarketBuyEvent {
    /// The id of the market where the player has bought the commodities.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The kind of commodity bought.
    #[serde(rename = "Type")]
    pub kind: Commodity,

    /// The amount of commodities bought.
    pub count: u16,

    /// The price per commodity in credits.
    pub buy_price: u64,

    /// The total cost of the entire transaction in credits.
    pub total_cost: u64,
}

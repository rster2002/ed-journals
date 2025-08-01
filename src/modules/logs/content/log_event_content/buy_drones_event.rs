//! Fired when the player buys limpets at a station.

use serde::{Deserialize, Serialize};

/// Fired when the player buys limpets at a station.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct BuyDronesEvent {
    /// The type of drone that was bought. Currently, this is always [BuyDronesEventType::Drones].
    #[serde(rename = "Type")]
    pub kind: BuyDronesEventType,

    /// The number of limpet that were bought.
    pub count: u16,

    /// The buy price per limpet.
    pub buy_price: u64,

    /// The total paid credits for all the limpets.
    pub total_cost: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub enum BuyDronesEventType {
    Drones,
}

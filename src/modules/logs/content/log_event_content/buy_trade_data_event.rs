//! Fired when the player buys trading data for a specific system.

use serde::{Deserialize, Serialize};

/// Fired when the player buys trading data for a specific system.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyTradeDateEvent {
    /// The name of the system the player bought trading data for.
    pub system: String,

    /// The cost in credits the player paid.
    pub cost: u64,
}

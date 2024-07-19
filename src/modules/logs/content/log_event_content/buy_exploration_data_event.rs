use serde::{Deserialize, Serialize};

/// Fired when the player buys exploration data for a system.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyExplorationDataEvent {
    /// The name of the system the player has bought exploration data for.
    pub system: String,

    /// The total amount of credits that the exploration data cost.
    pub cost: u64,
}

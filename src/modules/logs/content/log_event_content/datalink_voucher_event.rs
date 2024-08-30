use serde::{Deserialize, Serialize};

/// Fired when a faction has paid for scanning a datapoint.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkVoucherEvent {
    /// Number of credits awarded.
    pub reward: u64,

    /// The faction that the data belonged to.
    pub victim_faction: String,

    /// The faction that paid the player.
    pub payee_faction: String,
}

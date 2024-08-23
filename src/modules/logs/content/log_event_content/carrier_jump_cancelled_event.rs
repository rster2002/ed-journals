//! Fired when the current planned carrier jump was cancelled.

use serde::{Deserialize, Serialize};

/// Fired when the current planned carrier jump was cancelled.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpCancelled {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
}

//! Fired when the player cancels their fleet carrier decommission.

use serde::{Deserialize, Serialize};

/// Fired when the player cancels their fleet carrier decommission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierCancelDecommissionEvent {
    /// The id of the fleet carrier. This is functionally the same as the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
}

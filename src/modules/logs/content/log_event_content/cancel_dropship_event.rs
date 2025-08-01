//! Fired when the player cancels a dropship.

use serde::{Deserialize, Serialize};

/// Fired when the player cancels a dropship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CancelDropshipEvent {
    /// Amount of credits refunded to the player.
    pub refund: u64,
}

//! Fired when the player cancels a taxi.

use serde::{Deserialize, Serialize};

/// Fired when the player cancels a taxi.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CancelTaxiEvent {
    /// Amount of credits refunded to the player.
    pub refund: u64,
}

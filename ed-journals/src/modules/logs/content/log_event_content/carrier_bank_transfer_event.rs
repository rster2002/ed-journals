//! Fired when the player transfers credits either to or from their fleet carrier.

use serde::{Deserialize, Serialize};

/// Fired when the player transfers credits either to or from their fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierBankTransferEvent {
    /// The id of the fleet carrier. This is functionally the same as the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The amount the player deposited to the carrier.
    pub deposit: Option<u64>,

    /// The amount the player withdrew from the carrier.
    pub withdraw: Option<u64>,

    /// The player's balance after the transfer.
    pub player_balance: u64,

    /// The carrier's balance after the transfer.
    pub carrier_balance: u64,
}

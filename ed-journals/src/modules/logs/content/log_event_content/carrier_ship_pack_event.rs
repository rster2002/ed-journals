//! Fired when managing ship packs on a fleet carrier.

use serde::{Deserialize, Serialize};

/// Fired when managing ship packs on a fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierShipPackEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The kind of operation performed.
    pub operation: CarrierShipPackEventOperation,

    /// The name of the pack theme.
    pub pack_theme: String,

    /// The tier of the pack.
    pub pack_tier: u8,

    /// The number of credits that the transaction has cost. Mutually exclusive with the `refund`
    /// field.
    pub cost: Option<u64>,

    /// The number of credits which have been refunded to the commanders account. Mutually exclusive
    /// with the `cost` field.
    pub refund: Option<u64>,
}

/// The kind of operation performed for the given ship pack.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierShipPackEventOperation {
    /// For when buying a new ship pack.
    BuyPack,

    /// For when selling previously bought ship pack.
    SellPack,

    /// For when restocking a previously bought ship pack.
    RestockPack,
}

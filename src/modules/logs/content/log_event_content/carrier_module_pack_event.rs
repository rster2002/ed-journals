//! Fired when managing module packs on a fleet carrier.

use serde::{Deserialize, Serialize};

/// Fired when managing module packs on a fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierModulePackEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The kind of operation performed.
    pub operation: CarrierModulePackEventOperation,

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

/// The kind of operation performed for the given module pack.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierModulePackEventOperation {
    /// For when buying a new module pack.
    BuyPack,

    /// For when selling previously bought module pack.
    SellPack,

    /// For when restocking a previously bought module pack.
    RestockPack,
}

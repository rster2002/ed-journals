//! Fired when redeeming any kind of voucher.

use serde::{Deserialize, Serialize};

/// Fired when redeeming any kind of voucher.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEvent {
    /// The type of voucher redeemed.
    #[serde(rename = "Type")]
    pub kind: RedeemVoucherEventType,

    /// The number of credits that were received from redeeming the voucher.
    pub amount: u64,

    /// List of factions that are linked to the voucher.
    #[serde(default)]
    pub factions: Vec<RedeemVoucherEventFaction>,

    /// This is used instead of the [factions] field when the [kind] is
    /// [RedeemVoucherEventType::Bounty].
    pub faction: Option<String>,

    /// Percentage that the broker received for redeeming the voucher.
    pub broker_percentage: Option<f32>,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RedeemVoucherEventType {
    CombatBond,

    #[serde(rename = "bounty")]
    Bounty,

    #[serde(rename = "trade")]
    Trade,

    #[serde(rename = "settlement")]
    Settlement,

    #[serde(rename = "scannable")]
    Scannable,

    #[serde(rename = "codex")]
    Codex,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEventFaction {
    pub faction: String,
    pub amount: u64,
}

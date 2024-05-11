use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEvent {
    #[serde(rename = "Type")]
    pub kind: RedeemVoucherEventType,
    pub amount: u64,

    #[serde(default)]
    pub factions: Vec<RedeemVoucherEventFaction>,

    /// This is used instead of the [factions] field when the [kind] is
    /// [RedeemVoucherEventType::Bounty].
    pub faction: Option<String>,
    pub broker_percentage: Option<f32>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RedeemVoucherEventType {
    CombatBond,

    #[serde(rename = "bounty")]
    Bounty,
    Trade,
    Settlement,
    Scannable,

    #[serde(rename = "codex")]
    Codex,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEventFaction {
    pub faction: String,
    pub amount: u64,
}

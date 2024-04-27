use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEvent {
    #[serde(rename = "Type")]
    pub kind: RedeemVoucherEventType,
    pub amount: u64,
    pub factions: Vec<RedeemVoucherEventFaction>,
    pub broker_percentage: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum RedeemVoucherEventType {
    CombatBond,

    #[serde(rename = "bounty")]
    Bounty,
    Trade,
    Settlement,
    Scannable,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct RedeemVoucherEventFaction {
    pub faction: String,
    pub amount: u64,
}

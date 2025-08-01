use serde::{Deserialize, Serialize};

// TODO
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CapShipBondEvent {
    pub awarding_faction: String,

    #[serde(rename = "AwardingFaction_Localised")]
    pub awarding_faction_localized: Option<String>,
    pub reward: u64,
    pub victim_faction: String,

    #[serde(rename = "VictimFaction_Localised")]
    pub victim_faction_localized: Option<String>,
}

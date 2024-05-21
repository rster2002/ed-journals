use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FactionKillBondEvent {
    pub reward: u64,
    pub awarding_faction: String,

    #[serde(rename = "AwardingFaction_Localised")]
    pub awarding_faction_localized: Option<String>,

    pub victim_faction: String,

    #[serde(rename = "VictimFaction_Localised")]
    pub victim_faction_localized: Option<String>,
}

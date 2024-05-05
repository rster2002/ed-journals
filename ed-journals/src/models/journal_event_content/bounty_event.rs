use crate::models::journal_event_content::shared::odyssey::citizen::Citizen;
use crate::models::journal_event_content::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum BountyEvent {
    Normal(BountyEventNormal),
    Skimmer(BountyEventSkimmer),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventNormal {
    pub rewards: Vec<BountyEventNormalReward>,
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localized: Option<String>,
    pub target: BountyEventTarget,

    #[serde(rename = "Target_Localised")]
    pub target_localized: Option<String>,
    pub total_reward: u64,
    pub victim_faction: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum BountyEventTarget {
    Ship(ShipType),
    Citizen(Citizen),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventNormalReward {
    pub faction: String,
    pub reward: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventSkimmer {
    pub faction: String,
    pub target: String,
    pub reward: u64,
    pub victim_faction: String,
}

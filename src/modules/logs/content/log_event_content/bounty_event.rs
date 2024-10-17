use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Citizen;
use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum BountyEvent {
    Normal(BountyEventNormal),
    Skimmer(BountyEventSkimmer),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventNormal {
    pub rewards: Vec<BountyEventNormalReward>,
    pub pilot_name: Option<String>,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localized: Option<String>,
    pub target: BountyEventTarget,

    #[serde(rename = "Target_Localised")]
    pub target_localized: Option<String>,
    pub total_reward: u64,
    pub victim_faction: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BountyEventTarget {
    #[serde(rename = "skimmerdrone")]
    SentrySkimmer,

    #[serde(rename = "bombskimmerdrone")]
    Stringer,

    #[serde(untagged)]
    Ship(ShipType),

    #[serde(untagged)]
    Citizen(Citizen),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventNormalReward {
    pub faction: String,
    pub reward: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventSkimmer {
    pub faction: Option<String>,
    pub target: String,
    pub reward: u64,
    pub victim_faction: String,
}

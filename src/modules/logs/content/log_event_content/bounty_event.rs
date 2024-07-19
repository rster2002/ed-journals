use serde::{Deserialize, Serialize};

use crate::modules::odyssey::Citizen;
use crate::modules::ship::ShipType;

/// Fired when the player takes on a bounty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum BountyEvent {
    /// For when the player takes on a normal (ship) target.
    Normal(BountyEventNormal),

    /// For when the player takes on a Skimmer drone as target.
    Skimmer(BountyEventSkimmer),
}

/// Details about the normal bounty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventNormal {
    /// A list of rewards which are rewarded when the player hands in the bounty.
    pub rewards: Vec<BountyEventNormalReward>,

    /// The name of the pilot the player took down, if any.
    pub pilot_name: Option<String>,

    /// The localized name of the pilot the player took down.
    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localized: Option<String>,

    /// The target of the bounty.
    pub target: BountyEventTarget,

    /// The name of the target localized.
    #[serde(rename = "Target_Localised")]
    pub target_localized: Option<String>,

    /// The total reward of credits that are rewarded for this bounty.
    pub total_reward: u64,

    /// The name of the faction the victim is a part of.
    pub victim_faction: String,
}

/// The type of target for the bounty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BountyEventTarget {
    /// For Skimmer drone targets.
    #[serde(rename = "skimmerdrone")]
    SentrySkimmer,

    /// For bomb Skimmer drone targets.
    #[serde(rename = "bombskimmerdrone")]
    Stringer,

    /// For targets that fly the specified ship.
    #[serde(untagged)]
    Ship(ShipType),

    /// For on-foot targets.
    #[serde(untagged)]
    Citizen(Citizen),
}

/// Details about the reward for the bounty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventNormalReward {
    /// The name of the faction that contributed to the reward.
    pub faction: String,

    /// The contribution of the faction to the total reward.
    pub reward: u64,
}

/// Details about a Skimmer bounty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BountyEventSkimmer {
    /// The name of the faction that placed the bounty.
    pub faction: String,

    /// The name of the target.
    pub target: String,

    /// The total number of credits rewarded when completing the bounty.
    pub reward: u64,

    /// The name of the faction the target Skimmer drone is part of.
    pub victim_faction: String,
}

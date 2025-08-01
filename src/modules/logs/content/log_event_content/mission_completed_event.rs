use serde::{Deserialize, Serialize};

use crate::modules::mixed::{MixedMaterial, MixedMaterialCategory};
use crate::modules::station::MissionType;
use crate::modules::trading::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEvent {
    pub faction: String,
    pub name: MissionType,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    // TODO check when this is [None]
    pub target_faction: Option<String>,

    // TODO check when this is [None]
    pub destination_system: Option<String>,

    // TODO check when this is [None]
    pub destination_station: Option<String>,
    pub destination_settlement: Option<String>,
    pub donation: Option<String>,
    pub donated: Option<u64>,

    // TODO this is [None] for donation missions, but should be encapsulated differently
    pub reward: Option<u64>,

    #[serde(default)]
    pub permits_awarded: Vec<String>,

    #[serde(default)]
    pub commodity_reward: Vec<MissionCompletedEventCommodityReward>,

    #[serde(default)]
    pub materials_reward: Vec<MissionCompletedEventMaterialsReward>,

    pub faction_effects: Vec<MissionCompletedEventFactionEffect>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEventFactionEffect {
    pub faction: String,
    pub effects: Vec<MissionCompletedEventFactionEffectEffect>,
    pub influence: Vec<MissionCompletedEventFactionEffectInfluence>,
    pub reputation_trend: MissionCompletedEventTrend,

    // TODO replace with struct or enum or something
    pub reputation: String,
}

// Ah yes, the FactionEffectEffect
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEventFactionEffectEffect {
    pub effect: MissionCompletedEventFactionEffectEffectEffect,
    pub trend: MissionCompletedEventTrend,
}

// Well, I, uh. Leave me alone alright!
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub enum MissionCompletedEventFactionEffectEffectEffect {
    #[serde(rename = "$MISSIONUTIL_Interaction_Summary_EP_up;")]
    EconomicStationIncrease,

    #[serde(rename = "$MISSIONUTIL_Interaction_Summary_EP_down;")]
    EconomicStationDecrease,

    #[serde(rename = "$MISSIONUTIL_Interaction_Summary_Outbreak_up;")]
    OutbreakStationIncrease,

    #[serde(rename = "$MISSIONUTIL_Interaction_Summary_Outbreak_down;")]
    OutbreakStationDecrease,

    #[serde(rename = "$MISSIONUTIL_Interaction_Summary_SP_up;")]
    SecurityFactionIncrease,

    #[serde(rename = "$MISSIONUTIL_Interaction_Summary_SP_down;")]
    SecurityFactionDecrease,

    #[serde(rename = "$MISSIONUTIL_Interaction_Summary_Outbreak_down;")]
    OutbreakDecrease,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub enum MissionCompletedEventTrend {
    UpGood,
    UpBad,
    DownGood,
    DownBad,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEventFactionEffectInfluence {
    pub system_address: u64,
    pub trend: MissionCompletedEventTrend,

    // TODO replace with struct or enum or something
    pub influence: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEventCommodityReward {
    pub name: Commodity,
    pub count: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEventMaterialsReward {
    pub name: MixedMaterial,
    pub category: MixedMaterialCategory,
    pub count: u16,
}

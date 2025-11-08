use serde::{Deserialize, Serialize};

use crate::modules::civilization::{FactionHappiness, FactionState, Government, Superpower};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    pub faction_state: FactionState,
    pub government: Government,
    pub influence: f32,
    pub allegiance: Superpower,
    pub happiness: FactionHappiness,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localized: Option<String>,
    pub my_reputation: f32,

    #[serde(default)]
    pub pending_states: Vec<FactionActiveState>,

    #[serde(default)]
    pub recovering_states: Vec<FactionActiveState>,

    #[serde(default)]
    pub active_states: Vec<FactionActiveState>,

    #[serde(default)]
    pub squadron_faction: bool,

    #[serde(default)]
    pub happiest_system: bool,

    #[serde(default)]
    pub home_system: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct FactionActiveState {
    pub state: FactionState,
}

use serde::{Serialize, Deserialize};

use crate::modules::models::civilization::faction::faction_happiness::FactionHappiness;
use crate::modules::models::civilization::faction_state::FactionState;
use crate::modules::models::civilization::government::Government;
use crate::modules::models::civilization::superpower::Superpower;

mod faction_happiness;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
#[serde(rename_all = "PascalCase")]
pub struct FactionActiveState {
    pub state: FactionState,
}

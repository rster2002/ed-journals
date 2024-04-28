mod faction_happiness;

use crate::models::journal_event_kind::shared::civilization::faction::faction_happiness::FactionHappiness;
use crate::models::journal_event_kind::shared::civilization::faction_state::FactionState;
use crate::models::journal_event_kind::shared::civilization::government::Government;
use crate::models::journal_event_kind::shared::civilization::superpower::Superpower;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    pub faction_state: FactionState,
    pub government: Government,
    pub influence: f32,
    pub allegiance: Superpower,

    // TODO check possible values
    pub happiness: FactionHappiness,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: Option<String>,
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

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FactionActiveState {
    pub state: FactionState,
}

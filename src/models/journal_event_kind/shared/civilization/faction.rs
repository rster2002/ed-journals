use serde::Deserialize;
use crate::models::journal_event_kind::shared::civilization::faction_state::FactionState;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    pub faction_state: String,
    pub government: String,
    pub influence: f32,
    pub allegiance: String,
    pub happiness: String,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: String,
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

use serde::{Deserialize, Serialize};

use crate::modules::civilization::{FactionHappiness, FactionState, Government, Superpower};

/// Information related to a faction at a given location.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    /// The name of the faction.
    pub name: String,

    /// The state the faction is currently in.
    pub faction_state: FactionState,

    /// The type of government the faction adheres to.
    pub government: Government,

    /// The influence the faction has at the given location.
    pub influence: f32,

    /// The superpower the faction is aligned with.
    pub allegiance: Superpower,

    /// The general happiness of the faction at the give location.
    pub happiness: FactionHappiness,

    /// Localized description of the happiness of the faction at the given location.
    #[serde(rename = "Happiness_Localised")]
    pub happiness_localized: Option<String>,

    /// The reputation of the current player with the faction.
    pub my_reputation: f32,

    #[serde(default)]
    pub pending_states: Vec<FactionActiveState>,

    #[serde(default)]
    pub recovering_states: Vec<FactionActiveState>,

    #[serde(default)]
    pub active_states: Vec<FactionActiveState>,

    /// Whether the faction is part of a player controlled squadron.
    #[serde(default)]
    pub squadron_faction: bool,

    #[serde(default)]
    pub happiest_system: bool,

    /// Whether the faction at the given location is in their home system.
    #[serde(default)]
    pub home_system: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FactionActiveState {
    pub state: FactionState,
}

//! Fired when the player received a kill bond for a faction kill.

use serde::{Deserialize, Serialize};

/// Fired when the player received a kill bond for a faction kill.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FactionKillBondEvent {
    /// The reward in credits.
    pub reward: u64,

    /// The name of the rewarding faction.
    pub awarding_faction: String,

    /// The localized name of the rewarding faction.
    #[serde(rename = "AwardingFaction_Localised")]
    pub awarding_faction_localized: Option<String>,

    /// The name of the victim faction.
    pub victim_faction: String,

    /// The localized name of the victim faction.
    #[serde(rename = "VictimFaction_Localised")]
    pub victim_faction_localized: Option<String>,
}

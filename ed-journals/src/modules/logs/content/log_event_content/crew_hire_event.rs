//! Fired when the player fires a new crew member.

use serde::{Deserialize, Serialize};

use crate::modules::commander::CombatRank;

/// Fired when the player fires a new crew member.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewHireEvent {
    /// The name of the newly hired crew member.
    pub name: String,

    /// The id of the newly hired crew member.
    #[serde(rename = "CrewID")]
    pub crew_id: u64,

    /// The name of the faction the crew member is from.
    pub faction: String,

    /// The number of credits paid hiring the crew member.
    pub cost: u64,

    /// The combat rank of the crew member.
    pub combat_rank: CombatRank,
}

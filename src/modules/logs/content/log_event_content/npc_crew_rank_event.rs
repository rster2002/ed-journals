//! Fired when there is a change in the combat rank of one of the hired crew members.

use serde::{Deserialize, Serialize};

use crate::modules::commander::CombatRank;

/// Fired when there is a change in the combat rank of one of the hired crew members.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct NPCCrewRankEvent {
    /// The name of the crew member that has a change in combat rank.
    pub npc_crew_name: String,

    /// The id of the crew member that has a change in combat rank.
    pub npc_crew_id: u64,

    /// The new combat rank for this NPC.
    pub rank_combat: CombatRank,
}

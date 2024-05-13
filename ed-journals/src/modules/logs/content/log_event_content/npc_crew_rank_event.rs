use serde::{Serialize, Deserialize};

use crate::modules::shared::commander::combat_rank::CombatRank;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NPCCrewRankEvent {
    pub npc_crew_name: String,
    pub npc_crew_id: u64,
    pub rank_combat: CombatRank,
}

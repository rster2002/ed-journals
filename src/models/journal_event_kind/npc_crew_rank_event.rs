use serde::Deserialize;
use crate::models::journal_event_kind::shared::commander::combat_rank::CombatRank;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct NPCCrewRankEvent {
    pub npc_crew_name: String,
    pub npc_crew_id: u64,
    pub rank_combat: CombatRank,
}

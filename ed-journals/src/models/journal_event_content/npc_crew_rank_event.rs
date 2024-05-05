use crate::models::journal_event_content::shared::commander::combat_rank::CombatRank;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NPCCrewRankEvent {
    pub npc_crew_name: String,
    pub npc_crew_id: u64,
    pub rank_combat: CombatRank,
}

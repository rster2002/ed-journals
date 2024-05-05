use serde::Deserialize;
use crate::journal_event_content::shared::commander::combat_rank::CombatRank;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PVPKillEvent {
    pub combat_rank: CombatRank,
    pub victim: String,
}

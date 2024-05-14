use crate::modules::models::commander::combat_rank::CombatRank;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PVPKillEvent {
    pub combat_rank: CombatRank,
    pub victim: String,
}

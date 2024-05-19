use serde::{Serialize, Deserialize};
use crate::modules::commander::CombatRank;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PVPKillEvent {
    pub combat_rank: CombatRank,
    pub victim: String,
}

use serde::{Deserialize, Serialize};

use crate::modules::commander::CombatRank;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewHireEvent {
    pub name: String,

    #[serde(rename = "CrewID")]
    pub crew_id: u64,
    pub faction: String,
    pub cost: u64,
    pub combat_rank: CombatRank,
}

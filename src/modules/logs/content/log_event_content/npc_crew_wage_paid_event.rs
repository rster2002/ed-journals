use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NPCCrewWagePaidEvent {
    pub npc_crew_name: String,
    pub npc_crew_id: u64,
    pub amount: u64,
}

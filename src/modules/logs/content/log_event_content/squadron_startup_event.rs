use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SquadronStartupEvent {
    pub squadron_name: String,
    pub current_rank: u8,
}

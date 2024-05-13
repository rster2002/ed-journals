use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SquadronDemotionEvent {
    pub squadron_name: String,

    // TODO replace with enum
    pub old_rank: u8,
    pub new_rank: u8,
}

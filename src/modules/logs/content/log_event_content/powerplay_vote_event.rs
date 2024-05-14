use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayVoteEvent {
    pub power: String,
    pub system: String,
    pub votes: u64,
}

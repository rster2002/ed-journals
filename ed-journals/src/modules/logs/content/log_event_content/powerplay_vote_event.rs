use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayVoteEvent {
    pub power: String,
    pub system: String,
    pub votes: u64,
}

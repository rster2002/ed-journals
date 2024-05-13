use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Conflict {
    pub war_type: ConflictWarType,
    pub status: String,
    pub faction_1: ConflictFaction,
    pub faction_2: ConflictFaction,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ConflictWarType {
    #[serde(rename = "election")]
    Election,

    #[serde(rename = "war")]
    War,

    #[serde(rename = "civilwar")]
    CivilWar,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ConflictFaction {
    pub name: String,
    pub stake: String,
    pub won_days: u32,
}

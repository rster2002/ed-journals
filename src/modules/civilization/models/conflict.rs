use serde::{Deserialize, Serialize};

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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
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

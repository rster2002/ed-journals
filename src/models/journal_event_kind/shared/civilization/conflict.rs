use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct Conflict {
    pub war_type: ConflictWarType,
    pub status: String,
    pub faction_1: ConflictFaction,
    pub faction_2: ConflictFaction,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum ConflictWarType {
    #[serde(rename = "election")]
    Election,

    #[serde(rename = "war")]
    War,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ConflictFaction {
    pub name: String,
    pub stake: String,
    pub won_days: u32,
}

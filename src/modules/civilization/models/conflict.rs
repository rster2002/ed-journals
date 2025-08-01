use serde::{Deserialize, Serialize};

/// Information about a current conflict or war in a system.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct Conflict {
    /// The kind of conflict that is happening.
    pub war_type: ConflictWarType,

    /// The current status of the conflict.
    pub status: ConflictStatus,

    /// One of the factions that are part of the conflict.
    pub faction_1: ConflictFaction,

    /// The second one the factions that are part of the conflict.
    pub faction_2: ConflictFaction,
}

/// The kind of conflict.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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

/// The status of a given conflict.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ConflictStatus {
    #[serde(rename = "")]
    None,

    Active,
    Pending,
}

/// Information about the participating faction in a conflict.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ConflictFaction {
    /// The name of the faction participating in a conflict.
    pub name: String,

    /// The name of the asset that is at stake in the conflict.
    pub stake: String,

    /// The number of days the faction has won in the conflict.
    pub won_days: u32,
}

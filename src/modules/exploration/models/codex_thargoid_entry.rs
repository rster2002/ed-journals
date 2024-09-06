use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexThargoidEntry {
    #[serde(rename = "$Codex_Ent_Thargoid_Large_Spike_Name;")]
    LargeSpike,

    #[serde(rename = "$Codex_Ent_Thargoid_Tower_Name;")]
    Tower,

    #[serde(rename = "$Codex_Ent_Thargoid_Tower_Low_Name;")]
    TowerLow,

    #[serde(rename = "$Codex_Ent_Thargoid_Tower_Med_Name;")]
    TowerMedium,

    #[serde(rename = "$Codex_Ent_Thargoid_Tower__High_Name;")]
    TowerHigh,

    #[serde(rename = "$Codex_Ent_Thargoid_Tower_ExtraHigh_Name;")]
    TowerExtraHigh,

    #[serde(rename = "$Codex_Ent_Thargoid_Coral_Tree_Name;")]
    CoralTree,

    #[serde(rename = "$Codex_Ent_Thargoid_Coral_Root_Name;")]
    CoralRoot,
}

impl Display for CodexThargoidEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexThargoidEntry::LargeSpike => "Large Thargoid Spike",
            CodexThargoidEntry::Tower => "Thargoid Tower",
            CodexThargoidEntry::TowerLow => "Low Thargoid Tower",
            CodexThargoidEntry::TowerMedium => "Medium Thargoid Tower",
            CodexThargoidEntry::TowerHigh => "High Thargoid Tower",
            CodexThargoidEntry::TowerExtraHigh => "Extra High Thargoid Tower",
            CodexThargoidEntry::CoralTree => "Thargoid Coral Tree",
            CodexThargoidEntry::CoralRoot => "Thargoid Coral Root",
        })
    }
}

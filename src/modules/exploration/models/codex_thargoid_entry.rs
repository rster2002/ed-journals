use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexThargoidEntry {
    LargeSpike,
    Tower,
    TowerLow,
    TowerMedium,
    TowerHigh,
    TowerExtraHigh,
    CoralTree,
    CoralRoot,

    CausticGenerator,
    Banshee,
    Barnacles,
    Basilisk,
    Berserker,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexThargoidEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexThargoidEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CoxexThargoidEntryError {
    #[error("Failed to parse Thargoid codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown Thargoid entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexThargoidEntry {
    type Err = CoxexThargoidEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CoxexThargoidEntryError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "thargoid_large_spike" => CodexThargoidEntry::LargeSpike,
            "thargoid_tower" => CodexThargoidEntry::Tower,
            "thargoid_tower_low" => CodexThargoidEntry::TowerLow,
            "thargoid_tower_med" => CodexThargoidEntry::TowerMedium,
            "thargoid_tower__high" => CodexThargoidEntry::TowerHigh,
            "thargoid_tower_extrahigh" => CodexThargoidEntry::TowerExtraHigh,
            "thargoid_coral_tree" => CodexThargoidEntry::CoralTree,
            "thargoid_coral_root" => CodexThargoidEntry::CoralRoot,

            "caustic_generator" => CodexThargoidEntry::CausticGenerator,
            "banshee" => CodexThargoidEntry::Banshee,
            "barnacles" => CodexThargoidEntry::Barnacles,
            "basilisk" => CodexThargoidEntry::Basilisk,
            "berserker" => CodexThargoidEntry::Berserker,

            #[cfg(feature = "allow-unknown")]
            _ => CodexThargoidEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CoxexThargoidEntryError::UnknownEntry(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(CodexThargoidEntry);

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

            CodexThargoidEntry::CausticGenerator => "Caustic Generator",
            CodexThargoidEntry::Banshee => "Banshee",
            CodexThargoidEntry::Barnacles => "Barnacles",
            CodexThargoidEntry::Basilisk => "Basilisk",
            CodexThargoidEntry::Berserker => "Berserker",

            #[cfg(feature = "allow-unknown")]
            CodexThargoidEntry::Unknown(unknown) => return write!(f, "Unknown codex entry: '{}'", unknown),
        })
    }
}

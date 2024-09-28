use std::fmt::Display;
use std::str::FromStr;
use serde::Serialize;
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
    Coral,
    CoralTree,
    CoralRoot,

    Canister,
    Datascan,
    Pod,
    Transmitter,

    UnknownArtifact,
    UnknownProbe,
    UnknownRelay,

    Barnacles,
    CausticGenerator,
    Banshee,
    Revenant,
    Scavenger,

    Scouts,
    Marauder,
    Berserker,
    Regenerator,
    Inciter,

    Interceptors,
    Basilisk,
    Cyclops,
    Glaive,
    Scythe,
    Hunter,
    Medusa,
    Hydra,
    Orthrus,

    WreckedInterceptor,
    WreckedScout,

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
pub enum CoxexThargoidError {
    #[error("Failed to parse Thargoid codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown Thargoid entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexThargoidEntry {
    type Err = CoxexThargoidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CoxexThargoidError::FailedToParse(s.to_string()));
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
            "thargoid_tower__high" | "thargoid_tower_high" => CodexThargoidEntry::TowerHigh,
            "thargoid_tower_extrahigh" => CodexThargoidEntry::TowerExtraHigh,
            "thargoid_coral" => CodexThargoidEntry::Coral,
            "thargoid_coral_tree" => CodexThargoidEntry::CoralTree,
            "thargoid_coral_root" => CodexThargoidEntry::CoralRoot,

            "tg_canister" => CodexThargoidEntry::Canister,
            "tg_datascan" => CodexThargoidEntry::Datascan,
            "tg_pod" => CodexThargoidEntry::Pod,
            "tg_transmitter" => CodexThargoidEntry::Transmitter,

            "unknownartifact" => CodexThargoidEntry::UnknownArtifact,
            "unknownprobe" => CodexThargoidEntry::UnknownProbe,
            "unknownrelay" => CodexThargoidEntry::UnknownRelay,

            "caustic_generator" => CodexThargoidEntry::CausticGenerator,
            "banshee" => CodexThargoidEntry::Banshee,
            "barnacles" => CodexThargoidEntry::Barnacles,
            "revenant" => CodexThargoidEntry::Revenant,
            "scavenger" => CodexThargoidEntry::Scavenger,

            "scouts" => CodexThargoidEntry::Scouts,
            "marauder" => CodexThargoidEntry::Marauder,
            "berserker" => CodexThargoidEntry::Berserker,
            "regenerator" => CodexThargoidEntry::Regenerator,
            "inciter" => CodexThargoidEntry::Inciter,

            "interceptors" => CodexThargoidEntry::Interceptors,
            "basilisk" => CodexThargoidEntry::Basilisk,
            "cyclops" => CodexThargoidEntry::Cyclops,
            "glaive" => CodexThargoidEntry::Glaive,
            "scythe" => CodexThargoidEntry::Scythe,
            "hunters" => CodexThargoidEntry::Hunter,
            "medusa" => CodexThargoidEntry::Medusa,
            "hydra" => CodexThargoidEntry::Hydra,
            "orthrus" => CodexThargoidEntry::Orthrus,

            "wrecked_interceptor" => CodexThargoidEntry::WreckedInterceptor,
            "wrecked_scout" => CodexThargoidEntry::WreckedScout,

            #[cfg(feature = "allow-unknown")]
            _ => CodexThargoidEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CoxexThargoidError::UnknownEntry(string.to_string())),
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
            CodexThargoidEntry::Coral => "Thargoid Coral",
            CodexThargoidEntry::CoralTree => "Thargoid Coral Tree",
            CodexThargoidEntry::CoralRoot => "Thargoid Coral Root",

            CodexThargoidEntry::Canister => "Thargoid Canister",
            CodexThargoidEntry::Datascan => "Thargoid Datascan",
            CodexThargoidEntry::Pod => "Thargoid Pod",
            CodexThargoidEntry::Transmitter => "Thargoid Transmitter",

            CodexThargoidEntry::UnknownArtifact => "Unknown Artifact",
            CodexThargoidEntry::UnknownProbe => "Unknown Probe",
            CodexThargoidEntry::UnknownRelay => "Unknown Relay",

            CodexThargoidEntry::CausticGenerator => "Caustic Generator",
            CodexThargoidEntry::Banshee => "Banshee",
            CodexThargoidEntry::Barnacles => "Barnacles",
            CodexThargoidEntry::Revenant => "Revenant",
            CodexThargoidEntry::Scavenger => "Scavenger",

            CodexThargoidEntry::Scouts => "Scouts",
            CodexThargoidEntry::Marauder => "Marauder",
            CodexThargoidEntry::Berserker => "Berserker",
            CodexThargoidEntry::Regenerator => "Regenerator",
            CodexThargoidEntry::Inciter => "Inciter",

            CodexThargoidEntry::Interceptors => "Interceptors",
            CodexThargoidEntry::Basilisk => "Basilisk",
            CodexThargoidEntry::Cyclops => "Cyclops",
            CodexThargoidEntry::Glaive => "Glaive",
            CodexThargoidEntry::Scythe => "Scythe",
            CodexThargoidEntry::Hunter => "Hunter",
            CodexThargoidEntry::Medusa => "Medusa",
            CodexThargoidEntry::Hydra => "Hydra",
            CodexThargoidEntry::Orthrus => "Orthrus",

            CodexThargoidEntry::WreckedInterceptor => "Wrecked Interceptor",
            CodexThargoidEntry::WreckedScout => "Wrecked Scout",

            #[cfg(feature = "allow-unknown")]
            CodexThargoidEntry::Unknown(unknown) => return write!(f, "Unknown codex entry: '{}'", unknown),
        })
    }
}

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::exploration::models::codex_planet_entry::CodexPlanetError;
use crate::exploration::shared::codex_regex::CODEX_REGEX;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexAnomalyEntry {
    LagrangeClouds,

    BlueGasClouds,
    StormingBlueGasClouds,
    ChargedGasClouds,
    GreenGasClouds,
    StormingGreenGasClouds,
    LightGasClouds,
    GasClouds,
    OrangeGasClouds,
    StormingOrangeGasClouds,
    PinkGasClouds,
    StormingPinkGasClouds,
    RedGasClouds,
    StormingRedGasClouds,
    StandardGasClouds,
    YellowGasClouds,
    StormingYellowGasClouds,

    // ETypeAnomalies,
    // KTypeAnomalies,
    // LTypeAnomalies,
    // PTypeAnomalies,
    // QTypeAnomalies,
    // TTypeAnomalies,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexAnomalyEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexAnomalyEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexAnomalyError {
    #[error("Failed to parse codex anomaly entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown codex anomaly entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexAnomalyEntry {
    type Err = CodexAnomalyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CodexAnomalyError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "clouds" => CodexAnomalyEntry::LagrangeClouds,
            "gas_clds_blue" => CodexAnomalyEntry::BlueGasClouds,
            "gas_clds_blue_storm" => CodexAnomalyEntry::StormingBlueGasClouds,
            "gas_clds_charged" => CodexAnomalyEntry::ChargedGasClouds,
            "gas_clds_green" => CodexAnomalyEntry::GreenGasClouds,
            "gas_clds_green_storm" => CodexAnomalyEntry::StormingGreenGasClouds,
            "gas_clds_light" => CodexAnomalyEntry::LightGasClouds,
            "gas_clds" => CodexAnomalyEntry::GasClouds,
            "gas_clds_orange" => CodexAnomalyEntry::OrangeGasClouds,
            "gas_clds_orange_storm" => CodexAnomalyEntry::StormingOrangeGasClouds,
            "gas_clds_pink" => CodexAnomalyEntry::PinkGasClouds,
            "gas_clds_pink_storm" => CodexAnomalyEntry::StormingPinkGasClouds,
            "gas_clds_red" => CodexAnomalyEntry::RedGasClouds,
            "gas_clds_red_storm" => CodexAnomalyEntry::StormingRedGasClouds,
            "gas_clds_standard" => CodexAnomalyEntry::StandardGasClouds,
            "gas_clds_yellow" => CodexAnomalyEntry::YellowGasClouds,
            "gas_clds_yellow_storm" => CodexAnomalyEntry::StormingYellowGasClouds,

            #[cfg(feature = "allow-unknown")]
            _ => CodexAnomalyEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexAnomalyError::UnknownEntry(string.to_string())),
        })
    }
}

impl Display for CodexAnomalyEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexAnomalyEntry::LagrangeClouds => "Lagrange Clouds",
            CodexAnomalyEntry::BlueGasClouds => "Blue Gas Clouds",
            CodexAnomalyEntry::StormingBlueGasClouds => "Storming Blue Gas Clouds",
            CodexAnomalyEntry::ChargedGasClouds => "Charged Gas Clouds",
            CodexAnomalyEntry::GreenGasClouds => "Green Gas Clouds",
            CodexAnomalyEntry::StormingGreenGasClouds => "Storming Green Gas Clouds",
            CodexAnomalyEntry::LightGasClouds => "Light Gas Clouds",
            CodexAnomalyEntry::GasClouds => "Gas Clouds",
            CodexAnomalyEntry::OrangeGasClouds => "Orange Gas Clouds",
            CodexAnomalyEntry::StormingOrangeGasClouds => "Storming Orange Gas Clouds",
            CodexAnomalyEntry::PinkGasClouds => "Pink Gas Clouds",
            CodexAnomalyEntry::StormingPinkGasClouds => "Storming Pink Gas Clouds",
            CodexAnomalyEntry::RedGasClouds => "Red Gas Clouds",
            CodexAnomalyEntry::StormingRedGasClouds => "Storming Red Gas Clouds",
            CodexAnomalyEntry::StandardGasClouds => "Standing Gas Clouds",
            CodexAnomalyEntry::YellowGasClouds => "Yellow Gas Clouds",
            CodexAnomalyEntry::StormingYellowGasClouds => "Storming Yellow Gas Clouds",

            #[cfg(feature = "allow-unknown")]
            CodexAnomalyEntry::Unknown(unknown) => return write!(f, "Unknown anomaly codex entry: {}", unknown),

            // CodexAnomalyEntry::ETypeAnomalies => "E Type Anomalies",
            // CodexAnomalyEntry::KTypeAnomalies => "K Type Anomalies",
            // CodexAnomalyEntry::LTypeAnomalies => "L Type Anomalies",
            // CodexAnomalyEntry::PTypeAnomalies => "P Type Anomalies",
            // CodexAnomalyEntry::QTypeAnomalies => "Q Type Anomalies",
            // CodexAnomalyEntry::TTypeAnomalies => "T Type Anomalies",
        })
    }
}

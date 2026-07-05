use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// A type of planetary signal that can be present on a planet.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlanetarySignalType {
    /// Human signals indicate settlements and outposts.
    #[serde(rename = "$SAA_SignalType_Human;")]
    Human,

    /// Biological signals indicate that there are plants on the surface of the planet.
    #[serde(rename = "$SAA_SignalType_Biological;")]
    Biological,

    /// Geological signals can include things like geysers and crystal shards.
    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,

    /// Unknown signals related to Thargoid activity on the planet.
    #[serde(rename = "$SAA_SignalType_Thargoid;")]
    Thargoid,

    /// Unknown signals related to Guardian activity on the planet.
    #[serde(rename = "$SAA_SignalType_Guardian;")]
    Guardian,

    /// Unknown anomalous signal.
    #[serde(rename = "$SAA_SignalType_PlanetAnomaly;")]
    PlanetAnomaly,

    /// Other unspecified signal.
    #[serde(rename = "$SAA_SignalType_Other;")]
    Other,

    /// Signal related to a specific commodity.
    #[serde(untagged)]
    Commodity(Commodity),

    /// An unknown signal that could not be parsed.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl PlanetarySignalType {
    /// Whether the current variant is unknown.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, PlanetarySignalType::Unknown(_))
    }
}

impl Display for PlanetarySignalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlanetarySignalType::Human => "Human",
                PlanetarySignalType::Biological => "Biological",
                PlanetarySignalType::Geological => "Geological",
                PlanetarySignalType::Thargoid => "Thargoid",
                PlanetarySignalType::Guardian => "Guardian",
                PlanetarySignalType::PlanetAnomaly => "Planet Anomaly",
                PlanetarySignalType::Other => "Other",
                PlanetarySignalType::Commodity(commodity) => return commodity.fmt(f),

                #[cfg(feature = "allow-unknown")]
                PlanetarySignalType::Unknown(unknown) =>
                    return write!(f, "Unknown planetary signal: {unknown}"),
            }
        )
    }
}

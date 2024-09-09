use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexPlanetEntry {
    EarthLike,

    AmmoniaWorld,
    DenseAmmoniaWorld,
    DenseWaterWorld,
    DenseTerraformableHighMetalContentPlanet,
    DenseTerraformableIcePlanet,
    DenseTerraformableMetalRichPlanet,
    DenseTerraformableRockyIcePlanet,
    DenseTerraformableRockyPlanet,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexPlanetEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexPlanetEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexPlanetError {
    #[error("Failed to parse planet codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown planet codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexPlanetEntry {
    type Err = CodexPlanetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CodexPlanetError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "earth_likes" => CodexPlanetEntry::EarthLike,

            "ammonia_worlds" => CodexPlanetEntry::AmmoniaWorld,
            "dense_ammonia_worlds" => CodexPlanetEntry::DenseAmmoniaWorld,
            "dense_water_worlds" => CodexPlanetEntry::DenseWaterWorld,
            "dense_ter_high_metal_content" => CodexPlanetEntry::DenseTerraformableHighMetalContentPlanet,
            "dense_ter_ice" => CodexPlanetEntry::DenseTerraformableIcePlanet,
            "dense_ter_metal_rich" => CodexPlanetEntry::DenseTerraformableMetalRichPlanet,
            "dense_ter_rocky_ice" => CodexPlanetEntry::DenseTerraformableRockyIcePlanet,
            "dense_ter_rocky" => CodexPlanetEntry::DenseTerraformableRockyPlanet,

            #[cfg(feature = "allow-unknown")]
            _ => CodexPlanetEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexPlanetError::UnknownEntry(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(CodexPlanetEntry);

impl Display for CodexPlanetEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexPlanetEntry::EarthLike => "earth_like",

            CodexPlanetEntry::AmmoniaWorld => "Ammonia World",
            CodexPlanetEntry::DenseAmmoniaWorld => "Dense Ammonia World",
            CodexPlanetEntry::DenseWaterWorld => "Dense Water World",
            CodexPlanetEntry::DenseTerraformableHighMetalContentPlanet => "Dense Terraformable High Metal Content Planet",
            CodexPlanetEntry::DenseTerraformableIcePlanet => "Dense Terraformable Ice Planet",
            CodexPlanetEntry::DenseTerraformableMetalRichPlanet => "Dense Terraformable Metal-Rich Planet",
            CodexPlanetEntry::DenseTerraformableRockyIcePlanet => "Dense Terraformable Rocky Ice Planet",
            CodexPlanetEntry::DenseTerraformableRockyPlanet => "Dense Terraformable Rocky Planet",

            #[cfg(feature = "allow-unknown")]
            CodexPlanetEntry::Unknown(unknown) => return write!(f, "Unknown planet codex entry: {}", unknown),
        })
    }
}

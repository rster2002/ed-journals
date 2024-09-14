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
    DenseTerraformableHighMetalPlanet,
    DenseTerraformableIce,
    DenseTerraformableMetalRich,
    DenseTerraformableRockyIce,
    DenseTerraformableRocky,

    GiantWithAmmoniaLife,
    GiantWithWaterLife,

    GreenGiants,
    GreenGiantWithAmmoniaLife,
    GreenGiantWithWaterLife,
    GreenSudarskyClassI,
    GreenSudarskyClassII,
    GreenSudarskyClassIII,
    GreenSudarskyClassIV,
    GreenSudarskyClassV,
    GreenWaterGiant,
    GreenWaterGiantWithLife,

    HighMetalContentNoAtmosphere,
    IceNoAtmosphere,

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
            "dense_ter_high_metal_content" => CodexPlanetEntry::DenseTerraformableHighMetalPlanet,
            "dense_ter_ice" => CodexPlanetEntry::DenseTerraformableIce,
            "dense_ter_metal_rich" => CodexPlanetEntry::DenseTerraformableMetalRich,
            "dense_ter_rocky_ice" => CodexPlanetEntry::DenseTerraformableRockyIce,
            "dense_ter_rocky" => CodexPlanetEntry::DenseTerraformableRocky,

            "giant_with_ammonia_life" => CodexPlanetEntry::GiantWithAmmoniaLife,
            "giant_with_water_life" => CodexPlanetEntry::GiantWithWaterLife,

            "greengiants" => CodexPlanetEntry::GreenGiants,
            "green_giant_with_ammonia_life" => CodexPlanetEntry::GreenGiantWithAmmoniaLife,
            "green_giant_with_water_life" => CodexPlanetEntry::GreenGiantWithWaterLife,
            "green_sudarsky_class_i" => CodexPlanetEntry::GreenSudarskyClassI,
            "green_sudarsky_class_ii" => CodexPlanetEntry::GreenSudarskyClassII,
            "green_sudarsky_class_iii" => CodexPlanetEntry::GreenSudarskyClassIII,
            "green_sudarsky_class_iv" => CodexPlanetEntry::GreenSudarskyClassIV,
            "green_sudarsky_class_v" => CodexPlanetEntry::GreenSudarskyClassV,
            "green_water_giant" => CodexPlanetEntry::GreenWaterGiant,
            "green_water_giant_with_life" => CodexPlanetEntry::GreenWaterGiantWithLife,

            "high_metal_content_no_atmos" => CodexPlanetEntry::HighMetalContentNoAtmosphere,
            "ice_no_atmos" => CodexPlanetEntry::IceNoAtmosphere,

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
            CodexPlanetEntry::DenseTerraformableHighMetalPlanet => "Dense Terraformable High Metal Content Planet",
            CodexPlanetEntry::DenseTerraformableIce => "Dense Terraformable Ice Planet",
            CodexPlanetEntry::DenseTerraformableMetalRich => "Dense Terraformable Metal-Rich Planet",
            CodexPlanetEntry::DenseTerraformableRockyIce => "Dense Terraformable Rocky Ice Planet",
            CodexPlanetEntry::DenseTerraformableRocky => "Dense Terraformable Rocky Planet",

            CodexPlanetEntry::GreenGiants => "Green Giants",
            CodexPlanetEntry::GiantWithAmmoniaLife => "Giant with Ammonia Life",
            CodexPlanetEntry::GiantWithWaterLife => "Giant with Water Life",
            CodexPlanetEntry::GreenGiantWithAmmoniaLife => "Green Giant with Ammonia Life",
            CodexPlanetEntry::GreenGiantWithWaterLife => "Green Giant with Water Life",
            CodexPlanetEntry::GreenSudarskyClassI => "Green Sudarsky Class I",
            CodexPlanetEntry::GreenSudarskyClassII => "Green Sudarsky Class II",
            CodexPlanetEntry::GreenSudarskyClassIII => "Green Sudarsky Class III",
            CodexPlanetEntry::GreenSudarskyClassIV => "Green Sudarsky Class IV",
            CodexPlanetEntry::GreenSudarskyClassV => "Green Sudarsky Class V",
            CodexPlanetEntry::GreenWaterGiant => "Green Water Giant",
            CodexPlanetEntry::GreenWaterGiantWithLife => "Green Water Giant with Life",

            CodexPlanetEntry::HighMetalContentNoAtmosphere => "High Metal Content Planet, No Atmosphere",
            CodexPlanetEntry::IceNoAtmosphere => "Ice Planet, No Atmosphere",

            #[cfg(feature = "allow-unknown")]
            CodexPlanetEntry::Unknown(unknown) => return write!(f, "Unknown planet codex entry: {}", unknown),
        })
    }
}

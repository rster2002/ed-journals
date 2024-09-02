use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum PlanetClass {
    MetalRichBody,
    HighMetalContentBody,
    RockyBody,
    IcyBody,
    RockyIceBody,
    EarthlikeBody,
    WaterWorld,
    AmmoniaWorld,
    WaterGiant,
    WaterGiantWithLife,
    GasGiantWithWaterBasedLife,
    GasGiantWithAmmoniaBasedLife,
    SudarskyClassIGasGiant,
    SudarskyClassIIGasGiant,
    SudarskyClassIIIGasGiant,
    SudarskyClassIVGasGiant,
    SudarskyClassVGasGiant,
    HeliumRichGasGiant,
    HeliumGasGiant,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum PlanetClassError {
    #[error("Failed to parse planet class: '{0}'")]
    FailedToParse(String),

    #[error("Unknown planet class: '{0}'")]
    UnknownPlanetClass(String),
}

impl FromStr for PlanetClass {
    type Err = PlanetClassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.to_ascii_lowercase()
            .replace('_', " ");

        let s: &str = &string.trim_end_matches('s')
            .trim_end_matches(" body")
            .trim_end_matches(" world");

        Ok(match s {
            "metal rich" => PlanetClass::MetalRichBody,
            "high metal content" => PlanetClass::HighMetalContentBody,
            "rocky" => PlanetClass::RockyBody,
            "icy" | "ice" => PlanetClass::IcyBody,
            "rocky ice" => PlanetClass::RockyIceBody,
            "earthlike" => PlanetClass::EarthlikeBody,
            "water" => PlanetClass::WaterWorld,
            "ammonia" => PlanetClass::AmmoniaWorld,
            "water giant" => PlanetClass::WaterGiant,
            "water giant with life" | "giant with water life" => PlanetClass::WaterGiantWithLife,
            "gas giant with water based life" => PlanetClass::GasGiantWithWaterBasedLife,
            "gas giant with ammonia based life" => PlanetClass::GasGiantWithAmmoniaBasedLife,
            "sudarsky class i gas giant" | "sudarsky class i" => PlanetClass::SudarskyClassIGasGiant,
            "sudarsky class ii gas giant" | "sudarsky class ii" => PlanetClass::SudarskyClassIIGasGiant,
            "sudarsky class iii gas giant" | "sudarsky class iii" => PlanetClass::SudarskyClassIIIGasGiant,
            "sudarsky class iv gas giant" | "sudarsky class iv" => PlanetClass::SudarskyClassIVGasGiant,
            "sudarsky class v gas giant" | "sudarsky class v" => PlanetClass::SudarskyClassVGasGiant,
            "helium rich gas giant" => PlanetClass::HeliumRichGasGiant,
            "helium gas giant" => PlanetClass::HeliumGasGiant,

            #[cfg(feature = "allow-unknown")]
            _ => PlanetClass::Unknown(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(PlanetClassError::UnknownPlanetClass(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(PlanetClass);

impl PlanetClass {
    /// Returns the base exploration value of the star planet class.
    pub fn base_value(&self) -> u64 {
        match self {
            PlanetClass::MetalRichBody => 21_790,
            PlanetClass::AmmoniaWorld => 96_932,
            PlanetClass::SudarskyClassIGasGiant => 1_656,
            PlanetClass::SudarskyClassIIGasGiant => 9_654,
            PlanetClass::HighMetalContentBody => 9_654,
            PlanetClass::WaterWorld => 64_831,
            PlanetClass::EarthlikeBody => 64_831,
            _ => 300,
        }
    }

    /// Returns the bonus exploration value if the planet is terraformable.
    pub fn terraformable_bonus(&self) -> u64 {
        match self {
            PlanetClass::MetalRichBody => 65_631,
            PlanetClass::AmmoniaWorld => 0,
            PlanetClass::SudarskyClassIGasGiant => 0,
            PlanetClass::SudarskyClassIIGasGiant => 100_677,
            PlanetClass::HighMetalContentBody => 100_677,
            PlanetClass::WaterWorld => 116_295,
            PlanetClass::EarthlikeBody => 116_295,
            _ => 93_328,
        }
    }
}

impl Display for PlanetClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PlanetClass::MetalRichBody => "Metal Rich Body",
            PlanetClass::HighMetalContentBody => "High Metal Content Body",
            PlanetClass::RockyBody => "Rocky Body",
            PlanetClass::IcyBody => "Icy Body",
            PlanetClass::RockyIceBody => "Rocky Ice Body",
            PlanetClass::EarthlikeBody => "Earth-like Body",
            PlanetClass::WaterWorld => "Water World",
            PlanetClass::AmmoniaWorld => "Ammonia World",
            PlanetClass::WaterGiant => "Water Giant",
            PlanetClass::WaterGiantWithLife => "Water Giant with life",
            PlanetClass::GasGiantWithWaterBasedLife => "Gas Giant with water based life",
            PlanetClass::GasGiantWithAmmoniaBasedLife => "Gas Giant with ammonia based life",
            PlanetClass::SudarskyClassIGasGiant => "Sudarsky Class I Gas Giant",
            PlanetClass::SudarskyClassIIGasGiant => "Sudarsky Class II Gas Giant",
            PlanetClass::SudarskyClassIIIGasGiant => "Sudarsky Class III Gas Giant",
            PlanetClass::SudarskyClassIVGasGiant => "Sudarsky Class IV Gas Giant",
            PlanetClass::SudarskyClassVGasGiant => "Sudarsky Class V Gas Giant",
            PlanetClass::HeliumRichGasGiant => "Helium Rich Gas Giant",
            PlanetClass::HeliumGasGiant => "Helium Gas Giant",

            #[cfg(feature = "allow-unknown")]
            PlanetClass::Unknown(unknown) => return write!(f, "Unknown planet class: '{}'", unknown)
        })
    }
}

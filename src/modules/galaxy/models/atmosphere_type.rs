use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AtmosphereType {
    None,
    Ammonia,
    Helium,
    Nitrogen,
    AmmoniaRich,
    Neon,
    CarbonDioxideRich,
    Methane,
    WaterRich,
    CarbonDioxide,
    Water,
    ArgonRich,
    MethaneRich,
    SilicateVapour,
    Argon,
    EarthLike,
    NeonRich,
    Oxygen,
    AmmoniaOxygen,
    SulfurDioxide,
    MetallicVapour,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum AtmosphereTypeError {
    #[error("Unknown atmosphere type: '{0}'")]
    UnknownAtmosphereType(String),
}

impl FromStr for AtmosphereType {
    type Err = AtmosphereTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase: &str = &s.to_ascii_lowercase();

        Ok(match lowercase {
            "none" | "" | " atmosphere" => AtmosphereType::None,
            "ammonia" => AtmosphereType::Ammonia,
            "helium" => AtmosphereType::Helium,
            "nitrogen" => AtmosphereType::Nitrogen,
            "ammonia rich" | "ammoniarich" | "ammonia-rich" => AtmosphereType::AmmoniaRich,
            "neon" => AtmosphereType::Neon,
            "carbon dioxide rich" | "carbondioxiderich" | "carbon dioxide-rich" => AtmosphereType::CarbonDioxideRich,
            "methane" => AtmosphereType::Methane,
            "water rich" | "waterrich" | "water-rich" => AtmosphereType::WaterRich,
            "carbon dioxide" | "carbondioxide" => AtmosphereType::CarbonDioxide,
            "water" => AtmosphereType::Water,
            "argon rich" | "argonrich" | "argon-rich" => AtmosphereType::ArgonRich,
            "methane rich" | "methanerich" | "methane-rich" => AtmosphereType::MethaneRich,
            "silicate vapour" | "silicatevapour" => AtmosphereType::SilicateVapour,
            "argon" => AtmosphereType::Argon,
            "earth like" | "earthlike" => AtmosphereType::EarthLike,
            "neon rich" | "neonrich" | "neon-rich" => AtmosphereType::NeonRich,
            "oxygen" => AtmosphereType::Oxygen,
            "ammonia oxygen" | "ammoniaoxygen" => AtmosphereType::AmmoniaOxygen,
            "sulphur dioxide" | "sulphurdioxide" | "sulfur dioxide" | "sulfurdioxide" => AtmosphereType::SulfurDioxide,
            "metallic vapour" | "metallicvapour" => AtmosphereType::MetallicVapour,

            #[cfg(feature = "strict")]
            _ => return Err(AtmosphereTypeError::UnknownAtmosphereType(s.to_string())),

            #[cfg(not(feature = "strict"))]
            _ => AtmosphereType::Unknown(s.to_string()),
        })
    }
}

from_str_deserialize_impl!(AtmosphereType);

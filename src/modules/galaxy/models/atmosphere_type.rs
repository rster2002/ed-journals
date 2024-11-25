use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AtmosphereType {
    None,

    #[serde(alias = "ammonia")]
    Ammonia,

    #[serde(alias = "helium")]
    Helium,

    #[serde(alias = "nitrogen")]
    Nitrogen,

    #[serde(alias = "ammonia rich")]
    AmmoniaRich,

    #[serde(alias = "neon")]
    Neon,

    #[serde(alias = "carbon dioxide rich")]
    CarbonDioxideRich,

    #[serde(alias = "methane")]
    Methane,

    #[serde(alias = "water rich")]
    WaterRich,

    #[serde(alias = "carbon dioxide")]
    CarbonDioxide,

    #[serde(alias = "water")]
    Water,

    #[serde(alias = "argon rich")]
    ArgonRich,

    #[serde(alias = "methane rich")]
    MethaneRich,

    #[serde(alias = "silicate vapour")]
    SilicateVapour,

    #[serde(alias = "argon")]
    Argon,

    #[serde(alias = "earth like")]
    EarthLike,

    #[serde(alias = "neon rich")]
    NeonRich,

    #[serde(alias = "oxygen")]
    Oxygen,

    #[serde(alias = "ammonia oxygen")]
    AmmoniaOxygen,

    #[serde(alias = "sulfur dioxide", alias = "SulphurDioxide")]
    SulfurDioxide,

    #[serde(alias = "metallic vapour")]
    MetallicVapour,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for AtmosphereType {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_ascii_lowercase()))
    }
}

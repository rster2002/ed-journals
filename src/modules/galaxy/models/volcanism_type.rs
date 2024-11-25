use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VolcanismType {
    None,

    #[serde(alias = "water magma")]
    WaterMagma,

    #[serde(alias = "sulphur dioxide magma")]
    SulphurDioxideMagma,

    #[serde(alias = "ammonia magma")]
    AmmoniaMagma,

    #[serde(alias = "methane magma")]
    MethaneMagma,

    #[serde(alias = "nitrogen magma")]
    NitrogenMagma,

    #[serde(alias = "silicate magma")]
    SilicateMagma,

    #[serde(alias = "metallic magma")]
    MetallicMagma,

    #[serde(alias = "water geysers")]
    WaterGeysers,

    #[serde(alias = "carbon dioxide geysers")]
    CarbonDioxideGeysers,

    #[serde(alias = "ammonia geysers")]
    AmmoniaGeysers,

    #[serde(alias = "methane geysers")]
    MethaneGeysers,

    #[serde(alias = "nitrogen geysers")]
    NitrogenGeysers,

    #[serde(alias = "helium geysers")]
    HeliumGeysers,

    #[serde(alias = "silicate vapour geysers")]
    SilicateVapourGeysers,

    Iron,

    #[serde(alias = "rocky magma")]
    RockyMagma,
}

impl FromStr for VolcanismType {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_ascii_lowercase()))
    }
}

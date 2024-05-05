use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ShipType {
    #[serde(alias = "anaconda")]
    Anaconda,

    #[serde(alias = "sidewinder")]
    SideWinder,

    #[serde(rename = "eagle")]
    Eagle,

    #[serde(rename = "empire_eagle")]
    ImperialEagle,

    #[serde(rename = "vulture")]
    Vulture,

    #[serde(rename = "empire_trader")]
    Clipper,

    #[serde(rename = "empire_courier")]
    ImperialCourier,

    #[serde(rename = "adder")]
    Adder,

    #[serde(rename = "federation_gunship")]
    FederalGunship,

    #[serde(rename = "federation_dropship_mkii")]
    FederalAssaultShip,

    #[serde(rename = "federation_dropship")]
    FederalDropship,

    #[serde(rename = "diamondback")]
    DiamondBackScout,

    #[serde(rename = "diamondbackxl")]
    DiamondBackExplorer,

    #[serde(rename = "viper")]
    ViperMkIII,

    #[serde(rename = "viper_mkiv")]
    ViperMkIV,

    #[serde(alias = "cobramkiii")]
    CobraMkIII,

    #[serde(rename = "cobramkiv")]
    CobraMkIV,

    #[serde(rename = "Krait_Light", alias = "krait_light")]
    KraitPhantom,

    #[serde(rename = "Krait_MkII", alias = "krait_mkii")]
    KraitMkII,

    #[serde(alias = "python")]
    Python,

    #[serde(rename = "TypeX", alias = "typex")]
    AllianceChieftain,

    #[serde(rename = "Type7", alias = "type7")]
    Type7,

    #[serde(rename = "Type9", alias = "type9")]
    Type9,

    #[serde(rename = "Type9_Military", alias = "type9_military")]
    Type10,

    #[serde(rename = "asp_scout")]
    AspScout,

    #[serde(rename = "Asp", alias = "asp")]
    AspExplorer,

    #[serde(rename = "FerDeLance", alias = "ferdelance")]
    FerDeLance,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for ShipType {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
    }
}

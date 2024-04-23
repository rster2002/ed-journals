use std::str::FromStr;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::exploration::planetary_signal_type::PlanetarySignalType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ShipType {
    #[serde(alias = "sidewinder")]
    SideWinder,

    #[serde(alias = "cobramkiii")]
    CobraMkIII,

    #[serde(rename = "Krait_Light", alias = "krait_light")]
    KraitPhantom,

    #[serde(rename = "Krait_MkII", alias = "krait_mkii")]
    KraitMkII,
    Python,

    #[serde(rename = "TypeX", alias = "typex")]
    AllianceChieftain,

    #[serde(rename = "Type7", alias = "type7")]
    Type7,

    #[serde(rename = "Type9", alias = "type9")]
    Type9,

    #[serde(rename = "Type9_Military")]
    Type10,

    #[serde(rename = "Asp", alias = "asp")]
    AspExplorer,

    #[serde(rename = "ferdelance")]
    FerDeLance,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for ShipType {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

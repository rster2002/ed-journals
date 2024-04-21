use std::str::FromStr;
use serde::Deserialize;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::exploration::planetary_signal_type::PlanetarySignalType;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ShipType {
    SideWinder,
    CobraMkIII,
    KraitPhantom,
    KraitMkII,
    Python,
    AllianceChieftain,
    Type7,
    Type9,
    Type10,
    AspExplorer,
    FerDeLance,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

impl FromStr for ShipType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SideWinder" => Ok(ShipType::SideWinder),
            "CobraMkIII" => Ok(ShipType::CobraMkIII),
            "Krait_Light" => Ok(ShipType::KraitPhantom),
            "krait_light" => Ok(ShipType::KraitPhantom),
            "krait_mkii" => Ok(ShipType::KraitMkII),
            "Type7" => Ok(ShipType::Type7),
            "type7" => Ok(ShipType::Type7),
            "Python" => Ok(ShipType::Python),
            "TypeX" => Ok(ShipType::AllianceChieftain),
            "typex" => Ok(ShipType::AllianceChieftain),
            "Type9" => Ok(ShipType::Type9),
            "type9" => Ok(ShipType::Type9),
            "asp" => Ok(ShipType::AspExplorer),
            "ferdelance" => Ok(ShipType::FerDeLance),
            "Type9_Military" => Ok(ShipType::Type10),

            #[cfg(not(feature = "strict"))]
            _ => Ok(ShipType::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(s.to_string()),
        }
    }
}

from_str_deserialize_impl!(ShipType);

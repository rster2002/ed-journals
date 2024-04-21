use std::str::FromStr;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PlanetarySignalType {
    Human,
    Biological,
    Geological,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

impl FromStr for PlanetarySignalType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$SAA_SignalType_Human;" => Ok(PlanetarySignalType::Human),
            "$SAA_SignalType_Biological;" => Ok(PlanetarySignalType::Biological),
            "$SAA_SignalType_Geological;" => Ok(PlanetarySignalType::Geological),

            #[cfg(not(feature = "strict"))]
            _ => Ok(PlanetarySignalType::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(s.to_string()),
        }
    }
}

from_str_deserialize_impl!(PlanetarySignalType);

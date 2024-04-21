use std::str::FromStr;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StationType {
    Outpost,
    FleetCarrier,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

impl FromStr for StationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Outpost" => Ok(StationType::Outpost),
            "FleetCarrier" => Ok(StationType::FleetCarrier),

            #[cfg(not(feature = "strict"))]
            _ => Ok(StationType::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(s.to_string()),
        }
    }
}

from_str_deserialize_impl!(StationType);

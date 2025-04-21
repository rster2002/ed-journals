use crate::from_str_deserialize_impl;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum StationType {
    AsteroidBase,
    Bernal,
    Orbis,
    Coriolis,
    Ocellus,
    Outpost,
    FleetCarrier,
    MegaShip,
    CraterOutpost,
    CraterPort,
    OnFootSettlement,
    SurfaceStation,
    PlanetaryConstructionDepot,
    SpaceConstructionDepot,

    /// This is a special case. Stations involved in colonisation sometimes have
    /// a `"StationType"=""`. So it is not unknown, there is just no type assigned (yet).
    /// This might be changed/fixed in later game-version, but has been seen in 4.1.0.0 and 4.1.1.0.
    None,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum StationTypeError {
    #[error("Failed to parse StationType: '{0}'")]
    FailedToParse(String),
}

impl FromStr for StationType {
    type Err = StationTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AsteroidBase" => Self::AsteroidBase,
            "Bernal" => Self::Bernal,
            "Orbis" => Self::Orbis,
            "Coriolis" => Self::Coriolis,
            "Ocellus" => Self::Ocellus,
            "Outpost" => Self::Outpost,
            "FleetCarrier" => Self::FleetCarrier,
            "MegaShip" => Self::MegaShip,
            "CraterOutpost" => Self::CraterOutpost,
            "CraterPort" => Self::CraterPort,
            "OnFootSettlement" => Self::OnFootSettlement,
            "SurfaceStation" => Self::SurfaceStation,
            "PlanetaryConstructionDepot" => Self::PlanetaryConstructionDepot,
            "SpaceConstructionDepot" => Self::SpaceConstructionDepot,
            "None" => Self::None,
            "" => Self::None,

            #[cfg(feature = "allow-unknown")]
            _ => StationType::Unknown(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(StationTypeError::FailedToParse(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(StationType);

impl Display for StationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StationType::AsteroidBase => "Asteroid Base",
                StationType::Bernal => "Bernal",
                StationType::Orbis => "Orbis",
                StationType::Coriolis => "Coriolis",
                StationType::Ocellus => "Ocellus",
                StationType::Outpost => "Outpost",
                StationType::FleetCarrier => "Fleet Carrier",
                StationType::MegaShip => "Mega Ship",
                StationType::CraterOutpost => "Crater Outpost",
                StationType::CraterPort => "Crater Port",
                StationType::OnFootSettlement => "On-Foot Settlement",
                StationType::SurfaceStation => "Surface Station",
                StationType::PlanetaryConstructionDepot => "Planetary Construction Depot",
                StationType::SpaceConstructionDepot => "Space Construction Depot",
                StationType::None => "None",

                #[cfg(feature = "allow-unknown")]
                StationType::Unknown(unknown) =>
                    return write!(f, "Unknown station type: {}", unknown),
            }
        )
    }
}

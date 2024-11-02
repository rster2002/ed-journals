use std::fmt::{Display, Formatter};

use serde::Serialize;
use thiserror::Error;

use crate::try_from_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum FederationRank {
    None,
    Recruit,
    Cadet,
    Midshipman,
    PettyOfficer,
    ChiefPettyOfficer,
    WarrantOfficer,
    Ensign,
    Lieutenant,
    LieutenantCommander,
    PostCommander,
    PostCaptain,
    RearAdmiral,
    ViceAdmiral,
    Admiral,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum FederationRankError {
    #[error("Unknown federation rank with id '{0}'")]
    UnknownFederationRank(u8),
}

impl TryFrom<u8> for FederationRank {
    type Error = FederationRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FederationRank::None),
            1 => Ok(FederationRank::Recruit),
            2 => Ok(FederationRank::Cadet),
            3 => Ok(FederationRank::Midshipman),
            4 => Ok(FederationRank::PettyOfficer),
            5 => Ok(FederationRank::ChiefPettyOfficer),
            6 => Ok(FederationRank::WarrantOfficer),
            7 => Ok(FederationRank::Ensign),
            8 => Ok(FederationRank::Lieutenant),
            9 => Ok(FederationRank::LieutenantCommander),
            10 => Ok(FederationRank::PostCommander),
            11 => Ok(FederationRank::PostCaptain),
            12 => Ok(FederationRank::RearAdmiral),
            13 => Ok(FederationRank::ViceAdmiral),
            14 => Ok(FederationRank::Admiral),

            #[cfg(feature = "allow-unknown")]
            _ => Ok(FederationRank::Unknown(value)),

            #[cfg(not(feature = "allow-unknown"))]
            _ => Err(FederationRankError::UnknownFederationRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => FederationRank);

impl Display for FederationRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FederationRank::None => "None",
                FederationRank::Recruit => "Recruit",
                FederationRank::Cadet => "Cadet",
                FederationRank::Midshipman => "Midshipman",
                FederationRank::PettyOfficer => "Petty Officer",
                FederationRank::ChiefPettyOfficer => "Chief Petty Officer",
                FederationRank::WarrantOfficer => "WarrantOfficer",
                FederationRank::Ensign => "Ensign",
                FederationRank::Lieutenant => "Lieutenant",
                FederationRank::LieutenantCommander => "Lieutenant Commander",
                FederationRank::PostCommander => "Post Commander",
                FederationRank::PostCaptain => "Post Captain",
                FederationRank::RearAdmiral => "Rear Admiral",
                FederationRank::ViceAdmiral => "Vice Admiral",
                FederationRank::Admiral => "Admiral",

                #[cfg(feature = "allow-unknown")]
                FederationRank::Unknown(unknown) =>
                    return write!(f, "Unknown federation rank nr: {}", unknown),
            }
        )
    }
}

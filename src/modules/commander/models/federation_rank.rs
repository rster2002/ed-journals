use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;

use crate::{deserialize_in_order_impl, try_from_deserialize_impl};

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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum FederationRankError {
    #[error("Unknown federation rank with id '{0}'")]
    UnknownId(u8),

    #[error("Unknown federation rank string '{0}'")]
    UnknownString(String),
}

impl TryFrom<u8> for FederationRank {
    type Error = FederationRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => FederationRank::None,
            1 => FederationRank::Recruit,
            2 => FederationRank::Cadet,
            3 => FederationRank::Midshipman,
            4 => FederationRank::PettyOfficer,
            5 => FederationRank::ChiefPettyOfficer,
            6 => FederationRank::WarrantOfficer,
            7 => FederationRank::Ensign,
            8 => FederationRank::Lieutenant,
            9 => FederationRank::LieutenantCommander,
            10 => FederationRank::PostCommander,
            11 => FederationRank::PostCaptain,
            12 => FederationRank::RearAdmiral,
            13 => FederationRank::ViceAdmiral,
            14 => FederationRank::Admiral,

            #[cfg(feature = "allow-unknown")]
            _ => FederationRank::Unknown(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(FederationRankError::UnknownId(value)),
        })
    }
}

impl FromStr for FederationRank {
    type Err = FederationRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "None" => FederationRank::None,
            "Recruit" => FederationRank::Recruit,
            "Cadet" => FederationRank::Cadet,
            "Midshipman" => FederationRank::Midshipman,
            "PettyOfficer" => FederationRank::PettyOfficer,
            "ChiefPettyOfficer" => FederationRank::ChiefPettyOfficer,
            "WarrantOfficer" => FederationRank::WarrantOfficer,
            "Ensign" => FederationRank::Ensign,
            "Lieutenant" => FederationRank::Lieutenant,
            "LieutenantCommander" => FederationRank::LieutenantCommander,
            "PostCommander" => FederationRank::PostCommander,
            "PostCaptain" => FederationRank::PostCaptain,
            "RearAdmiral" => FederationRank::RearAdmiral,
            "ViceAdmiral" => FederationRank::ViceAdmiral,
            "Admiral" => FederationRank::Admiral,

            #[cfg(feature = "allow-unknown")]
            _ => FederationRank::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(FederationRankError::UnknownString(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    FederationRank =>
        A ? u8,
        B # String,
);

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

                #[cfg(feature = "allow-unknown")]
                FederationRank::UnknownString(unknown) =>
                    return write!(f, "Unknown federation rank string: '{}'", unknown),
            }
        )
    }
}

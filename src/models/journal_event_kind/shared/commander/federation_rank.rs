use serde::Deserialize;
use thiserror::Error;
use crate::models::journal_event_kind::shared::commander::exploration_rank::ExplorationRank;
use crate::try_from_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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

    #[cfg(not(feature = "strict"))]
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

            #[cfg(not(feature = "strict"))]
            _ => Ok(FederationRank::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(FederationRankError::UnknownFederationRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => FederationRank);

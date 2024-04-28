use serde::Deserialize;
use thiserror::Error;
use crate::models::journal_event_kind::shared::commander::combat_rank::{CombatRank, CombatRankError};
use crate::try_from_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum EmpireRank {
    None,
    Outsider,
    Serf,
    Master,
    Squire,
    Knight,
    Lord,
    Baron,
    Viscount,
    Count,
    Earl,
    Marquis,
    Duke,
    Prince,
    King,

    #[cfg(not(feature = "strict"))]
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum EmpireRankError {
    #[error("Unknown empire rank with id '{0}'")]
    UnknownEmpireRank(u8),
}

impl TryFrom<u8> for EmpireRank  {
    type Error = EmpireRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EmpireRank::None),
            1 => Ok(EmpireRank::Outsider),
            2 => Ok(EmpireRank::Serf),
            3 => Ok(EmpireRank::Master),
            4 => Ok(EmpireRank::Squire),
            5 => Ok(EmpireRank::Knight),
            6 => Ok(EmpireRank::Lord),
            7 => Ok(EmpireRank::Baron),
            8 => Ok(EmpireRank::Viscount),
            9 => Ok(EmpireRank::Count),
            10 => Ok(EmpireRank::Earl),
            11 => Ok(EmpireRank::Marquis),
            12 => Ok(EmpireRank::Duke),
            13 => Ok(EmpireRank::Prince),
            14 => Ok(EmpireRank::King),

            #[cfg(not(feature = "strict"))]
            _ => Ok(EmpireRank::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(EmpireRankError::UnknownEmpireRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => EmpireRank);

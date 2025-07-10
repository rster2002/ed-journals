use std::fmt::{Display, Formatter};

use serde::Serialize;
use thiserror::Error;

use crate::try_from_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum EmpireRankError {
    #[error("Unknown empire rank with id '{0}'")]
    UnknownEmpireRank(u8),
}

impl TryFrom<u8> for EmpireRank {
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

            #[cfg(feature = "allow-unknown")]
            _ => Ok(EmpireRank::Unknown(value)),

            #[cfg(not(feature = "allow-unknown"))]
            _ => Err(EmpireRankError::UnknownEmpireRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => EmpireRank);

impl Display for EmpireRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EmpireRank::None => "None",
                EmpireRank::Outsider => "Outsider",
                EmpireRank::Serf => "Serf",
                EmpireRank::Master => "Master",
                EmpireRank::Squire => "Squire",
                EmpireRank::Knight => "Knight",
                EmpireRank::Lord => "Lord",
                EmpireRank::Baron => "Baron",
                EmpireRank::Viscount => "Viscount",
                EmpireRank::Count => "Count",
                EmpireRank::Earl => "Earl",
                EmpireRank::Marquis => "Marquis",
                EmpireRank::Duke => "Duke",
                EmpireRank::Prince => "Prince",
                EmpireRank::King => "King",

                #[cfg(feature = "allow-unknown")]
                EmpireRank::Unknown(unknown) =>
                    return write!(f, "Unknown empire rank nr: {unknown}"),
            }
        )
    }
}

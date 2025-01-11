use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;

use crate::{deserialize_in_order_impl, try_from_deserialize_impl};

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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum EmpireRankError {
    #[error("Unknown empire rank with id '{0}'")]
    UnknownId(u8),

    #[error("Unknown empire rank string: '{0}'")]
    UnknownString(String),
}

impl TryFrom<u8> for EmpireRank {
    type Error = EmpireRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => EmpireRank::None,
            1 => EmpireRank::Outsider,
            2 => EmpireRank::Serf,
            3 => EmpireRank::Master,
            4 => EmpireRank::Squire,
            5 => EmpireRank::Knight,
            6 => EmpireRank::Lord,
            7 => EmpireRank::Baron,
            8 => EmpireRank::Viscount,
            9 => EmpireRank::Count,
            10 => EmpireRank::Earl,
            11 => EmpireRank::Marquis,
            12 => EmpireRank::Duke,
            13 => EmpireRank::Prince,
            14 => EmpireRank::King,

            #[cfg(feature = "allow-unknown")]
            _ => EmpireRank::Unknown(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(EmpireRankError::UnknownId(value)),
        })
    }
}

impl FromStr for EmpireRank {
    type Err = EmpireRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "None" => EmpireRank::None,
            "Outsider" => EmpireRank::Outsider,
            "Serf" => EmpireRank::Serf,
            "Master" => EmpireRank::Master,
            "Squire" => EmpireRank::Squire,
            "Knight" => EmpireRank::Knight,
            "Lord" => EmpireRank::Lord,
            "Baron" => EmpireRank::Baron,
            "Viscount" => EmpireRank::Viscount,
            "Count" => EmpireRank::Count,
            "Earl" => EmpireRank::Earl,
            "Marquis" => EmpireRank::Marquis,
            "Duke" => EmpireRank::Duke,
            "Prince" => EmpireRank::Prince,
            "King" => EmpireRank::King,

            #[cfg(feature = "allow-unknown")]
            _ => EmpireRank::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(EmpireRankError::UnknownString(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    EmpireRank =>
        A ? u8,
        B # String,
);

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
                    return write!(f, "Unknown empire rank nr: {}", unknown),

                #[cfg(feature = "allow-unknown")]
                EmpireRank::UnknownString(unknown) =>
                    return write!(f, "Unknown empire rank string: '{}'", unknown),
            }
        )
    }
}

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::deserialize_in_order_impl;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum CombatRank {
    Harmless,
    MostlyHarmless,
    Novice,
    Competent,
    Expert,
    Master,
    Dangerous,
    Deadly,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownU8(u8),

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum CombatRankError {
    #[error("Unknown combat rank with id '{0}'")]
    UnknownId(u8),

    #[error("Unknown combat rank string: '{0}'")]
    UnknownString(String),

    // #[error("Incorrect type used for combat rank")]
    // IncorrectType,
}

impl TryFrom<u8> for CombatRank {
    type Error = CombatRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => CombatRank::Harmless,
            1 => CombatRank::MostlyHarmless,
            2 => CombatRank::Novice,
            3 => CombatRank::Competent,
            4 => CombatRank::Expert,
            5 => CombatRank::Master,
            6 => CombatRank::Dangerous,
            7 => CombatRank::Deadly,
            8 => CombatRank::Elite,
            9 => CombatRank::EliteI,
            10 => CombatRank::EliteII,
            11 => CombatRank::EliteIII,
            12 => CombatRank::EliteIV,
            13 => CombatRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => CombatRank::UnknownU8(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CombatRankError::UnknownId(value)),
        })
    }
}

impl FromStr for CombatRank {
    type Err = CombatRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Harmless" => CombatRank::Harmless,
            "MostlyHarmless" => CombatRank::MostlyHarmless,
            "Novice" => CombatRank::Novice,
            "Competent" => CombatRank::Competent,
            "Expert" => CombatRank::Expert,
            "Master" => CombatRank::Master,
            "Dangerous" => CombatRank::Dangerous,
            "Deadly" => CombatRank::Deadly,
            "Elite" => CombatRank::Elite,
            "EliteI" => CombatRank::EliteI,
            "EliteII" => CombatRank::EliteII,
            "EliteIII" => CombatRank::EliteIII,
            "EliteIV" => CombatRank::EliteIV,
            "EliteV" => CombatRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => CombatRank::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CombatRankError::UnknownString(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    CombatRank =>
        A ? u8,
        B # String,
);

impl Display for CombatRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CombatRank::Harmless => "Harmless",
                CombatRank::MostlyHarmless => "Mostly Harmless",
                CombatRank::Novice => "Novice",
                CombatRank::Competent => "Competent",
                CombatRank::Expert => "Expert",
                CombatRank::Master => "Master",
                CombatRank::Dangerous => "Dangerous",
                CombatRank::Deadly => "Deadly",
                CombatRank::Elite => "Elite",
                CombatRank::EliteI => "Elite I",
                CombatRank::EliteII => "Elite II",
                CombatRank::EliteIII => "Elite III",
                CombatRank::EliteIV => "Elite IV",
                CombatRank::EliteV => "Elite V",

                #[cfg(feature = "allow-unknown")]
                CombatRank::UnknownU8(unknown) =>
                    return write!(f, "Unknown combat rank nr: {}", unknown),

                #[cfg(feature = "allow-unknown")]
                CombatRank::UnknownString(unknown) =>
                    return write!(f, "Unknown combat rank: {}", unknown),
            }
        )
    }
}

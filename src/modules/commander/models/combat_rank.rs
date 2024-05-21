use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
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

    #[cfg(not(feature = "strict"))]
    UnknownU8(u8),

    #[cfg(not(feature = "strict"))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum CombatRankError {
    #[error("Unknown combat rank with id '{0}'")]
    UnknownCombatRank(u8),

    #[error("Unknown combat rank string: '{0}'")]
    UnknownCombatString(String),

    #[error("Incorrect type used for combat rank")]
    IncorrectType,
}

impl TryFrom<u8> for CombatRank {
    type Error = CombatRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CombatRank::Harmless),
            1 => Ok(CombatRank::MostlyHarmless),
            2 => Ok(CombatRank::Novice),
            3 => Ok(CombatRank::Competent),
            4 => Ok(CombatRank::Expert),
            5 => Ok(CombatRank::Master),
            6 => Ok(CombatRank::Dangerous),
            7 => Ok(CombatRank::Deadly),
            8 => Ok(CombatRank::Elite),
            9 => Ok(CombatRank::EliteI),
            10 => Ok(CombatRank::EliteII),
            11 => Ok(CombatRank::EliteIII),
            12 => Ok(CombatRank::EliteIV),
            13 => Ok(CombatRank::EliteV),

            #[cfg(not(feature = "strict"))]
            _ => Ok(CombatRank::UnknownU8(value)),

            #[cfg(feature = "strict")]
            _ => Err(CombatRankError::UnknownCombatRank(value)),
        }
    }
}

impl FromStr for CombatRank {
    type Err = CombatRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Harmless" => Ok(CombatRank::Harmless),
            "MostlyHarmless" => Ok(CombatRank::MostlyHarmless),
            "Novice" => Ok(CombatRank::Novice),
            "Competent" => Ok(CombatRank::Competent),
            "Expert" => Ok(CombatRank::Expert),
            "Master" => Ok(CombatRank::Master),
            "Dangerous" => Ok(CombatRank::Dangerous),
            "Deadly" => Ok(CombatRank::Deadly),
            "Elite" => Ok(CombatRank::Elite),
            "EliteI" => Ok(CombatRank::EliteI),
            "EliteII" => Ok(CombatRank::EliteII),
            "EliteIII" => Ok(CombatRank::EliteIII),
            "EliteIV" => Ok(CombatRank::EliteIV),
            "EliteV" => Ok(CombatRank::EliteV),

            #[cfg(not(feature = "strict"))]
            _ => Ok(CombatRank::UnknownString(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(CombatRankError::UnknownCombatString(s.to_string())),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CombatInput {
    U8(u8),
    String(String),
}

impl<'de> serde::Deserialize<'de> for CombatRank {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = CombatInput::deserialize(deserializer)?;

        match input {
            CombatInput::U8(value) => Ok(CombatRank::try_from(value).map_err(|_| {
                serde::de::Error::custom(format!("Failed to deserialize u8: got '{}'", value))
            })?),
            CombatInput::String(value) => Ok(CombatRank::from_str(&value).map_err(|_| {
                serde::de::Error::custom(format!("Failed to deserialize string: got '{}'", value))
            })?),
        }
    }
}

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

                #[cfg(not(feature = "strict"))]
                CombatRank::UnknownU8(unknown) =>
                    return write!(f, "Unknown combat rank nr: {}", unknown),

                #[cfg(not(feature = "strict"))]
                CombatRank::UnknownString(unknown) =>
                    return write!(f, "Unknown combat rank: {}", unknown),
            }
        )
    }
}

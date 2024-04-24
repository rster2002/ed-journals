use serde::{Deserialize, Deserializer};
use thiserror::Error;
use crate::try_from_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum CombatRankError {
    #[error("Unknown combat rank with id '{0}'")]
    UnknownCombatRank(u8),
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
            _ => Ok(CombatRank::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(CombatRankError::UnknownCombatRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => CombatRank);

// impl<'de> Deserialize<'de> for CombatRank {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
//         let value = u8::deserialize(deserializer)?;
//
//         CombatRank::try_from(value)
//             .map_err(|e| serde::de::Error::custom(format!("Failed to deserialize: got '{}'", value)))
//     }
// }

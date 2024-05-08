use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::try_from_deserialize_impl;

#[derive(Debug, Clone, PartialEq)]
pub enum MercenaryRank {
    Defenceless,
    MostlyDefenceless,
    Rookie,
    Soldier,
    Gunslinger,
    Warrior,
    Gladiator,
    Deadeye,
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
pub enum MercenaryRankError {
    #[error("Unknown mercenary rank with id '{0}'")]
    UnknownMercenaryRank(u8),
}

impl TryFrom<u8> for MercenaryRank {
    type Error = MercenaryRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MercenaryRank::Defenceless),
            1 => Ok(MercenaryRank::MostlyDefenceless),
            2 => Ok(MercenaryRank::Rookie),
            3 => Ok(MercenaryRank::Soldier),
            4 => Ok(MercenaryRank::Gunslinger),
            5 => Ok(MercenaryRank::Warrior),
            6 => Ok(MercenaryRank::Gladiator),
            7 => Ok(MercenaryRank::Deadeye),
            8 => Ok(MercenaryRank::Elite),
            9 => Ok(MercenaryRank::EliteI),
            10 => Ok(MercenaryRank::EliteII),
            11 => Ok(MercenaryRank::EliteIII),
            12 => Ok(MercenaryRank::EliteIV),
            13 => Ok(MercenaryRank::EliteV),

            #[cfg(not(feature = "strict"))]
            _ => Ok(MercenaryRank::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(MercenaryRankError::UnknownMercenaryRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => MercenaryRank);

impl Display for MercenaryRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MercenaryRank::Defenceless => "Defenceless",
            MercenaryRank::MostlyDefenceless => "Mostly Defenceless",
            MercenaryRank::Rookie => "Rookie",
            MercenaryRank::Soldier => "Soldier",
            MercenaryRank::Gunslinger => "Gunslinger",
            MercenaryRank::Warrior => "Warrior",
            MercenaryRank::Gladiator => "Gladiator",
            MercenaryRank::Deadeye => "Deadeye",
            MercenaryRank::Elite => "Elite",
            MercenaryRank::EliteI => "Elite I",
            MercenaryRank::EliteII => "Elite II",
            MercenaryRank::EliteIII => "Elite III",
            MercenaryRank::EliteIV => "Elite IV",
            MercenaryRank::EliteV => "Elite V",

            #[cfg(not(feature = "strict"))]
            MercenaryRank::Unknown(unknown) => return write!(f, "Unknown mercenary rank nr: {}", unknown),
        })
    }
}

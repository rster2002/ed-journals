use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;

use crate::{deserialize_in_order_impl, try_from_deserialize_impl};

#[derive(Debug, Serialize, Clone, PartialEq)]
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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(u8),

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum MercenaryRankError {
    #[error("Unknown mercenary rank with id '{0}'")]
    UnknownId(u8),

    #[error("Unknown mercenary rank string '{0}'")]
    UnknownString(String),
}

impl TryFrom<u8> for MercenaryRank {
    type Error = MercenaryRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => MercenaryRank::Defenceless,
            1 => MercenaryRank::MostlyDefenceless,
            2 => MercenaryRank::Rookie,
            3 => MercenaryRank::Soldier,
            4 => MercenaryRank::Gunslinger,
            5 => MercenaryRank::Warrior,
            6 => MercenaryRank::Gladiator,
            7 => MercenaryRank::Deadeye,
            8 => MercenaryRank::Elite,
            9 => MercenaryRank::EliteI,
            10 => MercenaryRank::EliteII,
            11 => MercenaryRank::EliteIII,
            12 => MercenaryRank::EliteIV,
            13 => MercenaryRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => MercenaryRank::Unknown(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(MercenaryRankError::UnknownId(value)),
        })
    }
}

impl FromStr for MercenaryRank {
    type Err = MercenaryRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Defenceless" => MercenaryRank::Defenceless,
            "MostlyDefenceless" => MercenaryRank::MostlyDefenceless,
            "Rookie" => MercenaryRank::Rookie,
            "Soldier" => MercenaryRank::Soldier,
            "Gunslinger" => MercenaryRank::Gunslinger,
            "Warrior" => MercenaryRank::Warrior,
            "Gladiator" => MercenaryRank::Gladiator,
            "Deadeye" => MercenaryRank::Deadeye,
            "Elite" => MercenaryRank::Elite,
            "EliteI" => MercenaryRank::EliteI,
            "EliteII" => MercenaryRank::EliteII,
            "EliteIII" => MercenaryRank::EliteIII,
            "EliteIV" => MercenaryRank::EliteIV,
            "EliteV" => MercenaryRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => MercenaryRank::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(MercenaryRankError::UnknownString(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    MercenaryRank =>
        A ? u8,
        B # String,
);

impl Display for MercenaryRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
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

                #[cfg(feature = "allow-unknown")]
                MercenaryRank::Unknown(unknown) =>
                    return write!(f, "Unknown mercenary rank nr: {}", unknown),

                #[cfg(feature = "allow-unknown")]
                MercenaryRank::UnknownString(unknown) =>
                    return write!(f, "Unknown mercenary rank string: {}", unknown),
            }
        )
    }
}

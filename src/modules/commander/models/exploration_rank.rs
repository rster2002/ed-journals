use std::fmt::{Display, Formatter};

use serde::Serialize;
use thiserror::Error;

use crate::try_from_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum ExplorationRank {
    Aimless,
    MostlyAimless,
    Scout,
    Surveyor,
    Trailblazer,
    Pathfinder,
    Ranger,
    Pioneer,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum ExplorationRankError {
    #[error("Unknown exploration rank with id '{0}'")]
    UnknownExplorationRank(u8),
}

impl TryFrom<u8> for ExplorationRank {
    type Error = ExplorationRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExplorationRank::Aimless),
            1 => Ok(ExplorationRank::MostlyAimless),
            2 => Ok(ExplorationRank::Scout),
            3 => Ok(ExplorationRank::Surveyor),
            4 => Ok(ExplorationRank::Trailblazer),
            5 => Ok(ExplorationRank::Pathfinder),
            6 => Ok(ExplorationRank::Ranger),
            7 => Ok(ExplorationRank::Pioneer),
            8 => Ok(ExplorationRank::Elite),
            9 => Ok(ExplorationRank::EliteI),
            10 => Ok(ExplorationRank::EliteII),
            11 => Ok(ExplorationRank::EliteIII),
            12 => Ok(ExplorationRank::EliteIV),
            13 => Ok(ExplorationRank::EliteV),

            #[cfg(feature = "allow-unknown")]
            _ => Ok(ExplorationRank::Unknown(value)),

            #[cfg(not(feature = "allow-unknown"))]
            _ => Err(ExplorationRankError::UnknownExplorationRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => ExplorationRank);

impl Display for ExplorationRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExplorationRank::Aimless => "Aimless",
                ExplorationRank::MostlyAimless => "Mostly Aimless",
                ExplorationRank::Scout => "Scout",
                ExplorationRank::Surveyor => "Surveyor",
                ExplorationRank::Trailblazer => "Trailblazer",
                ExplorationRank::Pathfinder => "Pathfinder",
                ExplorationRank::Ranger => "Ranger",
                ExplorationRank::Pioneer => "Pioneer",
                ExplorationRank::Elite => "Elite",
                ExplorationRank::EliteI => "Elite I",
                ExplorationRank::EliteII => "Elite II",
                ExplorationRank::EliteIII => "Elite III",
                ExplorationRank::EliteIV => "Elite IV",
                ExplorationRank::EliteV => "Elite V",

                #[cfg(feature = "allow-unknown")]
                ExplorationRank::Unknown(unknown) =>
                    return write!(f, "Unknown exploration rank nr: {unknown}"),
            }
        )
    }
}

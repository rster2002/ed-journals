use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::commander::ExobiologyRank;
use crate::{deserialize_in_order_impl, try_from_deserialize_impl};

#[derive(Debug, Serialize, Clone, PartialEq)]
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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum ExplorationRankError {
    #[error("Unknown exploration rank with id '{0}'")]
    UnknownExplorationRank(u8),

    #[error("Unknown exploration rank string: '{0}'")]
    UnknownExplorationString(String),
}

impl TryFrom<u8> for ExplorationRank {
    type Error = ExplorationRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ExplorationRank::Aimless,
            1 => ExplorationRank::MostlyAimless,
            2 => ExplorationRank::Scout,
            3 => ExplorationRank::Surveyor,
            4 => ExplorationRank::Trailblazer,
            5 => ExplorationRank::Pathfinder,
            6 => ExplorationRank::Ranger,
            7 => ExplorationRank::Pioneer,
            8 => ExplorationRank::Elite,
            9 => ExplorationRank::EliteI,
            10 => ExplorationRank::EliteII,
            11 => ExplorationRank::EliteIII,
            12 => ExplorationRank::EliteIV,
            13 => ExplorationRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => ExplorationRank::Unknown(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(ExplorationRankError::UnknownExplorationRank(value)),
        })
    }
}

impl FromStr for ExplorationRank {
    type Err = ExplorationRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Aimless" => ExplorationRank::Aimless,
            "MostlyAimless" => ExplorationRank::MostlyAimless,
            "Scout" => ExplorationRank::Scout,
            "Surveyor" => ExplorationRank::Surveyor,
            "Trailblazer" => ExplorationRank::Trailblazer,
            "Pathfinder" => ExplorationRank::Pathfinder,
            "Ranger" => ExplorationRank::Ranger,
            "Pioneer" => ExplorationRank::Pioneer,
            "Elite" => ExplorationRank::Elite,
            "EliteI" => ExplorationRank::EliteI,
            "EliteII" => ExplorationRank::EliteII,
            "EliteIII" => ExplorationRank::EliteIII,
            "EliteIV" => ExplorationRank::EliteIV,
            "EliteV" => ExplorationRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => ExplorationRank::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(ExplorationRankError::UnknownExplorationString(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    ExplorationRank =>
        A ? u8,
        B # String,
);

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
                    return write!(f, "Unknown exploration rank nr: {}", unknown),

                #[cfg(feature = "allow-unknown")]
                ExplorationRank::UnknownString(string) =>
                    return write!(f, "Unknown exploration rank string '{}'", string),
            }
        )
    }
}

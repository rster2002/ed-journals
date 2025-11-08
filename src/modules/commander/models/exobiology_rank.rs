use std::fmt::{Display, Formatter};

use serde::Serialize;
use thiserror::Error;

use crate::try_from_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum ExobiologyRank {
    Directionless,
    MostlyDirectionless,
    Compiler,
    Collector,
    Cataloguer,
    Taxonomist,
    Ecologist,
    Geneticist,
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
pub enum ExobiologyRankError {
    #[error("Unknown exobiology rank with id '{0}'")]
    UnknownExobiologyRank(u8),
}

impl TryFrom<u8> for ExobiologyRank {
    type Error = ExobiologyRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExobiologyRank::Directionless),
            1 => Ok(ExobiologyRank::MostlyDirectionless),
            2 => Ok(ExobiologyRank::Compiler),
            3 => Ok(ExobiologyRank::Collector),
            4 => Ok(ExobiologyRank::Cataloguer),
            5 => Ok(ExobiologyRank::Taxonomist),
            6 => Ok(ExobiologyRank::Ecologist),
            7 => Ok(ExobiologyRank::Geneticist),
            8 => Ok(ExobiologyRank::Elite),
            9 => Ok(ExobiologyRank::EliteI),
            10 => Ok(ExobiologyRank::EliteII),
            11 => Ok(ExobiologyRank::EliteIII),
            12 => Ok(ExobiologyRank::EliteIV),
            13 => Ok(ExobiologyRank::EliteV),

            #[cfg(feature = "allow-unknown")]
            _ => Ok(ExobiologyRank::Unknown(value)),

            #[cfg(not(feature = "allow-unknown"))]
            _ => Err(ExobiologyRankError::UnknownExobiologyRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => ExobiologyRank);

impl Display for ExobiologyRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExobiologyRank::Directionless => "Directionless",
                ExobiologyRank::MostlyDirectionless => "Mostly Directionless",
                ExobiologyRank::Compiler => "Compiler",
                ExobiologyRank::Collector => "Collector",
                ExobiologyRank::Cataloguer => "Cataloguer",
                ExobiologyRank::Taxonomist => "Taxonomist",
                ExobiologyRank::Ecologist => "Ecologist",
                ExobiologyRank::Geneticist => "Geneticist",
                ExobiologyRank::Elite => "Elite",
                ExobiologyRank::EliteI => "Elite I",
                ExobiologyRank::EliteII => "Elite II",
                ExobiologyRank::EliteIII => "Elite III",
                ExobiologyRank::EliteIV => "Elite IV",
                ExobiologyRank::EliteV => "Elite V",

                #[cfg(feature = "allow-unknown")]
                ExobiologyRank::Unknown(unknown) =>
                    return write!(f, "Unknown exobiology rank nr: {unknown}"),
            }
        )
    }
}

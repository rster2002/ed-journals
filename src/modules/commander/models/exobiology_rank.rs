use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::commander::{CombatRank, CombatRankError};
use crate::{deserialize_in_order_impl, try_from_deserialize_impl};

#[derive(Debug, Serialize, Clone, PartialEq)]
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
    UnknownU8(u8),

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum ExobiologyRankError {
    #[error("Unknown exobiology rank with id '{0}'")]
    UnknownId(u8),

    #[error("Unknown exobiology rank string: '{0}'")]
    UnknownString(String),
}

impl TryFrom<u8> for ExobiologyRank {
    type Error = ExobiologyRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ExobiologyRank::Directionless,
            1 => ExobiologyRank::MostlyDirectionless,
            2 => ExobiologyRank::Compiler,
            3 => ExobiologyRank::Collector,
            4 => ExobiologyRank::Cataloguer,
            5 => ExobiologyRank::Taxonomist,
            6 => ExobiologyRank::Ecologist,
            7 => ExobiologyRank::Geneticist,
            8 => ExobiologyRank::Elite,
            9 => ExobiologyRank::EliteI,
            10 => ExobiologyRank::EliteII,
            11 => ExobiologyRank::EliteIII,
            12 => ExobiologyRank::EliteIV,
            13 => ExobiologyRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => ExobiologyRank::UnknownU8(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(ExobiologyRankError::UnknownId(value)),
        })
    }
}

impl FromStr for ExobiologyRank {
    type Err = ExobiologyRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Directionless" => ExobiologyRank::Directionless,
            "MostlyDirectionless" => ExobiologyRank::MostlyDirectionless,
            "Compiler" => ExobiologyRank::Compiler,
            "Collector" => ExobiologyRank::Collector,
            "Cataloguer" => ExobiologyRank::Cataloguer,
            "Taxonomist" => ExobiologyRank::Taxonomist,
            "Ecologist" => ExobiologyRank::Ecologist,
            "Geneticist" => ExobiologyRank::Geneticist,
            "Elite" => ExobiologyRank::Elite,
            "EliteI" => ExobiologyRank::EliteI,
            "EliteII" => ExobiologyRank::EliteII,
            "EliteIII" => ExobiologyRank::EliteIII,
            "EliteIV" => ExobiologyRank::EliteIV,
            "EliteV" => ExobiologyRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => ExobiologyRank::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(ExobiologyRankError::UnknownString(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    ExobiologyRank =>
        A ? u8,
        B # String,
);

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
                ExobiologyRank::UnknownU8(unknown) =>
                    return write!(f, "Unknown exobiology rank nr: {}", unknown),

                #[cfg(feature = "allow-unknown")]
                ExobiologyRank::UnknownString(unknown) =>
                    return write!(f, "Unknown exobiology rank string: '{}'", unknown),
            }
        )
    }
}

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;

use crate::{deserialize_in_order_impl, try_from_deserialize_impl};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum TradeRank {
    Penniless,
    MostlyPenniless,
    Peddler,
    Dealer,
    Merchant,
    Broker,
    Entrepreneur,
    Tycoon,
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
pub enum TradeRankError {
    #[error("Unknown trade rank with id '{0}'")]
    UnknownId(u8),

    #[error("Unknown trade rank string '{0}'")]
    UnknownString(String),
}

impl TryFrom<u8> for TradeRank {
    type Error = TradeRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => TradeRank::Penniless,
            1 => TradeRank::MostlyPenniless,
            2 => TradeRank::Peddler,
            3 => TradeRank::Dealer,
            4 => TradeRank::Merchant,
            5 => TradeRank::Broker,
            6 => TradeRank::Entrepreneur,
            7 => TradeRank::Tycoon,
            8 => TradeRank::Elite,
            9 => TradeRank::EliteI,
            10 => TradeRank::EliteII,
            11 => TradeRank::EliteIII,
            12 => TradeRank::EliteIV,
            13 => TradeRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => TradeRank::Unknown(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(TradeRankError::UnknownId(value)),
        })
    }
}

impl FromStr for TradeRank {
    type Err = TradeRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Penniless" => TradeRank::Penniless,
            "MostlyPenniless" => TradeRank::MostlyPenniless,
            "Peddler" => TradeRank::Peddler,
            "Dealer" => TradeRank::Dealer,
            "Merchant" => TradeRank::Merchant,
            "Broker" => TradeRank::Broker,
            "Entrepreneur" => TradeRank::Entrepreneur,
            "Tycoon" => TradeRank::Tycoon,
            "Elite" => TradeRank::Elite,
            "EliteI" => TradeRank::EliteI,
            "EliteII" => TradeRank::EliteII,
            "EliteIII" => TradeRank::EliteIII,
            "EliteIV" => TradeRank::EliteIV,
            "EliteV" => TradeRank::EliteV,

            #[cfg(feature = "allow-unknown")]
            _ => TradeRank::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(TradeRankError::UnknownString(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    TradeRank =>
        A ? u8,
        B # String,
);

impl Display for TradeRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TradeRank::Penniless => "Penniless",
                TradeRank::MostlyPenniless => "Mostly Penniless",
                TradeRank::Peddler => "Peddler",
                TradeRank::Dealer => "Dealer",
                TradeRank::Merchant => "Merchant",
                TradeRank::Broker => "Broker",
                TradeRank::Entrepreneur => "Entrepreneur",
                TradeRank::Tycoon => "Tycoon",
                TradeRank::Elite => "Elite",
                TradeRank::EliteI => "Elite I",
                TradeRank::EliteII => "Elite II",
                TradeRank::EliteIII => "Elite III",
                TradeRank::EliteIV => "Elite IV",
                TradeRank::EliteV => "Elite V",

                #[cfg(feature = "allow-unknown")]
                TradeRank::Unknown(unknown) =>
                    return write!(f, "Unknown trade rank nr: {}", unknown),

                #[cfg(feature = "allow-unknown")]
                TradeRank::UnknownString(unknown) =>
                    return write!(f, "Unknown trade rank string: '{}'", unknown),
            }
        )
    }
}

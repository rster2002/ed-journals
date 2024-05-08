use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::try_from_deserialize_impl;

#[derive(Debug, Clone, PartialEq)]
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

    #[cfg(not(feature = "strict"))]
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum TradeRankError {
    #[error("Unknown trade rank with id '{0}'")]
    UnknownTradeRank(u8),
}

impl TryFrom<u8> for TradeRank {
    type Error = TradeRankError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TradeRank::Penniless),
            1 => Ok(TradeRank::MostlyPenniless),
            2 => Ok(TradeRank::Peddler),
            3 => Ok(TradeRank::Dealer),
            4 => Ok(TradeRank::Merchant),
            5 => Ok(TradeRank::Broker),
            6 => Ok(TradeRank::Entrepreneur),
            7 => Ok(TradeRank::Tycoon),
            8 => Ok(TradeRank::Elite),
            9 => Ok(TradeRank::EliteI),
            10 => Ok(TradeRank::EliteII),
            11 => Ok(TradeRank::EliteIII),
            12 => Ok(TradeRank::EliteIV),
            13 => Ok(TradeRank::EliteV),

            #[cfg(not(feature = "strict"))]
            _ => Ok(TradeRank::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(TradeRankError::UnknownTradeRank(value)),
        }
    }
}

try_from_deserialize_impl!(u8 => TradeRank);

impl Display for TradeRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
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

            #[cfg(not(feature = "strict"))]
            TradeRank::Unknown(unknown) => return write!(f, "Unknown trade rank nr: {}", unknown),
        })
    }
}

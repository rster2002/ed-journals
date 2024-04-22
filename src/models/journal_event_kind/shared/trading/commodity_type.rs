use std::str::FromStr;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CommodityType {
    Wine,

    KamitraCigars,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum CommodityTypeParseError {
    #[error("Unknown commodity: '{0}'")]
    UnknownCommodity(String),
}

impl FromStr for CommodityType {
    type Err = CommodityTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wine" => Ok(CommodityType::Wine),
            "kamitracigars" => Ok(CommodityType::KamitraCigars),

            #[cfg(not(feature = "strict"))]
            _ => Ok(CommodityType::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(CommodityTypeParseError::UnknownCommodity(s.to_string())),
        }
    }
}

from_str_deserialize_impl!(CommodityType);


use std::str::FromStr;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Economy {
    #[serde(rename = "$economy_Colony;")]
    Colony,

    #[serde(rename = "$economy_Industrial;")]
    Industrial,

    #[serde(rename = "$economy_Refinery;")]
    Refinery,

    #[serde(rename = "$economy_Extraction;")]
    Extraction,

    #[serde(rename = "$economy_Military;")]
    Military,

    #[serde(rename = "$economy_None;")]
    None,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

// #[derive(Debug, Error)]
// pub enum SystemEconomyParseError {
//     #[error("Unknown system economy: '{0}'")]
//     UnknownSystemEconomy(String),
// }
//
// impl FromStr for Economy {
//     type Err = SystemEconomyParseError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "$economy_Industrial;" => Ok(Economy::Industrial),
//
//             #[cfg(not(feature = "strict"))]
//             _ => Ok(Economy::Unknown(s.to_string())),
//
//             #[cfg(feature = "strict")]
//             _ => Err(SystemEconomyParseError::UnknownSystemEconomy(s.to_string())),
//         }
//     }
// }

use std::str::FromStr;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Superpower {
    Federation,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

// #[derive(Debug, Error)]
// pub enum SuperpowerParseError {
//     #[error("Unknown superpower: '{0}'")]
//     UnknownSuperpower(String),
// }
//
// impl FromStr for Superpower {
//     type Err = SuperpowerParseError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Federation" =>
//
//             #[cfg(not(feature = "strict"))]
//             _ => Ok(Superpower::Unknown(s.to_string())),
//
//             #[cfg(feature = "strict")]
//             _ => Err(SuperpowerParseError::UnknownSuperpower(s.to_string())),
//         }
//     }
// }

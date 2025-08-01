use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum SuitType {
    FlightSuit,
    ArtemisSuit,
    DominatorSuit,
    MaverickSuit,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum SuitTypeError {
    #[error("Unknown suit type: '{0}'")]
    UnknownSuitType(String),
}

impl FromStr for SuitType {
    type Err = SuitTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: &str = &s.to_ascii_lowercase();

        Ok(match string {
            "flightsuit" => SuitType::FlightSuit,
            "explorationsuit" => SuitType::ArtemisSuit,
            "tacticalsuit" => SuitType::DominatorSuit,
            "utilitysuit" => SuitType::MaverickSuit,

            #[cfg(feature = "allow-unknown")]
            _ => SuitType::Unknown(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(SuitTypeError::UnknownSuitType(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(SuitType);

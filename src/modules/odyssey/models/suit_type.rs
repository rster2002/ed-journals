use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum SuitType {
    FlightSuit,
    ArtemisSuit,
    DominatorSuit,
    MaverickSuit,

    #[cfg(not(feature = "strict"))]
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

            #[cfg(not(feature = "strict"))]
            _ => SuitType::Unknown(s.to_string()),

            #[cfg(feature = "strict")]
            _ => return Err(SuitTypeError::UnknownSuitType(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(SuitType);

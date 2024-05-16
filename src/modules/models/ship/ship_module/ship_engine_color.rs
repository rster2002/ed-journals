use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipEngineColor {
    name: String,
}

#[derive(Debug, Error)]
pub enum ShipEngineColorError {
    #[error("Failed to parse engine color: '{0}'")]
    FailedToParse(String),
}

impl FromStr for ShipEngineColor {
    type Err = ShipEngineColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("enginecustomisation_") {
            return Err(ShipEngineColorError::FailedToParse(s.to_string()));
        }

        Ok(ShipEngineColor {
            name: s.to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipEngineColor);

use std::str::FromStr;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipPaintJob {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipPaintJobError {
    #[error("Failed to parse paint job: '{0}'")]
    FailedToParse(String),
}

impl FromStr for ShipPaintJob {
    type Err = ShipPaintJobError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("paintjob_") {
            return Err(ShipPaintJobError::FailedToParse(s.to_string()));
        }

        Ok(ShipPaintJob {
            name: s.to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipPaintJob);

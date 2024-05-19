use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use thiserror::Error;

use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipPaintJob {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipPaintJobError {
    #[error("Failed to parse paint job: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref PAINTJOB_REGEX: Regex = Regex::new(r#"^paintjob_(\w+)$"#).unwrap();
}

impl FromStr for ShipPaintJob {
    type Err = ShipPaintJobError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = PAINTJOB_REGEX.captures(s) else {
            return Err(ShipPaintJobError::FailedToParse(s.to_string()));
        };

        Ok(ShipPaintJob {
            name: captures.get(1)
                .expect("Should have been captured already")
                .as_str()
                .to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipPaintJob);

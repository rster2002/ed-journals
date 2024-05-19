use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipBobble {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipBobbleError {
    #[error("Failed to parse bobble: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref BOBBLE_REGEX: Regex = Regex::new(r#"^bobble_(\w+)$"#).unwrap();
}

impl FromStr for ShipBobble {
    type Err = ShipBobbleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = BOBBLE_REGEX.captures(s) else {
            return Err(ShipBobbleError::FailedToParse(s.to_string()));
        };

        Ok(ShipBobble {
            name: captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipBobble);

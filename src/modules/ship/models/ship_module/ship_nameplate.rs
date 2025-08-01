use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipNameplate {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipNameplateError {
    #[error("Failed to parse nameplate: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref NAMEPLATE_REGEX: Regex = Regex::new(r#"^nameplate_(\w+)$"#).unwrap();
}

impl FromStr for ShipNameplate {
    type Err = ShipNameplateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = NAMEPLATE_REGEX.captures(s) else {
            return Err(ShipNameplateError::FailedToParse(s.to_string()));
        };

        Ok(ShipNameplate {
            name: captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipNameplate);

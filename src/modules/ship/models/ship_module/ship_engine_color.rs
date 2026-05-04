use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipEngineColor {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipEngineColorError {
    #[error("Failed to parse engine color: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref ENGINE_COLOR_REGEX: Regex = Regex::new(r#"^enginecustomisation_(\w+)$"#).unwrap();
}

impl FromStr for ShipEngineColor {
    type Err = ShipEngineColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = ENGINE_COLOR_REGEX.captures(s) else {
            return Err(ShipEngineColorError::FailedToParse(s.to_string()));
        };

        Ok(ShipEngineColor {
            name: captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipEngineColor);

use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipStringLights {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipStringLightsError {
    #[error("Failed to parse string lights: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref STRING_LIGHTS_COLOR_REGEX: Regex = Regex::new(r#"^string_lights_(\w+)$"#).unwrap();
}

impl FromStr for ShipStringLights {
    type Err = ShipStringLightsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = STRING_LIGHTS_COLOR_REGEX.captures(s) else {
            return Err(ShipStringLightsError::FailedToParse(s.to_string()));
        };

        Ok(ShipStringLights {
            name: captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipStringLights);

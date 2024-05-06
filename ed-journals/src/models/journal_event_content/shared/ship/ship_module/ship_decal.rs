use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipDecal {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipDecalError {
    #[error("Failed to parse decal: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref DECAL_REGEX: Regex = Regex::new(r#"^decal_(\w+)$"#).unwrap();
}

impl FromStr for ShipDecal {
    type Err = ShipDecalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = DECAL_REGEX.captures(s) else {
            return Err(ShipDecalError::FailedToParse(s.to_string()));
        };

        let name = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        Ok(ShipDecal {
            name,
        })
    }
}

from_str_deserialize_impl!(ShipDecal);

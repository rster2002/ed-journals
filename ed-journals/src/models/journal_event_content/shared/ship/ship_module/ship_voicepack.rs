use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipVoicepack {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipVoicepackError {
    #[error("Failed to parse voicepack: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref VOICEPACK_REGEX: Regex = Regex::new(r#"^voicepack_(\w+)$"#).unwrap();
}

impl FromStr for ShipVoicepack {
    type Err = ShipVoicepackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = VOICEPACK_REGEX.captures(s) else {
            return Err(ShipVoicepackError::FailedToParse(s.to_string()));
        };

        let name = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        Ok(ShipVoicepack {
            name,
        })
    }
}

from_str_deserialize_impl!(ShipVoicepack);

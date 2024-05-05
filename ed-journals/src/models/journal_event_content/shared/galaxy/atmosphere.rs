use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::{Match, Regex};
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_content::shared::galaxy::atmosphere_type::AtmosphereType;

#[derive(Debug, Clone, PartialEq)]
pub struct Atmosphere {
    pub hot: bool,
    pub density: AtmosphereDensity,
    pub kind: AtmosphereType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AtmosphereDensity {
    Thick,
    Normal,
    Thin,
}

#[derive(Debug, Error)]
pub enum AtmosphereError {
    #[error("Unknown atmosphere type: {0}")]
    UnknownAtmosphereType(serde_json::Error),

    #[error("Failed to parse atmosphere: '{0}'")]
    FailedToParse(String),
}

const ATMOSPHERE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(hot )?((thin|thick) )?([a-zA-Z ]+)? atmosphere$"#).unwrap());

impl FromStr for Atmosphere {
    type Err = AtmosphereError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Ok(Atmosphere {
                hot: false,
                density: AtmosphereDensity::Normal,
                kind: AtmosphereType::None,
            });
        }

        let Some(captures) = ATMOSPHERE_REGEX.captures(s) else {
            return Err(AtmosphereError::FailedToParse(s.to_string()));
        };

        let hot = captures.get(1).is_some();

        let density = match captures.get(3) {
            Some(capture) => match capture.as_str() {
                "thin" => AtmosphereDensity::Thin,
                "thick" => AtmosphereDensity::Thick,
                _ => AtmosphereDensity::Normal,
            },
            None => AtmosphereDensity::Normal,
        };

        // Sometimes it shit like 'thick  atmosphere' appears as input, so this handles that...
        let Some(kind_capture) = captures.get(4) else {
            return Ok(Atmosphere {
                kind: AtmosphereType::AmmoniaOxygen,
                hot,
                density,
            });
        };

        let kind = kind_capture
            .as_str()
            .parse()
            .map_err(|e| AtmosphereError::UnknownAtmosphereType(e))?;

        Ok(Atmosphere {
            kind,
            hot,
            density,
        })
    }
}

from_str_deserialize_impl!(Atmosphere);

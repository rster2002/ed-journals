use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::ship_type::{ShipType, ShipTypeError};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ShipModule {
    Cockpit(ShipType),
    CargoBayDoor,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum ShipModuleParseError {
    #[error("Failed to parse cockpit module ship type: {0}")]
    ShipTypeError(#[from] ShipTypeError),

    #[error("Unknown ship module: {0}")]
    UnknownShipModule(String),
}

const COCKPIT_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$(.+)_cockpit_name;$"#).unwrap());

impl FromStr for ShipModule {
    type Err = ShipModuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = COCKPIT_MODULE_REGEX.captures(s) {
            let ship_type = captures.get(1)
                .expect("Should have already been matched")
                .as_str()
                .parse()?;

            return Ok(ShipModule::Cockpit(ship_type));
        }

        match s {
            "$modularcargobaydoor_name;" => Ok(ShipModule::CargoBayDoor),

            #[cfg(not(feature = "strict"))]
            _ => Ok(ShipModule::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(ShipModuleParseError::UnknownShipModule(s.to_string())),
        }
    }
}

from_str_deserialize_impl!(ShipModule);

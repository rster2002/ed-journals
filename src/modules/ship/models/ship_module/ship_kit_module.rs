use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::ship::{ShipType, ShipTypeError};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipKitModule {
    pub ship: ShipType,
    pub name: String,
    pub piece: String,
}

#[derive(Debug, Error)]
pub enum ShipKitModuleError {
    #[error("Failed to parse ship kit number: {0}")]
    FailedToParseShipKitNr(#[from] ParseIntError),

    #[error(transparent)]
    ShipTypeError(#[from] ShipTypeError),

    #[error("Failed to parse ship kit module: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref SHIP_KIT_MODULE_REGEX: Regex =
        Regex::new(r#"^([a-z0-9_]+?)_shipkit([a-z0-9]+)_([a-z0-9]+)$"#).unwrap();
}

impl FromStr for ShipKitModule {
    type Err = ShipKitModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = SHIP_KIT_MODULE_REGEX.captures(s) else {
            return Err(ShipKitModuleError::FailedToParse(s.to_string()));
        };

        let ship = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        let name = captures
            .get(2)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        let piece = captures
            .get(3)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        Ok(ShipKitModule { ship, name, piece })
    }
}

from_str_deserialize_impl!(ShipKitModule);

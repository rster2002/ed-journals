use std::num::ParseIntError;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::ship::ship_type::ShipType;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipKitModule {
    pub ship: ShipType,
    pub kit_nr: u8,
    pub name: String,
}

#[derive(Debug, Error)]
pub enum ShipKitModuleError {
    #[error("Failed to parse ship kit number: {0}")]
    FailedToParseShipKitNr(#[from] ParseIntError),

    #[error("Failed to parse skip type for ship kit: {0}")]
    FailedToParseShipType(#[from] serde_json::Error),

    #[error("Failed to parse ship kit module: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref SHIP_KIT_MODULE_REGEX: Regex = Regex::new(r#"^([a-z0-9_]+?)_shipkit(\d)(\w+)$"#).unwrap();
}

impl FromStr for ShipKitModule {
    type Err = ShipKitModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = SHIP_KIT_MODULE_REGEX.captures(s) else {
            return Err(ShipKitModuleError::FailedToParse(s.to_string()));
        };

        let ship = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        let kit_nr = captures.get(2)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        let name = captures.get(3)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        Ok(ShipKitModule {
            ship,
            kit_nr,
            name,
        })
    }
}

from_str_deserialize_impl!(ShipKitModule);

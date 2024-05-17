use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipWeaponColor {
    name: String,
}

#[derive(Debug, Error)]
pub enum ShipWeaponColorError {
    #[error("Failed to parse weapon color: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref WEAPON_COLOR_REGEX: Regex = Regex::new(r#"^weaponcustomisation_(\w+)$"#).unwrap();
}

impl FromStr for ShipWeaponColor {
    type Err = ShipWeaponColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = WEAPON_COLOR_REGEX.captures(s) else {
            return Err(ShipWeaponColorError::FailedToParse(s.to_string()));
        };

        Ok(ShipWeaponColor {
            name: captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .to_string(),
        })
    }
}

from_str_deserialize_impl!(ShipWeaponColor);

use crate::from_str_deserialize_impl;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

use crate::models::journal_event_content::shared::ship::ship_type::ShipType;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipCockpitModule(pub ShipType);

#[derive(Debug, Error)]
pub enum CockpitModuleParseError {
    #[error("Failed to parse cockpit module")]
    FailedRegex,

    #[error("Unknown ship type")]
    UnknownShipType,
}

const COCKPIT_MODULE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^\$(.+)_cockpit_name;$"#).unwrap());

impl FromStr for ShipCockpitModule {
    type Err = CockpitModuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = COCKPIT_MODULE_REGEX.captures(s) else {
            return Err(CockpitModuleParseError::FailedRegex);
        };

        let ship_type = captures
            .get(1)
            .expect("Should have already been matched")
            .as_str()
            .parse()
            .map_err(|_| CockpitModuleParseError::UnknownShipType)?;

        Ok(ShipCockpitModule(ship_type))
    }
}

from_str_deserialize_impl!(ShipCockpitModule);

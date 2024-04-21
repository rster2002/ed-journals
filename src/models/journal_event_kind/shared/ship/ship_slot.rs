use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShipSlot {
    pub slot_nr: u8,
    pub slot_size: u8,
}

#[derive(Debug, Error)]
pub enum ParseShipSlotError {
    #[error("Ship slot string was not formatted correctly")]
    IncorrectFormat,

    #[error("Failed to parse the slot nr")]
    FailedToParseNr,

    #[error("Failed to parse the size of the slot")]
    FailedToParseSize,
}

const SLOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Slot02_Size5"#).unwrap());

impl FromStr for ShipSlot {
    type Err = ParseShipSlotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = SLOT_REGEX.captures(s)
            .ok_or(ParseShipSlotError::IncorrectFormat)?;

        let slot_nr = captures.get(1)
            .expect("Regex should have already been matched")
            .as_str()
            .parse()
            .map_err(|_| ParseShipSlotError::FailedToParseNr)?;

        let slot_size = captures.get(2)
            .expect("Regex should have already been matched")
            .as_str()
            .parse()
            .map_err(|_| ParseShipSlotError::FailedToParseSize)?;

        Ok(ShipSlot {
            slot_nr,
            slot_size,
        })
    }
}

from_str_deserialize_impl!(ShipSlot);

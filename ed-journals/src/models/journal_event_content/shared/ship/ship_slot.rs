use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::models::journal_event_content::shared::ship::ship_slot::core_slot::CoreSlot;
use crate::models::journal_event_content::shared::ship::ship_slot::hardpoint_size::{
    HardpointSize, HardpointSizeParseError,
};

mod core_slot;
pub mod hardpoint_size;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipSlot {
    pub slot_nr: u8,
    pub kind: SkipSlotKind,
}

// TODO kinda want to refactor this to use untagged variants
#[derive(Debug, Clone, PartialEq)]
pub enum SkipSlotKind {
    UtilityMount,
    Hardpoint(HardpointSize),
    OptionalInternal(u8),
    Military,
    CoreInternal(CoreSlot),
}

#[derive(Debug, Error)]
pub enum ParseShipSlotError {
    #[error("Failed to parse slot number in: '{0}'")]
    FailedToParseSlotNr(String),

    #[error(transparent)]
    HardpointSizeParseError(#[from] HardpointSizeParseError),

    #[error("Failed to parse optional internal size: {0}")]
    OptionalInternalSizeParseError(#[source] ParseIntError),

    #[error("Failed to parse ship slot: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref UTILITY_HARDPOINT_REGEX: Regex = Regex::new(r#"^TinyHardpoint(\d+)$"#).unwrap();
    static ref HARDPOINT_REGEX: Regex =
        Regex::new(r#"^(Small|Medium|Large|Huge)Hardpoint(\d+)$"#).unwrap();
    static ref OPTIONAL_INTERNAL_REGEX: Regex = Regex::new(r#"^Slot(\d+)_Size(\d+)$"#).unwrap();
    static ref MILITARY_REGEX: Regex = Regex::new(r#"^Military(\d+)$"#).unwrap();
}

impl FromStr for ShipSlot {
    type Err = ParseShipSlotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = UTILITY_HARDPOINT_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ParseShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: SkipSlotKind::UtilityMount,
            });
        }

        if let Some(captures) = HARDPOINT_REGEX.captures(s) {
            let size = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()?;

            let slot_nr = captures
                .get(2)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ParseShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: SkipSlotKind::Hardpoint(size),
            });
        }

        if let Some(captures) = OPTIONAL_INTERNAL_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ParseShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            let size = captures
                .get(2)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(ParseShipSlotError::OptionalInternalSizeParseError)?;

            return Ok(ShipSlot {
                slot_nr,
                kind: SkipSlotKind::OptionalInternal(size),
            });
        }

        if let Some(captures) = MILITARY_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ParseShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: SkipSlotKind::Military,
            });
        }

        if let Ok(core_slot) = s.parse() {
            return Ok(ShipSlot {
                slot_nr: 1,
                kind: SkipSlotKind::CoreInternal(core_slot),
            });
        }

        Err(ParseShipSlotError::FailedToParse(s.to_string()))
    }
}

from_str_deserialize_impl!(ShipSlot);

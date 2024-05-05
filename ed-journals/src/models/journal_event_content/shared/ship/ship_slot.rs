mod core_slot;
pub mod hardpoint_size;

use crate::from_str_deserialize_impl;
use crate::models::journal_event_content::shared::ship::ship_slot::core_slot::CoreSlot;
use crate::models::journal_event_content::shared::ship::ship_slot::hardpoint_size::{
    HardpointSize, HardpointSizeParseError,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

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

const UTILITY_HARDPOINT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^TinyHardpoint(\d+)$"#).unwrap());
const HARDPOINT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^(Small|Medium|Large|Huge)Hardpoint(\d+)$"#).unwrap());
const OPTIONAL_INTERNAL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^Slot(\d+)_Size(\d+)$"#).unwrap());
const MILITARY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^Military(\d+)$"#).unwrap());

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
            let slot_nr = captures.get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ParseShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: SkipSlotKind::Military,
            })
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

// pub struct ShipSlot {
//     pub slot_nr: u8,
//     pub slot_size: u8,
// }
//

// pub enum ParseShipSlotError {
//     #[error("Ship slot string was not formatted correctly")]
//     IncorrectFormat,
//
//     #[error("Failed to parse the slot nr")]
//     FailedToParseNr,
//
//     #[error("Failed to parse the size of the slot")]
//     FailedToParseSize,
// }
//
// const SLOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Slot02_Size5"#).unwrap());
//
// impl FromStr for ShipSlot {
//     type Err = ParseShipSlotError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let captures = SLOT_REGEX.captures(s)
//             .ok_or(ParseShipSlotError::IncorrectFormat)?;
//
//         let slot_nr = captures.get(1)
//             .expect("Regex should have already been matched")
//             .as_str()
//             .parse()
//             .map_err(|_| ParseShipSlotError::FailedToParseNr)?;
//
//         let slot_size = captures.get(2)
//             .expect("Regex should have already been matched")
//             .as_str()
//             .parse()
//             .map_err(|_| ParseShipSlotError::FailedToParseSize)?;
//
//         Ok(ShipSlot {
//             slot_nr,
//             slot_size,
//         })
//     }
// }
//
// from_str_deserialize_impl!(ShipSlot);

use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::ship::models::ship_slot::core_slot::CoreSlot;
use crate::modules::ship::{HardpointSize, HardpointSizeError};

pub mod core_slot;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipSlot {
    pub slot_nr: u8,
    pub kind: ShipSlotKind,
}

// TODO kinda want to refactor this to use untagged variants
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ShipSlotKind {
    ShipCockpit,
    CargoHatch,
    UtilityMount,
    Hardpoint(HardpointSize),
    OptionalInternal(u8),
    Military,
    CoreInternal(CoreSlot),
    DataLinkScanner,
    CodexScanner,
    DiscoveryScanner,

    // Cosmetic
    PaintJob,
    Decal,
    VesselVoice,
    Nameplate,
    IDPlate,
    Bobble,
    StringLights,
    EngineColor,
    WeaponColor,
    ShipKitSpoiler,
    ShipKitWings,
    ShipKitTail,
    ShipKitBumper,
}

#[derive(Debug, Error)]
pub enum ShipSlotError {
    #[error("Failed to parse slot number in: '{0}'")]
    FailedToParseSlotNr(String),

    #[error(transparent)]
    HardpointSizeParseError(#[from] HardpointSizeError),

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
    static ref DECAL_REGEX: Regex = Regex::new(r#"^Decal(\d+)$"#).unwrap();
    static ref NAMEPLATE_REGEX: Regex = Regex::new(r#"^ShipName(\d+)$"#).unwrap();
    static ref ID_PLATE_REGEX: Regex = Regex::new(r#"^ShipID(\d+)$"#).unwrap();
    static ref BOBBLE_REGEX: Regex = Regex::new(r#"^Bobble(\d+)$"#).unwrap();
}

impl FromStr for ShipSlot {
    type Err = ShipSlotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let specific = match s {
            "ShipCockpit" => Some(ShipSlotKind::ShipCockpit),
            "CargoHatch" => Some(ShipSlotKind::CargoHatch),
            "PaintJob" => Some(ShipSlotKind::PaintJob),
            "VesselVoice" => Some(ShipSlotKind::VesselVoice),
            "DataLinkScanner" => Some(ShipSlotKind::DataLinkScanner),
            "CodexScanner" => Some(ShipSlotKind::CodexScanner),
            "DiscoveryScanner" => Some(ShipSlotKind::DiscoveryScanner),
            "EngineColour" => Some(ShipSlotKind::EngineColor),
            "WeaponColour" => Some(ShipSlotKind::WeaponColor),
            "StringLights" => Some(ShipSlotKind::StringLights),
            "ShipKitSpoiler" => Some(ShipSlotKind::ShipKitSpoiler),
            "ShipKitWings" => Some(ShipSlotKind::ShipKitWings),
            "ShipKitTail" => Some(ShipSlotKind::ShipKitTail),
            "ShipKitBumper" => Some(ShipSlotKind::ShipKitBumper),
            _ => None,
        };

        if let Some(kind) = specific {
            return Ok(ShipSlot { slot_nr: 0, kind });
        }

        if let Some(captures) = UTILITY_HARDPOINT_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::UtilityMount,
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
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::Hardpoint(size),
            });
        }

        if let Some(captures) = OPTIONAL_INTERNAL_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            let size = captures
                .get(2)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(ShipSlotError::OptionalInternalSizeParseError)?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::OptionalInternal(size),
            });
        }

        if let Some(captures) = MILITARY_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::Military,
            });
        }

        if let Some(captures) = DECAL_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::Decal,
            });
        }

        if let Some(captures) = NAMEPLATE_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::Nameplate,
            });
        }

        if let Some(captures) = ID_PLATE_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::IDPlate,
            });
        }

        if let Some(captures) = BOBBLE_REGEX.captures(s) {
            let slot_nr = captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()
                .map_err(|_| ShipSlotError::FailedToParseSlotNr(s.to_string()))?;

            return Ok(ShipSlot {
                slot_nr,
                kind: ShipSlotKind::Bobble,
            });
        }

        if let Ok(core_slot) = s.parse() {
            return Ok(ShipSlot {
                slot_nr: 1,
                kind: ShipSlotKind::CoreInternal(core_slot),
            });
        }

        Err(ShipSlotError::FailedToParse(s.to_string()))
    }
}

from_str_deserialize_impl!(ShipSlot);

impl Display for ShipSlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ShipSlotKind::ShipCockpit => write!(f, "Ship Cockpit"),
            ShipSlotKind::CargoHatch => write!(f, "Cargo Hatch"),
            ShipSlotKind::UtilityMount => write!(f, "Utility Mount"),
            ShipSlotKind::Hardpoint(size) => write!(f, "{} Hardpoint", size.size_str()),
            ShipSlotKind::OptionalInternal(size) => write!(f, "Size {size} Optional Internal"),
            ShipSlotKind::Military => write!(f, "Military Slot"),
            ShipSlotKind::CoreInternal(core_slot) => write!(f, "{core_slot} Core Internal"),
            ShipSlotKind::DataLinkScanner => write!(f, "Data Link Scanner"),
            ShipSlotKind::CodexScanner => write!(f, "Codex Scanner"),
            ShipSlotKind::DiscoveryScanner => write!(f, "Discovery Scanner"),

            // Cosmetic
            ShipSlotKind::PaintJob => write!(f, "Paint job"),
            ShipSlotKind::Decal => write!(f, "Decal"),
            ShipSlotKind::VesselVoice => write!(f, "COVAS Voice"),
            ShipSlotKind::Nameplate => write!(f, "Nameplate"),
            ShipSlotKind::IDPlate => write!(f, "ID-Plate"),
            ShipSlotKind::Bobble => write!(f, "Bobble"),
            ShipSlotKind::StringLights => write!(f, "String Lights"),
            ShipSlotKind::EngineColor => write!(f, "Engine Colour"),
            ShipSlotKind::WeaponColor => write!(f, "Weapon Colour"),
            ShipSlotKind::ShipKitSpoiler => write!(f, "Ship Kit Spoiler"),
            ShipSlotKind::ShipKitWings => write!(f, "Ship Kit Wing"),
            ShipSlotKind::ShipKitTail => write!(f, "Ship Kit Tail"),
            ShipSlotKind::ShipKitBumper => write!(f, "Ship Kit Bumper"),
        }
    }
}

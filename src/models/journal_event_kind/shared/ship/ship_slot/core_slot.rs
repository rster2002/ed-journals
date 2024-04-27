use std::str::FromStr;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CoreSlot {
    Armour,
    PowerPlant,
    MainEngines,
    PowerDistributor,
    Sensors,
    LifeSupport,
    FrameShiftDrive,
}

#[derive(Debug, Error)]
pub enum CoreSlotParseError {
    #[error("Unknown core slot '{0}'")]
    UnknownCoreSlot(String),
}

impl FromStr for CoreSlot {
    type Err = CoreSlotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PowerPlant" => Ok(CoreSlot::PowerPlant),
            "MainEngines" => Ok(CoreSlot::MainEngines),
            "PowerDistributor" => Ok(CoreSlot::PowerDistributor),
            "Radar" => Ok(CoreSlot::Sensors),
            "LifeSupport" => Ok(CoreSlot::LifeSupport),
            "Armour" => Ok(CoreSlot::Armour),
            "FrameShiftDrive" => Ok(CoreSlot::FrameShiftDrive),

            _ => Err(CoreSlotParseError::UnknownCoreSlot(s.to_string())),
        }
    }
}

from_str_deserialize_impl!(CoreSlot);


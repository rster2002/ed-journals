use std::str::FromStr;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CoreSlot {
    PowerPlant,
    MainEngines,
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
            _ => Err(CoreSlotParseError::UnknownCoreSlot(s.to_string())),
        }
    }
}

from_str_deserialize_impl!(CoreSlot);


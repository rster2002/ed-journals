use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum CoreSlot {
    Armour,
    PowerPlant,
    MainEngines,
    PowerDistributor,
    Sensors,
    LifeSupport,
    FrameShiftDrive,
    FuelTank,
    PlanetaryApproachSuite,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
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
            "FuelTank" => Ok(CoreSlot::FuelTank),
            "PlanetaryApproachSuite" => Ok(CoreSlot::FuelTank),

            _ => Err(CoreSlotParseError::UnknownCoreSlot(s.to_string())),
        }
    }
}

from_str_deserialize_impl!(CoreSlot);

impl Display for CoreSlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CoreSlot::Armour => "Armor",
                CoreSlot::PowerPlant => "Power Plant",
                CoreSlot::MainEngines => "Thrusters",
                CoreSlot::PowerDistributor => "Power Distributor",
                CoreSlot::Sensors => "Sensors",
                CoreSlot::LifeSupport => "Life Support",
                CoreSlot::FrameShiftDrive => "Frame Shift Drive",
                CoreSlot::FuelTank => "Fuel Tank",
                CoreSlot::PlanetaryApproachSuite => "Planetary Approach Suite",
            }
        )
    }
}

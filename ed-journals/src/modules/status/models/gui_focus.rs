use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::try_from_deserialize_impl;

#[derive(Debug, PartialEq, Eq)]
pub enum GuiFocus {
    NoFocus,
    InternalPanel,
    ExternalPanel,
    CommsPanel,
    RolePanel,
    StationServices,
    GalaxyMap,
    SystemMap,
    Orrery,
    FSSMode,
    SAAMode,
    Codex,
}

#[derive(Debug, Error)]
pub enum GuiFocusError {
    #[error("Unknown value for GUI focus: {0}")]
    UnknownValue(u8),
}

impl TryFrom<u8> for GuiFocus {
    type Error = GuiFocusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => GuiFocus::NoFocus,
            1 => GuiFocus::InternalPanel,
            2 => GuiFocus::ExternalPanel,
            3 => GuiFocus::CommsPanel,
            4 => GuiFocus::RolePanel,
            5 => GuiFocus::StationServices,
            6 => GuiFocus::GalaxyMap,
            7 => GuiFocus::SystemMap,
            8 => GuiFocus::Orrery,
            9 => GuiFocus::FSSMode,
            10 => GuiFocus::SAAMode,
            11 => GuiFocus::Codex,
            _ => return Err(GuiFocusError::UnknownValue(value)),
        })
    }
}

try_from_deserialize_impl!(u8 => GuiFocus);

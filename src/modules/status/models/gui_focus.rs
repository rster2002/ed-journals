use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

use crate::deserialize_in_order_impl;

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownU8(u8),

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum GuiFocusError {
    #[error("Unknown value for GUI focus: {0}")]
    UnknownValue(u8),

    #[error("Unknown string value for GUI focus: '{0}'")]
    UnknownStringValue(String),
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

            #[cfg(feature = "allow-unknown")]
            _ => GuiFocus::UnknownU8(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(GuiFocusError::UnknownValue(value)),
        })
    }
}

impl FromStr for GuiFocus {
    type Err = GuiFocusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "NoFocus" => GuiFocus::NoFocus,
            "InternalPanel" => GuiFocus::InternalPanel,
            "ExternalPanel" => GuiFocus::ExternalPanel,
            "CommsPanel" => GuiFocus::CommsPanel,
            "RolePanel" => GuiFocus::RolePanel,
            "StationServices" => GuiFocus::StationServices,
            "GalaxyMap" => GuiFocus::GalaxyMap,
            "SystemMap" => GuiFocus::SystemMap,
            "Orrery" => GuiFocus::Orrery,
            "FSSMode" => GuiFocus::FSSMode,
            "SAAMode" => GuiFocus::SAAMode,
            "Codex" => GuiFocus::Codex,

            #[cfg(feature = "allow-unknown")]
            _ => GuiFocus::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(GuiFocusError::UnknownStringValue(s.to_string())),
        })
    }
}

deserialize_in_order_impl!(
    GuiFocus =>
        A ? u8,
        B # String,
);

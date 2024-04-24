mod ship_hardpoint_module;
mod ship_internal_module;
mod module_class;

use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::cockpit_module::CockpitModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::ShipInternalModule;
use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ShipModule {

    // #[serde(rename = "int_engine_size5_class5")]
    // Thrusters5A,
    //
    // #[serde(rename = "int_engine_size6_class5")]
    // Thrusters6A,
    //
    // #[serde(rename = "int_powerplant_size7_class5")]
    // PowerPlant7A,
    //
    // #[serde(rename = "$hpt_beamlaser_gimbal_small_name;")]
    // SmallGimballedBeamLaser,
    //
    // #[serde(rename = "$hpt_beamlaser_gimbal_medium_name;", alias = "hpt_beamlaser_gimbal_medium")]
    // MediumGimballedBeamLaser,
    //
    // #[serde(rename = "$hpt_atventdisruptorpylon_fixed_large_name;")]
    // LargeNaniteTorpedoPylon,
    //
    #[serde(rename = "$hpt_heatsinklauncher_turret_tiny_name;")]
    HeatsinkLauncher,

    #[serde(rename = "$modularcargobaydoor_name;")]
    CargoBayDoor,

    #[serde(untagged)]
    Internal(ShipInternalModule),

    #[serde(untagged)]
    Hardpoint(ShipHardpointModule),

    #[serde(untagged)]
    Cockpit(CockpitModule),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

// #[derive(Debug, Error)]
// pub enum ShipModuleParseError {
//     #[error(transparent)]
//     SerdeJsonError(#[from] serde_json::Error),
//
//     #[error("Unknown ship module: {0}")]
//     UnknownShipModule(String),
// }
//

//
// impl FromStr for ShipModule {
//     type Err = ShipModuleParseError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {

//
//         match s {
//             "$modularcargobaydoor_name;" => Ok(ShipModule::CargoBayDoor),
//
//             #[cfg(not(feature = "strict"))]
//             _ => Ok(ShipModule::Unknown(s.to_string())),
//
//             #[cfg(feature = "strict")]
//             _ => Err(ShipModuleParseError::UnknownShipModule(s.to_string())),
//         }
//     }
// }
//
// from_str_deserialize_impl!(ShipModule);

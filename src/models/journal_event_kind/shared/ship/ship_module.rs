pub mod ship_hardpoint_module;
pub mod ship_internal_module;
pub mod module_class;

use std::str::FromStr;
use serde::Deserialize;
use crate::models::journal_event_kind::shared::ship::cockpit_module::CockpitModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::ShipInternalModule;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ShipModule {
    /// Special case for the cargo bay door.
    #[serde(rename = "$modularcargobaydoor_name;")]
    CargoBayDoor,

    /// Any internal module, this includes core and optional modules.
    #[serde(untagged)]
    Internal(ShipInternalModule),

    /// For external modules, both full-sized hardpoints and utility modules.
    #[serde(untagged)]
    Hardpoint(ShipHardpointModule),

    #[serde(untagged)]
    Cockpit(CockpitModule),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

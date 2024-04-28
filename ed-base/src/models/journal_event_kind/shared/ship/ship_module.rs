pub mod module_class;
pub mod ship_armor_module;
pub mod ship_cockpit_module;
pub mod ship_hardpoint_module;
pub mod ship_internal_module;

use crate::models::journal_event_kind::shared::ship::ship_module::ship_armor_module::ShipArmorModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_cockpit_module::ShipCockpitModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::ShipInternalModule;
use serde::Deserialize;

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
    Armor(ShipArmorModule),

    #[serde(untagged)]
    Cockpit(ShipCockpitModule),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

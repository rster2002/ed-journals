use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::journal_event_content::shared::ship::ship_module::ship_decal::ShipDecal;
use crate::journal_event_content::shared::ship::ship_module::ship_nameplate::ShipNameplate;
use crate::journal_event_content::shared::ship::ship_module::ship_paint_job::ShipPaintJob;
use crate::journal_event_content::shared::ship::ship_module::ship_voicepack::ShipVoicepack;

use crate::models::journal_event_content::shared::ship::ship_module::ship_cockpit_module::ShipCockpitModule;
use crate::models::journal_event_content::shared::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::ShipInternalModule;

pub mod module_class;
pub mod ship_cockpit_module;
pub mod ship_hardpoint_module;
pub mod ship_internal_module;
mod ship_paint_job;
mod ship_decal;
mod ship_voicepack;
mod ship_nameplate;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ShipModule {
    /// Special case for the cargo bay door.
    #[serde(
        alias = "modularcargobaydoor",
        alias = "modularcargobaydoorfdl",
        alias = "$modularcargobaydoor_name;",
    )]
    CargoBayDoor,

    /// Any internal module, this includes core and optional modules.
    #[serde(untagged)]
    Internal(ShipInternalModule),

    /// For external modules, both full-sized hardpoints and utility modules.
    #[serde(untagged)]
    Hardpoint(ShipHardpointModule),

    #[serde(untagged)]
    Cockpit(ShipCockpitModule),

    #[serde(untagged)]
    PaintJob(ShipPaintJob),

    #[serde(untagged)]
    Decal(ShipDecal),

    #[serde(untagged)]
    VoicePack(ShipVoicepack),

    #[serde(untagged)]
    Nameplate(ShipNameplate),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for ShipModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipModule::CargoBayDoor => write!(f, "Cargo Hatch"),
            ShipModule::Internal(internal_module) => internal_module.fmt(f),
            ShipModule::Hardpoint(hardpoint_module) => hardpoint_module.fmt(f),
            ShipModule::Cockpit(_) => write!(f, "Cockpit"),
            ShipModule::PaintJob(_) => write!(f, "Paint job"),
            ShipModule::Decal(_) => write!(f, "Decal"),
            ShipModule::VoicePack(_) => write!(f, "Voicepack"),
            ShipModule::Nameplate(_) => write!(f, "Nameplate"),

            #[cfg(not(feature = "strict"))]
            ShipModule::Unknown(unknown) => write!(f, "Unknown module: {}", unknown),
        }
    }
}

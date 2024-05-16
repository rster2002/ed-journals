use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

use crate::modules::models::ship::ship_module::ship_decal::ShipDecal;
use crate::modules::models::ship::ship_module::ship_nameplate::ShipNameplate;
use crate::modules::models::ship::ship_module::ship_paint_job::ShipPaintJob;
use crate::modules::models::ship::ship_module::ship_voicepack::ShipVoicepack;
use crate::modules::models::ship::ship_module::ship_cockpit_module::ShipCockpitModule;
use crate::modules::models::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
use crate::modules::models::ship::ship_module::ship_internal_module::ShipInternalModule;

pub mod module_class;
pub mod ship_cockpit_module;
pub mod ship_decal;
pub mod ship_hardpoint_module;
pub mod ship_internal_module;
pub mod ship_nameplate;
pub mod ship_paint_job;
pub mod ship_voicepack;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ShipModule {
    /// Special case for the cargo bay door.
    #[serde(
        alias = "modularcargobaydoor",
        alias = "modularcargobaydoorfdl",
        alias = "$modularcargobaydoor_name;"
    )]
    CargoBayDoor,

    #[serde(alias = "hpt_shipdatalinkscanner")]
    DataLinkScanner,

    #[serde(alias = "int_codexscanner")]
    CodexScanner,

    #[serde(alias = "int_stellarbodydiscoveryscanner_standard")]
    DiscoverScanner,

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
            ShipModule::DataLinkScanner => write!(f, "Data Link Scanner"),
            ShipModule::CodexScanner => write!(f, "Codex Scanner"),
            ShipModule::DiscoverScanner => write!(f, "Discovery Scanner"),
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

#[cfg(test)]
mod tests {
    use serde_json::{Error, Value};
    use crate::models::ship::ship_module::ShipModule;

    #[test]
    fn modules_are_parsed_correctly() {
        let mut test_cases = include_str!("zz_ship_module_test_cases.txt")
            .lines();

        let mut count = 0;

        for line in test_cases {
            let result = serde_json::from_value::<ShipModule>(Value::String(line.to_string()));
            count += 1;

            if result.is_err() {
                dbg!(&line);
                dbg!(&result);
            }

            assert!(result.is_ok());
        }

        assert!(count > 1000);
    }
}

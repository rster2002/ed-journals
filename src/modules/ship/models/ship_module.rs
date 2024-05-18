use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};
use crate::models::ship::ship_module::ship_bobble::ShipBobble;
use crate::models::ship::ship_module::ship_cockpit_module::ShipCockpitModule;
use crate::models::ship::ship_module::ship_decal::ShipDecal;
use crate::models::ship::ship_module::ship_engine_color::ShipEngineColor;
use crate::models::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
use crate::models::ship::ship_module::ship_internal_module::ShipInternalModule;
use crate::models::ship::ship_module::ship_kit_module::ShipKitModule;
use crate::models::ship::ship_module::ship_nameplate::ShipNameplate;
use crate::models::ship::ship_module::ship_paint_job::ShipPaintJob;
use crate::models::ship::ship_module::ship_string_lights::ShipStringLights;
use crate::models::ship::ship_module::ship_voicepack::ShipVoicepack;
use crate::models::ship::ship_module::ship_weapon_color::ShipWeaponColor;
use crate::models::ship::ship_module::ship_hardpoint_module::hardpoint_size::HardpointSize;

pub mod module_class;
pub mod ship_cockpit_module;
pub mod ship_decal;
pub mod ship_hardpoint_module;
pub mod ship_internal_module;
pub mod ship_nameplate;
pub mod ship_paint_job;
pub mod ship_voicepack;
pub mod ship_engine_color;
pub mod ship_kit_module;
pub mod ship_bobble;
pub mod ship_weapon_color;
pub mod ship_string_lights;

/// Any ship module, covering all the different kinds of modules: hardpoints, utility modules,
/// core internals and optional internals. This also covers cosmetic items like paint job, decals,
/// engine colors etc.
///
/// The game considers full-sized hardpoints and utility modules both use [ShipHardpointModule],
/// where utility modules have a hardpoint size of [HardpointSize::Tiny].
///
/// The same is true for core internals and optional internals which both use [ShipInternalModule].
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

    // Cosmetic
    #[serde(untagged)]
    PaintJob(ShipPaintJob),

    #[serde(untagged)]
    Decal(ShipDecal),

    #[serde(untagged)]
    VoicePack(ShipVoicepack),

    #[serde(untagged)]
    Nameplate(ShipNameplate),

    #[serde(untagged)]
    EngineColor(ShipEngineColor),

    #[serde(untagged)]
    WeaponColor(ShipWeaponColor),

    #[serde(untagged)]
    ShipKitModule(ShipKitModule),

    #[serde(untagged)]
    Bobble(ShipBobble),

    #[serde(untagged)]
    StringLights(ShipStringLights),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl ShipModule {
    pub fn is_hardpoint_module(&self) -> bool {
        matches!(self, ShipModule::Hardpoint(_))
    }

    pub fn is_full_sized_hardpoint_module(&self) -> bool {
        let ShipModule::Hardpoint(hardpoint) = self else {
            return false;
        };

        hardpoint.is_full_sized_module()
    }

    pub fn is_utility_module(&self) -> bool {
        let ShipModule::Hardpoint(hardpoint) = self else {
            return false;
        };

        hardpoint.is_utility_module()
    }

    pub fn is_internal_module(&self) -> bool {
        matches!(self, ShipModule::Hardpoint(_))
    }

    pub fn is_core_internal(&self) -> bool {
        let ShipModule::Internal(internal) = self else {
            return false;
        };

        internal.is_core_internal()
    }

    pub fn is_optional_internal(&self) -> bool {
        let ShipModule::Internal(internal) = self else {
            return false;
        };

        internal.is_optional_internal()
    }

    pub fn is_powerplay_module(&self) -> bool {
        match self {
            ShipModule::Internal(internal) => internal.is_powerplay_module(),
            ShipModule::Hardpoint(hardpoint) => hardpoint.is_powerplay_module(),
            _ => false,
        }
    }

    pub fn is_guardian_module(&self) -> bool {
        match self {
            ShipModule::Internal(internal) => internal.is_guardian_module(),
            ShipModule::Hardpoint(hardpoint) => hardpoint.is_guardian_module(),
            _ => false,
        }
    }

    pub fn is_cosmetic(&self) -> bool {
        match self {
            ShipModule::PaintJob(_)
            | ShipModule::Decal(_)
            | ShipModule::VoicePack(_)
            | ShipModule::Nameplate(_)
            | ShipModule::EngineColor(_)
            | ShipModule::WeaponColor(_)
            | ShipModule::ShipKitModule(_)
            | ShipModule::Bobble(_)
            | ShipModule::StringLights(_) => true,
            _ => false,
        }
    }
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
            ShipModule::EngineColor(_) => write!(f, "Engine Color"),
            ShipModule::WeaponColor(_) => write!(f, "Weapon Color"),
            ShipModule::Bobble(_) => write!(f, "Bobble"),
            ShipModule::StringLights(_) => write!(f, "String Lights"),
            ShipModule::ShipKitModule(module) => write!(f, "Skip kit module: {}", module.name),

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

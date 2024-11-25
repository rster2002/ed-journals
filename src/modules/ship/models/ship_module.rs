use crate::modules::ship::models::ship_module::ship_bobble::ShipBobble;
use crate::modules::ship::models::ship_module::ship_engine_color::ShipEngineColor;
use crate::modules::ship::models::ship_module::ship_kit_module::ShipKitModule;
use crate::modules::ship::models::ship_module::ship_string_lights::ShipStringLights;
use crate::modules::ship::models::ship_module::ship_weapon_color::ShipWeaponColor;
use crate::modules::ship::{
    ShipCockpitModule, ShipDecal, ShipHardpointModule, ShipInternalModule, ShipNameplate,
    ShipPaintJob, ShipVoicepack,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod module_class;
pub mod ship_bobble;
pub mod ship_cockpit_module;
pub mod ship_decal;
pub mod ship_engine_color;
pub mod ship_hardpoint_module;
pub mod ship_internal_module;
pub mod ship_kit_module;
pub mod ship_nameplate;
pub mod ship_paint_job;
pub mod ship_string_lights;
pub mod ship_voicepack;
pub mod ship_weapon_color;

/// Any ship module, covering all the different kinds of modules: hardpoints, utility modules,
/// core internals and optional internals. This also covers cosmetic items like paint job, decals,
/// engine colors etc.
///
/// The game considers full-sized hardpoints and utility modules both use [ShipHardpointModule],
/// where utility modules have a hardpoint size of [HardpointSize::Tiny].
///
/// The same is true for core internals and optional internals which both use [ShipInternalModule].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum ShipModule {
    /// Special case for the cargo bay door.
    #[serde(
        alias = "modularcargobaydoor",
        alias = "modularcargobaydoorfdl",
        alias = "$modularcargobaydoor_name;"
    )]
    CargoBayDoor,

    /// Spacial case for the data link scanner.
    #[serde(alias = "hpt_shipdatalinkscanner")]
    DataLinkScanner,

    /// Spacial case for the codex scanner.
    #[serde(alias = "int_codexscanner")]
    CodexScanner,

    /// Spacial case for the discovery scanner.
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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl ShipModule {
    /// Whether the module is any kind of hardpoint module, including utility modules.
    pub fn is_hardpoint_module(&self) -> bool {
        matches!(self, ShipModule::Hardpoint(_))
    }

    /// Whether the module is a full-sized hardpoint module. This does not include utility modules.
    pub fn is_full_sized_hardpoint_module(&self) -> bool {
        let ShipModule::Hardpoint(hardpoint) = self else {
            return false;
        };

        hardpoint.is_full_sized_module()
    }

    /// Whether the module is a utility module.
    pub fn is_utility_module(&self) -> bool {
        let ShipModule::Hardpoint(hardpoint) = self else {
            return false;
        };

        hardpoint.is_utility_module()
    }

    /// Whether the module is any internal module. This includes both core- and optional internals.
    pub fn is_internal_module(&self) -> bool {
        matches!(self, ShipModule::Internal(_))
    }

    /// Whether the module is a core internal module.
    pub fn is_core_internal(&self) -> bool {
        let ShipModule::Internal(internal) = self else {
            return false;
        };

        internal.is_core_internal()
    }

    /// Whether the module is an optional internal module.
    pub fn is_optional_internal(&self) -> bool {
        let ShipModule::Internal(internal) = self else {
            return false;
        };

        internal.is_optional_internal()
    }

    /// Whether the module is a module that is unlocked through powerplay.
    pub fn is_powerplay_module(&self) -> bool {
        match self {
            ShipModule::Internal(internal) => internal.is_powerplay_module(),
            ShipModule::Hardpoint(hardpoint) => hardpoint.is_powerplay_module(),
            _ => false,
        }
    }

    /// Whether the module is a module that is unlocked using guardian parts at a guardian
    /// technology broker.
    pub fn is_guardian_module(&self) -> bool {
        match self {
            ShipModule::Internal(internal) => internal.is_guardian_module(),
            ShipModule::Hardpoint(hardpoint) => hardpoint.is_guardian_module(),
            _ => false,
        }
    }

    /// Whether the module is a cosmetic module. The game tracks these cosmetics as modules that
    /// slot into special slots.
    pub fn is_cosmetic(&self) -> bool {
        matches!(
            self,
            ShipModule::PaintJob(_)
                | ShipModule::Decal(_)
                | ShipModule::VoicePack(_)
                | ShipModule::Nameplate(_)
                | ShipModule::EngineColor(_)
                | ShipModule::WeaponColor(_)
                | ShipModule::ShipKitModule(_)
                | ShipModule::Bobble(_)
                | ShipModule::StringLights(_)
        )
    }
}

impl FromStr for ShipModule {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
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

            #[cfg(feature = "allow-unknown")]
            ShipModule::Unknown(unknown) => write!(f, "Unknown module: {}", unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::modules::ship::ShipModule;
    use crate::ship::{HardpointMounting, HardpointSize, ModuleClass};

    #[test]
    fn modules_are_parsed_correctly() {
        let test_cases = include_str!("zz_ship_module_test_cases.txt").lines();

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

    #[test]
    fn all_eddn_test_cases_are_parsed_correctly() {
        let content = include_str!("zz_ship_modules.txt");
        let lines = content.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            let mut parts = line.split(',');
            parts.next().unwrap();

            let input = parts.next().unwrap();
            parts.next().unwrap();
            parts.next().unwrap();

            let mounting: Option<HardpointMounting> = parts
                .next()
                .and_then(|string| {
                    if string.is_empty() {
                        None
                    } else {
                        Some(string)
                    }
                })
                .map(|mounting| mounting.parse())
                .transpose()
                .unwrap();

            parts.next().unwrap();
            parts.next().unwrap();

            let size = parts.next().unwrap().parse::<u8>().unwrap();

            let class = parts.next().unwrap().parse::<ModuleClass>().unwrap();

            let parsed = serde_json::from_value::<ShipModule>(Value::String(input.to_string()));

            dbg!(&input);
            dbg!(&parsed);
            assert!(parsed.is_ok());

            match parsed.unwrap() {
                // ShipModule::CargoBayDoor => {}
                // ShipModule::DataLinkScanner => {}
                // ShipModule::CodexScanner => {}
                // ShipModule::DiscoverScanner => {}
                ShipModule::Internal(internal) => {
                    dbg!(&internal);

                    assert_eq!(internal.size, size);
                    assert_eq!(internal.class, class);
                }
                ShipModule::Hardpoint(hardpoint) => {
                    dbg!(&hardpoint);

                    let mounting = mounting.unwrap_or(HardpointMounting::Turreted);
                    let hardpoint_size = HardpointSize::try_from(size).unwrap();

                    assert_eq!(hardpoint.mounting, mounting);
                    assert_eq!(hardpoint.class, class);
                    assert_eq!(hardpoint.size, hardpoint_size);
                }
                _ => {}
            }
        }
    }
}

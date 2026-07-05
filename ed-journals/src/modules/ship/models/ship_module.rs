use crate::from_str_deserialize_impl;
use crate::modules::ship::models::ship_module::ship_bobble::ShipBobble;
use crate::modules::ship::models::ship_module::ship_engine_color::ShipEngineColor;
use crate::modules::ship::models::ship_module::ship_kit_module::ShipKitModule;
use crate::modules::ship::models::ship_module::ship_string_lights::ShipStringLights;
use crate::modules::ship::models::ship_module::ship_weapon_color::ShipWeaponColor;
use crate::modules::ship::{
    ShipCockpitModule, ShipDecal, ShipHardpointModule, ShipInternalModule, ShipNameplate,
    ShipPaintJob, ShipVoicepack,
};
use crate::ship::FighterType;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

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
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ShipModule {
    /// Special case for the cargo bay door.
    CargoBayDoor,

    /// Spacial case for the data link scanner.
    DataLinkScanner,

    /// Spacial case for the codex scanner.
    CodexScanner,

    /// Spacial case for the discovery scanner.
    DiscoverScanner,

    /// Some fighter types show up as ship modules when unlocking them from technology brokers.
    Fighter(FighterType),

    /// Any internal module, this includes core and optional modules.
    Internal(ShipInternalModule),

    /// For external modules, both full-sized hardpoints and utility modules.
    Hardpoint(ShipHardpointModule),

    /// Special module for a cockpit for a specific ship.
    Cockpit(ShipCockpitModule),

    // Cosmetic
    PaintJob(ShipPaintJob),
    Decal(ShipDecal),
    VoicePack(ShipVoicepack),
    Nameplate(ShipNameplate),
    EngineColor(ShipEngineColor),
    WeaponColor(ShipWeaponColor),
    ShipKitModule(ShipKitModule),
    Bobble(ShipBobble),
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

    fn exact_match(s: &str) -> Option<ShipModule> {
        Some(match s {
            "modularcargobaydoor"
            | "modularcargobaydoorfdl"
            | "$modularcargobaydoor_name;"
            | "$modularcargobaydoorfdl_name;" => ShipModule::CargoBayDoor,

            "hpt_shipdatalinkscanner" => ShipModule::DataLinkScanner,
            "int_codexscanner" => ShipModule::CodexScanner,
            "int_stellarbodydiscoveryscanner_standard" => ShipModule::DiscoverScanner,

            _ => return None,
        })
    }
}

#[derive(Debug, Error)]
pub enum ShipModuleError {
    #[error("Unknown ship module: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for ShipModule {
    type Err = ShipModuleError;

    // TODO this needs to be cleaner eventually
    #[cfg(not(feature = "allow-unknown"))]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(exact_match) = Self::exact_match(s) {
            return Ok(exact_match);
        }

        if let Ok(fighter) = FighterType::from_str(s) {
            return Ok(ShipModule::Fighter(fighter));
        }

        if let Ok(interal) = ShipInternalModule::from_str(s) {
            return Ok(ShipModule::Internal(interal));
        }

        if let Ok(hardpoint) = ShipHardpointModule::from_str(s) {
            return Ok(ShipModule::Hardpoint(hardpoint));
        }

        if let Ok(cockpit) = ShipCockpitModule::from_str(s) {
            return Ok(ShipModule::Cockpit(cockpit));
        }

        if let Ok(paint_job) = ShipPaintJob::from_str(s) {
            return Ok(ShipModule::PaintJob(paint_job));
        }

        if let Ok(decal_job) = ShipDecal::from_str(s) {
            return Ok(ShipModule::Decal(decal_job));
        }

        if let Ok(voice_pack) = ShipVoicepack::from_str(s) {
            return Ok(ShipModule::VoicePack(voice_pack));
        }

        if let Ok(nameplate) = ShipNameplate::from_str(s) {
            return Ok(ShipModule::Nameplate(nameplate));
        }

        if let Ok(engine_color) = ShipEngineColor::from_str(s) {
            return Ok(ShipModule::EngineColor(engine_color));
        }

        if let Ok(weapon_color) = ShipWeaponColor::from_str(s) {
            return Ok(ShipModule::WeaponColor(weapon_color));
        }

        if let Ok(ship_kit_module) = ShipKitModule::from_str(s) {
            return Ok(ShipModule::ShipKitModule(ship_kit_module));
        }

        if let Ok(bobble) = ShipBobble::from_str(s) {
            return Ok(ShipModule::Bobble(bobble));
        }

        if let Ok(string_lights) = ShipStringLights::from_str(s) {
            return Ok(ShipModule::StringLights(string_lights));
        }

        Err(ShipModuleError::UnknownEntry(s.to_string()))
    }

    #[cfg(feature = "allow-unknown")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(exact_match) = Self::exact_match(s) {
            return Ok(exact_match);
        }

        if let Ok(fighter) = FighterType::from_str(s) {
            if !fighter.is_unknown() {
                return Ok(ShipModule::Fighter(fighter));
            }
        }

        if let Ok(internal) = ShipInternalModule::from_str(s) {
            return Ok(ShipModule::Internal(internal));
        }

        if let Ok(hardpoint) = ShipHardpointModule::from_str(s) {
            return Ok(ShipModule::Hardpoint(hardpoint));
        }

        if let Ok(cockpit) = ShipCockpitModule::from_str(s) {
            return Ok(ShipModule::Cockpit(cockpit));
        }

        if let Ok(paint_job) = ShipPaintJob::from_str(s) {
            return Ok(ShipModule::PaintJob(paint_job));
        }

        if let Ok(decal_job) = ShipDecal::from_str(s) {
            return Ok(ShipModule::Decal(decal_job));
        }

        if let Ok(voice_pack) = ShipVoicepack::from_str(s) {
            return Ok(ShipModule::VoicePack(voice_pack));
        }

        if let Ok(nameplate) = ShipNameplate::from_str(s) {
            return Ok(ShipModule::Nameplate(nameplate));
        }

        if let Ok(engine_color) = ShipEngineColor::from_str(s) {
            return Ok(ShipModule::EngineColor(engine_color));
        }

        if let Ok(weapon_color) = ShipWeaponColor::from_str(s) {
            return Ok(ShipModule::WeaponColor(weapon_color));
        }

        if let Ok(ship_kit_module) = ShipKitModule::from_str(s) {
            return Ok(ShipModule::ShipKitModule(ship_kit_module));
        }

        if let Ok(bobble) = ShipBobble::from_str(s) {
            return Ok(ShipModule::Bobble(bobble));
        }

        if let Ok(string_lights) = ShipStringLights::from_str(s) {
            return Ok(ShipModule::StringLights(string_lights));
        }

        Err(ShipModuleError::UnknownEntry(s.to_string()))
    }
}

from_str_deserialize_impl!(ShipModule);

impl Display for ShipModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipModule::CargoBayDoor => write!(f, "Cargo Hatch"),
            ShipModule::DataLinkScanner => write!(f, "Data Link Scanner"),
            ShipModule::CodexScanner => write!(f, "Codex Scanner"),
            ShipModule::DiscoverScanner => write!(f, "Discovery Scanner"),
            ShipModule::Fighter(fighter) => write!(f, "{}", fighter),
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
            ShipModule::Unknown(unknown) => write!(f, "Unknown module: {unknown}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::modules::ship::ShipModule;
    use crate::ship::{
        ArmorGrade, ArmorModule, HardpointMounting, HardpointSize, InternalModule, ModuleClass,
        ShipInternalModule, ShipType,
    };

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
    fn specific_ship_module_test_cases_are_parsed_correctly() {
        let test_cases = [(
            "$federation_corvette_armour_grade1_name;",
            ShipModule::Internal(ShipInternalModule {
                module: InternalModule::Armor(ArmorModule {
                    ship: ShipType::FederalCorvette,
                    grade: ArmorGrade::LightweightAlloy,
                }),
                size: 1,
                class: ModuleClass::C,
                free: false,
            }),
        )];

        for (input, expected) in test_cases {
            let result = serde_json::from_value::<ShipModule>(Value::String(input.to_string()));

            if result.is_err() {
                dbg!(&input);
                dbg!(&result);
            }

            assert_eq!(result.unwrap(), expected);
        }
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

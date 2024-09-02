use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::ship::{
    ArmorModule, ArmorModuleError, InternalModule, InternalType, ModuleClass, ModuleClassError,
};

pub mod armor_grade;
pub mod armor_module;
pub mod internal_module;
pub mod internal_type;

/// Model for internal modules, this includes both core internals and optional internals.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipInternalModule {
    /// The kind internal module, which can both be core- and optional internals.
    pub module: InternalModule,

    /// The module size.
    pub size: u8,

    /// The class of the module.
    pub class: ModuleClass,

    /// Whether the module is a free module.
    pub free: bool,
}

#[derive(Debug, Error)]
pub enum ShipInternalModuleError {
    #[error("Failed to parse module: {0}]")]
    FailedToParseModule(#[source] serde_json::Error),

    #[error("Failed to parse size: {0}")]
    FailedToParseSize(#[from] ParseIntError),

    #[error("Failed to parse class number: {0}")]
    FailedToParseClassNumber(#[source] ParseIntError),

    #[error(transparent)]
    ArmorModuleError(#[from] ArmorModuleError),

    #[error(transparent)]
    UnknownClass(#[from] ModuleClassError),

    #[error("Failed to parse internal ship module: '{0}'")]
    FailedToParse(String),
}

impl ShipInternalModule {
    /// Returns whether the module is a core- or optional type.
    pub fn internal_type(&self) -> InternalType {
        self.module.internal_type()
    }

    /// Whether the module is a core internal.
    pub fn is_core_internal(&self) -> bool {
        self.module.is_core()
    }

    /// Whether the module is an option internal.
    pub fn is_optional_internal(&self) -> bool {
        self.module.is_optional()
    }

    /// Whether the module is a module that is unlocked through powerplay.
    pub fn is_powerplay_module(&self) -> bool {
        self.module.is_powerplay_module()
    }

    /// Whether the module is a module that is unlocked using guardian parts at a guardian
    /// technology broker.
    pub fn is_guardian_module(&self) -> bool {
        self.module.is_guardian_module()
    }
}

lazy_static! {
    static ref SHIP_INTERNAL_MODULE_REGEX: Regex =
        Regex::new(r#"^\$?[iI]nt_([a-zA-Z_]+?)(_[sS]ize(1|2|3|4|5|6|7|8))?(_[cC]lass(1|2|3|4|5))?(_[tT]iny)?(_[a-zA-Z_]+?)?(_name;)?$"#).unwrap();
}

impl FromStr for ShipInternalModule {
    type Err = ShipInternalModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let armor_result = ArmorModule::from_str(s);

        match armor_result {
            Ok(armor_module) => {
                return Ok(ShipInternalModule {
                    size: 1,
                    class: (&armor_module).into(),
                    module: InternalModule::Armor(armor_module),
                    free: false,
                })
            }
            Err(ArmorModuleError::FailedToParse(_)) => {}
            Err(e) => return Err(e.into()),
        }

        let Some(captures) = SHIP_INTERNAL_MODULE_REGEX.captures(s) else {
            return Err(ShipInternalModuleError::FailedToParse(s.to_string()));
        };

        let module_string = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str();

        let module_suffix = captures
            .get(7)
            .map(|capture| capture.as_str())
            .unwrap_or_default();

        let mut free = false;
        let mut module_string = format!("{}{}", module_string, module_suffix);

        if module_string.ends_with("_free") {
            module_string = module_string.replace("_free", "");
            free = true;
        }

        let module = module_string
            .parse()
            .map_err(ShipInternalModuleError::FailedToParseModule)?;

        if captures.get(6).is_some() {
            return Ok(ShipInternalModule {
                module,
                size: 1,
                class: ModuleClass::I,
                free,
            });
        }

        let size = match captures.get(3) {
            Some(capture) => capture.as_str().parse()?,
            None => 1,
        };

        // Phew... Okay. So. This part checks if there is a capture for the '_grade' part. If there
        // is one it will first check `special_grades` if it should be another grade instead. Else
        // if there is no match, then it will still check `special_grades` but without providing
        // a ModuleGrade input. If none of these things return a Some value, then the class will
        // default to `ModuleClass::E`.
        let class = captures
            .get(5)
            .map(|capture| {
                capture
                    .as_str()
                    .parse::<u8>()
                    .map_err(ShipInternalModuleError::FailedToParseClassNumber)
            })
            .transpose()?
            .map(|grade_nr| grade_nr.try_into())
            .transpose()?
            .map(|class| module.special_grades(size, Some(&class)).unwrap_or(class))
            .or_else(|| module.special_grades(size, None))
            .unwrap_or(ModuleClass::E);

        Ok(ShipInternalModule {
            module,
            size,
            class,
            free,
        })
    }
}

from_str_deserialize_impl!(ShipInternalModule);

impl Display for ShipInternalModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} {}", self.size, self.class, self.module)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::modules::ship::{
        ArmorGrade, ArmorModule, InternalModule, ModuleClass, ShipInternalModule, ShipType,
    };

    #[test]
    fn ship_internal_module_test_cases_are_parsed_correctly() {
        let test_cases = [
            (
                "$int_dronecontrol_collection_size3_class5_name;",
                ShipInternalModule {
                    module: InternalModule::CollectorLimpetController,
                    size: 3,
                    class: ModuleClass::A,
                    free: false,
                },
            ),
            (
                "int_engine_size5_class5",
                ShipInternalModule {
                    module: InternalModule::Thrusters,
                    size: 5,
                    class: ModuleClass::A,
                    free: false,
                },
            ),
            (
                "$int_engine_size5_class5_name;",
                ShipInternalModule {
                    module: InternalModule::Thrusters,
                    size: 5,
                    class: ModuleClass::A,
                    free: false,
                },
            ),
            (
                "$int_supercruiseassist_name;",
                ShipInternalModule {
                    module: InternalModule::SupercruiseAssist,
                    size: 1,
                    class: ModuleClass::E,
                    free: false,
                },
            ),
            (
                "$int_detailedsurfacescanner_tiny_name;",
                ShipInternalModule {
                    module: InternalModule::DetailedSurfaceScanner,
                    size: 1,
                    class: ModuleClass::I,
                    free: false,
                },
            ),
            (
                "$int_lifesupport_size4_class1_name;",
                ShipInternalModule {
                    module: InternalModule::LifeSupport,
                    size: 4,
                    class: ModuleClass::E,
                    free: false,
                },
            ),
            (
                "Int_Hyperdrive_Size5_Class5",
                ShipInternalModule {
                    module: InternalModule::FrameShiftDrive,
                    size: 5,
                    class: ModuleClass::A,
                    free: false,
                },
            ),
            (
                "$int_cargorack_size7_class1_name;",
                ShipInternalModule {
                    module: InternalModule::CargoRack,
                    size: 7,
                    class: ModuleClass::E,
                    free: false,
                },
            ),
            (
                "$int_shieldgenerator_size8_class3_fast_name;",
                ShipInternalModule {
                    module: InternalModule::BiWeaveShieldGenerator,
                    size: 8,
                    class: ModuleClass::C,
                    free: false,
                },
            ),
            (
                "$type9_military_armour_grade1_name;",
                ShipInternalModule {
                    module: InternalModule::Armor(ArmorModule {
                        ship: ShipType::Type10Defender,
                        grade: ArmorGrade::LightweightAlloy,
                    }),
                    size: 1,
                    class: ModuleClass::C,
                    free: false,
                },
            ),
            (
                "krait_mkii_armour_grade3",
                ShipInternalModule {
                    module: InternalModule::Armor(ArmorModule {
                        ship: ShipType::KraitMkII,
                        grade: ArmorGrade::MilitaryGradeComposite,
                    }),
                    size: 1,
                    class: ModuleClass::A,
                    free: false,
                },
            ),
            (
                "int_planetapproachsuite_advanced",
                ShipInternalModule {
                    module: InternalModule::PlanetApproachSuite,
                    size: 1,
                    class: ModuleClass::I,
                    free: false,
                },
            ),
        ];

        for (input, expected) in test_cases {
            let result = ShipInternalModule::from_str(input);

            dbg!(&result);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}

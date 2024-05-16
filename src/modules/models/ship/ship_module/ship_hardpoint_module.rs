use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::models::ship::ship_module::module_class::{ModuleClass, ModuleClassError};
use crate::modules::models::ship::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
use crate::modules::models::ship::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
use crate::modules::models::ship::ship_module::ship_hardpoint_module::hardpoint_type::HardpointType;
use crate::modules::models::ship::ship_slot::hardpoint_size::{HardpointSize, HardpointSizeParseError};

pub mod hardpoint_module;
pub mod hardpoint_mounting;
pub mod hardpoint_type;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipHardpointModule {
    pub module: HardpointModule,
    pub mounting: HardpointMounting,
    pub size: HardpointSize,
    pub class: ModuleClass,
}

#[derive(Debug, Error)]
pub enum ShipHardpointModuleError {
    #[error("Failed to parse hardpoint module: {0}")]
    FailedToParseHardpointModule(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint mounting: {0}")]
    FailedToParseHardpointMounting(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint size: {0}")]
    FailedToParseHardpointSize(#[from] HardpointSizeParseError),

    #[error("Failed to parse class number: {0}")]
    FailedToParseClassNumber(#[source] ParseIntError),

    #[error("Mounting type cannot be missing when the module size is not 'tiny'")]
    MissingMounting,

    #[error(transparent)]
    ModuleClassError(#[from] ModuleClassError),

    #[error("Failed to parse ship hardpoint module: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref GRADED_HARDPOINT_MODULE_REGEX: Regex = Regex::new(r#"^\$?[hH]pt_(\w+?)_size(\d+)_class(\d+)(_name;)?$"#).unwrap();
    static ref HARDPOINT_MODULE_REGEX: Regex = Regex::new(r#"^\$?[hH]pt_(\w+?)(_([gG]imbal|[fF]ixed|[tT]urret))?_([tT]iny|[sS]mall|[mM]edium|[lL]arge|[hH]uge)(_[a-zA-Z0-9_]+?)?(_name;)?$"#).unwrap();
}

impl FromStr for ShipHardpointModule {
    type Err = ShipHardpointModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = GRADED_HARDPOINT_MODULE_REGEX.captures(s) {
            return Self::from_graded_hardpoint_captures(captures);
        }

        if let Some(captures) = HARDPOINT_MODULE_REGEX.captures(s) {
            return Self::from_hardpoint_captures(captures);
        };

        Err(ShipHardpointModuleError::FailedToParse(s.to_string()))
    }
}

impl ShipHardpointModule {
    pub fn hardpoint_type(&self) -> HardpointType {
        self.module.hardpoint_type()
    }

    pub fn is_full_sized(&self) -> bool {
        self.module.is_full_sized()
    }

    pub fn is_utility(&self) -> bool {
        self.module.is_utility()
    }

    fn from_graded_hardpoint_captures(
        captures: Captures,
    ) -> Result<ShipHardpointModule, ShipHardpointModuleError> {
        let module = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(ShipHardpointModuleError::FailedToParseHardpointModule)?;

        let class = captures
            .get(3)
            .expect("Should have been captured already")
            .as_str()
            .parse::<u8>()
            .map_err(ShipHardpointModuleError::FailedToParseClassNumber)?
            .try_into()?;

        Ok(ShipHardpointModule {
            module,
            mounting: HardpointMounting::Turreted,
            size: HardpointSize::Tiny,
            class,
        })
    }

    fn from_hardpoint_captures(
        captures: Captures,
    ) -> Result<ShipHardpointModule, ShipHardpointModuleError> {
        let module = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str();

        let module_suffix = captures
            .get(5)
            .map(|capture| capture.as_str())
            .unwrap_or_default();

        let module = format!("{}{}", module, module_suffix)
            .parse()
            .map_err(ShipHardpointModuleError::FailedToParseHardpointModule)?;

        let size = captures
            .get(4)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        // Sometimes the mounting is missing when the size is 'tiny'
        let mounting = match captures.get(3) {
            Some(capture) => capture
                .as_str()
                .parse()
                .map_err(ShipHardpointModuleError::FailedToParseHardpointMounting),
            None => {
                if matches!(size, HardpointSize::Tiny) {
                    Ok(HardpointMounting::Turreted)
                } else {
                    Err(ShipHardpointModuleError::MissingMounting)
                }
            }
        }?;

        Ok(ShipHardpointModule {
            module,
            mounting,
            class: match &size {
                HardpointSize::Tiny => ModuleClass::I,
                _ => ModuleClass::E,
            },
            size,
        })
    }
}

from_str_deserialize_impl!(ShipHardpointModule);

impl Display for ShipHardpointModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let HardpointSize::Tiny = self.size {
            return write!(f, "{}{} {}", self.size.size_nr(), self.class, self.module);
        }

        write!(
            f,
            "{}{}/{} {}",
            self.size.size_nr(),
            self.class,
            self.mounting,
            self.module
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::modules::models::ship::ship_module::module_class::ModuleClass;
    use crate::modules::models::ship::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
    use crate::modules::models::ship::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
    use crate::modules::models::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;
    use crate::modules::models::ship::ship_slot::hardpoint_size::HardpointSize;

    #[test]
    fn ship_hardpoint_module_test_cases_all_pass() {
        let test_cases = [
            (
                "$hpt_beamlaser_gimbal_medium_name;",
                ShipHardpointModule {
                    module: HardpointModule::BeamLaser,
                    mounting: HardpointMounting::Gimballed,
                    size: HardpointSize::Medium,
                    class: ModuleClass::E,
                },
            ),
            (
                "$hpt_heatsinklauncher_turret_tiny_name;",
                ShipHardpointModule {
                    module: HardpointModule::HeatsinkLauncher,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::I,
                },
            ),
            (
                "Hpt_CausticSinkLauncher_Turret_Tiny",
                ShipHardpointModule {
                    module: HardpointModule::CausticSinkLauncher,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::I,
                },
            ),
            (
                "$hpt_chafflauncher_tiny_name;",
                ShipHardpointModule {
                    module: HardpointModule::ChaffLauncher,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::I,
                },
            ),
            (
                "$hpt_shieldbooster_size0_class5_name;",
                ShipHardpointModule {
                    module: HardpointModule::ShieldBooster,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::A,
                },
            ),
            (
                "Hpt_Guardian_GaussCannon_Fixed_Medium",
                ShipHardpointModule {
                    module: HardpointModule::GuardianGaussCannon,
                    mounting: HardpointMounting::Fixed,
                    size: HardpointSize::Medium,

                    // TODO this should be B
                    class: ModuleClass::E,
                },
            ),
        ];

        for (case, expected) in test_cases {
            let result = ShipHardpointModule::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn incorrect_combination_of_missing_mounting_type_and_size_is_rejected() {
        let result = ShipHardpointModule::from_str("$hpt_chafflauncher_mediun_name;");
        assert!(result.is_err());
    }
}

use std::num::ParseIntError;
use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::ship_module::module_class::{ModuleClass, ModuleClassError};
use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::internal_module::InternalModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::internal_type::InternalType;

pub mod internal_module;
mod internal_type;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShipInternalModule {
    pub module: InternalModule,
    pub size: u8,
    pub class: ModuleClass,
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
    UnknownClass(#[from] ModuleClassError),

    #[error("Failed to parse internal ship module: '{0}'")]
    FailedToParse(String),
}

const SHIP_INTERNAL_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$?[iI]nt_([a-zA-Z_]+?)(_[sS]ize(1|2|3|4|5|6|7|8))?(_[cC]lass(1|2|3|4|5))?(_[tT]iny)?(_name;)?$"#).unwrap());

impl FromStr for ShipInternalModule {
    type Err = ShipInternalModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = SHIP_INTERNAL_MODULE_REGEX.captures(s) else {
            return Err(ShipInternalModuleError::FailedToParse(s.to_string()));
        };

        let module = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| ShipInternalModuleError::FailedToParseModule(e))?;

        if captures.get(6).is_some() {
            return Ok(ShipInternalModule {
                module,
                size: 1,
                class: ModuleClass::I,
            });
        }

        let size = match captures.get(3) {
            Some(capture) => capture
                .as_str()
                .parse()?,
            None => 1,
        };

        let class = match captures.get(5) {
            Some(capture) => capture
                .as_str()
                .parse::<u8>()
                .map_err(|e| ShipInternalModuleError::FailedToParseClassNumber(e))?
                .try_into()?,
            None => ModuleClass::E,
        };

        Ok(ShipInternalModule {
            module,
            size,
            class,
        })
    }
}

from_str_deserialize_impl!(ShipInternalModule);

impl ShipInternalModule {
    pub fn internal_type(&self) -> InternalType {
        self.module.internal_type()
    }

    pub fn is_core(&self) -> bool {
        self.module.is_core()
    }

    pub fn is_optional(&self) -> bool {
        self.module.is_optional()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use serde::Deserialize;
    use crate::models::journal_event_kind::shared::ship::ship_module::module_class::ModuleClass;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::internal_module::InternalModule;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::ShipInternalModule;

    #[test]
    fn ship_internal_module_test_cases_are_parsed_correctly() {
        let test_cases = [
            ("$int_dronecontrol_collection_size3_class5_name;", ShipInternalModule {
                module: InternalModule::CollectorLimpetController,
                size: 3,
                class: ModuleClass::A,
            }),
            ("int_engine_size5_class5", ShipInternalModule {
                module: InternalModule::Thrusters,
                size: 5,
                class: ModuleClass::A,
            }),
            ("$int_engine_size5_class5_name;", ShipInternalModule {
                module: InternalModule::Thrusters,
                size: 5,
                class: ModuleClass::A,
            }),
            ("$int_supercruiseassist_name;", ShipInternalModule {
                module: InternalModule::SupercruiseAssist,
                size: 1,
                class: ModuleClass::E,
            }),
            ("$int_detailedsurfacescanner_tiny_name;", ShipInternalModule {
                module: InternalModule::DetailedSurfaceScanner,
                size: 1,
                class: ModuleClass::I,
            }),
            ("$int_lifesupport_size4_class1_name;", ShipInternalModule {
                module: InternalModule::LifeSupport,
                size: 4,
                class: ModuleClass::E,
            }),
            ("Int_Hyperdrive_Size5_Class5", ShipInternalModule {
                module: InternalModule::FrameShiftDrive,
                size: 5,
                class: ModuleClass::A,
            }),
        ];

        for (input, expected) in test_cases {
            let result = ShipInternalModule::from_str(input);

            dbg!(&result);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}

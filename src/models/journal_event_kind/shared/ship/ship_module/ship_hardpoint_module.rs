use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_type::HardpointType;
use crate::models::journal_event_kind::shared::ship::ship_slot::hardpoint_size::{HardpointSize, HardpointSizeParseError};

pub mod hardpoint_module;
pub mod hardpoint_mounting;
mod hardpoint_type;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShipHardpointModule {
    pub module: HardpointModule,
    pub mounting: HardpointMounting,
    pub size: HardpointSize,
}

#[derive(Debug, Error)]
pub enum ShipHardpointModuleError {
    #[error("Failed to parse hardpoint module: {0}")]
    FailedToParseHardpointModule(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint mounting: {0}")]
    FailedToParseHardpointMounting(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint size: {0}")]
    FailedToParseHardpointSize(#[from] HardpointSizeParseError),

    #[error("Mounting type cannot be missing when the module size is not 'tiny'")]
    MissingMounting,

    #[error("Failed to parse ship hardpoint module: '{0}'")]
    FailedToParse(String),
}

const HARDPOINT_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$?[hH]pt_(\w+?)(_([gG]imbal|[fG]ixed|[tT]urret))?_([tT]iny|[sS]mall|[mS]edium|[lL]arge|[hH]uge)(_name;)?$"#).unwrap());

impl FromStr for ShipHardpointModule {
    type Err = ShipHardpointModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = HARDPOINT_MODULE_REGEX.captures(s) else {
            return Err(ShipHardpointModuleError::FailedToParse(s.to_string()));
        };

        let module = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| ShipHardpointModuleError::FailedToParseHardpointModule(e))?;

        let size = captures.get(4)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        // Sometimes the mounting is missing when the size is 'tiny'
        let mounting = match captures.get(3) {
            Some(capture) => capture
                .as_str()
                .parse()
                .map_err(|e| ShipHardpointModuleError::FailedToParseHardpointMounting(e)),
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
            size,
        })
    }
}

from_str_deserialize_impl!(ShipHardpointModule);

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
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::{ShipHardpointModule, ShipHardpointModuleError};
    use crate::models::journal_event_kind::shared::ship::ship_slot::hardpoint_size::HardpointSize;

    #[test]
    fn ship_hardpoint_module_test_cases_all_pass() {
        let test_cases = [
            ("$hpt_beamlaser_gimbal_medium_name;", ShipHardpointModule {
                module: HardpointModule::BeamLaser,
                mounting: HardpointMounting::Gimballed,
                size: HardpointSize::Medium,
            }),
            ("$hpt_heatsinklauncher_turret_tiny_name;", ShipHardpointModule {
                module: HardpointModule::HeatsinkLauncher,
                mounting: HardpointMounting::Turreted,
                size: HardpointSize::Tiny,
            }),
            ("Hpt_CausticSinkLauncher_Turret_Tiny", ShipHardpointModule {
                module: HardpointModule::CausticSinkLauncher,
                mounting: HardpointMounting::Turreted,
                size: HardpointSize::Tiny,
            }),
            ("$hpt_chafflauncher_tiny_name;", ShipHardpointModule {
                module: HardpointModule::ChaffLauncher,
                mounting: HardpointMounting::Turreted,
                size: HardpointSize::Tiny,
            })
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

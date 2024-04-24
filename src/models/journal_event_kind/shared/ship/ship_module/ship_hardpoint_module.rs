use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
use crate::models::journal_event_kind::shared::ship::ship_slot::hardpoint_size::{HardpointSize, HardpointSizeParseError};

mod hardpoint_module;
mod hardpoint_mounting;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShipHardpointModule {
    pub module: HardpointModule,
    pub mounting: HardpointMounting,
    pub size: HardpointSize,
}

#[derive(Debug, Error)]
pub enum ShipHardpointModuleParseError {
    #[error("Failed to parse hardpoint module: {0}")]
    FailedToParseHardpointModule(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint mounting: {0}")]
    FailedToParseHardpointMounting(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint size: {0}")]
    FailedToParseHardpointSize(#[from] HardpointSizeParseError),

    #[error("Failed to parse ship hardpoint module: '{0}'")]
    FailedToParse(String),
}

const HARDPOINT_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$?hpt_(\w+)_(gimbal|fixed|turret)_(small|medium|large|huge)(_name;)?$"#).unwrap());

impl FromStr for ShipHardpointModule {
    type Err = ShipHardpointModuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = HARDPOINT_MODULE_REGEX.captures(s) else {
            return Err(ShipHardpointModuleParseError::FailedToParse(s.to_string()));
        };

        let module = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| ShipHardpointModuleParseError::FailedToParseHardpointModule(e))?;

        let mounting = captures.get(2)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| ShipHardpointModuleParseError::FailedToParseHardpointMounting(e))?;

        let size = captures.get(3)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        Ok(ShipHardpointModule {
            module,
            mounting,
            size,
        })
    }
}

from_str_deserialize_impl!(ShipHardpointModule);

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::ShipHardpointModule;

    #[test]
    fn ship_hardpoint_module_test_cases_all_pass() {
        let test_cases = [
            "$hpt_beamlaser_gimbal_medium_name;"
        ];

        for case in test_cases {
            let result = ShipHardpointModule::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}

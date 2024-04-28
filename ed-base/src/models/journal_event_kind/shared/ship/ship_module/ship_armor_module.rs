mod armor_grade;

use std::num::ParseIntError;
use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::ship_module::module_class::{ModuleClass, ModuleClassError};
use crate::models::journal_event_kind::shared::ship::ship_module::ship_armor_module::armor_grade::{ArmorGrade, ArmorGradeError};
use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShipArmorModule {
    pub ship: ShipType,
    pub grade: ArmorGrade,
}

#[derive(Debug, Error)]
pub enum ShipArmorModuleError {
    #[error("Failed to parse ship type: {0}")]
    FailedToParseShipType(#[source] serde_json::Error),

    #[error("Failed to parse armor module: {0}")]
    FailedToParseArmorModule(#[source] serde_json::Error),

    #[error("Failed to parse grade number: {0}")]
    FailedToParseGradeNumber(#[from] ParseIntError),

    #[error(transparent)]
    ArmorGradeError(#[from] ArmorGradeError),

    #[error(transparent)]
    ModuleClassError(#[from] ModuleClassError),

    #[error("Failed to parse armor module: '{0}'")]
    FailedToParse(String),
}

const ARMOR_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$(\w+?)_armour_grade(\d+)_name;$"#).unwrap());

impl FromStr for ShipArmorModule {
    type Err = ShipArmorModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = ARMOR_MODULE_REGEX.captures(s) else {
            return Err(ShipArmorModuleError::FailedToParse(s.to_string()));
        };

        let ship = captures.get(1)
            .expect("Should have already been matched")
            .as_str()
            .parse()
            .map_err(|e| ShipArmorModuleError::FailedToParseShipType(e))?;

        let grade = captures.get(2)
            .expect("Should have already been matched")
            .as_str()
            .parse::<u8>()?
            .try_into()?;

        Ok(ShipArmorModule {
            ship,
            grade,
        })
    }
}

from_str_deserialize_impl!(ShipArmorModule);

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::models::journal_event_kind::shared::ship::ship_module::module_class::ModuleClass;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_armor_module::armor_grade::ArmorGrade;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_armor_module::ShipArmorModule;
    use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;

    #[test]
    fn ship_armor_module_test_cases_are_parsed_correctly() {
        let test_cases = [
            ("$type9_military_armour_grade1_name;", ShipArmorModule {
                ship: ShipType::Type10,
                grade: ArmorGrade::LightweightAlloys,
            }),
            ("$type9_military_armour_grade3_name;", ShipArmorModule {
                ship: ShipType::Type10,
                grade: ArmorGrade::MilitaryGradeComposite,
            }),
        ];

        for (case, expected) in test_cases {
            let result = ShipArmorModule::from_str(case);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}

use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::models::journal_event_content::shared::ship::ship_module::module_class::ModuleClassError;
use crate::models::journal_event_content::shared::ship::ship_module::ship_internal_module::armor_grade::{ArmorGrade, ArmorGradeError};
use crate::models::journal_event_content::shared::ship::ship_type::ShipType;

#[derive(Debug, Clone, PartialEq)]
pub struct ArmorModule {
    pub ship: ShipType,
    pub grade: ArmorGrade,
}

#[derive(Debug, Error)]
pub enum ArmorModuleError {
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

lazy_static! {
    static ref ARMOR_MODULE_REGEX: Regex =
        Regex::new(r#"^\$?(\w+?)_armour_grade(\d+)(_name;)?$"#).unwrap();
}

impl FromStr for ArmorModule {
    type Err = ArmorModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = ARMOR_MODULE_REGEX.captures(s) else {
            return Err(ArmorModuleError::FailedToParse(s.to_string()));
        };

        let ship = captures
            .get(1)
            .expect("Should have already been matched")
            .as_str()
            .parse()
            .map_err(ArmorModuleError::FailedToParseShipType)?;

        let grade = captures
            .get(2)
            .expect("Should have already been matched")
            .as_str()
            .parse::<u8>()?
            .try_into()?;

        Ok(ArmorModule { ship, grade })
    }
}

from_str_deserialize_impl!(ArmorModule);

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//
//     use crate::models::journal_event_kind::shared::ship::ship_module::armor_module::armor_grade::ArmorGrade;
//     use crate::models::journal_event_kind::shared::ship::ship_module::armor_module::ShipArmorModule;
//     use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
//
//     #[test]
//     fn ship_armor_module_test_cases_are_parsed_correctly() {
//         let test_cases = [
//             (
//                 "$type9_military_armour_grade1_name;",
//                 ShipArmorModule {
//                     ship: ShipType::Type10,
//                     grade: ArmorGrade::LightweightAlloys,
//                 },
//             ),
//             (
//                 "$type9_military_armour_grade3_name;",
//                 ShipArmorModule {
//                     ship: ShipType::Type10,
//                     grade: ArmorGrade::MilitaryGradeComposite,
//                 },
//             ),
//             (
//                 "krait_mkii_armour_grade3",
//                 ShipArmorModule {
//                     ship: ShipType::Anaconda,
//                     grade: ArmorGrade::LightweightAlloys,
//                 }
//             ),
//         ];
//
//         for (case, expected) in test_cases {
//             let result = ShipArmorModule::from_str(case);
//
//             assert!(result.is_ok());
//             assert_eq!(result.unwrap(), expected);
//         }
//     }
// }

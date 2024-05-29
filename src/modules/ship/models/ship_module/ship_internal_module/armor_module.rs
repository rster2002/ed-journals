use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::ship::{
    ArmorGrade, ArmorGradeError, ModuleClass, ModuleClassError, ShipType, ShipTypeError,
};

/// Armor module for a specific ship.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ArmorModule {
    pub ship: ShipType,
    pub grade: ArmorGrade,
}

#[derive(Debug, Error)]
pub enum ArmorModuleError {
    #[error("Failed to parse armor module: {0}")]
    FailedToParseArmorModule(#[source] serde_json::Error),

    #[error("Failed to parse grade number: {0}")]
    FailedToParseGradeNumber(#[from] ParseIntError),

    #[error(transparent)]
    ArmorGradeError(#[from] ArmorGradeError),

    #[error(transparent)]
    ModuleClassError(#[from] ModuleClassError),

    #[error(transparent)]
    ShipTypeError(#[from] ShipTypeError),

    #[error("Failed to parse armor module: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref ARMOR_MODULE_REGEX: Regex =
        Regex::new(r#"^\$?(\w+?)_[aA]rmour(_[gG]rade(\d+)|_(\w+))(_name;)?$"#).unwrap();
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
            .parse()?;

        if let Some(capture) = captures.get(3) {
            return Ok(ArmorModule {
                ship,
                grade: capture.as_str().parse::<u8>()?.try_into()?,
            });
        }

        if let Some(capture) = captures.get(4) {
            return Ok(ArmorModule {
                ship,
                grade: capture.as_str().parse()?,
            });
        }

        Err(ArmorModuleError::FailedToParse(s.to_string()))
    }
}

from_str_deserialize_impl!(ArmorModule);

impl Display for ArmorModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let grade: ModuleClass = (&self.grade).into();
        write!(f, "1{} {}", grade, self.grade)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::modules::ship::{ArmorGrade, ArmorModule, ShipType};

    #[test]
    fn ship_armor_module_test_cases_are_parsed_correctly() {
        let test_cases = [
            (
                "$type9_military_armour_grade1_name;",
                ArmorModule {
                    ship: ShipType::Type10Defender,
                    grade: ArmorGrade::LightweightAlloy,
                },
            ),
            (
                "$type9_military_armour_grade3_name;",
                ArmorModule {
                    ship: ShipType::Type10Defender,
                    grade: ArmorGrade::MilitaryGradeComposite,
                },
            ),
            (
                "krait_mkii_armour_grade4",
                ArmorModule {
                    ship: ShipType::KraitMkII,
                    grade: ArmorGrade::MirroredSurfaceComposite,
                },
            ),
            (
                "dolphin_armour_reactive",
                ArmorModule {
                    ship: ShipType::Dolphin,
                    grade: ArmorGrade::ReactiveSurfaceComposite,
                },
            ),
            (
                "empire_courier_armour_grade1",
                ArmorModule {
                    ship: ShipType::ImperialCourier,
                    grade: ArmorGrade::LightweightAlloy,
                },
            ),
            (
                "SideWinder_Armour_Grade1",
                ArmorModule {
                    ship: ShipType::SideWinder,
                    grade: ArmorGrade::LightweightAlloy,
                },
            ),
        ];

        for (case, expected) in test_cases {
            let result = ArmorModule::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn ship_armor_module_is_displayed_correctly() {
        let test_cases = [
            (
                ArmorModule {
                    ship: ShipType::Type10Defender,
                    grade: ArmorGrade::LightweightAlloy,
                },
                "1C Lightweight Alloy",
            ),
            (
                ArmorModule {
                    ship: ShipType::Type10Defender,
                    grade: ArmorGrade::MilitaryGradeComposite,
                },
                "1A Military Grade Composite",
            ),
            (
                ArmorModule {
                    ship: ShipType::KraitMkII,
                    grade: ArmorGrade::MirroredSurfaceComposite,
                },
                "1A Mirrored Surface Composite",
            ),
        ];

        for (case, expected) in test_cases {
            assert_eq!(&case.to_string(), expected);
        }
    }
}

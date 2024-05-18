use std::fmt::{Display, Formatter};
use serde::Serialize;

use thiserror::Error;
use crate::modules::ship::{ArmorGrade, ArmorModule};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ModuleClass {
    A,
    B,
    C,
    D,
    E,
    F,
    I,

    #[cfg(not(feature = "strict"))]
    Unknown,
}

#[derive(Debug, Error)]
pub enum ModuleClassError {
    #[error("Unknown module class: {0}")]
    UnknownModuleClass(u8),
}

impl TryFrom<u8> for ModuleClass {
    type Error = ModuleClassError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(ModuleClass::A),
            4 => Ok(ModuleClass::B),
            3 => Ok(ModuleClass::C),
            2 => Ok(ModuleClass::D),
            1 => Ok(ModuleClass::E),

            #[cfg(not(feature = "strict"))]
            _ => Ok(ModuleClass::Unknown),

            #[cfg(feature = "strict")]
            _ => Err(ModuleClassError::UnknownModuleClass(value)),
        }
    }
}

impl Display for ModuleClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModuleClass::A => "A",
                ModuleClass::B => "B",
                ModuleClass::C => "C",
                ModuleClass::D => "D",
                ModuleClass::E => "E",
                ModuleClass::F => "F",
                ModuleClass::I => "I",

                #[cfg(not(feature = "strict"))]
                ModuleClass::Unknown => "U",
            }
        )
    }
}

impl From<&ArmorGrade> for ModuleClass {
    fn from(value: &ArmorGrade) -> Self {
        match value {
            ArmorGrade::LightweightAlloy => ModuleClass::C,
            ArmorGrade::ReinforcedAlloy => ModuleClass::B,
            ArmorGrade::MilitaryGradeComposite => ModuleClass::A,
            ArmorGrade::MirroredSurfaceComposite => ModuleClass::A,
            ArmorGrade::ReactiveSurfaceComposite => ModuleClass::A,

            #[cfg(not(feature = "strict"))]
            ArmorGrade::Unknown(_) => ModuleClass::Unknown,

            #[cfg(not(feature = "strict"))]
            ArmorGrade::UnknownString(_) => ModuleClass::Unknown,
        }
    }
}

impl From<ArmorGrade> for ModuleClass {
    fn from(value: ArmorGrade) -> Self {
        (&value).into()
    }
}

impl From<&ArmorModule> for ModuleClass {
    fn from(value: &ArmorModule) -> Self {
        (&value.grade).into()
    }
}

impl From<ArmorModule> for ModuleClass {
    fn from(value: ArmorModule) -> Self {
        value.grade.into()
    }
}

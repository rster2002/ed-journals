use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::modules::ship::{ArmorGrade, ArmorModule};

/// The class of a module. Not all classes are available for every module.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ModuleClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown,
}

#[derive(Debug, Error)]
pub enum ModuleClassError {
    #[error("Unknown module class: {0}")]
    UnknownModuleClass(u8),

    #[error("Unknown module class string: '{0}'")]
    UnknownModuleClassString(String),
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

            #[cfg(feature = "allow-unknown")]
            _ => Ok(ModuleClass::Unknown),

            #[cfg(not(feature = "allow-unknown"))]
            _ => Err(ModuleClassError::UnknownModuleClass(value)),
        }
    }
}

impl FromStr for ModuleClass {
    type Err = ModuleClassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(ModuleClass::A),
            "B" | "b" => Ok(ModuleClass::B),
            "C" | "c" => Ok(ModuleClass::C),
            "D" | "d" => Ok(ModuleClass::D),
            "E" | "e" => Ok(ModuleClass::E),
            "F" | "f" => Ok(ModuleClass::F),
            "G" | "g" => Ok(ModuleClass::G),
            "H" | "h" => Ok(ModuleClass::H),
            "I" | "i" => Ok(ModuleClass::I),

            #[cfg(feature = "allow-unknown")]
            _ => Ok(ModuleClass::Unknown),

            #[cfg(not(feature = "allow-unknown"))]
            _ => Err(ModuleClassError::UnknownModuleClassString(s.to_string())),
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
                ModuleClass::G => "G",
                ModuleClass::H => "H",
                ModuleClass::I => "I",

                #[cfg(feature = "allow-unknown")]
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

            #[cfg(feature = "allow-unknown")]
            ArmorGrade::Unknown(_) => ModuleClass::Unknown,

            #[cfg(feature = "allow-unknown")]
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

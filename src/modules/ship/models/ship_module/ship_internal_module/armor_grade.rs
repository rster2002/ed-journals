use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ArmorGrade {
    LightweightAlloy,
    ReinforcedAlloy,
    MilitaryGradeComposite,
    MirroredSurfaceComposite,
    ReactiveSurfaceComposite,

    #[cfg(not(feature = "strict"))]
    Unknown(u8),

    #[cfg(not(feature = "strict"))]
    UnknownString(String),
}

#[derive(Debug, Error)]
pub enum ArmorGradeError {
    #[error("Unknown armor grade: {0}")]
    UnknownArmorGrade(u8),

    #[error("Unknown armor grade string: '{0}'")]
    UnknownArmorGradeString(String),
}

impl TryFrom<u8> for ArmorGrade {
    type Error = ArmorGradeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ArmorGrade::LightweightAlloy),
            2 => Ok(ArmorGrade::ReinforcedAlloy),
            3 => Ok(ArmorGrade::MilitaryGradeComposite),
            4 => Ok(ArmorGrade::MirroredSurfaceComposite),
            5 => Ok(ArmorGrade::ReactiveSurfaceComposite),

            #[cfg(not(feature = "strict"))]
            _ => Ok(ArmorGrade::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(ArmorGradeError::UnknownArmorGrade(value)),
        }
    }
}

impl FromStr for ArmorGrade {
    type Err = ArmorGradeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lightweight" => Ok(ArmorGrade::LightweightAlloy),
            "reinforced" => Ok(ArmorGrade::ReinforcedAlloy),
            "military" => Ok(ArmorGrade::MilitaryGradeComposite),
            "mirrored" => Ok(ArmorGrade::MirroredSurfaceComposite),
            "reactive" => Ok(ArmorGrade::ReactiveSurfaceComposite),

            #[cfg(not(feature = "strict"))]
            _ => Ok(ArmorGrade::UnknownString(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(ArmorGradeError::UnknownArmorGradeString(s.to_string())),
        }
    }
}

impl Display for ArmorGrade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArmorGrade::LightweightAlloy => "Lightweight Alloy",
                ArmorGrade::ReinforcedAlloy => "Reinforced Alloy",
                ArmorGrade::MilitaryGradeComposite => "Military Grade Composite",
                ArmorGrade::MirroredSurfaceComposite => "Mirrored Surface Composite",
                ArmorGrade::ReactiveSurfaceComposite => "Reactive Surface Composite",

                #[cfg(not(feature = "strict"))]
                ArmorGrade::Unknown(_) => "Unknown composite",

                #[cfg(not(feature = "strict"))]
                ArmorGrade::UnknownString(_) => "Unknown composite",
            }
        )
    }
}

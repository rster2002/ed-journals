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

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(u8),

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
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
        Ok(match value {
            1 => ArmorGrade::LightweightAlloy,
            2 => ArmorGrade::ReinforcedAlloy,
            3 => ArmorGrade::MilitaryGradeComposite,
            4 => ArmorGrade::MirroredSurfaceComposite,
            5 => ArmorGrade::ReactiveSurfaceComposite,

            #[cfg(feature = "allow-unknown")]
            _ => ArmorGrade::Unknown(value),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(ArmorGradeError::UnknownArmorGrade(value)),
        })
    }
}

impl FromStr for ArmorGrade {
    type Err = ArmorGradeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: &str = &s.to_ascii_lowercase();

        Ok(match string {
            "lightweight" => ArmorGrade::LightweightAlloy,
            "reinforced" => ArmorGrade::ReinforcedAlloy,
            "military" => ArmorGrade::MilitaryGradeComposite,
            "mirrored" => ArmorGrade::MirroredSurfaceComposite,
            "reactive" => ArmorGrade::ReactiveSurfaceComposite,

            #[cfg(feature = "allow-unknown")]
#[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
            _ => ArmorGrade::UnknownString(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(ArmorGradeError::UnknownArmorGradeString(s.to_string())),
        })
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

                #[cfg(feature = "allow-unknown")]
#[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
                ArmorGrade::Unknown(_) => "Unknown composite",

                #[cfg(feature = "allow-unknown")]
#[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
                ArmorGrade::UnknownString(_) => "Unknown composite",
            }
        )
    }
}

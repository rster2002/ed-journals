use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ArmorGrade {
    LightweightAlloys,
    MilitaryGradeComposite,

    #[cfg(not(feature = "strict"))]
    Unknown(u8),
}

#[derive(Debug, Error)]
pub enum ArmorGradeError {
    #[error("Unknown armor grade: {0}")]
    UnknownArmorGrade(u8),
}

impl TryFrom<u8> for ArmorGrade {
    type Error = ArmorGradeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ArmorGrade::LightweightAlloys),
            3 => Ok(ArmorGrade::MilitaryGradeComposite),

            #[cfg(not(feature = "strict"))]
            _ => Ok(ArmorGrade::Unknown(value)),

            #[cfg(feature = "strict")]
            _ => Err(ArmorGradeError::UnknownArmorGrade(value)),
        }
    }
}

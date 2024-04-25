use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ModuleClass {
    A,
    B,
    C,
    D,
    E,
    I,
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
            _ => Err(ModuleClassError::UnknownModuleClass(value)),
        }
    }
}

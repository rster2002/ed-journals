use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ModuleClass {
    A,
    B,
    C,
    D,
    E,
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
            1 => Ok(ModuleClass::A),
            2 => Ok(ModuleClass::B),
            3 => Ok(ModuleClass::C),
            4 => Ok(ModuleClass::D),
            5 => Ok(ModuleClass::E),
            _ => Err(ModuleClassError::UnknownModuleClass(value)),
        }
    }
}

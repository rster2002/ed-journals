use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum HardpointSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

#[derive(Debug, Error)]
pub enum HardpointSizeParseError {
    #[error("Unknown hardpoint size: {0}")]
    UnknownHardpointSize(String),
}

impl FromStr for HardpointSize {
    type Err = HardpointSizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tiny" => Ok(HardpointSize::Tiny),
            "tiny" => Ok(HardpointSize::Tiny),
            "Small" => Ok(HardpointSize::Small),
            "small" => Ok(HardpointSize::Small),
            "Medium" => Ok(HardpointSize::Medium),
            "medium" => Ok(HardpointSize::Medium),
            "Large" => Ok(HardpointSize::Large),
            "large" => Ok(HardpointSize::Large),
            "Huge" => Ok(HardpointSize::Huge),
            "huge" => Ok(HardpointSize::Huge),
            _ => Err(HardpointSizeParseError::UnknownHardpointSize(s.to_string())),
        }
    }
}

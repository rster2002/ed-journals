use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;

use thiserror::Error;

/// The sizes that hardpoints can have. [HardpointSize::Tiny] is used for utility mounts and the
/// other sizes are used for full-sized modules.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum HardpointSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

impl HardpointSize {
    /// Returns the number for the given size. This is for example used to display the size for
    /// modules.
    pub fn size_nr(&self) -> u8 {
        match self {
            HardpointSize::Tiny => 0,
            HardpointSize::Small => 1,
            HardpointSize::Medium => 2,
            HardpointSize::Large => 3,
            HardpointSize::Huge => 4,
        }
    }

    /// Returns the character that is used to display the size for ship slots.
    pub fn size_char(&self) -> char {
        match self {
            HardpointSize::Tiny => 'T',
            HardpointSize::Small => 'S',
            HardpointSize::Medium => 'M',
            HardpointSize::Large => 'L',
            HardpointSize::Huge => 'H',
        }
    }

    pub fn size_str(&self) -> &'static str {
        match self {
            HardpointSize::Tiny => "Tiny",
            HardpointSize::Small => "Small",
            HardpointSize::Medium => "Medium",
            HardpointSize::Large => "Large",
            HardpointSize::Huge => "Huge",
        }
    }
}

#[derive(Debug, Error)]
pub enum HardpointSizeError {
    #[error("Unknown hardpoint size: {0}")]
    UnknownHardpointSize(String),
}

impl FromStr for HardpointSize {
    type Err = HardpointSizeError;

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
            _ => Err(HardpointSizeError::UnknownHardpointSize(s.to_string())),
        }
    }
}

impl Display for HardpointSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.size_str())
    }
}

use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexPlanetEntry {
    AmmoniaWorld,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexPlanetEntry {
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexPlanetEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexPlanetError {
    #[error("Failed to parse planet codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown planet codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexPlanetEntry {
    type Err = CodexPlanetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CodexPlanetError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "ammonia_world" => CodexPlanetEntry::AmmoniaWorld,

            #[cfg(feature = "allow-unknown")]
            _ => CodexPlanetEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexPlanetError::UnknownEntry(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(CodexPlanetEntry);

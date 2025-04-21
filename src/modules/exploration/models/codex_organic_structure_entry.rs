use crate::exploration::shared::codex_regex::CODEX_REGEX;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// Codex entries related to organic structures other than the ones already covered by
/// [crate::exobiology::Genus], [crate::exobiology::Species] and [crate::exobiology::Variant].
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum CodexOrganicStructureEntry {
    StolonTree,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexOrganicStructureEntry {
    /// Whether the current variant is unknown.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexOrganicStructureEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexOrganicStructureError {
    #[error("Failed to parse planet codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown planet codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexOrganicStructureEntry {
    type Err = CodexOrganicStructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CodexOrganicStructureError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "l_seed_sdrt02_v3" => CodexOrganicStructureEntry::StolonTree,

            #[cfg(feature = "allow-unknown")]
            _ => CodexOrganicStructureEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexOrganicStructureError::UnknownEntry(string.to_string())),
        })
    }
}

impl Display for CodexOrganicStructureEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CodexOrganicStructureEntry::StolonTree => "Stolon Tree",

                #[cfg(feature = "allow-unknown")]
                CodexOrganicStructureEntry::Unknown(unknown) =>
                    return write!(f, "Unknown organic structure codex entry: {}", unknown),
            }
        )
    }
}

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexGuardianEntry {
    AncientCasket,
    AncientKey,
    AncientOrb,
    AncientRelic,
    AncientRelics,
    AncientTablet,
    AncientTotem,
    AncientUrn,

    GuardianBeacons,
    GuardianDataBank,
    GuardianDataLogs,
    GuardianPad,
    GuardianPylon,
    GuardianSentinel,
    GuardianTerminal,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexGuardianEntry {
    #[cfg(feature = "allow-unknown")]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexGuardianEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexGuardianEntryError {
    #[error("Failed to parse guardian codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown guardian codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexGuardianEntry {
    type Err = CodexGuardianEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CodexGuardianEntryError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "ancient_casket" => CodexGuardianEntry::AncientCasket,
            "ancient_key" => CodexGuardianEntry::AncientKey,
            "ancient_orb" => CodexGuardianEntry::AncientOrb,
            "ancient_relic" => CodexGuardianEntry::AncientRelic,
            "ancient_relics" => CodexGuardianEntry::AncientRelics,
            "ancient_tablet" => CodexGuardianEntry::AncientTablet,
            "ancient_totem" => CodexGuardianEntry::AncientTotem,
            "ancient_urn" => CodexGuardianEntry::AncientUrn,

            "guardian_beacons" => CodexGuardianEntry::GuardianBeacons,
            "guardian_data_bank" => CodexGuardianEntry::GuardianDataBank,
            "guardian_data_logs" => CodexGuardianEntry::GuardianDataLogs,
            "guardian_pad" => CodexGuardianEntry::GuardianPad,
            "guardian_pylon" => CodexGuardianEntry::GuardianPylon,
            "guardian_sentinel" => CodexGuardianEntry::GuardianSentinel,
            "guardian_terminal" => CodexGuardianEntry::GuardianTerminal,

            #[cfg(feature = "allow-unknown")]
            _ => CodexGuardianEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexGuardianEntryError::UnknownEntry(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(CodexGuardianEntry);

impl Display for CodexGuardianEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexGuardianEntry::AncientCasket => "Ancient Casket",
            CodexGuardianEntry::AncientKey => "Ancient Key",
            CodexGuardianEntry::AncientOrb => "Ancient Orb",
            CodexGuardianEntry::AncientRelic => "Ancient Relic",
            CodexGuardianEntry::AncientRelics => "Ancient Relics",
            CodexGuardianEntry::AncientTablet => "Ancient Tablet",
            CodexGuardianEntry::AncientTotem => "Ancient Totem",
            CodexGuardianEntry::AncientUrn => "Ancient Urn",

            CodexGuardianEntry::GuardianBeacons => "Guardian Beacons",
            CodexGuardianEntry::GuardianDataBank => "Guardian Data Bank",
            CodexGuardianEntry::GuardianDataLogs => "Guardian Data Logs",
            CodexGuardianEntry::GuardianPad => "Guardian Pad",
            CodexGuardianEntry::GuardianPylon => "Guardian Pylon",
            CodexGuardianEntry::GuardianSentinel => "Guardian Sentinel",
            CodexGuardianEntry::GuardianTerminal => "Guardian Terminal",

            #[cfg(feature = "allow-unknown")]
            CodexGuardianEntry::Unknown(unknown) => return write!(f, "Unknown guardian codex entry: {}", unknown),
        })
    }
}

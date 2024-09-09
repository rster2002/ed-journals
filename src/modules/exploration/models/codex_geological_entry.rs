use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::exploration::StarClassCodexEntryError;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexGeologicalEntry {
    // IceFumarole,
    // Geyser,
    // IceGeyser,
    // LavaSpout,
    // GasVent,

    Fumarole,
    FumaroleAmmoniaGeysers,
    FumaroleCarbondioxideGeysers,
    FumaroleHeliumGeysers,
    FumaroleMethaneGeysers,
    FumaroleNitrogenGeysers,
    FumaroleSilicateVapourGeysers,
    FumaroleSulfurDioxideMagma,
    FumaroleWaterGeysers,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexGeologicalEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexGeologicalEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexGeologicalError {
    #[error("Failed to parse geological codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown geological codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexGeologicalEntry {
    type Err = CodexGeologicalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CodexGeologicalError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "fumarole" => CodexGeologicalEntry::Fumarole,
            "fumarole_ammoniageysers" => CodexGeologicalEntry::FumaroleAmmoniaGeysers,
            "fumarole_carbondioxidegeysers" => CodexGeologicalEntry::FumaroleCarbondioxideGeysers,
            "fumarole_heliumgeysers" => CodexGeologicalEntry::FumaroleHeliumGeysers,
            "fumarole_methanegeysers" => CodexGeologicalEntry::FumaroleMethaneGeysers,
            "fumarole_nitrogengeysers" => CodexGeologicalEntry::FumaroleNitrogenGeysers,
            "fumarole_silicatevapourgeysers" => CodexGeologicalEntry::FumaroleSilicateVapourGeysers,
            "fumarole_sulphurdioxidemagma" => CodexGeologicalEntry::FumaroleSulfurDioxideMagma,
            "fumarole_watergeysers" => CodexGeologicalEntry::FumaroleWaterGeysers,

            #[cfg(feature = "allow-unknown")]
            _ => CodexGeologicalEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexGeologicalError::UnknownEntry(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(CodexGeologicalEntry);

impl Display for CodexGeologicalEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            // CodexGeologicalEntry::Fumarole => "Fumarole",
            // CodexGeologicalEntry::IceFumarole => "Ice Fumarole",
            // CodexGeologicalEntry::Geyser => "Geyser",
            // CodexGeologicalEntry::IceGeyser => "Ice Geyser",
            // CodexGeologicalEntry::LavaSpout => "Lava Spout",
            // CodexGeologicalEntry::GasVent => "Gas Vent",
            CodexGeologicalEntry::Fumarole => "Fumarole",
            CodexGeologicalEntry::FumaroleAmmoniaGeysers => "Fumarole Ammonia Geysers",
            CodexGeologicalEntry::FumaroleCarbondioxideGeysers => "Fumarole Carbondioxide Geysers",
            CodexGeologicalEntry::FumaroleHeliumGeysers => "Fumarole Helium Geysers",
            CodexGeologicalEntry::FumaroleMethaneGeysers => "Fumarole MethaneGeysers",
            CodexGeologicalEntry::FumaroleNitrogenGeysers => "Fumarole Nitrogen Geysers",
            CodexGeologicalEntry::FumaroleSilicateVapourGeysers => "Fumarole Silicate VapourGeysers",
            CodexGeologicalEntry::FumaroleSulfurDioxideMagma => "Fumarole Sulfur Dioxide Magma",
            CodexGeologicalEntry::FumaroleWaterGeysers => "Fumarole Water Geysers",

            #[cfg(feature = "allow-unknown")]
            CodexGeologicalEntry::Unknown(unknown) => return write!(f, "Unknown geological codex entry: {}", unknown),
        })
    }
}

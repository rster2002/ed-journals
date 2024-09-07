use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;

/// New-type used for parsing star class codex entries.
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexStarClassEntry {
    ATypes,
    ATypeStar,
    ATypeGiant,
    ATypeSupergiant,
    ATypeHypergiant,

    AeBeTypes,
    AeBeTypeStar,
    AeBeTypeGiant,
    AeBeTypeSupergiant,
    AeBeTypeHypergiant,


    OTypeStar,
    BTypeStar,
    FTypeStar,
    GTypeStar,
    KTypeStar,
    MTypeStar,
    LTypeStar,
    TTypeStar,
    TTauriTypeStar,
    YTypeStar,
    CNTypeStar,
    CJTypeStar,
    MSTypeStar,
    STypeStar,
    WolfRayetTypeStar,
    WhiteDwarf,
    NeutronStar,
    BlackHole,
    SupermassiveBlackHole,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

/// Enum for errors that occur when parsing a star class codex entry.
#[derive(Debug, Error)]
pub enum StarClassCodexEntryError {
    #[error("Failed to parse star class codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown star class codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexStarClassEntry {
    type Err = StarClassCodexEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(StarClassCodexEntryError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "a_types" => CodexStarClassEntry::ATypes,
            "a_type" => CodexStarClassEntry::ATypeStar,
            "a_typegiant" => CodexStarClassEntry::ATypeGiant,
            "a_typesupergiant" => CodexStarClassEntry::ATypeSupergiant,
            "a_typehypergiant" => CodexStarClassEntry::ATypeHypergiant,

            "aebe_types" => CodexStarClassEntry::AeBeTypes,
            "aebe_type" => CodexStarClassEntry::AeBeTypeStar,
            "aebe_typegiant" => CodexStarClassEntry::AeBeTypeGiant,
            "aebe_typesupergiant" => CodexStarClassEntry::AeBeTypeSupergiant,
            "aebe_typehypergiant" => CodexStarClassEntry::AeBeTypeHypergiant,

            "o_type" => CodexStarClassEntry::OTypeStar,
            "b_type" => CodexStarClassEntry::BTypeStar,
            "f_type" => CodexStarClassEntry::FTypeStar,
            "g_type" => CodexStarClassEntry::GTypeStar,
            "k_type" => CodexStarClassEntry::KTypeStar,
            "m_type" => CodexStarClassEntry::MTypeStar,
            "l_type" => CodexStarClassEntry::LTypeStar,
            "t_type" => CodexStarClassEntry::TTypeStar,
            // T Tauri Type
            "y_type" => CodexStarClassEntry::YTypeStar,
            "cn_type" => CodexStarClassEntry::CNTypeStar,
            "cj_type" => CodexStarClassEntry::CJTypeStar,
            "ms_type" => CodexStarClassEntry::MSTypeStar,
            "s_type" => CodexStarClassEntry::STypeStar,

            // Wolf-Rayet Star
            // White Dwarf star
            // Neutron star
            // Black hole
            // Supermassive black hole

            "neutron_stars" => CodexStarClassEntry::NeutronStar,

            #[cfg(feature = "allow-unknown")]
            _ => CodexStarClassEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(StarClassCodexEntryError::UnknownEntry(s.to_string())),
        })
        // let Some(captures) = STAR_CLASS_CODEX_ENTRY_REGEX.captures(s) else {
        //     return Err(StarClassCodexEntryError::FailedToParse(s.to_string()));
        // };
        //
        // Ok(StarClassCodexEntry(
        //     captures
        //         .get(1)
        //         .expect("Should have been captured already")
        //         .as_str()
        //         .parse()?,
        // ))
    }
}

from_str_deserialize_impl!(CodexStarClassEntry);

impl Display for CodexStarClassEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexStarClassEntry::ATypes => "A Type Stars",
            CodexStarClassEntry::ATypeStar => "A Type Star",
            CodexStarClassEntry::ATypeGiant => "A Type Giant",
            CodexStarClassEntry::ATypeSupergiant => "A Type Supergiant",
            CodexStarClassEntry::ATypeHypergiant => "A Type Hypergiant",

            CodexStarClassEntry::AeBeTypes => "AeBe Type Stars",
            CodexStarClassEntry::AeBeTypeStar => "AeBe Type Star",
            CodexStarClassEntry::AeBeTypeGiant => "AeBe Type Giant",
            CodexStarClassEntry::AeBeTypeSupergiant => "AeBe Type Supergiant",
            CodexStarClassEntry::AeBeTypeHypergiant => "AeBe Type Hypergiant",

            CodexStarClassEntry::OTypeStar => "O Type Star",
            CodexStarClassEntry::BTypeStar => "B Type Star",
            CodexStarClassEntry::FTypeStar => "F Type Star",
            CodexStarClassEntry::GTypeStar => "G Type Star",
            CodexStarClassEntry::KTypeStar => "K Type Star",
            CodexStarClassEntry::MTypeStar => "M Type Star",
            CodexStarClassEntry::LTypeStar => "L Type Star",
            CodexStarClassEntry::TTypeStar => "T Type Star",
            CodexStarClassEntry::TTauriTypeStar => "T Tauri TypeStar",
            CodexStarClassEntry::YTypeStar => "Y TypeStar",
            CodexStarClassEntry::CNTypeStar => "CN TypeStar",
            CodexStarClassEntry::CJTypeStar => "CJ TypeStar",
            CodexStarClassEntry::MSTypeStar => "MS TypeStar",
            CodexStarClassEntry::STypeStar => "S TypeStar",
            CodexStarClassEntry::WolfRayetTypeStar => "Wolf-Rayet Type Star",
            CodexStarClassEntry::WhiteDwarf => "White Dwarf",
            CodexStarClassEntry::NeutronStar => "Neutron Star",
            CodexStarClassEntry::BlackHole => "Black Hole",
            CodexStarClassEntry::SupermassiveBlackHole => "Supermassive Black Hole",
        })
    }
}

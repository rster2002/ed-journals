use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

/// New-type used for parsing star class codex entries.
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum StarClassCodexEntry {
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
}

/// Enum for errors that occur when parsing a star class codex entry.
#[derive(Debug, Error)]
pub enum StarClassCodexEntryError {
    #[error("Failed to parse star class codex entry: '{0}'")]
    FailedToParse(String),
}

impl FromStr for StarClassCodexEntry {
    type Err = StarClassCodexEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: &str = &s.to_ascii_lowercase();

        Ok(match string {
            "$codex_ent_a_types_name;" => StarClassCodexEntry::ATypes,
            "$codex_ent_a_type_name;" => StarClassCodexEntry::ATypeStar,
            "$codex_ent_a_typegiant_name;" => StarClassCodexEntry::ATypeGiant,
            "$codex_ent_a_typesupergiant_name;" => StarClassCodexEntry::ATypeSupergiant,
            "$codex_ent_a_typehypergiant_name;" => StarClassCodexEntry::ATypeHypergiant,

            "$codex_ent_aebe_types_name;" => StarClassCodexEntry::AeBeTypes,
            "$codex_ent_aebe_type_name;" => StarClassCodexEntry::AeBeTypeStar,
            "$codex_ent_aebe_typegiant_name;" => StarClassCodexEntry::AeBeTypeGiant,
            "$codex_ent_aebe_typesupergiant_name;" => StarClassCodexEntry::AeBeTypeSupergiant,
            "$codex_ent_aebe_typehypergiant_name;" => StarClassCodexEntry::AeBeTypeHypergiant,

            "$codex_ent_o_type_name;" => StarClassCodexEntry::OTypeStar,
            "$codex_ent_b_type_name;" => StarClassCodexEntry::BTypeStar,
            "$codex_ent_f_type_name;" => StarClassCodexEntry::FTypeStar,
            "$codex_ent_g_type_name;" => StarClassCodexEntry::GTypeStar,
            "$codex_ent_k_type_name;" => StarClassCodexEntry::KTypeStar,
            "$codex_ent_m_type_name;" => StarClassCodexEntry::MTypeStar,
            "$codex_ent_l_type_name;" => StarClassCodexEntry::LTypeStar,
            "$codex_ent_t_type_name;" => StarClassCodexEntry::TTypeStar,
            // T Tauri Type
            "$codex_ent_y_type_name;" => StarClassCodexEntry::YTypeStar,
            "$codex_ent_cn_type_name;" => StarClassCodexEntry::CNTypeStar,
            "$codex_ent_cj_type_name;" => StarClassCodexEntry::CJTypeStar,
            "$codex_ent_ms_type_name;" => StarClassCodexEntry::MSTypeStar,
            "$codex_ent_s_type_name;" => StarClassCodexEntry::STypeStar,

            // Wolf-Rayet Star
            // White Dwarf star
            // Neutron star
            // Black hole
            // Supermassive black hole

            "$codex_ent_neutron_stars_name;" => StarClassCodexEntry::NeutronStar,
            _ => return Err(StarClassCodexEntryError::FailedToParse(s.to_string())),
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

impl Display for StarClassCodexEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StarClassCodexEntry::ATypes => "A Type Stars",
            StarClassCodexEntry::ATypeStar => "A Type Star",
            StarClassCodexEntry::ATypeGiant => "A Type Giant",
            StarClassCodexEntry::ATypeSupergiant => "A Type Supergiant",
            StarClassCodexEntry::ATypeHypergiant => "A Type Hypergiant",

            StarClassCodexEntry::AeBeTypes => "AeBe Type Stars",
            StarClassCodexEntry::AeBeTypeStar => "AeBe Type Star",
            StarClassCodexEntry::AeBeTypeGiant => "AeBe Type Giant",
            StarClassCodexEntry::AeBeTypeSupergiant => "AeBe Type Supergiant",
            StarClassCodexEntry::AeBeTypeHypergiant => "AeBe Type Hypergiant",

            StarClassCodexEntry::OTypeStar => "O Type Star",
            StarClassCodexEntry::BTypeStar => "B Type Star",
            StarClassCodexEntry::FTypeStar => "F Type Star",
            StarClassCodexEntry::GTypeStar => "G Type Star",
            StarClassCodexEntry::KTypeStar => "K Type Star",
            StarClassCodexEntry::MTypeStar => "M Type Star",
            StarClassCodexEntry::LTypeStar => "L Type Star",
            StarClassCodexEntry::TTypeStar => "T Type Star",
            StarClassCodexEntry::TTauriTypeStar => "T Tauri TypeStar",
            StarClassCodexEntry::YTypeStar => "Y TypeStar",
            StarClassCodexEntry::CNTypeStar => "CN TypeStar",
            StarClassCodexEntry::CJTypeStar => "CJ TypeStar",
            StarClassCodexEntry::MSTypeStar => "MS TypeStar",
            StarClassCodexEntry::STypeStar => "S TypeStar",
            StarClassCodexEntry::WolfRayetTypeStar => "Wolf-Rayet Type Star",
            StarClassCodexEntry::WhiteDwarf => "White Dwarf",
            StarClassCodexEntry::NeutronStar => "Neutron Star",
            StarClassCodexEntry::BlackHole => "Black Hole",
            StarClassCodexEntry::SupermassiveBlackHole => "Supermassive Black Hole",
        })
    }
}

from_str_deserialize_impl!(StarClassCodexEntry);

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

/// New-type used for parsing star class codex entries.
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum StarClassCodexEntry {
    OTypeStar,
    BTypeStar,
    ATypeStar,
    FTypeStar,
    GTypeStar,
    KTypeStar,
    MTypeStar,
    LTypeStar,
    TTypeStar,
    TTauriTypeStar,
    AeBeTypeStar,
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
        Ok(match s {
            "$Codex_Ent_O_Type_Name;" => StarClassCodexEntry::OTypeStar,
            "$Codex_Ent_B_Type_Name;" => StarClassCodexEntry::BTypeStar,
            "$Codex_Ent_A_Type_Name;" => StarClassCodexEntry::ATypeStar,
            "$Codex_Ent_F_Type_Name;" => StarClassCodexEntry::FTypeStar,
            "$Codex_Ent_G_Type_Name;" => StarClassCodexEntry::GTypeStar,
            "$Codex_Ent_K_Type_Name;" => StarClassCodexEntry::KTypeStar,
            "$Codex_Ent_M_Type_Name;" => StarClassCodexEntry::MTypeStar,
            "$Codex_Ent_L_Type_Name;" => StarClassCodexEntry::LTypeStar,
            "$Codex_Ent_T_Type_Name;" => StarClassCodexEntry::TTypeStar,
            // T Tauri Type
            "$Codex_Ent_AeBe_Type_Name;" => StarClassCodexEntry::AeBeTypeStar,
            "$Codex_Ent_Y_Type_Name;" => StarClassCodexEntry::YTypeStar,
            "$Codex_Ent_CN_Type_Name;" => StarClassCodexEntry::CNTypeStar,
            "$Codex_Ent_CJ_Type_Name;" => StarClassCodexEntry::CJTypeStar,
            "$Codex_Ent_MS_Type_Name;" => StarClassCodexEntry::MSTypeStar,
            "$Codex_Ent_S_Type_Name;" => StarClassCodexEntry::STypeStar,

            // Wolf-Rayet Star
            // White Dwarf star
            // Neutron star
            // Black hole
            // Supermassive black hole

            "$Codex_Ent_Neutron_Stars_Name;" => StarClassCodexEntry::NeutronStar,
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
            StarClassCodexEntry::OTypeStar => "O Type Star",
            StarClassCodexEntry::BTypeStar => "B Type Star",
            StarClassCodexEntry::ATypeStar => "A Type Star",
            StarClassCodexEntry::FTypeStar => "F Type Star",
            StarClassCodexEntry::GTypeStar => "G Type Star",
            StarClassCodexEntry::KTypeStar => "K Type Star",
            StarClassCodexEntry::MTypeStar => "M Type Star",
            StarClassCodexEntry::LTypeStar => "L Type Star",
            StarClassCodexEntry::TTypeStar => "T Type Star",
            StarClassCodexEntry::TTauriTypeStar => "T Tauri TypeStar",
            StarClassCodexEntry::AeBeTypeStar => "AeBe TypeStar",
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

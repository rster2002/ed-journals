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

    BTypes,
    BTypeStar,
    BTypeGiant,
    BTypeSupergiant,
    BTypeHypergiant,

    CTypes,
    CTypeStar,
    CTypeGiant,
    CTypeSupergiant,
    CTypeHypergiant,

    CHTypes,
    CHTypeStar,
    CHTypeGiant,
    CHTypeSupergiant,
    CHTypeHypergiant,

    CHDTypes,
    CHDTypeStar,
    CHDTypeGiant,
    CHDTypeSupergiant,
    CHDTypeHypergiant,

    CJTypes,
    CJTypeStar,
    CJTypeGiant,
    CJTypeSupergiant,
    CJTypeHypergiant,

    CNTypes,
    CNTypeStar,
    CNTypeGiant,
    CNTypeSupergiant,
    CNTypeHypergiant,

    CSTypes,
    CSTypeStar,
    CSTypeGiant,
    CSTypeSupergiant,
    CSTypeHypergiant,

    DType,

    DAType,
    DATypeHypergiant,

    DABType,
    DAOType,
    DAVType,
    DAZType,
    DBType,
    DBVType,
    DBZType,
    DCType,
    DCVType,

    DOType,
    DOVType,
    DQType,
    DXType,

    FTypes,
    FTypeStar,
    FTypeGiant,
    FTypeSupergiant,
    FTypeHypergiant,

    GTypes,
    GTypeStar,
    GTypeGiant,
    GTypeSupergiant,
    GTypeHypergiant,

    OTypeStar,
    KTypeStar,
    MTypeStar,
    LTypeStar,
    TTypeStar,
    TTauriTypeStar,
    YTypeStar,
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

impl CodexStarClassEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexStarClassEntry::Unknown(_))
    }
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

            "b_types" => CodexStarClassEntry::BTypes,
            "b_type" => CodexStarClassEntry::BTypeStar,
            "b_typegiant" => CodexStarClassEntry::BTypeGiant,
            "b_typesupergiant" => CodexStarClassEntry::BTypeSupergiant,
            "b_typehypergiant" => CodexStarClassEntry::BTypeHypergiant,

            "c_types" => CodexStarClassEntry::CTypes,
            "c_type" => CodexStarClassEntry::CTypeStar,
            "c_typegiant" => CodexStarClassEntry::CTypeGiant,
            "c_typesupergiant" => CodexStarClassEntry::CTypeSupergiant,
            "c_typehypergiant" => CodexStarClassEntry::CTypeHypergiant,

            "ch_types" => CodexStarClassEntry::CHTypes,
            "ch_type" => CodexStarClassEntry::CHTypeStar,
            "ch_typegiant" => CodexStarClassEntry::CHTypeGiant,
            "ch_typesupergiant" => CodexStarClassEntry::CHTypeSupergiant,
            "ch_typehypergiant" => CodexStarClassEntry::CHTypeHypergiant,

            "chd_types" => CodexStarClassEntry::CHDTypes,
            "chd_type" => CodexStarClassEntry::CHDTypeStar,
            "chd_typegiant" => CodexStarClassEntry::CHDTypeGiant,
            "chd_typesupergiant" => CodexStarClassEntry::CHDTypeSupergiant,
            "chd_typehypergiant" => CodexStarClassEntry::CHDTypeHypergiant,

            "cj_types" => CodexStarClassEntry::CJTypes,
            "cj_type" => CodexStarClassEntry::CJTypeStar,
            "cj_typegiant" => CodexStarClassEntry::CJTypeGiant,
            "cj_typesupergiant" => CodexStarClassEntry::CJTypeSupergiant,
            "cj_typehypergiant" => CodexStarClassEntry::CJTypeHypergiant,

            "cn_types" => CodexStarClassEntry::CNTypes,
            "cn_type" => CodexStarClassEntry::CNTypeStar,
            "cn_typegiant" => CodexStarClassEntry::CNTypeGiant,
            "cn_typesupergiant" => CodexStarClassEntry::CNTypeSupergiant,
            "cn_typehypergiant" => CodexStarClassEntry::CNTypeHypergiant,

            "cs_types" => CodexStarClassEntry::CSTypes,
            "cs_type" => CodexStarClassEntry::CSTypeStar,
            "cs_typegiant" => CodexStarClassEntry::CSTypeGiant,
            "cs_typesupergiant" => CodexStarClassEntry::CSTypeSupergiant,
            "cs_typehypergiant" => CodexStarClassEntry::CSTypeHypergiant,

            "d_type" => CodexStarClassEntry::DType,

            "da_type" => CodexStarClassEntry::DAType,
            "da_typehypergiant" => CodexStarClassEntry::DATypeHypergiant,
            "dab_type" => CodexStarClassEntry::DABType,
            "dao_type" => CodexStarClassEntry::DAOType,
            "dav_type" => CodexStarClassEntry::DAVType,
            "daz_type" => CodexStarClassEntry::DAZType,
            "db_type" => CodexStarClassEntry::DBType,
            "dbv_type" => CodexStarClassEntry::DBVType,
            "dbz_type" => CodexStarClassEntry::DBZType,
            "dc_type" => CodexStarClassEntry::DCType,
            "dcv_type" => CodexStarClassEntry::DCVType,

            "do_type" => CodexStarClassEntry::DOType,
            "dov_type" => CodexStarClassEntry::DOVType,
            "dq_type" => CodexStarClassEntry::DQType,
            "dx_type" => CodexStarClassEntry::DXType,

            "f_types" => CodexStarClassEntry::FTypes,
            "f_type" => CodexStarClassEntry::FTypeStar,
            "f_typegiant" => CodexStarClassEntry::FTypeGiant,
            "f_typesupergiant" => CodexStarClassEntry::FTypeSupergiant,
            "f_typehypergiant" => CodexStarClassEntry::FTypeHypergiant,

            "g_types" => CodexStarClassEntry::GTypes,
            "g_type" => CodexStarClassEntry::GTypeStar,
            "g_typegiant" => CodexStarClassEntry::GTypeGiant,
            "g_typesupergiant" => CodexStarClassEntry::GTypeSupergiant,
            "g_typehypergiant" => CodexStarClassEntry::GTypeHypergiant,

            "o_type" => CodexStarClassEntry::OTypeStar,
            "k_type" => CodexStarClassEntry::KTypeStar,
            "m_type" => CodexStarClassEntry::MTypeStar,
            "l_type" => CodexStarClassEntry::LTypeStar,
            "t_type" => CodexStarClassEntry::TTypeStar,
            // T Tauri Type
            "y_type" => CodexStarClassEntry::YTypeStar,
            "ms_type" => CodexStarClassEntry::MSTypeStar,
            "s_type" => CodexStarClassEntry::STypeStar,

            // Wolf-Rayet Star
            // White Dwarf star
            // Neutron star
            // Black hole
            // Supermassive black hole

            "black_holes" => CodexStarClassEntry::BlackHole,
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

            CodexStarClassEntry::BTypes => "B Type Stars",
            CodexStarClassEntry::BTypeStar => "B Type Star",
            CodexStarClassEntry::BTypeGiant => "B Type Giant",
            CodexStarClassEntry::BTypeSupergiant => "B Type Supergiant",
            CodexStarClassEntry::BTypeHypergiant => "B Type Hypergiant",

            CodexStarClassEntry::CTypes => "C Type Stars",
            CodexStarClassEntry::CTypeStar => "C Type Star",
            CodexStarClassEntry::CTypeGiant => "C Type Giant",
            CodexStarClassEntry::CTypeSupergiant => "C Type Supergiant",
            CodexStarClassEntry::CTypeHypergiant => "C Type Hypergiant",

            CodexStarClassEntry::CHTypes => "CH Type Stars",
            CodexStarClassEntry::CHTypeStar => "CH Type Star",
            CodexStarClassEntry::CHTypeGiant => "CH Type Giant",
            CodexStarClassEntry::CHTypeSupergiant => "CH Type Supergiant",
            CodexStarClassEntry::CHTypeHypergiant => "CH Type Hypergiant",

            CodexStarClassEntry::CHDTypes => "CHD Type Stars",
            CodexStarClassEntry::CHDTypeStar => "CHD Type Star",
            CodexStarClassEntry::CHDTypeGiant => "CHD Type Giant",
            CodexStarClassEntry::CHDTypeSupergiant => "CHD Type Supergiant",
            CodexStarClassEntry::CHDTypeHypergiant => "CHD Type Hypergiant",

            CodexStarClassEntry::CJTypes => "CJ Type Stars",
            CodexStarClassEntry::CJTypeStar => "CJ Type Star",
            CodexStarClassEntry::CJTypeGiant => "CJ Type Giant",
            CodexStarClassEntry::CJTypeSupergiant => "CJ Type Supergiant",
            CodexStarClassEntry::CJTypeHypergiant => "CJ Type Hypergiant",

            CodexStarClassEntry::CNTypes => "CN Type Stars",
            CodexStarClassEntry::CNTypeStar => "CN Type Star",
            CodexStarClassEntry::CNTypeGiant => "CN Type Giant",
            CodexStarClassEntry::CNTypeSupergiant => "CN Type Supergiant",
            CodexStarClassEntry::CNTypeHypergiant => "CN Type Hypergiant",

            CodexStarClassEntry::CSTypes => "CS Type Stars",
            CodexStarClassEntry::CSTypeStar => "CS Type Star",
            CodexStarClassEntry::CSTypeGiant => "CS Type Giant",
            CodexStarClassEntry::CSTypeSupergiant => "CS Type Supergiant",
            CodexStarClassEntry::CSTypeHypergiant => "CS Type Hypergiant",

            CodexStarClassEntry::DType => "D Type Star",

            CodexStarClassEntry::DAType => "DA Type",
            CodexStarClassEntry::DATypeHypergiant => "DA Type Hypergiant",
            CodexStarClassEntry::DABType => "DAB Type",
            CodexStarClassEntry::DAOType => "DAO Type",
            CodexStarClassEntry::DAVType => "DAV Type",
            CodexStarClassEntry::DAZType => "DAZ Type",
            CodexStarClassEntry::DBType => "DB Type",
            CodexStarClassEntry::DBVType => "DBV Type",
            CodexStarClassEntry::DBZType => "DBZ Type",
            CodexStarClassEntry::DCType => "DC Type",
            CodexStarClassEntry::DCVType => "CV Type",

            CodexStarClassEntry::DOType => "DO Type",
            CodexStarClassEntry::DOVType => "DOV Type",
            CodexStarClassEntry::DQType => "DQ Type",
            CodexStarClassEntry::DXType => "DX Type",

            CodexStarClassEntry::FTypes => "F Type Stars",
            CodexStarClassEntry::FTypeStar => "F Type Star",
            CodexStarClassEntry::FTypeGiant => "F Type Giant",
            CodexStarClassEntry::FTypeSupergiant => "F Type Supergiant",
            CodexStarClassEntry::FTypeHypergiant => "F Type Hypergiant",

            CodexStarClassEntry::GTypes => "G Type Stars",
            CodexStarClassEntry::GTypeStar => "G Type Star",
            CodexStarClassEntry::GTypeGiant => "G Type Giant",
            CodexStarClassEntry::GTypeSupergiant => "G Type Supergiant",
            CodexStarClassEntry::GTypeHypergiant => "G Type Hypergiant",

            CodexStarClassEntry::OTypeStar => "O Type Star",
            CodexStarClassEntry::KTypeStar => "K Type Star",
            CodexStarClassEntry::MTypeStar => "M Type Star",
            CodexStarClassEntry::LTypeStar => "L Type Star",
            CodexStarClassEntry::TTypeStar => "T Type Star",
            CodexStarClassEntry::TTauriTypeStar => "T Tauri TypeStar",
            CodexStarClassEntry::YTypeStar => "Y TypeStar",
            CodexStarClassEntry::MSTypeStar => "MS TypeStar",
            CodexStarClassEntry::STypeStar => "S TypeStar",
            CodexStarClassEntry::WolfRayetTypeStar => "Wolf-Rayet Type Star",
            CodexStarClassEntry::WhiteDwarf => "White Dwarf",
            CodexStarClassEntry::NeutronStar => "Neutron Star",
            CodexStarClassEntry::BlackHole => "Black Hole",
            CodexStarClassEntry::SupermassiveBlackHole => "Supermassive Black Hole",

            #[cfg(feature = "allow-unknown")]
            CodexStarClassEntry::Unknown(unknown) => return write!(f, "Unknown star codex entry: {}", unknown),
        })
    }
}

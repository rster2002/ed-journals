use std::str::FromStr;
use serde::Deserialize;
use crate::from_str_deserialize_impl;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Genus {
    #[serde(rename = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,

    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    CrystallineShards,

    #[serde(rename = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,

    #[serde(rename = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,

    #[serde(rename = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,

    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,

    #[serde(rename = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,

    #[serde(rename = "$Codex_Ent_Shrubs_Genus_Name;")]
    Frutexa,

    #[serde(rename = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,

    #[serde(rename = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,

    #[serde(rename = "$Codex_Ent_Clypeus_Genus_Name;")]
    Clypeus,

    #[serde(rename = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,

    #[serde(rename = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,

    #[serde(rename = "$Codex_Ent_Fonticulus_Genus_Name;")]
    Fonticulua,

    #[serde(rename = "$Codex_Ent_Electricae_Genus_Name;")]
    Electricae,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

// impl FromStr for Genus {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "$Codex_Ent_Stratum_Genus_Name;" => Ok(Genus::Stratum),
//             "$Codex_Ent_Ground_Struct_Ice_Name;" => Ok(Genus::CrystallineShards),
//             "$Codex_Ent_Bacterial_Genus_Name;" => Ok(Genus::Bacterium),
//             "$Codex_Ent_Aleoids_Genus_Name;" => Ok(Genus::Aleoida),
//             "$Codex_Ent_Conchas_Genus_Name;" => Ok(Genus::Concha),
//             "$Codex_Ent_Fungoids_Genus_Name;" => Ok(Genus::Fungoida),
//             "$Codex_Ent_Osseus_Genus_Name;" => Ok(Genus::Osseus),
//             "$Codex_Ent_Shrubs_Genus_Name;" => Ok(Genus::Frutexa),
//             "$Codex_Ent_Tussocks_Genus_Name;" => Ok(Genus::Tussock),
//             "$Codex_Ent_Cactoid_Genus_Name;" => Ok(Genus::Cactoida),
//             "$Codex_Ent_Clypeus_Genus_Name;" => Ok(Genus::Clypeus),
//             "$Codex_Ent_Recepta_Genus_Name;" => Ok(Genus::Recepta),
//
//             #[cfg(not(feature = "strict"))]
//             _ => Ok(Genus::Unknown(s.to_string())),
//
//             #[cfg(feature = "strict")]
//             _ => Err(s.to_string()),
//         }
//     }
// }
//
// from_str_deserialize_impl!(Genus);

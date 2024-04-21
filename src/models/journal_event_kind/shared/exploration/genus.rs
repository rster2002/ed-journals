use std::str::FromStr;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Genus {
    Stratum,
    CrystallineShards,
    Bacterium,
    Aleoida,
    Concha,
    Fungoida,
    Osseus,
    Frutexa,
    Tussock,
    Cactoida,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

impl FromStr for Genus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$Codex_Ent_Stratum_Genus_Name;" => Ok(Genus::Stratum),
            "$Codex_Ent_Ground_Struct_Ice_Name;" => Ok(Genus::CrystallineShards),
            "$Codex_Ent_Bacterial_Genus_Name;" => Ok(Genus::Bacterium),
            "$Codex_Ent_Aleoids_Genus_Name;" => Ok(Genus::Aleoida),
            "$Codex_Ent_Conchas_Genus_Name;" => Ok(Genus::Concha),
            "$Codex_Ent_Fungoids_Genus_Name;" => Ok(Genus::Fungoida),
            "$Codex_Ent_Osseus_Genus_Name;" => Ok(Genus::Osseus),
            "$Codex_Ent_Shrubs_Genus_Name;" => Ok(Genus::Frutexa),
            "$Codex_Ent_Tussocks_Genus_Name;" => Ok(Genus::Tussock),
            "$Codex_Ent_Cactoid_Genus_Name;" => Ok(Genus::Cactoida),

            #[cfg(not(feature = "strict"))]
            _ => Ok(Genus::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(s.to_string()),
        }
    }
}

from_str_deserialize_impl!(Genus);

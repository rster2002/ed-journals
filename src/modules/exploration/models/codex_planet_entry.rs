use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexPlanetEntry {
    EarthLike,
    AmmoniaWorlds,
    WaterWorlds,
    WaterGiant,
    WaterGiantWithLife,
    SudarskyClassI,
    SudarskyClassII,
    SudarskyClassIII,
    SudarskyClassIV,
    SudarskyClassV,
    SupermassiveBlackHoles,

    DenseAmmoniaWorlds,
    DenseWaterWorlds,
    DenseTerraformableHighMetalContent,
    DenseTerraformableIce,
    DenseTerraformableMetalRich,
    DenseTerraformableRockyIce,
    DenseTerraformableRocky,

    StandardGiants,
    StandardPlanetStandard,
    StandardPlanetTerraformable,

    StandardAmmoniaWorlds,
    StandardGiantWithAmmoniaLife,
    StandardGiantWithWaterLife,
    StandardHelium,
    StandardHeliumRich,
    StandardHighMetalContentNoAtmosphere,
    StandardIceNoAtmosphere,
    StandardMetalRichNoAtmosphere,
    StandardRockyIceNoAtmosphere,
    StandardRockyNoAtmosphere,
    StandardSudarskyClassI,
    StandardSudarskyClassII,
    StandardSudarskyClassIII,
    StandardSudarskyClassIV,
    StandardSudarskyClassV,
    StandardWaterGiant,
    StandardWaterGiantWithLife,
    StandardWaterWorlds,

    StandardTerraformableHighMetalContent,
    StandardTerraformableIce,
    StandardTerraformableMetalRich,
    StandardTerraformableRockyIce,
    StandardTerraformableRocky,

    LightAmmoniaWorlds,
    LightWaterWorlds,
    LightTerraformableHighMetalContent,
    LightTerraformableIce,
    LightTerraformableMetalRich,
    LightTerraformableRockyIce,
    LightTerraformableRocky,

    GiantWithAmmoniaLife,
    GiantWithWaterLife,

    GreenGiants,
    GreenGiantWithAmmoniaLife,
    GreenGiantWithWaterLife,
    GreenSudarskyClassI,
    GreenSudarskyClassII,
    GreenSudarskyClassIII,
    GreenSudarskyClassIV,
    GreenSudarskyClassV,
    GreenWaterGiant,
    GreenWaterGiantWithLife,

    // TODO $codex_ent_trf_... entries. Not sure what they even are

    TerraformableHighMetalContent,
    TerraformableIce,
    TerraformableMetalRich,
    TerraformableRockyIce,
    TerraformableRocky,

    MetalRichNoAtmosphere,
    HighMetalContentNoAtmosphere,
    IceNoAtmosphere,
    RockyIceNoAtmosphere,
    RockyNoAtmosphere,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexPlanetEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
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
            "earth_likes" => CodexPlanetEntry::EarthLike,
            "ammonia_worlds" => CodexPlanetEntry::AmmoniaWorlds,
            "water_worlds" => CodexPlanetEntry::WaterWorlds,
            "water_giant" => CodexPlanetEntry::WaterGiant,
            "water_giant_with_life" => CodexPlanetEntry::WaterGiantWithLife,
            "sudarsky_class_i" => CodexPlanetEntry::SudarskyClassI,
            "sudarsky_class_ii" => CodexPlanetEntry::SudarskyClassII,
            "sudarsky_class_iii" => CodexPlanetEntry::SudarskyClassIII,
            "sudarsky_class_iv" => CodexPlanetEntry::SudarskyClassIV,
            "sudarsky_class_v" => CodexPlanetEntry::SudarskyClassV,
            "supermassiveblack_holes" => CodexPlanetEntry::SupermassiveBlackHoles,

            "dense_ammonia_worlds" => CodexPlanetEntry::DenseAmmoniaWorlds,
            "dense_water_worlds" => CodexPlanetEntry::DenseWaterWorlds,
            "dense_ter_high_metal_content" => CodexPlanetEntry::DenseTerraformableHighMetalContent,
            "dense_ter_ice" => CodexPlanetEntry::DenseTerraformableIce,
            "dense_ter_metal_rich" => CodexPlanetEntry::DenseTerraformableMetalRich,
            "dense_ter_rocky_ice" => CodexPlanetEntry::DenseTerraformableRockyIce,
            "dense_ter_rocky" => CodexPlanetEntry::DenseTerraformableRocky,

            "standard_ammonia_worlds" => CodexPlanetEntry::StandardAmmoniaWorlds,
            "standard_giant_with_ammonia_life" => CodexPlanetEntry::StandardGiantWithAmmoniaLife,
            "standard_giant_with_water_life" => CodexPlanetEntry::StandardGiantWithWaterLife,
            "standard_giants" => CodexPlanetEntry::StandardGiants,
            "standard_helium" => CodexPlanetEntry::StandardHelium,
            "standard_helium_rich" => CodexPlanetEntry::StandardHeliumRich,
            "standard_high_metal_content_no_atmos" => CodexPlanetEntry::StandardHighMetalContentNoAtmosphere,
            "standard_ice_no_atmos" => CodexPlanetEntry::StandardIceNoAtmosphere,
            "standard_metal_rich_no_atmos" => CodexPlanetEntry::StandardMetalRichNoAtmosphere,
            "standard_planetstandard" => CodexPlanetEntry::StandardPlanetStandard,
            "standard_planetterraf" => CodexPlanetEntry::StandardPlanetTerraformable,
            "standard_rocky_ice_no_atmos" => CodexPlanetEntry::StandardRockyIceNoAtmosphere,
            "standard_rocky_no_atmos" => CodexPlanetEntry::StandardRockyNoAtmosphere,
            "standard_sudarsky_class_i" => CodexPlanetEntry::StandardSudarskyClassI,
            "standard_sudarsky_class_ii" => CodexPlanetEntry::StandardSudarskyClassII,
            "standard_sudarsky_class_iii" => CodexPlanetEntry::StandardSudarskyClassIII,
            "standard_sudarsky_class_iv" => CodexPlanetEntry::StandardSudarskyClassIV,
            "standard_sudarsky_class_v" => CodexPlanetEntry::StandardSudarskyClassV,
            "standard_ter_high_metal_content" => CodexPlanetEntry::StandardTerraformableHighMetalContent,
            "standard_ter_ice" => CodexPlanetEntry::StandardTerraformableIce,
            "standard_ter_metal_rich" => CodexPlanetEntry::StandardTerraformableMetalRich,
            "standard_ter_rocky_ice" => CodexPlanetEntry::StandardTerraformableRockyIce,
            "standard_ter_rocky" => CodexPlanetEntry::StandardTerraformableRocky,
            "standard_water_giant" => CodexPlanetEntry::StandardWaterGiant,
            "standard_water_giant_with_life" => CodexPlanetEntry::StandardWaterGiantWithLife,
            "standard_water_worlds" => CodexPlanetEntry::StandardWaterWorlds,

            "light_ammonia_worlds" => CodexPlanetEntry::LightAmmoniaWorlds,
            "light_water_worlds" => CodexPlanetEntry::LightWaterWorlds,
            "light_ter_high_metal_content" => CodexPlanetEntry::LightTerraformableHighMetalContent,
            "light_ter_ice" => CodexPlanetEntry::LightTerraformableIce,
            "light_ter_metal_rich" => CodexPlanetEntry::LightTerraformableMetalRich,
            "light_ter_rocky_ice" => CodexPlanetEntry::LightTerraformableRockyIce,
            "light_ter_rocky" => CodexPlanetEntry::LightTerraformableRocky,

            "giant_with_ammonia_life" => CodexPlanetEntry::GiantWithAmmoniaLife,
            "giant_with_water_life" => CodexPlanetEntry::GiantWithWaterLife,

            "greengiants" => CodexPlanetEntry::GreenGiants,
            "green_giant_with_ammonia_life" => CodexPlanetEntry::GreenGiantWithAmmoniaLife,
            "green_giant_with_water_life" => CodexPlanetEntry::GreenGiantWithWaterLife,
            "green_sudarsky_class_i" => CodexPlanetEntry::GreenSudarskyClassI,
            "green_sudarsky_class_ii" => CodexPlanetEntry::GreenSudarskyClassII,
            "green_sudarsky_class_iii" => CodexPlanetEntry::GreenSudarskyClassIII,
            "green_sudarsky_class_iv" => CodexPlanetEntry::GreenSudarskyClassIV,
            "green_sudarsky_class_v" => CodexPlanetEntry::GreenSudarskyClassV,
            "green_water_giant" => CodexPlanetEntry::GreenWaterGiant,
            "green_water_giant_with_life" => CodexPlanetEntry::GreenWaterGiantWithLife,

            "ter_high_metal_content" => CodexPlanetEntry::TerraformableHighMetalContent,
            "ter_ice" => CodexPlanetEntry::TerraformableIce,
            "ter_metal_rich" => CodexPlanetEntry::TerraformableMetalRich,
            "ter_rocky_ice" => CodexPlanetEntry::TerraformableRockyIce,
            "ter_rocky" => CodexPlanetEntry::TerraformableRocky,

            "metal_rich_no_atmos" => CodexPlanetEntry::MetalRichNoAtmosphere,
            "high_metal_content_no_atmos" => CodexPlanetEntry::HighMetalContentNoAtmosphere,
            "ice_no_atmos" => CodexPlanetEntry::IceNoAtmosphere,
            "rocky_ice_no_atmos" => CodexPlanetEntry::RockyIceNoAtmosphere,
            "rocky_no_atmos" => CodexPlanetEntry::RockyNoAtmosphere,

            #[cfg(feature = "allow-unknown")]
            _ => CodexPlanetEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexPlanetError::UnknownEntry(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(CodexPlanetEntry);

impl Display for CodexPlanetEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexPlanetEntry::EarthLike => "earth_like",
            CodexPlanetEntry::AmmoniaWorlds => "Ammonia World",
            CodexPlanetEntry::WaterWorlds => "Water Worlds",
            CodexPlanetEntry::WaterGiant => "Water Giant",
            CodexPlanetEntry::WaterGiantWithLife => "Water Giant with Life",
            CodexPlanetEntry::SudarskyClassI => "Sudarsky Class I",
            CodexPlanetEntry::SudarskyClassII => "Sudarsky Class II",
            CodexPlanetEntry::SudarskyClassIII => "Sudarsky Class III",
            CodexPlanetEntry::SudarskyClassIV => "Sudarsky Class IV",
            CodexPlanetEntry::SudarskyClassV => "Sudarsky Class V",
            CodexPlanetEntry::SupermassiveBlackHoles => "Supermassive Black Holes",

            CodexPlanetEntry::DenseAmmoniaWorlds => "Dense Ammonia World",
            CodexPlanetEntry::DenseWaterWorlds => "Dense Water Worlds",
            CodexPlanetEntry::DenseTerraformableHighMetalContent => "Dense Terraformable High Metal Content Planet",
            CodexPlanetEntry::DenseTerraformableIce => "Dense Terraformable Ice Planet",
            CodexPlanetEntry::DenseTerraformableMetalRich => "Dense Terraformable Metal-Rich Planet",
            CodexPlanetEntry::DenseTerraformableRockyIce => "Dense Terraformable Rocky Ice Planet",
            CodexPlanetEntry::DenseTerraformableRocky => "Dense Terraformable Rocky Planet",

            CodexPlanetEntry::StandardAmmoniaWorlds => "Standard Ammonia Worlds",
            CodexPlanetEntry::StandardGiantWithAmmoniaLife => "Standard Giant with Ammonia Life",
            CodexPlanetEntry::StandardGiantWithWaterLife => "Standard Giant with Water Life",
            CodexPlanetEntry::StandardGiants => "Standard Giants",
            CodexPlanetEntry::StandardHelium => "Standard Helium",
            CodexPlanetEntry::StandardHeliumRich => "Standard Helium-Rich",
            CodexPlanetEntry::StandardHighMetalContentNoAtmosphere => "Standard High Metal Content, No Atmosphere",
            CodexPlanetEntry::StandardIceNoAtmosphere => "Standard Ice, No Atmosphere",
            CodexPlanetEntry::StandardMetalRichNoAtmosphere => "Standard Metal-Rich, No Atmosphere",
            CodexPlanetEntry::StandardPlanetStandard => "Standard Planet Standard",
            CodexPlanetEntry::StandardPlanetTerraformable => "Standard Planet Terraformable",
            CodexPlanetEntry::StandardRockyIceNoAtmosphere => "Standard Rocky Ice, No Atmosphere",
            CodexPlanetEntry::StandardRockyNoAtmosphere => "Standard Rocky, No Atmosphere",
            CodexPlanetEntry::StandardSudarskyClassI => "Standard Sudarsky Class I",
            CodexPlanetEntry::StandardSudarskyClassII => "Standard Sudarsky Class II",
            CodexPlanetEntry::StandardSudarskyClassIII => "Standard Sudarsky Class III",
            CodexPlanetEntry::StandardSudarskyClassIV => "Standard Sudarsky Class IV",
            CodexPlanetEntry::StandardSudarskyClassV => "Standard Sudarsky Class V",
            CodexPlanetEntry::StandardTerraformableHighMetalContent => "Standard Terraformable High Metal Content",
            CodexPlanetEntry::StandardTerraformableIce => "Standard Terraformable Ice",
            CodexPlanetEntry::StandardTerraformableMetalRich => "Standard Terraformable Metal-Rich",
            CodexPlanetEntry::StandardTerraformableRockyIce => "Standard Terraformable Rocky Ice",
            CodexPlanetEntry::StandardTerraformableRocky => "Standard Terraformable Rocky",
            CodexPlanetEntry::StandardWaterGiant => "Standard Water Giant",
            CodexPlanetEntry::StandardWaterGiantWithLife => "Standard Water Giant with Life",
            CodexPlanetEntry::StandardWaterWorlds => "Standard Water Worlds",

            CodexPlanetEntry::LightAmmoniaWorlds => "Light Ammonia Worlds",
            CodexPlanetEntry::LightWaterWorlds => "Light Water Worlds",
            CodexPlanetEntry::LightTerraformableHighMetalContent => "Light Terraformable High Metal Content Planet",
            CodexPlanetEntry::LightTerraformableIce => "Light Terraformable Ice Planet",
            CodexPlanetEntry::LightTerraformableMetalRich => "Light Terraformable Metal-Rich Planet",
            CodexPlanetEntry::LightTerraformableRockyIce => "Light Terraformable Rocky Ice Planet",
            CodexPlanetEntry::LightTerraformableRocky => "Light Terraformable Rocky Planet",

            CodexPlanetEntry::GreenGiants => "Green Giants",
            CodexPlanetEntry::GiantWithAmmoniaLife => "Giant with Ammonia Life",
            CodexPlanetEntry::GiantWithWaterLife => "Giant with Water Life",
            CodexPlanetEntry::GreenGiantWithAmmoniaLife => "Green Giant with Ammonia Life",
            CodexPlanetEntry::GreenGiantWithWaterLife => "Green Giant with Water Life",
            CodexPlanetEntry::GreenSudarskyClassI => "Green Sudarsky Class I",
            CodexPlanetEntry::GreenSudarskyClassII => "Green Sudarsky Class II",
            CodexPlanetEntry::GreenSudarskyClassIII => "Green Sudarsky Class III",
            CodexPlanetEntry::GreenSudarskyClassIV => "Green Sudarsky Class IV",
            CodexPlanetEntry::GreenSudarskyClassV => "Green Sudarsky Class V",
            CodexPlanetEntry::GreenWaterGiant => "Green Water Giant",
            CodexPlanetEntry::GreenWaterGiantWithLife => "Green Water Giant with Life",

            CodexPlanetEntry::TerraformableHighMetalContent => "Terraformable High Metal Content Planet",
            CodexPlanetEntry::TerraformableIce => "Terraformable Ice Planet",
            CodexPlanetEntry::TerraformableMetalRich => "Terraformable Metal-Rich Planet",
            CodexPlanetEntry::TerraformableRockyIce => "Terraformable Rocky Ice Planet",
            CodexPlanetEntry::TerraformableRocky => "Terraformable Rocky Planet",

            CodexPlanetEntry::MetalRichNoAtmosphere => "Metal-Rich, No Atmosphere",
            CodexPlanetEntry::HighMetalContentNoAtmosphere => "High Metal Content Planet, No Atmosphere",
            CodexPlanetEntry::IceNoAtmosphere => "Ice Planet, No Atmosphere",
            CodexPlanetEntry::RockyIceNoAtmosphere => "Rocky Ice, No Atmosphere",
            CodexPlanetEntry::RockyNoAtmosphere => "Rocky, No Atmosphere",

            #[cfg(feature = "allow-unknown")]
            CodexPlanetEntry::Unknown(unknown) => return write!(f, "Unknown planet codex entry: {}", unknown),
        })
    }
}

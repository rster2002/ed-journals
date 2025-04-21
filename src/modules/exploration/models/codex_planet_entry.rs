use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// Codex entries related to planet types.
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
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
    Helium,
    HeliumRich,

    // Unspecified terrestrial planets
    HighMetalContent,
    Ice,
    MetalRich,
    RockyIce,
    Rocky,

    // Unspecified no atmosphere planets
    MetalRichNoAtmosphere,
    HighMetalContentNoAtmosphere,
    IceNoAtmosphere,
    RockyIceNoAtmosphere,
    RockyNoAtmosphere,

    // Dense non-terrestrial planets
    DenseAmmoniaWorlds,
    DenseWaterWorlds,

    // Dense terrestrial planets
    DenseHighMetalContent,
    DenseIce,
    DenseMetalRich,
    DenseRockyIce,
    DenseRocky,

    // Standard groups
    StandardGiants,
    StandardWaterWorlds,
    StandardPlanetStandard,
    StandardPlanetTerraformable,

    // Standard non-terrestrial planets
    StandardAmmoniaWorlds,
    StandardGiantWithAmmoniaLife,
    StandardGiantWithWaterLife,
    StandardHelium,
    StandardHeliumRich,
    StandardWaterGiant,
    StandardWaterGiantWithLife,

    // Standard planets without an atmosphere
    StandardHighMetalContentNoAtmosphere,
    StandardIceNoAtmosphere,
    StandardMetalRichNoAtmosphere,
    StandardRockyIceNoAtmosphere,
    StandardRockyNoAtmosphere,

    // Standard Sudarsky classes
    StandardSudarskyClassI,
    StandardSudarskyClassII,
    StandardSudarskyClassIII,
    StandardSudarskyClassIV,
    StandardSudarskyClassV,

    // Standaard terrestrials
    StandardHighMetalContent,
    StandardIce,
    StandardMetalRich,
    StandardRockyIce,
    StandardRocky,

    // Light non-terrestrial planets
    LightAmmoniaWorlds,
    LightWaterWorlds,

    // Light terrestrial planets
    LightHighMetalContent,
    LightIce,
    LightMetalRich,
    LightRockyIce,
    LightRocky,

    // Giants with life
    GiantWithAmmoniaLife,
    GiantWithWaterLife,

    // Green giants
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
    GreenHelium,
    GreenHeliumRich,

    // Terraformable non-terrestrial planets
    TerraformableAmmoniaWorlds,
    TerraformableWaterWorlds,

    // Terraformable no atmosphere planets
    TerraformableHighMetalContentNoAtmosphere,
    TerraformableIceNoAtmosphere,
    TerraformableMetalRichNoAtmosphere,
    TerraformableRockyIceNoAtmosphere,
    TerraformableRockyNoAtmosphere,

    // Terraformable terrestrial planets
    TerraformableHighMetalContent,
    TerraformableIce,
    TerraformableMetalRich,
    TerraformableRockyIce,
    TerraformableRocky,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexPlanetEntry {
    /// Whether the current variant is unknown.
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

        let string: &str = &captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        // TODO TER might actually stand for 'terrestrial' and TRF might stand for 'terraformable'.
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
            "helium" => CodexPlanetEntry::Helium,
            "helium_rich" => CodexPlanetEntry::HeliumRich,

            // Unspecified terrestrial planets
            "ter_high_metal_content" => CodexPlanetEntry::HighMetalContent,
            "ter_ice" => CodexPlanetEntry::Ice,
            "ter_metal_rich" => CodexPlanetEntry::MetalRich,
            "ter_rocky_ice" => CodexPlanetEntry::RockyIce,
            "ter_rocky" => CodexPlanetEntry::Rocky,

            // Unspecified no atmosphere planets
            "metal_rich_no_atmos" => CodexPlanetEntry::MetalRichNoAtmosphere,
            "high_metal_content_no_atmos" => CodexPlanetEntry::HighMetalContentNoAtmosphere,
            "ice_no_atmos" => CodexPlanetEntry::IceNoAtmosphere,
            "rocky_ice_no_atmos" => CodexPlanetEntry::RockyIceNoAtmosphere,
            "rocky_no_atmos" => CodexPlanetEntry::RockyNoAtmosphere,

            // Dense non-terrestrial planets
            "dense_ammonia_worlds" => CodexPlanetEntry::DenseAmmoniaWorlds,
            "dense_water_worlds" => CodexPlanetEntry::DenseWaterWorlds,

            // Dense terrestrial planets
            "dense_ter_high_metal_content" => CodexPlanetEntry::DenseHighMetalContent,
            "dense_ter_ice" => CodexPlanetEntry::DenseIce,
            "dense_ter_metal_rich" => CodexPlanetEntry::DenseMetalRich,
            "dense_ter_rocky_ice" => CodexPlanetEntry::DenseRockyIce,
            "dense_ter_rocky" => CodexPlanetEntry::DenseRocky,

            // Standard groups
            "standard_giants" => CodexPlanetEntry::StandardGiants,
            "standard_planetstandard" => CodexPlanetEntry::StandardPlanetStandard,
            "standard_planetterraf" => CodexPlanetEntry::StandardPlanetTerraformable,
            "standard_water_worlds" => CodexPlanetEntry::StandardWaterWorlds,

            // Standard non-terrestrial planets
            "standard_ammonia_worlds" => CodexPlanetEntry::StandardAmmoniaWorlds,
            "standard_giant_with_ammonia_life" => CodexPlanetEntry::StandardGiantWithAmmoniaLife,
            "standard_giant_with_water_life" => CodexPlanetEntry::StandardGiantWithWaterLife,
            "standard_helium" => CodexPlanetEntry::StandardHelium,
            "standard_helium_rich" => CodexPlanetEntry::StandardHeliumRich,
            "standard_water_giant" => CodexPlanetEntry::StandardWaterGiant,
            "standard_water_giant_with_life" => CodexPlanetEntry::StandardWaterGiantWithLife,

            // Standard planets without an atmosphere
            "standard_high_metal_content_no_atmos" => {
                CodexPlanetEntry::StandardHighMetalContentNoAtmosphere
            }
            "standard_ice_no_atmos" => CodexPlanetEntry::StandardIceNoAtmosphere,
            "standard_metal_rich_no_atmos" => CodexPlanetEntry::StandardMetalRichNoAtmosphere,
            "standard_rocky_ice_no_atmos" => CodexPlanetEntry::StandardRockyIceNoAtmosphere,
            "standard_rocky_no_atmos" => CodexPlanetEntry::StandardRockyNoAtmosphere,

            // Standard Sudarsky classes
            "standard_sudarsky_class_i" => CodexPlanetEntry::StandardSudarskyClassI,
            "standard_sudarsky_class_ii" => CodexPlanetEntry::StandardSudarskyClassII,
            "standard_sudarsky_class_iii" => CodexPlanetEntry::StandardSudarskyClassIII,
            "standard_sudarsky_class_iv" => CodexPlanetEntry::StandardSudarskyClassIV,
            "standard_sudarsky_class_v" => CodexPlanetEntry::StandardSudarskyClassV,

            // Standaard terrestrials
            "standard_ter_high_metal_content" => CodexPlanetEntry::StandardHighMetalContent,
            "standard_ter_ice" => CodexPlanetEntry::StandardIce,
            "standard_ter_metal_rich" => CodexPlanetEntry::StandardMetalRich,
            "standard_ter_rocky_ice" => CodexPlanetEntry::StandardRockyIce,
            "standard_ter_rocky" => CodexPlanetEntry::StandardRocky,

            // Light non-terrestrial planets
            "light_ammonia_worlds" => CodexPlanetEntry::LightAmmoniaWorlds,
            "light_water_worlds" => CodexPlanetEntry::LightWaterWorlds,

            // Light terrestrial planets
            "light_ter_high_metal_content" => CodexPlanetEntry::LightHighMetalContent,
            "light_ter_ice" => CodexPlanetEntry::LightIce,
            "light_ter_metal_rich" => CodexPlanetEntry::LightMetalRich,
            "light_ter_rocky_ice" => CodexPlanetEntry::LightRockyIce,
            "light_ter_rocky" => CodexPlanetEntry::LightRocky,

            // Giants with life
            "giant_with_ammonia_life" => CodexPlanetEntry::GiantWithAmmoniaLife,
            "giant_with_water_life" => CodexPlanetEntry::GiantWithWaterLife,

            // Green giants
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
            "green_helium" => CodexPlanetEntry::GreenHelium,
            "green_helium_rich" => CodexPlanetEntry::GreenHeliumRich,

            // Terraformable non-terrestrial planets
            "trf_ammonia_worlds" => CodexPlanetEntry::TerraformableAmmoniaWorlds,
            "trf_water_worlds" => CodexPlanetEntry::TerraformableWaterWorlds,

            // Terraformable no atmosphere planets
            "trf_high_metal_content_no_atmos" => {
                CodexPlanetEntry::TerraformableHighMetalContentNoAtmosphere
            }
            "trf_ice_no_atmos" => CodexPlanetEntry::TerraformableIceNoAtmosphere,
            "trf_metal_rich_no_atmos" => CodexPlanetEntry::TerraformableMetalRichNoAtmosphere,
            "trf_rocky_ice_no_atmos" => CodexPlanetEntry::TerraformableRockyIceNoAtmosphere,
            "trf_rocky_no_atmos" => CodexPlanetEntry::TerraformableRockyNoAtmosphere,

            // Terraformable terrestrial planets
            "trf_ter_high_metal_content" => CodexPlanetEntry::TerraformableHighMetalContent,
            "trf_ter_ice" => CodexPlanetEntry::TerraformableIce,
            "trf_ter_metal_rich" => CodexPlanetEntry::TerraformableMetalRich,
            "trf_ter_rocky_ice" => CodexPlanetEntry::TerraformableRockyIce,
            "trf_ter_rocky" => CodexPlanetEntry::TerraformableRocky,

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
        write!(
            f,
            "{}",
            match self {
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
                CodexPlanetEntry::Helium => "Helium Planet",
                CodexPlanetEntry::HeliumRich => "Helium-Rich Planet",

                CodexPlanetEntry::DenseAmmoniaWorlds => "Dense Ammonia World",
                CodexPlanetEntry::DenseWaterWorlds => "Dense Water Worlds",
                CodexPlanetEntry::DenseHighMetalContent => "Dense High Metal Content Planet",
                CodexPlanetEntry::DenseIce => "Dense Ice Planet",
                CodexPlanetEntry::DenseMetalRich => "Dense Metal-Rich Planet",
                CodexPlanetEntry::DenseRockyIce => "Dense Rocky Ice Planet",
                CodexPlanetEntry::DenseRocky => "Dense Rocky Planet",

                CodexPlanetEntry::StandardAmmoniaWorlds => "Standard Ammonia Worlds",
                CodexPlanetEntry::StandardGiantWithAmmoniaLife =>
                    "Standard Giant with Ammonia Life",
                CodexPlanetEntry::StandardGiantWithWaterLife => "Standard Giant with Water Life",
                CodexPlanetEntry::StandardGiants => "Standard Giants",
                CodexPlanetEntry::StandardHelium => "Standard Helium",
                CodexPlanetEntry::StandardHeliumRich => "Standard Helium-Rich",
                CodexPlanetEntry::StandardHighMetalContentNoAtmosphere =>
                    "Standard High Metal Content, No Atmosphere",
                CodexPlanetEntry::StandardIceNoAtmosphere => "Standard Ice, No Atmosphere",
                CodexPlanetEntry::StandardMetalRichNoAtmosphere =>
                    "Standard Metal-Rich, No Atmosphere",
                CodexPlanetEntry::StandardPlanetStandard => "Standard Planet Standard",
                CodexPlanetEntry::StandardPlanetTerraformable => "Standard Planet Terraformable",
                CodexPlanetEntry::StandardRockyIceNoAtmosphere =>
                    "Standard Rocky Ice, No Atmosphere",
                CodexPlanetEntry::StandardRockyNoAtmosphere => "Standard Rocky, No Atmosphere",

                CodexPlanetEntry::StandardSudarskyClassI => "Standard Sudarsky Class I",
                CodexPlanetEntry::StandardSudarskyClassII => "Standard Sudarsky Class II",
                CodexPlanetEntry::StandardSudarskyClassIII => "Standard Sudarsky Class III",
                CodexPlanetEntry::StandardSudarskyClassIV => "Standard Sudarsky Class IV",
                CodexPlanetEntry::StandardSudarskyClassV => "Standard Sudarsky Class V",

                CodexPlanetEntry::StandardHighMetalContent => "Standard High Metal Content",
                CodexPlanetEntry::StandardIce => "Standard Ice",
                CodexPlanetEntry::StandardMetalRich => "Standard Metal-Rich",
                CodexPlanetEntry::StandardRockyIce => "Standard Rocky Ice",
                CodexPlanetEntry::StandardRocky => "Standard Rocky",

                CodexPlanetEntry::StandardWaterGiant => "Standard Water Giant",
                CodexPlanetEntry::StandardWaterGiantWithLife => "Standard Water Giant with Life",
                CodexPlanetEntry::StandardWaterWorlds => "Standard Water Worlds",

                CodexPlanetEntry::LightAmmoniaWorlds => "Light Ammonia Worlds",
                CodexPlanetEntry::LightWaterWorlds => "Light Water Worlds",
                CodexPlanetEntry::LightHighMetalContent => "Light High Metal Content Planet",
                CodexPlanetEntry::LightIce => "Light Ice Planet",
                CodexPlanetEntry::LightMetalRich => "Light Metal-Rich Planet",
                CodexPlanetEntry::LightRockyIce => "Light Rocky Ice Planet",
                CodexPlanetEntry::LightRocky => "Light Rocky Planet",

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
                CodexPlanetEntry::GreenHelium => "Green Helium Planet",
                CodexPlanetEntry::GreenHeliumRich => "Green Helium-Rich Planet",

                CodexPlanetEntry::HighMetalContent => "High Metal Content Planet",
                CodexPlanetEntry::Ice => "Ice Planet",
                CodexPlanetEntry::MetalRich => "Metal-Rich Planet",
                CodexPlanetEntry::RockyIce => "Rocky Ice Planet",
                CodexPlanetEntry::Rocky => "Rocky Planet",

                CodexPlanetEntry::MetalRichNoAtmosphere => "Metal-Rich, No Atmosphere",
                CodexPlanetEntry::HighMetalContentNoAtmosphere =>
                    "High Metal Content Planet, No Atmosphere",
                CodexPlanetEntry::IceNoAtmosphere => "Ice Planet, No Atmosphere",
                CodexPlanetEntry::RockyIceNoAtmosphere => "Rocky Ice Planet, No Atmosphere",
                CodexPlanetEntry::RockyNoAtmosphere => "Rocky Planet, No Atmosphere",

                // Terraformable non-terrestrial planets
                CodexPlanetEntry::TerraformableAmmoniaWorlds => "Terraformable Ammonia Worlds",
                CodexPlanetEntry::TerraformableWaterWorlds => "Terraformable Water Worlds",

                // Terraformable no atmosphere planets
                CodexPlanetEntry::TerraformableHighMetalContentNoAtmosphere =>
                    "Terraformable High Metal Content Planet, No Atmosphere",
                CodexPlanetEntry::TerraformableIceNoAtmosphere =>
                    "Terraformable Ice Planet, No Atmosphere",
                CodexPlanetEntry::TerraformableMetalRichNoAtmosphere =>
                    "Terraformable Metal-Rich Planet, No Atmosphere",
                CodexPlanetEntry::TerraformableRockyIceNoAtmosphere =>
                    "Terraformable Rocky Ice Planet, No Atmosphere",
                CodexPlanetEntry::TerraformableRockyNoAtmosphere =>
                    "Terraformable Rocky Planet, No Atmosphere",

                // Terraformable terrestrial planets
                CodexPlanetEntry::TerraformableHighMetalContent =>
                    "Terraformable High Metal Content Planet",
                CodexPlanetEntry::TerraformableIce => "Terraformable Ice Planet",
                CodexPlanetEntry::TerraformableMetalRich => "Terraformable Metal-Rich Planet",
                CodexPlanetEntry::TerraformableRockyIce => "Terraformable Rocky Ice Planet",
                CodexPlanetEntry::TerraformableRocky => "Terraformable Rocky Planet",

                #[cfg(feature = "allow-unknown")]
                CodexPlanetEntry::Unknown(unknown) =>
                    return write!(f, "Unknown planet codex entry: {}", unknown),
            }
        )
    }
}

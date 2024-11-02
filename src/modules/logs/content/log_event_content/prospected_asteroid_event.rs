//! Fired when a prospector limpet is used on an asteroid.

use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when a prospector limpet is used on an asteroid.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ProspectedAsteroidEvent {
    /// List of commodities that are available in the given asteroid.
    pub materials: Vec<ProspectedAsteroidEventMaterial>,

    /// The yield of the asteroid. How much of the materials are available.
    pub content: ProspectedAsteroidEventContent,

    /// Localized name of the yield of the asteroid.
    #[serde(rename = "Content_Localised")]
    pub content_localized: Option<String>,

    /// Whether the prospected asteroid contains a mother lode.
    pub motherlode_material: Option<Commodity>,

    /// Localized name of the mother lode commodity, if any.
    #[serde(rename = "MotherlodeMaterial_Localised")]
    pub motherlode_material_localized: Option<String>,

    /// Percentage of total yield remaining for the asteroid.
    pub remaining: f32,
}

/// Commodity entry for a prospected asteroid.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ProspectedAsteroidEventMaterial {
    /// The commodity available in the asteroid.
    pub name: Commodity,

    /// Localized name of the commodity available.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The factor the commodity is available in the given asteroid.
    pub proportion: f32,
}

/// Indicative yield for the asteroid.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ProspectedAsteroidEventContent {
    #[serde(rename = "$AsteroidMaterialContent_High;")]
    HighMaterialContent,

    #[serde(rename = "$AsteroidMaterialContent_Medium;")]
    MediumMaterialContent,

    #[serde(rename = "$AsteroidMaterialContent_Low;")]
    LowMaterialContent,
}

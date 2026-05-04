use serde::{Deserialize, Serialize};

use crate::modules::exobiology::{Genus, Species, Variant};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SellOrganicDataEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub bio_data: Vec<SellOrganicDataEventBioData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SellOrganicDataEventBioData {
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: Option<String>,

    pub species: Species,

    #[serde(rename = "Species_Localised")]
    pub species_localized: Option<String>,

    pub variant: Option<Variant>,

    #[serde(rename = "Variant_Localised")]
    pub variant_localized: Option<String>,

    pub value: u64,
    pub bonus: u64,
}

use serde::{Serialize, Deserialize};

use crate::modules::models::exploration::genus::Genus;
use crate::modules::models::exploration::species::Species;
use crate::modules::models::exploration::variant::Variant;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellOrganicDataEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub bio_data: Vec<SellOrganicDataEventBioData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SellOrganicDataEventBioData {
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: Option<String>,

    pub species: Species,

    #[serde(rename = "Species_Localised")]
    pub species_localized: Option<String>,

    pub variant: Variant,

    #[serde(rename = "Variant_Localised")]
    pub variant_localized: Option<String>,

    pub value: u64,
    pub bonus: u64,
}

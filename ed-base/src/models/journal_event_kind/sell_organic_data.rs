use crate::models::journal_event_kind::shared::exploration::genus::Genus;
use crate::models::journal_event_kind::shared::exploration::species::Species;
use crate::models::journal_event_kind::shared::exploration::variant::Variant;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SellOrganicDataEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub bio_data: Vec<SellOrganicDataEventBioData>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SellOrganicDataEventBioData {
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: String,

    pub species: Species,

    #[serde(rename = "Species_Localised")]
    pub species_localized: String,

    pub variant: Variant,

    #[serde(rename = "Variant_Localised")]
    pub variant_localized: String,

    pub value: u64,
    pub bonus: u64,
}

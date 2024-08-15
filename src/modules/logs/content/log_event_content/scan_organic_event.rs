use serde::{Deserialize, Serialize};

use crate::modules::exobiology::{Genus, Species, Variant};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanOrganicEvent {
    /// Possible values seem to be "Sample", "Analyze", "Log". It seems that the first scan for
    /// a bio species uses `Log`, then the second scan uses `Sample` The third one logs one `Sample`
    /// entry and immediately followed with `Analyze`. The contents seem to be the same.
    pub scan_type: ScanOrganicEventScanType,
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: Option<String>,

    pub species: Species,

    #[serde(rename = "Species_Localised")]
    pub species_localized: Option<String>,

    pub variant: Option<Variant>,

    #[serde(rename = "Variant_Localised")]
    pub variant_localized: Option<String>,

    pub system_address: u64,
    pub body: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Ord, PartialOrd, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ScanOrganicEventScanType {
    Sample,
    Analyse,
    Log,
}

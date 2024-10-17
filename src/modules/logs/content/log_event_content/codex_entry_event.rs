use serde::{Deserialize, Serialize};

use crate::modules::exploration::CodexEntry;
use crate::modules::galaxy::Region;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEvent {
    #[serde(rename = "EntryID")]
    pub entry_id: u64,
    pub name: CodexEntry,

    #[serde(rename = "Name_Localised")]
    pub name_localized: String,
    pub category: CodexEntryEventCategory,

    #[serde(rename = "Category_Localised")]
    pub category_localized: String,
    pub sub_category: CodexEntryEventSubcategory,

    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localized: String,
    pub region: Region,

    #[serde(rename = "Region_Localised")]
    pub region_localized: String,

    pub system: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: Option<u8>,

    // TODO sometimes missing, sometimes an empty string
    pub nearest_destination: Option<String>,

    /// Filled for example for exobiology entries.
    pub planetary_location: Option<CodexEntryEventPlanetaryLocation>,

    #[serde(default)]
    pub is_new_entry: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexEntryEventCategory {
    #[serde(rename = "$Codex_Category_Biology;")]
    Biology,

    #[serde(rename = "$Codex_Category_StellarBodies;")]
    StellarBodies,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexEntryEventSubcategory {
    #[serde(rename = "$Codex_SubCategory_Organic_Structures;")]
    OrganicStructures,

    #[serde(rename = "$Codex_SubCategory_Stars;")]
    Stars,

    #[serde(rename = "$Codex_SubCategory_Terrestrials;")]
    Terrestrials,

    #[serde(rename = "$Codex_SubCategory_Gas_Giants;")]
    GasGiants,

    #[serde(rename = "$Codex_SubCategory_Geology_and_Anomalies;")]
    GeologyAndAnomalies,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEventPlanetaryLocation {
    pub latitude: f32,
    pub longitude: f32,
}

use crate::models::journal_event_content::shared::exploration::codex_entry::CodexEntry;
use crate::models::journal_event_content::shared::galaxy::region::Region;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEvent {
    #[serde(rename = "EntryID")]
    pub entry_id: u64,
    pub name: CodexEntry,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub category: CodexEntryEventCategory,

    #[serde(rename = "Category_Localised")]
    pub category_localised: String,
    pub sub_category: CodexEntryEventSubcategory,

    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localised: String,
    pub region: Region,

    #[serde(rename = "Region_Localised")]
    pub region_localised: String,

    pub system: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,

    // TODO sometimes missing, sometimes an empty string
    pub nearest_destination: Option<String>,

    /// Filled for example for exobiology entries.
    pub planetary_location: Option<CodexEntryEventPlanetaryLocation>,

    #[serde(default)]
    pub is_new_entry: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CodexEntryEventCategory {
    #[serde(rename = "$Codex_Category_Biology;")]
    Biology,

    #[serde(rename = "$Codex_Category_StellarBodies;")]
    StellarBodies,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CodexEntryEventSubcategory {
    #[serde(rename = "$Codex_SubCategory_Organic_Structures;")]
    OrganicStructures,

    #[serde(rename = "$Codex_SubCategory_Stars;")]
    Stars,

    #[serde(rename = "$Codex_SubCategory_Terrestrials;")]
    Terrestrials,

    #[serde(rename = "$Codex_SubCategory_Gas_Giants;")]
    GasGiants,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEventPlanetaryLocation {
    pub latitude: f32,
    pub longitude: f32,
}

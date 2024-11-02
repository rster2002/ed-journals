//! Fired when the player registers a new entry in their codex.

use serde::{Deserialize, Serialize};

use crate::modules::exploration::CodexEntry;
use crate::modules::galaxy::Region;

/// Fired when the player registers a new entry in their codex.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEvent {
    /// The id of the entry.
    #[serde(rename = "EntryID")]
    pub entry_id: u64,

    /// The name of the codex entry.
    pub name: CodexEntry,

    /// The localized name of the codex entry.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The category for this entry.
    pub category: CodexEntryEventCategory,

    /// The localized name of the category for this entry.
    #[serde(rename = "Category_Localised")]
    pub category_localized: Option<String>,

    /// The subcategory for this entry.
    pub sub_category: CodexEntryEventSubcategory,

    /// The localized name of the subcategory for this entry.
    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localized: Option<String>,

    /// The region the codex entry has been scanned.
    pub region: Region,

    /// The localized name of the region the codex entry has been scanned.
    #[serde(rename = "Region_Localised")]
    pub region_localized: Option<String>,

    /// The name of the system the codex entry has been scanned.
    pub system: String,

    /// The address of the system the codex entry has been scanned.
    pub system_address: u64,

    /// The body id of the subject of the entry or has been scanned on.
    #[serde(rename = "BodyID")]
    pub body_id: Option<u8>,

    // TODO sometimes missing, sometimes an empty string
    pub nearest_destination: Option<String>,

    /// Filled for exobiology entries detailing where on the planet the entry has been scanned.
    pub planetary_location: Option<CodexEntryEventPlanetaryLocation>,

    /// Whether the player has never encountered this entry before, including in other regions.
    #[serde(default)]
    pub is_new_entry: bool,
}

/// Category for a given codex entry.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexEntryEventCategory {
    /// Includes things like genuses and species.
    #[serde(rename = "$Codex_Category_Biology;")]
    Biology,

    /// Includes things like star and planet classes.
    #[serde(rename = "$Codex_Category_StellarBodies;")]
    StellarBodies,

    /// Includes entries related to guardians.
    #[serde(rename = "$Codex_Category_Civilisations;")]
    Civilisations,

    /// Unknown codex category.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

/// Subcategory for a given codex entry.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexEntryEventSubcategory {
    /// Subcategory related to organic structures like genuses and species.
    #[serde(rename = "$Codex_SubCategory_Organic_Structures;")]
    OrganicStructures,

    /// Subcategory related to star bodies.
    #[serde(rename = "$Codex_SubCategory_Stars;")]
    Stars,

    /// Subcategory related to planets.
    #[serde(rename = "$Codex_SubCategory_Terrestrials;")]
    Terrestrials,

    /// Subcategory related to gas giants.
    #[serde(rename = "$Codex_SubCategory_Gas_Giants;")]
    GasGiants,

    #[serde(rename = "$Codex_SubCategory_Geology_and_Anomalies;")]
    GeologyAndAnomalies,

    /// Subcategory related to Thargoids and their activity.
    #[serde(rename = "$Codex_SubCategory_Thargoid;")]
    Thargoid,

    /// Unknown subcategory.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

/// Details about the location of the codex entry on a planet.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEventPlanetaryLocation {
    /// The latitude of the codex entry on a planet.
    pub latitude: f32,

    /// The longitude of the codex entry on a planet.
    pub longitude: f32,
}

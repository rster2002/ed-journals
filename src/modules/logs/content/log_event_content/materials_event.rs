//! Fired during startup containing a list of material the player currently has.

use serde::{Deserialize, Serialize};

use crate::modules::materials::Material;

/// Fired during startup containing a list of material the player currently has.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MaterialsEvent {
    /// List of raw materials the player has.
    #[serde(rename = "Raw")]
    pub raw: Vec<MaterialEventEntry>,

    /// List of encoded materials the player has.
    #[serde(rename = "Encoded")]
    pub encoded: Vec<MaterialEventEntry>,

    /// List of manufactured materials the player has.
    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<MaterialEventEntry>,
}

/// Entry for a given material that the player has.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MaterialEventEntry {
    /// The kind of material the player has inventory for.
    #[serde(rename = "Name")]
    pub name: Material,

    /// The amount of the given material the player has.
    #[serde(rename = "Count")]
    pub count: u16,
}

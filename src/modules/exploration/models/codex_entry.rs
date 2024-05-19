use serde::{Serialize, Deserialize};
use crate::modules::exobiology::{Genus, Species, Variant};
use crate::modules::exploration::StarClassCodexEntry;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexEntry {
    #[serde(untagged)]
    Genus(Genus),

    #[serde(untagged)]
    Species(Species),

    #[serde(untagged)]
    Variant(Variant),

    #[serde(untagged)]
    StarClass(StarClassCodexEntry),

    #[serde(untagged)]
    Unknown(String),
}

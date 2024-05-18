use serde::{Serialize, Deserialize};

use crate::modules::models::exploration::genus::Genus;
use crate::modules::models::exploration::species::Species;
use crate::modules::models::exploration::star_class_codex_entry::StarClassCodexEntry;
use crate::modules::models::exploration::variant::Variant;

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

use serde::Deserialize;

use crate::modules::shared::exploration::genus::Genus;
use crate::modules::shared::exploration::species::Species;
use crate::modules::shared::exploration::star_class_codex_entry::StarClassCodexEntry;
use crate::modules::shared::exploration::variant::Variant;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

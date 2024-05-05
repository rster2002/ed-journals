use serde::Deserialize;

use crate::models::journal_event_content::shared::exploration::genus::Genus;
use crate::models::journal_event_content::shared::exploration::species::Species;
use crate::models::journal_event_content::shared::exploration::star_class_codex_entry::StarClassCodexEntry;
use crate::models::journal_event_content::shared::exploration::variant::Variant;

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

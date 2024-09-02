use crate::exploration::models::planet_class_codex_entry::PlanetClassCodexEntry;
use crate::modules::exobiology::{Genus, Species, Variant};
use crate::modules::exploration::StarClassCodexEntry;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Codex entry name.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexEntry {
    #[serde(rename = "$Codex_Ent_Neutron_Stars_Name;")]
    NeutronStars,

    #[serde(untagged)]
    PlanetClass(PlanetClassCodexEntry),

    /// Genus codex entry registered when scanning the first genus in the given region.
    #[serde(untagged)]
    Genus(Genus),

    /// Genus codex entry registered when scanning the first species in the given region.
    #[serde(untagged)]
    Species(Species),

    /// Genus codex entry registered when scanning the first variant in the given region.
    #[serde(untagged)]
    Variant(Variant),

    /// Genus codex entry registered when scanning the star class in the given region.
    #[serde(untagged)]
    StarClass(StarClassCodexEntry),

    /// Unknown codex entry.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for CodexEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CodexEntry::NeutronStars => write!(f, "Neutron Star"),
            CodexEntry::PlanetClass(planet_class) => write!(f, "{}", planet_class),
            CodexEntry::Genus(genus) => write!(f, "{}", genus),
            CodexEntry::Species(species) => write!(f, "{}", species),
            CodexEntry::Variant(variant) => write!(f, "{}", variant),
            CodexEntry::StarClass(star_class) => write!(f, "{}", star_class),

            #[cfg(feature = "allow-unknown")]
            CodexEntry::Unknown(unknown) => write!(f, "Unknown: '{}'", unknown),
        }
    }
}

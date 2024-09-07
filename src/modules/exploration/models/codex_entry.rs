use crate::exploration::models::planet_class_codex_entry::PlanetClassCodexEntry;
use crate::modules::exobiology::{Genus, Species, Variant};
use crate::modules::exploration::CodexStarClassEntry;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;
use crate::exploration::models::codex_anomaly_entry::CodexAnomalyEntry;
use crate::exploration::models::codex_geological_entry::CodexGeologicalEntry;
use crate::exploration::models::codex_guardian_entry::CodexGuardianEntry;
use crate::exploration::models::codex_thargoid_entry::CodexThargoidEntry;
use crate::from_str_deserialize_impl;

/// Codex entry name.
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexEntry {
    #[serde(untagged)]
    PlanetClass(PlanetClassCodexEntry),

    #[serde(untagged)]
    Geological(CodexGeologicalEntry),

    #[serde(untagged)]
    Anomalous(CodexAnomalyEntry),

    #[serde(untagged)]
    Thargoid(CodexThargoidEntry),

    #[serde(untagged)]
    Guardian(CodexGuardianEntry),

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
    StarClass(CodexStarClassEntry),

    /// Unknown codex entry.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum CodexEntryError {
    #[error("Unknown codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexEntry {
    type Err = CodexEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(entry) = PlanetClassCodexEntry::from_str(s) {
            return Ok(CodexEntry::PlanetClass(entry));
        }

        if let Ok(entry) = CodexGeologicalEntry::from_str(s) {
            return Ok(CodexEntry::Geological(entry));
        }

        if let Ok(entry) = CodexAnomalyEntry::from_str(s) {
            return Ok(CodexEntry::Anomalous(entry));
        }

        if let Ok(entry) = CodexThargoidEntry::from_str(s) {
            return Ok(CodexEntry::Thargoid(entry));
        }

        if let Ok(entry) = Genus::from_str(s) {
            return Ok(CodexEntry::Genus(entry));
        }

        if let Ok(entry) = Species::from_str(s) {
            return Ok(CodexEntry::Species(entry));
        }

        if let Ok(entry) = Variant::from_str(s) {
            return Ok(CodexEntry::Variant(entry));
        }

        if let Ok(entry) = CodexStarClassEntry::from_str(s) {
            return Ok(CodexEntry::StarClass(entry));
        }

        Err(CodexEntryError::UnknownEntry(s.to_string()))
    }
}

from_str_deserialize_impl!(CodexEntry);

impl Display for CodexEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // CodexEntry::NeutronStars => write!(f, "Neutron Star"),
            // CodexEntry::BlackHoles => write!(f, "Black Hole"),
            CodexEntry::Geological(geological) => write!(f, "{}", geological),
            CodexEntry::Anomalous(anomalous) => write!(f, "{}", anomalous),
            CodexEntry::Thargoid(targoid) => write!(f, "{}", targoid),
            CodexEntry::PlanetClass(planet_class) => write!(f, "{}", planet_class),
            CodexEntry::Genus(genus) => write!(f, "{}", genus),
            CodexEntry::Species(species) => write!(f, "{}", species),
            CodexEntry::Variant(variant) => write!(f, "{}", variant),
            CodexEntry::StarClass(star_class) => write!(f, "{}", star_class),
            CodexEntry::Guardian(guardian) => write!(f, "{}", guardian),

            #[cfg(feature = "allow-unknown")]
            CodexEntry::Unknown(unknown) => write!(f, "Unknown: '{}'", unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::exploration::CodexEntry;

    #[test]
    fn codex_entries_are_parsed_correctly() {
        let content = include_str!("zz_codex_entries");
        let lines = content.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            let result = serde_json::from_value::<CodexEntry>(Value::String(line.to_string()));

            if result.is_err() {
                dbg!(&line, &result);
            }

            assert!(result.is_ok());
        }
    }
}

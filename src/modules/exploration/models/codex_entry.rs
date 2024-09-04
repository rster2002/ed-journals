use crate::exploration::models::planet_class_codex_entry::PlanetClassCodexEntry;
use crate::modules::exobiology::{Genus, Species, Variant};
use crate::modules::exploration::StarClassCodexEntry;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::exploration::models::codex_anomaly_entry::CodexAnomalyEntry;
use crate::exploration::models::codex_geological_entry::CodexGeologicalEntry;

/// Codex entry name.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexEntry {
    #[serde(untagged)]
    PlanetClass(PlanetClassCodexEntry),

    #[serde(untagged)]
    Geological(CodexGeologicalEntry),

    #[serde(untagged)]
    Anomalous(CodexAnomalyEntry),

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
            // CodexEntry::NeutronStars => write!(f, "Neutron Star"),
            // CodexEntry::BlackHoles => write!(f, "Black Hole"),
            CodexEntry::Geological(geological) => write!(f, "{}", geological),
            CodexEntry::Anomalous(anomalous) => write!(f, "{}", anomalous),
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

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::exploration::CodexEntry;

    #[test]
    fn codex_entries_are_parsed_correctly() {
        let cases = [
            "$Codex_Ent_TRF_Water_Worlds_Name;",
            "$Codex_Ent_Standard_Water_Worlds_Name;",
            "$Codex_Ent_Standard_Ter_High_Metal_Content_Name;",
            "$Codex_Ent_Standard_Metal_Rich_No_Atmos_Name;",
            "$Codex_Ent_Standard_Ice_No_Atmos_Name;",
            "$Codex_Ent_Standard_Sudarsky_Class_III_Name;",
            "$Codex_Ent_Earth_Likes_Name;",
            "$Codex_Ent_Standard_Giant_With_Ammonia_Life_Name;",
            "$Codex_Ent_Neutron_Stars_Name;",
        ];

        for case in cases {
            let result = serde_json::from_value::<CodexEntry>(Value::String(case.to_string()));

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}

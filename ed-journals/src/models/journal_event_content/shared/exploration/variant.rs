use crate::from_str_deserialize_impl;
use crate::models::journal_event_content::shared::exploration::species::Species;
use crate::models::journal_event_content::shared::exploration::variant_color::{
    VariantColor, VariantColorError,
};
use crate::models::journal_event_content::shared::exploration::variant_source::{
    VariantSource, VariantSourceError,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub species: Species,
    pub color: VariantColor,
}

#[derive(Debug, Error)]
pub enum VariantError {
    #[error("Failed to parse species: {0}")]
    FailedToParseSpecies(#[source] serde_json::Error),

    #[error(transparent)]
    VariantSourceError(#[from] VariantSourceError),

    #[error(transparent)]
    VariantColorError(#[from] VariantColorError),

    #[error("Failed to parse variant: '{0}'")]
    FailedToParse(String),
}

const VARIANT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^(\$Codex_Ent_([a-zA-Z]+)_(\d+))_([a-zA-Z]+)(_Name;)?$"#).unwrap());

impl FromStr for Variant {
    type Err = VariantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = VARIANT_REGEX.captures(s) else {
            return Err(VariantError::FailedToParse(s.to_string()));
        };

        let species = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str();

        let species = format!("{}_Name;", species)
            .parse()
            .map_err(VariantError::FailedToParseSpecies)?;

        let variant_source: VariantSource = captures
            .get(4)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        let color = (&species, &variant_source).try_into()?;

        Ok(Variant { species, color })
    }
}

from_str_deserialize_impl!(Variant);

#[cfg(test)]
mod tests {
    use crate::models::journal_event_content::shared::exploration::species::Species;
    use crate::models::journal_event_content::shared::exploration::variant::Variant;
    use crate::models::journal_event_content::shared::exploration::variant_color::VariantColor;
    use std::str::FromStr;

    #[test]
    fn variant_test_cases_are_processed_correctly() {
        let test_cases = [
            (
                "$Codex_Ent_Tussocks_01_F_Name;",
                Variant {
                    species: Species::TussockPennata,
                    color: VariantColor::Yellow,
                },
            ),
            (
                "$Codex_Ent_Stratum_07_T_Name;",
                Variant {
                    species: Species::StratumTectonicas,
                    color: VariantColor::Grey,
                },
            ),
            (
                "$Codex_Ent_Recepta_03_Yttrium_Name;",
                Variant {
                    species: Species::ReceptaConditivus,
                    color: VariantColor::Green,
                },
            ),
            (
                "$Codex_Ent_Fonticulus_02_M_Name;",
                Variant {
                    species: Species::FonticuluaCampestris,
                    color: VariantColor::Amethyst,
                },
            ),
            (
                "$Codex_Ent_Bacterial_05_Tellurium_Name;",
                Variant {
                    species: Species::BacteriumVesicula,
                    color: VariantColor::Red,
                },
            ),
        ];

        for (case, expected) in test_cases {
            let result = Variant::from_str(case);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}

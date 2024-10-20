use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::deserialize_in_order_impl;
use crate::modules::exobiology::{
    Species, VariantColor, VariantColorError, VariantSource, VariantSourceError,
};

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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

lazy_static! {
    static ref VARIANT_REGEX: Regex =
        Regex::new(r#"^(\$Codex_Ent_([a-zA-Z]+)_(\d+))_([a-zA-Z]+)(_Name;)?$"#).unwrap();
    static ref NONE_VARIANTS: HashSet<Species> = HashSet::from([
        Species::AnemoneLuteolum,
        Species::AnemonePuniceum,
        Species::AnemoneBlatteumBioluminescent,
        Species::AnemoneRubeumBioluminescent,
        Species::AnemonePrasinumBioluminescent,
        Species::AnemoneRoseumBioluminescent,
        Species::CrystallineShards,
        Species::BarkMound,
        Species::BrainTreeRoseum,
        Species::BrainTreeGypseeum,
        Species::BrainTreeOstrinum,
        Species::BrainTreeViride,
        Species::BrainTreeLividum,
        Species::BrainTreeAureum,
        Species::BrainTreePuniceum,
        Species::BrainTreeLindigoticum,
    ]);
}

impl FromStr for Variant {
    type Err = VariantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(species) = Species::from_str(s) {
            return Ok(Variant {
                species,
                color: VariantColor::None,
            });
        }

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

#[derive(Deserialize)]
struct VariantInput {
    pub species: Species,
    pub color: VariantColor,
}

impl From<VariantInput> for Variant {
    fn from(value: VariantInput) -> Self {
        Variant {
            species: value.species,
            color: value.color,
        }
    }
}

deserialize_in_order_impl!(
    Variant =>
        A # String,
        B ! VariantInput,
);

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let VariantColor::None = self.color {
            return self.species.fmt(f);
        }

        write!(f, "{} - {}", self.species, self.color)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::modules::exobiology::{Species, Variant, VariantColor};

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

    #[test]
    fn variants_test_file_entries_all_parse() {
        let content = include_str!("zz_variants.txt");
        let lines = content.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            dbg!(&line);
            let result = Variant::from_str(line);

            dbg!(&result);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn variants_datadump_test_file_entries_all_parse() {
        let content = include_str!("zz_datamined_variants.txt");
        let lines = content.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            dbg!(&line);
            let result = Variant::from_str(line);

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}

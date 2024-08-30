use std::fmt::{Display, Formatter};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::galaxy::{PlanetClass, PlanetClassError};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlanetClassCodexEntry {
    pub terraformable: bool,
    pub planet_class: PlanetClass,
}

#[derive(Debug, Error)]
pub enum PlanetClassCodexEntryError {
    #[error("Failed to parse planet class codex entry: '{0}'")]
    FailedToParse(String),

    #[error(transparent)]
    PlanetClassError(#[from] PlanetClassError),
}

lazy_static! {
    static ref PLANET_CLASS_CODEX_ENTRY_REGEX: Regex = Regex::new(r#"^\$Codex_Ent_(Standard|TRF)_(.+?)_Name;$"#).unwrap();
}

impl FromStr for PlanetClassCodexEntry {
    type Err = PlanetClassCodexEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PLANET_CLASS_CODEX_ENTRY_REGEX.captures(&s)
            .ok_or(PlanetClassCodexEntryError::FailedToParse(s.to_string()))?;

        let terraformable = captures.get(1)
            .expect("Should have been captured already")
            .as_str();

        let planet_class = captures.get(2)
            .expect("Should have been captured already")
            .as_str();

        Ok(PlanetClassCodexEntry {
            terraformable: terraformable == "TRF",
            planet_class: PlanetClass::from_str(&planet_class)?,
        })
    }
}

impl Display for PlanetClassCodexEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match self.terraformable {
                false => "Standard",
                true => "Terraformable",
            },
            self.planet_class
        )
    }
}

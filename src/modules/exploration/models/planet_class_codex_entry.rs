use crate::from_str_deserialize_impl;
use crate::galaxy::{PlanetClass, PlanetClassError};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct PlanetClassCodexEntry {
    pub terraformable: bool,
    pub has_atmosphere: bool,
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
    static ref PLANET_CLASS_CODEX_ENTRY_REGEX: Regex =
        Regex::new(r#"^\$[cC]odex_[eE]nt(_([sS]tandard|TRF))?(_[tT]er)?_(.+?)(_[nN]o_[aA]tmos)?_[nN]ame;$"#).unwrap();
}

impl FromStr for PlanetClassCodexEntry {
    type Err = PlanetClassCodexEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PLANET_CLASS_CODEX_ENTRY_REGEX
            .captures(s)
            .ok_or(PlanetClassCodexEntryError::FailedToParse(s.to_string()))?;

        let terraformable = captures
            .get(2)
            .is_some_and(|capture| capture.as_str() == "TRF");

        let planet_class = captures
            .get(4)
            .expect("Should have been captured already")
            .as_str();

        let no_atmosphere = captures.get(5).is_some();

        Ok(PlanetClassCodexEntry {
            terraformable,
            has_atmosphere: !no_atmosphere,
            planet_class: PlanetClass::from_str(planet_class)?,
        })
    }
}

from_str_deserialize_impl!(PlanetClassCodexEntry);

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

#[cfg(test)]
mod tests {
    use crate::exploration::PlanetClassCodexEntry;
    use std::str::FromStr;

    #[test]
    fn planet_class_codex_cases_are_parsed_correctly() {
        let cases = [
            "$Codex_Ent_TRF_Water_Worlds_Name;",
            "$Codex_Ent_Standard_Water_Worlds_Name;",
            "$Codex_Ent_Standard_Ter_High_Metal_Content_Name;",
            "$Codex_Ent_Standard_Metal_Rich_No_Atmos_Name;",
            "$Codex_Ent_Standard_Ice_No_Atmos_Name;",
            "$Codex_Ent_Standard_Sudarsky_Class_III_Name;",
            "$Codex_Ent_Earth_Likes_Name;",
            "$Codex_Ent_Standard_Giant_With_Ammonia_Life_Name;",
        ];

        for case in cases {
            let result = PlanetClassCodexEntry::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}

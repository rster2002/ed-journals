use std::fmt::{Display, Formatter};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::galaxy::{PlanetClass, PlanetClassError};

#[derive(Debug, Serialize, Clone, PartialEq)]
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
    static ref PLANET_CLASS_CODEX_ENTRY_REGEX: Regex = Regex::new(r#"^\$Codex_Ent_(Standard|TRF)(_Ter)?_(.+?)(_No_Atmos)?_Name;$"#).unwrap();
}

impl FromStr for PlanetClassCodexEntry {
    type Err = PlanetClassCodexEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PLANET_CLASS_CODEX_ENTRY_REGEX.captures(s)
            .ok_or(PlanetClassCodexEntryError::FailedToParse(s.to_string()))?;

        let terraformable = captures.get(1)
            .expect("Should have been captured already")
            .as_str();

        let planet_class = captures.get(3)
            .expect("Should have been captured already")
            .as_str();

        let no_atmosphere = captures.get(4)
            .is_some();

        Ok(PlanetClassCodexEntry {
            terraformable: terraformable == "TRF",
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
    use std::str::FromStr;
    use crate::exploration::PlanetClassCodexEntry;

    #[test]
    fn planet_class_codex_cases_are_parsed_correctly() {
        let cases = [
            "$Codex_Ent_TRF_Water_Worlds_Name;",
            "$Codex_Ent_Standard_Water_Worlds_Name;",
            "$Codex_Ent_Standard_Ter_High_Metal_Content_Name;",
            "$Codex_Ent_Standard_Metal_Rich_No_Atmos_Name;",
            "$Codex_Ent_Standard_Ice_No_Atmos_Name;",
            "$Codex_Ent_Standard_Sudarsky_Class_III_Name;",
        ];

        for case in cases {
            let result = PlanetClassCodexEntry::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}

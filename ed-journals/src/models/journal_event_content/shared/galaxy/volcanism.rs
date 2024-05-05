use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_content::shared::galaxy::volcanism_type::VolcanismType;

#[derive(Debug, Clone, PartialEq)]
pub struct Volcanism {
    pub kind: VolcanismType,
    pub classification: VolcanismClassification,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VolcanismClassification {
    Minor,
    Normal,
    Major,
}

#[derive(Debug, Error)]
pub enum VolcanismError {
    #[error("Unknown volcanism type: {0}")]
    UnknownVolcanismType(#[source] serde_json::Error),

    #[error("Failed to parse volcanism: '{0}'")]
    FailedToParse(String),
}

const VOLCANISM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("(^minor |^major |^)([a-zA-Z ]+) volcanism$").unwrap());

impl FromStr for Volcanism {
    type Err = VolcanismError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO I would prefer to move this to a impl FromStr for Option<Volcanism> but this is not
        //  possible. Some other way needs to be found.
        if s == "" {
            return Ok(Volcanism {
                kind: VolcanismType::None,
                classification: VolcanismClassification::Normal,
            })
        }

        let Some(captures) = VOLCANISM_REGEX.captures(s) else {
            return Err(VolcanismError::FailedToParse(s.to_string()));
        };

        let kind = captures.get(2)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| VolcanismError::UnknownVolcanismType(e))?;

        let classification = match captures.get(1)
            .expect("Should have been captured already")
            .as_str() {
            "minor " => VolcanismClassification::Minor,
            "major " => VolcanismClassification::Major,
            _ => VolcanismClassification::Normal,
        };

        Ok(Volcanism {
            kind,
            classification,
        })
    }
}

from_str_deserialize_impl!(Volcanism);

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::models::journal_event_content::shared::galaxy::volcanism::{Volcanism, VolcanismClassification};
    use crate::models::journal_event_content::shared::galaxy::volcanism_type::VolcanismType;

    #[test]
    fn volcanism_test_cases_are_parsed_correctly() {
        let test_cases = [
            ("minor silicate vapour geysers volcanism", Volcanism {
                kind: VolcanismType::SilicateVapourGeysers,
                classification: VolcanismClassification::Minor,
            }),
            ("major rocky magma volcanism", Volcanism {
                kind: VolcanismType::RockyMagma,
                classification: VolcanismClassification::Major,
            }),
            ("minor metallic magma volcanism", Volcanism {
                kind: VolcanismType::MetallicMagma,
                classification: VolcanismClassification::Minor,
            }),
        ];

        for (case, expected) in test_cases {
            let result = Volcanism::from_str(case);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}

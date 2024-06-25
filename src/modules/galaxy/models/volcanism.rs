use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::galaxy::VolcanismType;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Volcanism {
    pub kind: VolcanismType,
    pub classification: VolcanismClassification,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
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

lazy_static! {
    static ref VOLCANISM_REGEX: Regex =
        Regex::new("(^[mM]inor |^[mM]ajor |^)([a-zA-Z ]+?)( volcanism)?$").unwrap();
}

impl FromStr for Volcanism {
    type Err = VolcanismError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO I would prefer to move this to a impl FromStr for Option<Volcanism> but this is not
        //  possible. Some other way needs to be found.
        if s.is_empty() {
            return Ok(Volcanism {
                kind: VolcanismType::None,
                classification: VolcanismClassification::Normal,
            });
        }

        let Some(captures) = VOLCANISM_REGEX.captures(s) else {
            return Err(VolcanismError::FailedToParse(s.to_string()));
        };

        let kind = captures
            .get(2)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(VolcanismError::UnknownVolcanismType)?;

        let classification = match captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
        {
            "minor " | "Minor " => VolcanismClassification::Minor,
            "major " | "Major " => VolcanismClassification::Major,
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

    use crate::modules::galaxy::{Volcanism, VolcanismClassification, VolcanismType};

    #[test]
    fn volcanism_test_cases_are_parsed_correctly() {
        let test_cases = [
            (
                "minor silicate vapour geysers volcanism",
                Volcanism {
                    kind: VolcanismType::SilicateVapourGeysers,
                    classification: VolcanismClassification::Minor,
                },
            ),
            (
                "major rocky magma volcanism",
                Volcanism {
                    kind: VolcanismType::RockyMagma,
                    classification: VolcanismClassification::Major,
                },
            ),
            (
                "minor metallic magma volcanism",
                Volcanism {
                    kind: VolcanismType::MetallicMagma,
                    classification: VolcanismClassification::Minor,
                },
            ),
        ];

        for (case, expected) in test_cases {
            let result = Volcanism::from_str(case);

            dbg!(case);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}

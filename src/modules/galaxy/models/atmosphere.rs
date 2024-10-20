use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::galaxy::models::atmosphere_type::AtmosphereTypeError;
use crate::modules::galaxy::AtmosphereType;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Atmosphere {
    pub hot: bool,
    pub density: AtmosphereDensity,
    pub kind: AtmosphereType,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AtmosphereDensity {
    Thick,
    Normal,
    Thin,
}

#[derive(Debug, Error)]
pub enum AtmosphereError {
    #[error(transparent)]
    UnknownAtmosphereType(AtmosphereTypeError),

    #[error("Failed to parse atmosphere: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref ATMOSPHERE_REGEX: Regex =
        Regex::new(r#"^([hH]ot )?(([tT]hin|[tT]hick) )?([a-zA-Z -]+?)?( atmosphere)?$"#).unwrap();
}

impl FromStr for Atmosphere {
    type Err = AtmosphereError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Atmosphere {
                hot: false,
                density: AtmosphereDensity::Normal,
                kind: AtmosphereType::None,
            });
        }

        let Some(captures) = ATMOSPHERE_REGEX.captures(s) else {
            return Err(AtmosphereError::FailedToParse(s.to_string()));
        };

        let hot = captures.get(1).is_some();

        let density = match captures.get(3) {
            Some(capture) => match capture.as_str() {
                "thin" | "Thin" => AtmosphereDensity::Thin,
                "thick" | "Thick" => AtmosphereDensity::Thick,
                _ => AtmosphereDensity::Normal,
            },
            None => AtmosphereDensity::Normal,
        };

        let kind = captures
            .get(4)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(AtmosphereError::UnknownAtmosphereType)?;

        Ok(Atmosphere { kind, hot, density })
    }
}

from_str_deserialize_impl!(Atmosphere);

#[cfg(test)]
mod tests {
    use crate::galaxy::Atmosphere;
    use std::str::FromStr;

    #[test]
    fn atmosphere_test_cases_are_parsed_correctly() {
        let test_cases = [
            "argon rich atmosphere",
            "nitrogen atmosphere",
            "Thin Carbon dioxide-rich",
            "None",
            "thick  atmosphere", // Because of course this is in there
            "",                  // WHY?!
        ];

        for case in test_cases {
            let result = Atmosphere::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}

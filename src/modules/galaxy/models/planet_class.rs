use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum PlanetClass {
    MetalRichBody,
    HighMetalContentBody,
    RockyBody,
    IcyBody,
    RockyIceBody,
    EarthlikeBody,
    WaterWorld,
    AmmoniaWorld,
    WaterGiant,
    WaterGiantWithLife,
    GasGiantWithWaterBasedLife,
    GasGiantWithAmmoniaBasedLife,

    ClassIGasGiant,

    SudarskyClassIGasGiant,
    SudarskyClassIiGasGiant,
    SudarskyClassIiiGasGiant,
    SudarskyClassIvGasGiant,
    SudarskyClassVGasGiant,
    HeliumRichGasGiant,
    HeliumGasGiant,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum PlanetClassError {
    #[error("Failed to parse planet class: '{0}'")]
    FailedToParse(String),

    #[error("Unknown planet class: '{0}'")]
    UnknownPlanetClass(String),
}

lazy_static! {
    static ref PLANET_CLASS_REGEX: Regex = Regex::new("^([a-zA-Z0-9 ]+?)( body| world)?$").unwrap();
}

impl FromStr for PlanetClass {
    type Err = PlanetClassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = PLANET_CLASS_REGEX.captures(s) else {
            return Err(PlanetClassError::FailedToParse(s.to_string()));
        };

        let class_name: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match class_name {
            "metal rich" => PlanetClass::MetalRichBody,
            "high metal content" => PlanetClass::HighMetalContentBody,
            "rocky" => PlanetClass::RockyBody,
            "icy" => PlanetClass::IcyBody,
            "rocky ice" => PlanetClass::RockyIceBody,
            "earthlike" => PlanetClass::EarthlikeBody,
            "water" => PlanetClass::WaterWorld,
            "ammonia" => PlanetClass::AmmoniaWorld,
            "water giant" => PlanetClass::WaterGiant,
            "water giant with life" => PlanetClass::WaterGiantWithLife,
            "gas giant with water based life" => PlanetClass::GasGiantWithWaterBasedLife,
            "gas giant with ammonia based life" => PlanetClass::GasGiantWithAmmoniaBasedLife,

            "class i gas giant" => PlanetClass::ClassIGasGiant,

            "sudarsky class i gas giant" => PlanetClass::SudarskyClassIGasGiant,
            "sudarsky class ii gas giant" => PlanetClass::SudarskyClassIiGasGiant,
            "sudarsky class iii gas giant" => PlanetClass::SudarskyClassIiiGasGiant,
            "sudarsky class iv gas giant" => PlanetClass::SudarskyClassIvGasGiant,
            "sudarsky class v gas giant" => PlanetClass::SudarskyClassVGasGiant,
            "helium rich gas giant" => PlanetClass::HeliumRichGasGiant,
            "helium gas giant" => PlanetClass::HeliumGasGiant,
            _ => return Err(PlanetClassError::UnknownPlanetClass(class_name.to_string())),
        })
    }
}

from_str_deserialize_impl!(PlanetClass);

impl PlanetClass {
    /// Returns the base exploration value of the star planet class.
    pub fn base_value(&self) -> u64 {
        match self {
            PlanetClass::MetalRichBody => 21_790,
            PlanetClass::AmmoniaWorld => 96_932,
            PlanetClass::SudarskyClassIGasGiant => 1_656,
            PlanetClass::SudarskyClassIiGasGiant => 9_654,
            PlanetClass::HighMetalContentBody => 9_654,
            PlanetClass::WaterWorld => 64_831,
            PlanetClass::EarthlikeBody => 64_831,
            _ => 300,
        }
    }

    /// Returns the bonus exploration value if the planet is terraformable.
    pub fn terraformable_bonus(&self) -> u64 {
        match self {
            PlanetClass::MetalRichBody => 65_631,
            PlanetClass::AmmoniaWorld => 0,
            PlanetClass::SudarskyClassIGasGiant => 0,
            PlanetClass::SudarskyClassIiGasGiant => 100_677,
            PlanetClass::HighMetalContentBody => 100_677,
            PlanetClass::WaterWorld => 116_295,
            PlanetClass::EarthlikeBody => 116_295,
            _ => 93_328,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::galaxy::PlanetClass;

    #[test]
    fn planet_class_test_cases_are_parsed_correctly() {
        let test_cases = [
            "rocky body",
        ];

        for case in test_cases {
            let result = PlanetClass::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}

use crate::from_str_deserialize_impl;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;
use thiserror::Error;

/// Mission_Sightseeing_Criminal_BOOM
#[derive(Debug, Clone, PartialEq)]
pub struct SightseeingMission {
    // TODO use enum
    pub citizen: String,

    // TODO use enum
    pub state: String,
}

#[derive(Debug, Error)]
pub enum SightSeeingMissionError {
    #[error("Failed to parse sightseeing mission: '{0}'")]
    FailedToParse(String),
}

const SIGHTSEEING_MISSION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^Mission_Sightseeing_([a-zA-Z]+?)_([A-Z]+?)(_name)?$"#).unwrap());

impl FromStr for SightseeingMission {
    type Err = SightSeeingMissionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = SIGHTSEEING_MISSION_REGEX.captures(s) else {
            return Err(SightSeeingMissionError::FailedToParse(s.to_string()));
        };

        let citizen = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        let state = captures
            .get(2)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        Ok(SightseeingMission { citizen, state })
    }
}

from_str_deserialize_impl!(SightseeingMission);

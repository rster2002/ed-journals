use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::civilization::faction_state::FactionState;

/// Mission_Sightseeing_Criminal_BOOM
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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

const SIGHTSEEING_MISSION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^Mission_Sightseeing_([a-zA-Z]+?)_([A-Z]+?)(_name)?$"#).unwrap());

impl FromStr for SightseeingMission {
    type Err = SightSeeingMissionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = SIGHTSEEING_MISSION_REGEX.captures(s) else {
            return Err(SightSeeingMissionError::FailedToParse(s.to_string()));
        };

        let citizen = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        let state = captures.get(2)
            .expect("Should have been captured already")
            .as_str()
            .to_string();

        Ok(SightseeingMission  {
            citizen,
            state,
        })
    }
}

from_str_deserialize_impl!(SightseeingMission);



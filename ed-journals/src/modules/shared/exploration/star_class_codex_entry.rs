use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::shared::galaxy::star_class::{StarClass, StarClassError};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct StarClassCodexEntry(pub StarClass);

#[derive(Debug, Error)]
pub enum StarClassCodexEntryError {
    #[error("Failed to parse star class codex entry: '{0}'")]
    FailedToParse(String),

    #[error(transparent)]
    FailedToParseStarClass(#[from] StarClassError),
}

lazy_static! {
    static ref STAR_CLASS_CODEX_ENTRY_REGEX: Regex =
        Regex::new(r#"^\$Codex_Ent_(\w+)_Type_Name;$"#).unwrap();
}

impl FromStr for StarClassCodexEntry {
    type Err = StarClassCodexEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = STAR_CLASS_CODEX_ENTRY_REGEX.captures(s) else {
            return Err(StarClassCodexEntryError::FailedToParse(s.to_string()));
        };

        Ok(StarClassCodexEntry(
            captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()?,
        ))
    }
}

from_str_deserialize_impl!(StarClassCodexEntry);

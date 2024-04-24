use std::num::ParseIntError;
use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::ship_module::module_class::{ModuleClass, ModuleClassError};
use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::internal_module::InternalModule;

mod internal_module;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShipInternalModule {
    pub module: InternalModule,
    pub size: u8,
    pub class: ModuleClass,
}

#[derive(Debug, Error)]
pub enum ShipInternalModuleParseError {
    #[error("Failed to parse module: {0}]")]
    FailedToParseModule(#[source] serde_json::Error),

    #[error("Failed to parse size: {0}")]
    FailedToParseSize(#[from] ParseIntError),

    #[error("Failed to parse class number: {0}")]
    FailedToParseClassNumber(#[source] ParseIntError),

    #[error(transparent)]
    UnknownClass(#[from] ModuleClassError),

    #[error("Failed to parse internal ship module: '{0}'")]
    FailedToParse(String),
}

const SHIP_INTERNAL_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^int_(\w+)_size(1|2|3|4|5|6|7|8)_class(1|2|3|4|5)$"#).unwrap());

impl FromStr for ShipInternalModule {
    type Err = ShipInternalModuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = SHIP_INTERNAL_MODULE_REGEX.captures(s) else {
            return Err(ShipInternalModuleParseError::FailedToParse(s.to_string()));
        };

        let module = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| ShipInternalModuleParseError::FailedToParseModule(e))?;

        let size = captures.get(2)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        let class = captures.get(3)
            .expect("Should have been captured already")
            .as_str()
            .parse::<u8>()
            .map_err(|e| ShipInternalModuleParseError::FailedToParseClassNumber(e))?
            .try_into()?;

        Ok(ShipInternalModule {
            module,
            size,
            class,
        })
    }
}

from_str_deserialize_impl!(ShipInternalModule);

// int_engine_size5_class5

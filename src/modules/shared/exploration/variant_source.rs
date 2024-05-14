use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use thiserror::Error;

use crate::modules::shared::galaxy::star_class::StarClass;
use crate::modules::shared::materials::material::Material;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum VariantSource {
    StarClass(StarClass),
    Material(Material),
}

#[derive(Debug, Error)]
pub enum VariantSourceError {
    #[error("Failed to parse variant source: {0}")]
    FailedToParse(#[source] serde_json::Error),

    #[error(
        "The provided material cannot be used as a variant source as it's not a raw material."
    )]
    NotARawMaterial,
}

impl FromStr for VariantSource {
    type Err = VariantSourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variant_source = serde_json::from_value(Value::String(s.to_ascii_lowercase()))
            .map_err(VariantSourceError::FailedToParse)?;

        if let VariantSource::Material(material) = &variant_source {
            if !material.is_raw() {
                return Err(VariantSourceError::NotARawMaterial);
            }
        }

        Ok(variant_source)
    }
}

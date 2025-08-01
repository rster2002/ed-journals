use serde::{Deserialize, Serialize};

use crate::modules::exobiology::Genus;
use crate::modules::exploration::PlanetarySignalType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFoundEvent {
    pub body_name: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,

    #[serde(default)]
    pub signals: Vec<SAASignalsFoundEventSignal>,

    #[serde(default)]
    pub genuses: Vec<SAASignalsFoundEventGenus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFoundEventSignal {
    #[serde(rename = "Type")]
    pub kind: PlanetarySignalType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
    pub count: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFoundEventGenus {
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: Option<String>,
}

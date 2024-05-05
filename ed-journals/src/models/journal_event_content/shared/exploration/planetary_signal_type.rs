use std::fmt::{Display, Formatter};
use serde::Deserialize;

use crate::models::journal_event_content::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum PlanetarySignalType {
    #[serde(rename = "$SAA_SignalType_Human;")]
    Human,

    #[serde(rename = "$SAA_SignalType_Biological;")]
    Biological,

    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,

    #[serde(rename = "$SAA_SignalType_Thargoid;")]
    Thargoid,

    #[serde(rename = "$SAA_SignalType_Guardian;")]
    Guardian,

    #[serde(rename = "$SAA_SignalType_Other;")]
    Other,

    #[serde(untagged)]
    Commodity(Commodity),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for PlanetarySignalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PlanetarySignalType::Human => "Human",
            PlanetarySignalType::Biological => "Biological",
            PlanetarySignalType::Geological => "Geological",
            PlanetarySignalType::Thargoid => "Thargoid",
            PlanetarySignalType::Guardian => "Guardian",
            PlanetarySignalType::Other => "Other",
            PlanetarySignalType::Commodity(commodity) => return commodity.fmt(f),

            #[cfg(not(feature = "strict"))]
            PlanetarySignalType::Unknown(unknown) => return write!(f, "Unknown planetary signal: {}", unknown),
        })
    }
}

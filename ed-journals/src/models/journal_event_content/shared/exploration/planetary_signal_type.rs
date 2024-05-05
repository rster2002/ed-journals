use crate::models::journal_event_content::shared::trading::commodity::Commodity;
use serde::Deserialize;

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

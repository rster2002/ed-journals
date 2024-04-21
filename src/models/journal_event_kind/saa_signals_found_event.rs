use serde::Deserialize;
use crate::models::journal_event_kind::shared::exploration::genus::Genus;
use crate::models::journal_event_kind::shared::exploration::planetary_signal_type::PlanetarySignalType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFoundEvent {
    pub body_name: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub signals: Vec<SAAScanCompleteEventSignal>,
    pub genuses: Vec<SAAScanCompleteEventGenus>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SAAScanCompleteEventSignal {
    #[serde(rename = "Type")]
    pub kind: PlanetarySignalType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: String,
    pub count: u8,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SAAScanCompleteEventGenus {
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: String,
}

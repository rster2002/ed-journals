use serde::Deserialize;

use crate::models::journal_event_kind::shared::exploration::planetary_signal_type::PlanetarySignalType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalsEvent {
    pub body_name: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub system_address: u64,
    pub signals: Vec<FSSBodySignalEventSignal>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalEventSignal {
    #[serde(rename = "Type")]
    pub kind: PlanetarySignalType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: String,
    pub count: u8,
}

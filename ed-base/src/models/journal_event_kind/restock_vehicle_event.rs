use crate::models::journal_event_kind::shared::ship::fighter_loadout::FighterLoadout;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct RestockVehicleEvent {
    #[serde(rename = "Type")]
    pub kind: RestockVehicleEventType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
    pub loadout: FighterLoadout,

    // TODO figure out when this is [None]
    #[serde(rename = "ID")]
    pub id: Option<u8>,
    pub cost: u64,
    pub count: u8,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RestockVehicleEventType {
    #[serde(rename = "testbuggy")]
    SRV,

    #[serde(rename = "independent_fighter")]
    TaipanFighter,
}

use serde::Deserialize;
use crate::models::journal_event_kind::shared::civilization::economy::Economy;
use crate::models::journal_event_kind::shared::civilization::faction_state::FactionState;
use crate::models::journal_event_kind::shared::civilization::government::Government;
use crate::models::journal_event_kind::shared::station::station_info::StationInfo;
use crate::models::journal_event_kind::shared::station::station_service::StationService;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ApproachSettlementEvent {
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localized: String,

    #[serde(flatten)]
    pub station_info: Option<StationInfo>,

    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub body_name: String,
    pub latitude: f32,
    pub longitude: f32,
}



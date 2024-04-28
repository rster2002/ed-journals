use crate::models::journal_event_kind::shared::civilization::economy::Economy;
use crate::models::journal_event_kind::shared::civilization::faction_state::FactionState;
use crate::models::journal_event_kind::shared::civilization::government::Government;
use crate::models::journal_event_kind::shared::station::station_service::StationService;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StationInfo {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_faction: StationInfoFaction,
    pub station_government: Government,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localized: String,
    pub station_services: Vec<StationService>,
    pub station_economy: Economy,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localized: String,
    pub station_economies: Vec<StationInfoEconomy>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StationInfoFaction {
    pub name: String,
    pub faction_state: Option<FactionState>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StationInfoEconomy {
    pub name: Economy,

    #[serde(rename = "Name_Localised")]
    pub name_localized: String,
    pub proportion: f32,
}

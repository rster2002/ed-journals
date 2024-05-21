use serde::{Deserialize, Serialize};

use crate::modules::civilization::{Economy, FactionState, Government};
use crate::modules::station::StationService;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StationInfo {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_faction: StationInfoFaction,
    pub station_government: Government,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localized: Option<String>,
    pub station_services: Vec<StationService>,
    pub station_economy: Economy,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localized: Option<String>,
    pub station_economies: Vec<StationInfoEconomy>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StationInfoFaction {
    pub name: String,
    pub faction_state: Option<FactionState>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StationInfoEconomy {
    pub name: Economy,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub proportion: f32,
}

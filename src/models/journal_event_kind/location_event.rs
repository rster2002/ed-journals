use serde::Deserialize;
use crate::models::journal_event_kind::shared::station::station_service::StationService;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEvent {
    // TODO check when this is filled
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f32>,
    pub docked: bool,

    // TODO check when this is filled
    pub station_name: Option<String>,

    // TODO check when this is filled
    pub station_type: Option<String>,

    // TODO check when this is filled
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    // TODO check when this is filled
    pub station_faction: Option<LocationEventStationFaction>,

    // TODO check when this is filled
    pub station_government: Option<String>,

    // TODO check when this is filled
    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: Option<String>,

    // TODO check when this is filled
    pub station_allegiance: Option<String>,

    // TODO check when this is filled
    pub station_services: Option<Vec<StationService>>,

    // TODO check when this is filled
    pub station_economy: Option<String>,

    // TODO check when this is filled
    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: Option<String>,
    pub station_economies: Option<Vec<LocationEventStationEconomy>>,

    pub taxi: bool,
    pub multicrew: bool,
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: [f32; 3],

    pub system_allegiance: String,
    pub system_economy: String,

    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: String,
    pub system_second_economy: String,

    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,
    pub system_government: String,

    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,
    pub system_security: String,

    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,
    pub population: u64,
    pub body: String,

    // TODO check when this is filled
    pub body_id: Option<u32>,
    pub body_type: String,

    // TODO check when this is filled
    #[serde(default)]
    pub factions: Vec<LocationEventFaction>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventStationFaction {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventStationEconomy {
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub proportion: f32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventFaction {
    pub name: String,
    pub faction_state: String,
    pub government: String,
    pub influence: f32,
    pub allegiance: String,
    pub happiness: String,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: String,
    pub my_reputation: f32,

    #[serde(default)]
    pub active_states: Vec<LocationEventFactionActiveState>,

    #[serde(default)]
    pub recovering_states: Vec<LocationEventFactionRecoveringState>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventFactionActiveState {
    pub state: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventFactionRecoveringState {
    pub state: String,
    pub trend: u32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventSystemFaction {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventConflict {
    pub war_type: String,
    pub status: String,
    pub faction_1: LocationEventConflictFaction,
    pub faction_2: LocationEventConflictFaction,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LocationEventConflictFaction {
    pub name: String,
    pub stake: String,
    pub won_days: u32,
}

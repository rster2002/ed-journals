use serde::{Deserialize, Serialize};

use crate::modules::civilization::{Economy, FactionState, Government};
use crate::modules::station::StationService;

/// Information about the station or 'market'. This includes information about locations where the
/// player can dock and can access various services. Note however that while the game uses the term
/// 'station' in the fields, the actual location doesn't always have to be an actual space station
/// and this model can also be set at settlements and fleet carriers.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StationInfo {
    /// The id of the market. This is what the game uses behind the scenes instead of the actual
    /// location and this is what all things like commodities and missions are linked to.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The primary faction of the market.
    pub station_faction: StationInfoFaction,

    /// The primary government for this market. This is determined by the primary faction.
    pub station_government: Government,

    /// The localized name of the primary government.
    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localized: Option<String>,

    /// A list of services which are available at the market.
    pub station_services: Vec<StationService>,

    /// The primary economy which the market is part of.
    pub station_economy: Economy,

    /// The localized name of the economy the market is part of.
    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localized: Option<String>,

    /// Even through there is a primary economy, some markets might be part of multiple economies.
    pub station_economies: Vec<StationInfoEconomy>,
}

/// Information about a faction for the market.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StationInfoFaction {
    /// The name of the faction.
    pub name: String,

    /// The current state the faction is in, if any.
    pub faction_state: Option<FactionState>,
}

/// Information about an economy.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StationInfoEconomy {
    /// The type of economy.
    pub name: Economy,

    /// The localized name of the economy.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The proportion of the economy. This is a number between 0 and 1 where it indicates how big
    /// of a factor the economy is at the given market.
    pub proportion: f32,
}

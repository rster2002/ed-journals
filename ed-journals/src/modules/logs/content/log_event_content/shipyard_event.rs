use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockingTimeoutEvent {
    pub station_name: String,
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockingDeniedEvent {
    pub station_name: String,
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub reason: DockingDeniedReason,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase", untagged)]
pub enum DockingDeniedReason {
    NoSpace,
    TooLarge,
    Hostile,
    Offences,
    Distance,
    ActiveFighter,
    NoReason,
    Other(String),
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEvent {
    pub station_name: String,
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub landing_pads: DockingRequestedEventLandingPads,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockingRequestedEventLandingPads {
    pub small: u8,
    pub medium: u8,
    pub large: u8,
}

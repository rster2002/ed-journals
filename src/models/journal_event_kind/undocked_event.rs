use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct UndockedEvent {
    pub station_name: String,

    // TODO replace this with an enum
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub taxi: bool,
    pub multicrew: bool,
}

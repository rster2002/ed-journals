use serde::Deserialize;
use crate::models::journal_event_kind::shared::station::station_type::StationType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct UndockedEvent {
    pub station_name: String,

    // TODO replace this with an enum
    pub station_type: StationType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub taxi: bool,
    pub multicrew: bool,
}

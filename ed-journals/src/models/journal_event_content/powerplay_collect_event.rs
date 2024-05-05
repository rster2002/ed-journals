use serde::Deserialize;
use crate::journal_event_content::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayCollectEvent {
    pub power: String,

    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
}

use crate::modules::shared::trading::commodity::Commodity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayCollectEvent {
    pub power: String,

    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
}

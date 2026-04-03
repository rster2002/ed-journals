use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayCollectEvent {
    pub power: String,

    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
}

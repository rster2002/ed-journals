use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayDeliverEvent {
    pub power: String,

    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
}

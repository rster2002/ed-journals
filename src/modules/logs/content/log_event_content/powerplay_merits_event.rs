use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayMeritsEvent {
    pub power: String,

    pub total_merits: u32,
    pub merits_gained: u16,
}

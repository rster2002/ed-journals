use serde::{Deserialize, Serialize};

use crate::modules::materials::{Material, MaterialCategory};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ScientificResearchEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub name: Material,
    pub category: MaterialCategory,
    pub count: u16,
}
